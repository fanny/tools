use std::{borrow, collections::BTreeMap};

use rome_rowan::{AstNode, Language, RawSyntaxKind, SyntaxKind, SyntaxNode};
use rustc_hash::FxHashSet;

use crate::{
    context::RuleContext,
    matcher::{GroupKey, MatchQueryParams},
    query::{QueryKey, QueryMatch, Queryable},
    signals::RuleSignal,
    AnalysisFilter, QueryMatcher, Rule, RuleGroup, RuleKey, SignalEntry,
};

/// Defines all the phases that the [RuleRegistry] supports.
#[repr(usize)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Phases {
    Syntax = 0,
    Semantic = 1,
}

/// Defines which phase a rule will run. This will be defined
/// by the set of services a rule demands.
pub trait Phase {
    fn phase() -> Phases;
}

/// If a rule do not need any service it can run on the syntax phase.
impl Phase for () {
    fn phase() -> Phases {
        Phases::Syntax
    }
}

/// The rule registry holds type-erased instances of all active analysis rules
/// for each phase.
/// What defines a phase is the set of services that a phase offers. Currently
/// we have:
/// - Syntax Phase: No services are offered, thus its rules can be run immediately;
/// - Semantic Phase: Offers the semantic model, thus these rules can only run
/// after the "SemanticModel" is ready, which demands a whole transverse of the parsed tree.
#[derive(Default)]
pub struct RuleRegistry<L: Language> {
    /// Stores metadata information for all the rules in the registry, sorted
    /// alphabetically
    metadata: BTreeMap<MetadataKey, MetadataValue>,
    /// Holds a collection of rules for each phase.
    phase_rules: [PhaseRules<L>; 2],
}

/// Holds a collection of rules for each phase.
#[derive(Default)]
struct PhaseRules<L: Language> {
    /// Holds a collection of rules for each [SyntaxKind] node type that has
    /// lint rules associated with it
    ast_rules: Vec<SyntaxKindRules<L>>,
    control_flow: Vec<RegistryRule<L>>,
    /// Holds a list of states for all the rules in this phase
    rule_states: Vec<RuleState<L>>,
}

impl<L: Language + Default> RuleRegistry<L> {
    pub fn push_group<G: RuleGroup<Language = L>>(&mut self, filter: &AnalysisFilter) {
        G::push_rules(self, filter);
    }

    /// Add the rule `R` to the list of rules stores in this registry instance
    pub fn push<G, R>(&mut self)
    where
        G: RuleGroup<Language = L> + 'static,
        R: Rule + 'static,
        R::Query: Queryable<Language = L>,
        <R::Query as Queryable>::Output: Clone,
    {
        let phase = R::phase() as usize;
        let phase = &mut self.phase_rules[phase];

        let rule = RegistryRule::new::<G, R>(phase.rule_states.len());

        match <R::Query as Queryable>::KEY {
            QueryKey::Syntax(key) => {
                // Iterate on all the SyntaxKind variants this node can match
                for kind in key.iter() {
                    // Convert the numerical value of `kind` to an index in the
                    // `nodes` vector
                    let RawSyntaxKind(index) = kind.to_raw();
                    let index = usize::from(index);

                    // Ensure the vector has enough capacity by inserting empty
                    // `SyntaxKindRules` as required
                    if phase.ast_rules.len() <= index {
                        phase.ast_rules.resize_with(index + 1, SyntaxKindRules::new);
                    }

                    // Insert a handle to the rule `R` into the `SyntaxKindRules` entry
                    // corresponding to the SyntaxKind index
                    let node = &mut phase.ast_rules[index];
                    node.rules.push(rule);
                }
            }
            QueryKey::ControlFlowGraph => {
                phase.control_flow.push(rule);
            }
        }

        phase.rule_states.push(RuleState::default());

        self.metadata.insert(
            MetadataKey {
                inner: (G::NAME, R::NAME),
            },
            MetadataValue {
                version: R::VERSION,
                docs: R::DOCS,
                deprecated: R::DEPRECATED,
                recommended: R::RECOMMENDED,
            },
        );
    }

    /// Returns an iterator over the name and documentation of all active rules
    /// in this instance of the registry
    pub fn metadata(self) -> impl Iterator<Item = RuleMetadata> {
        self.metadata.into_iter().map(|(key, value)| {
            let (group, name) = key.inner;
            RuleMetadata {
                group,
                name,
                docs: value.docs,
                version: value.version,
                deprecated: value.deprecated,
                recommended: value.recommended,
            }
        })
    }
}

impl<L: Language> QueryMatcher<L> for RuleRegistry<L> {
    fn find_group(&self, group: &str) -> Option<GroupKey> {
        self.metadata.keys().find_map(|key| {
            let (key, _) = key.inner;
            if key == group {
                Some(GroupKey::new(key))
            } else {
                None
            }
        })
    }

    fn find_rule(&self, group: &str, rule: &str) -> Option<RuleKey> {
        let (key, _) = self.metadata.get_key_value(&(group, rule))?;
        Some(key.into_rule_key())
    }

    fn match_query(&mut self, mut params: MatchQueryParams<L>) {
        let phase = &mut self.phase_rules[params.phase as usize];

        let rules = match &params.query {
            QueryMatch::Syntax(node) => {
                // Convert the numerical value of the SyntaxKind to an index in the
                // `syntax` vector
                let RawSyntaxKind(kind) = node.kind().to_raw();
                let kind = usize::from(kind);

                // Lookup the syntax entry corresponding to the SyntaxKind index
                match phase.ast_rules.get_mut(kind) {
                    Some(entry) => &mut entry.rules,
                    None => return,
                }
            }
            QueryMatch::ControlFlowGraph(..) => &mut phase.control_flow,
        };

        // Run all the rules registered to this QueryMatch
        for rule in rules {
            let state = &mut phase.rule_states[rule.state_index];
            (rule.run)(&mut params, state);
        }
    }
}

/// [SyntaxKindRules] holds a collection of [Rule]s that match a specific [SyntaxKind] value
struct SyntaxKindRules<L: Language> {
    rules: Vec<RegistryRule<L>>,
}

impl<L: Language> SyntaxKindRules<L> {
    fn new() -> Self {
        Self { rules: Vec::new() }
    }
}

pub(crate) type RuleLanguage<R> = QueryLanguage<<R as Rule>::Query>;
pub(crate) type QueryLanguage<N> = <N as Queryable>::Language;
pub(crate) type NodeLanguage<N> = <N as AstNode>::Language;

pub(crate) type RuleRoot<R> = LanguageRoot<RuleLanguage<R>>;
pub type LanguageRoot<L> = <L as Language>::Root;

/// Key struct for a rule in the metadata map, sorted alphabetically
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct MetadataKey {
    inner: (&'static str, &'static str),
}

impl MetadataKey {
    fn into_rule_key(self) -> RuleKey {
        let (group, rule) = self.inner;
        RuleKey::new(group, rule)
    }
}

impl<'a> borrow::Borrow<(&'a str, &'a str)> for MetadataKey {
    fn borrow(&self) -> &(&'a str, &'a str) {
        &self.inner
    }
}

struct MetadataValue {
    version: &'static str,
    docs: &'static str,
    deprecated: Option<&'static str>,
    recommended: bool,
}

/// Metadata entry for a rule in the registry
pub struct RuleMetadata {
    pub group: &'static str,
    pub name: &'static str,
    pub docs: &'static str,
    pub version: &'static str,
    pub deprecated: Option<&'static str>,
    pub recommended: bool,
}

/// Internal representation of a single rule in the registry
#[derive(Copy, Clone)]
pub struct RegistryRule<L: Language> {
    run: RuleExecutor<L>,
    state_index: usize,
}

/// Internal state for a given rule
#[derive(Default)]
struct RuleState<L: Language> {
    suppressions: RuleSuppressions<L>,
}

/// Set of nodes this rule has suppressed from matching its query
#[derive(Default)]
pub struct RuleSuppressions<L: Language> {
    inner: FxHashSet<SyntaxNode<L>>,
}

impl<L: Language> RuleSuppressions<L> {
    /// Supress query matching for the given node
    pub fn suppress_node(&mut self, node: SyntaxNode<L>) {
        self.inner.insert(node);
    }
}

/// Executor for rule as a generic function pointer
type RuleExecutor<L> = fn(&mut MatchQueryParams<L>, &mut RuleState<L>);

impl<L: Language + Default> RegistryRule<L> {
    fn new<G, R>(state_index: usize) -> Self
    where
        G: RuleGroup<Language = L> + 'static,
        R: Rule + 'static,
        R::Query: Queryable<Language = L> + 'static,
        <R::Query as Queryable>::Output: Clone,
    {
        /// Generic implementation of RuleExecutor for any rule type R
        fn run<G, R>(
            params: &mut MatchQueryParams<RuleLanguage<R>>,
            state: &mut RuleState<RuleLanguage<R>>,
        ) where
            G: RuleGroup + 'static,
            R: Rule + 'static,
            R::Query: 'static,
            <R::Query as Queryable>::Output: Clone,
        {
            if let QueryMatch::Syntax(node) = &params.query {
                if state.suppressions.inner.contains(node) {
                    return;
                }
            }

            // SAFETY: The rule should never get executed in the first place
            // if the query doesn't match
            let query_result = <R::Query as Queryable>::unwrap_match(&params.query);
            let ctx = match RuleContext::new(&query_result, params.root, params.services) {
                Ok(ctx) => ctx,
                Err(_) => return,
            };

            for result in R::run(&ctx) {
                let text_range =
                    R::text_range(&ctx, &result).unwrap_or_else(|| params.query.text_range());

                R::suppressed_nodes(&ctx, &result, &mut state.suppressions);

                let signal = Box::new(RuleSignal::<G, R>::new(
                    params.file_id,
                    params.root,
                    query_result.clone(),
                    result,
                    params.services,
                ));

                params.signal_queue.push(SignalEntry {
                    signal,
                    rule: RuleKey::rule::<G, R>(),
                    text_range,
                });
            }
        }

        Self {
            run: run::<G, R>,
            state_index,
        }
    }
}
