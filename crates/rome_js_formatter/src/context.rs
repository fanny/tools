use rome_formatter::printer::PrinterOptions;
use rome_formatter::{
    CommentKind, CommentPosition, CommentStyle, Comments, CstFormatContext, DecoratedComment,
    FormatContext, IndentStyle, LineWidth,
};
use rome_js_syntax::suppression::{parse_suppression_comment, SuppressionCategory};

use rome_js_syntax::{
    JsAnyClass, JsArrayHole, JsBlockStatement, JsCallArgumentList, JsCallArguments,
    JsConditionalExpression, JsFunctionBody, JsImportAssertionEntry, JsLanguage,
    JsNamedImportSpecifiers, JsObjectMemberList, JsParameters, JsPropertyObjectMember,
    JsStaticMemberExpression, JsSyntaxKind, SourceType, TsConditionalType, TsEnumDeclaration,
    TsInterfaceDeclaration,
};
use rome_rowan::{
    AstNode, AstNodeList, AstSeparatedList, Direction, SyntaxElement, SyntaxTriviaPieceComments,
};
use std::fmt;
use std::fmt::Debug;
use std::rc::Rc;
use std::str::FromStr;

#[derive(Debug, Clone, Default)]
pub struct JsFormatContext {
    /// The indent style.
    indent_style: IndentStyle,

    /// What's the max width of a line. Defaults to 80.
    line_width: LineWidth,

    /// The style for quotes. Defaults to double.
    quote_style: QuoteStyle,

    /// Information relative to the current file
    source_type: SourceType,

    /// The comments of the nodes and tokens in the program.
    comments: Rc<Comments<JsLanguage>>,
}

impl JsFormatContext {
    pub fn new(source_type: SourceType) -> Self {
        Self {
            source_type,
            ..JsFormatContext::default()
        }
    }

    pub fn with_indent_style(mut self, indent_style: IndentStyle) -> Self {
        self.indent_style = indent_style;
        self
    }

    pub fn with_line_width(mut self, line_width: LineWidth) -> Self {
        self.line_width = line_width;
        self
    }

    pub fn with_quote_style(mut self, quote_style: QuoteStyle) -> Self {
        self.quote_style = quote_style;
        self
    }

    pub fn with_source_type(mut self, source_type: SourceType) -> Self {
        self.source_type = source_type;
        self
    }

    pub fn line_width(&self) -> LineWidth {
        self.line_width
    }

    pub fn quote_style(&self) -> QuoteStyle {
        self.quote_style
    }

    pub fn source_type(&self) -> SourceType {
        self.source_type
    }
}

#[derive(Eq, PartialEq, Debug, Copy, Clone, Hash)]
pub struct TabWidth(u8);

impl From<u8> for TabWidth {
    fn from(value: u8) -> Self {
        TabWidth(value)
    }
}

impl From<TabWidth> for u8 {
    fn from(width: TabWidth) -> Self {
        width.0
    }
}

impl JsFormatContext {
    pub fn tab_width(&self) -> TabWidth {
        match self.indent_style {
            IndentStyle::Tab => 2.into(),
            IndentStyle::Space(quantities) => quantities.into(),
        }
    }
}

impl FormatContext for JsFormatContext {
    fn indent_style(&self) -> IndentStyle {
        self.indent_style
    }

    fn line_width(&self) -> LineWidth {
        self.line_width
    }

    fn as_print_options(&self) -> PrinterOptions {
        PrinterOptions::default()
            .with_indent(self.indent_style)
            .with_print_width(self.line_width)
    }
}

impl fmt::Display for JsFormatContext {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Indent style: {}", self.indent_style)?;
        writeln!(f, "Line width: {}", self.line_width.value())?;
        writeln!(f, "Quote style: {}", self.quote_style)
    }
}

impl CstFormatContext for JsFormatContext {
    type Language = JsLanguage;
    type Style = JsCommentStyle;

    fn comment_style(&self) -> Self::Style {
        JsCommentStyle
    }

    fn comments(&self) -> Rc<Comments<JsLanguage>> {
        self.comments.clone()
    }

    fn with_comments(mut self, comments: Rc<Comments<JsLanguage>>) -> Self {
        self.comments = comments;
        self
    }
}

#[derive(Eq, PartialEq, Copy, Clone, Debug, Default)]
pub struct JsCommentStyle;

impl CommentStyle<JsLanguage> for JsCommentStyle {
    fn is_suppression(&self, text: &str) -> bool {
        parse_suppression_comment(text)
            .flat_map(|suppression| suppression.categories)
            .any(|(category, _)| category == SuppressionCategory::Format)
    }

    fn position_comment(
        &self,
        comment: DecoratedComment<JsLanguage>,
    ) -> CommentPosition<JsLanguage> {
        dbg!(&comment);
        if let Some(following_node) = comment.following_node() {
            match following_node.kind() {
                JsSyntaxKind::JS_SCRIPT | JsSyntaxKind::JS_MODULE => {
                    let first_non_list = following_node
                        .descendants_with_tokens(Direction::Next)
                        // Skip the root itself
                        .skip(1)
                        .find(|element| !element.kind().is_list());

                    return match first_non_list {
                        // Attach any leading comments to the first statement or directive in a module or script to prevent
                        // that a rome-ignore comment on the first statement ignores the whole file.
                        Some(SyntaxElement::Node(node)) => {
                            CommentPosition::Leading { node, comment }
                        }
                        None | Some(SyntaxElement::Token(_)) => CommentPosition::Default(comment),
                    };
                }

                // Move leading comments in front of the `{` inside of the block
                // ```
                // if (test) /* comment */ {
                //  console.log('test');
                // }
                // ```
                //
                // becomes
                // ```
                // if (test) {
                //  /* comment */ console.log('test');
                // }
                // ```
                JsSyntaxKind::JS_BLOCK_STATEMENT => {
                    let block = JsBlockStatement::unwrap_cast(following_node.clone());

                    if let (Ok(_), Ok(r_curly_token)) =
                        (block.l_curly_token(), block.r_curly_token())
                    {
                        return if let Some(fist_statement) = block.statements().first() {
                            CommentPosition::Leading {
                                node: fist_statement.into_syntax(),
                                comment,
                            }
                        } else {
                            CommentPosition::Dangling {
                                token: r_curly_token,
                                comment,
                            }
                        };
                    }
                }

                // Move comments in front of the `{` inside of the function body
                JsSyntaxKind::JS_FUNCTION_BODY => {
                    let function_body = JsFunctionBody::unwrap_cast(following_node.clone());

                    if let (Ok(_), Ok(r_curly_token)) =
                        (function_body.l_curly_token(), function_body.r_curly_token())
                    {
                        let first_directive = function_body
                            .directives()
                            .first()
                            .map(|node| node.into_syntax());
                        let first_statement = function_body
                            .statements()
                            .first()
                            .map(|node| node.into_syntax());
                        return if let Some(first_node) = first_directive.or(first_statement) {
                            CommentPosition::Leading {
                                node: first_node,
                                comment,
                            }
                        } else {
                            CommentPosition::Dangling {
                                token: r_curly_token,
                                comment,
                            }
                        };
                    }
                }

                JsSyntaxKind::JS_ELSE_CLAUSE => {
                    if let Some(preceding_node) = comment.preceding_node() {
                        if JsBlockStatement::can_cast(preceding_node.kind()) {
                            return CommentPosition::Trailing {
                                node: preceding_node.clone(),
                                comment,
                            };
                        }
                    }
                }

                _ => {}
            }
        }

        if comment.following_node().is_none() {
            if let Some(preceding_parent) = comment
                .preceding_node()
                .and_then(|preceding| preceding.parent())
            {
                if JsAnyClass::can_cast(preceding_parent.kind()) {
                    let class = JsAnyClass::unwrap_cast(preceding_parent);

                    if let (Ok(l_curly_token), Ok(r_curly_token)) =
                        (class.l_curly_token(), class.r_curly_token())
                    {
                        if comment.following_token() == &l_curly_token {
                            return if let Some(first_member) = class.members().first() {
                                CommentPosition::Leading {
                                    node: first_member.into_syntax(),
                                    comment,
                                }
                            } else {
                                // attach it to the r_curly token
                                CommentPosition::Dangling {
                                    token: r_curly_token,
                                    comment,
                                }
                            };
                        }
                    }
                } else if TsEnumDeclaration::can_cast(preceding_parent.kind()) {
                    let enum_declaration = TsEnumDeclaration::unwrap_cast(preceding_parent);

                    if let (Ok(l_curly_token), Ok(r_curly_token)) = (
                        enum_declaration.l_curly_token(),
                        enum_declaration.r_curly_token(),
                    ) {
                        if comment.following_token() == &l_curly_token {
                            return match enum_declaration.members().first() {
                                Some(Ok(member)) => CommentPosition::Leading {
                                    node: member.into_syntax(),
                                    comment,
                                },
                                Some(Err(_)) => {
                                    // Don't move the comment, formatting will fail anyway
                                    CommentPosition::Default(comment)
                                }
                                None => CommentPosition::Dangling {
                                    token: r_curly_token,
                                    comment,
                                },
                            };
                        }
                    }
                } else if (!comment.is_trailing_token_trivia() || comment.lines_after() > 0)
                    && TsInterfaceDeclaration::can_cast(preceding_parent.kind())
                {
                    let interface_declaration =
                        TsInterfaceDeclaration::unwrap_cast(preceding_parent);

                    if let (Ok(l_curly_token), Ok(r_curly_token)) = (
                        interface_declaration.l_curly_token(),
                        interface_declaration.r_curly_token(),
                    ) {
                        if comment.following_token() == &l_curly_token {
                            return if let Some(first_member) =
                                interface_declaration.members().first()
                            {
                                CommentPosition::Leading {
                                    node: first_member.into_syntax(),
                                    comment,
                                }
                            } else {
                                CommentPosition::Dangling {
                                    token: r_curly_token,
                                    comment,
                                }
                            };
                        }
                    }
                }
            }
        }

        if let Some(preceding_node) = comment.preceding_node() {
            if JsArrayHole::can_cast(preceding_node.kind()) {
                return CommentPosition::Leading {
                    node: preceding_node.clone(),
                    comment,
                };
            }
        }

        let enclosing_node = comment.enclosing_node();

        // Handle comments before the `}` token
        //
        // ```javascript
        // import {
        //  loooooooooooooooooooong2 as moreeeeeeloooooooooooooooooooong2,
        //  // some other comment
        // } from "test";
        // ```
        if JsNamedImportSpecifiers::can_cast(enclosing_node.kind()) {
            let import_specifiers = JsNamedImportSpecifiers::unwrap_cast(enclosing_node.clone());

            if import_specifiers.r_curly_token().as_ref() == Ok(comment.following_token()) {
                if let Some(Ok(last_specifier)) = import_specifiers.specifiers().iter().last() {
                    return CommentPosition::Trailing {
                        node: last_specifier.into_syntax(),
                        comment,
                    };
                }
            }
        }
        // Push comments between `key` and `colon` or `colon` and `value` to the end to prevent that they
        // get formatted as dangling comments.
        else if JsImportAssertionEntry::can_cast(enclosing_node.kind())
            && comment.following_node().is_none()
        {
            return CommentPosition::Trailing {
                node: enclosing_node.clone(),
                comment,
            };
        }
        // Makes comments that appear in a static member expression (anywhere between object and member)
        // that are not on the same line as the object to become a leading comment of the expression.
        // ```javascript
        // a
        //  /* test */.other -> /* test */ a.other
        // ```
        //
        // It doesn't change comments that are on the same line as the object.
        // ```javascript
        // a /* test */.other -> a /* test */.other
        // ```
        else if JsStaticMemberExpression::can_cast(enclosing_node.kind())
            && !comment.is_trailing_token_trivia()
        {
            return CommentPosition::Leading {
                node: enclosing_node.clone(),
                comment,
            };
        }
        // Make line comments inside of an empty call arguments trailing comments of the call arguments
        // so that they get moved out of the parentheses.
        // ```javascript
        // expect( // remains a dangling comment
        //     // test
        // )
        // ```
        // becomes
        // ```javascript
        // expect(); // remains a dangling comment
        // // test
        // ```
        else if JsCallArguments::can_cast(enclosing_node.kind()) {
            let arguments = JsCallArguments::unwrap_cast(enclosing_node.clone());

            if arguments.r_paren_token().as_ref() == Ok(comment.following_token())
                && arguments.args().is_empty()
                && comment.kind().is_line()
            {
                return CommentPosition::Trailing {
                    node: arguments.into_syntax(),
                    comment,
                };
            }
        }
        // Moves leading comments of `?` and `:` for conditional expressions to the following node if they are part of
        // the tokens leading trivia (not if they are on the same line as the trivia)
        // ```javascript
        // type T4 = test extends string
        //     /* something */
        //     ? unknown : test extends number ? undefined
        //     /* else */
        //   :
        //         undefined;
        // ```
        else if (TsConditionalType::can_cast(enclosing_node.kind())
            || JsConditionalExpression::can_cast(enclosing_node.kind()))
            && !comment.is_trailing_token_trivia()
        {
            if let Some(consequent_or_alternate) = comment.following_token().next_sibling() {
                return CommentPosition::Leading {
                    node: consequent_or_alternate,
                    comment,
                };
            }
        } else if JsParameters::can_cast(enclosing_node.kind()) {
            let parameters = JsParameters::unwrap_cast(enclosing_node);

            if parameters.r_paren_token().as_ref() == Ok(comment.following_token()) {
                if let Some(Ok(last_parameter)) = parameters.items().iter().last() {
                    return CommentPosition::Trailing {
                        node: last_parameter.into_syntax(),
                        comment,
                    };
                }
            }
        }
        //         if JsNamedImportSpecifiers::can_cast(enclosing_node.kind()) {
        //     let import_specifiers = JsNamedImportSpecifiers::unwrap_cast(enclosing_node.clone());
        //
        //     if import_specifiers.r_curly_token().as_ref() == Ok(comment.following_token()) {
        //         if let Some(Ok(last_specifier)) = import_specifiers.specifiers().iter().last() {
        //             return CommentPosition::Trailing {
        //                 node: last_specifier.into_syntax(),
        //                 comment,
        //             };
        //         }
        //     }
        // }

        CommentPosition::Default(comment)
    }

    fn get_comment_kind(&self, comment: &SyntaxTriviaPieceComments<JsLanguage>) -> CommentKind {
        if comment.text().starts_with("/*") {
            if comment.has_newline() {
                CommentKind::Block
            } else {
                CommentKind::InlineBlock
            }
        } else {
            CommentKind::Line
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum QuoteStyle {
    Double,
    Single,
}

impl Default for QuoteStyle {
    fn default() -> Self {
        Self::Double
    }
}

impl FromStr for QuoteStyle {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "double" | "Double" => Ok(Self::Double),
            "single" | "Single" => Ok(Self::Single),
            // TODO: replace this error with a diagnostic
            _ => Err("Value not supported for QuoteStyle"),
        }
    }
}

impl fmt::Display for QuoteStyle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            QuoteStyle::Double => write!(f, "Double Quotes"),
            QuoteStyle::Single => write!(f, "Single Quotes"),
        }
    }
}

impl QuoteStyle {
    pub fn as_char(&self) -> char {
        match self {
            QuoteStyle::Double => '"',
            QuoteStyle::Single => '\'',
        }
    }

    pub fn as_string(&self) -> &str {
        match self {
            QuoteStyle::Double => "\"",
            QuoteStyle::Single => "'",
        }
    }

    /// Returns the quote, prepended with a backslash (escaped)
    pub fn as_escaped(&self) -> &str {
        match self {
            QuoteStyle::Double => "\\\"",
            QuoteStyle::Single => "\\'",
        }
    }

    pub fn as_bytes(&self) -> u8 {
        self.as_char() as u8
    }

    /// Returns the quote in HTML entity
    pub fn as_html_entity(&self) -> &str {
        match self {
            QuoteStyle::Double => "&quot;",
            QuoteStyle::Single => "&apos;",
        }
    }

    /// Given the current quote, it returns the other one
    pub fn other(&self) -> Self {
        match self {
            QuoteStyle::Double => QuoteStyle::Single,
            QuoteStyle::Single => QuoteStyle::Double,
        }
    }
}
