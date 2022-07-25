use crate::prelude::*;
use rome_formatter::{write, CstFormatContext};

use rome_js_syntax::JsIfStatementFields;

use rome_js_syntax::{JsAnyStatement, JsIfStatement};

#[derive(Debug, Clone, Default)]
pub struct FormatJsIfStatement;

impl FormatNodeRule<JsIfStatement> for FormatJsIfStatement {
    fn fmt_fields(&self, node: &JsIfStatement, f: &mut JsFormatter) -> FormatResult<()> {
        let JsIfStatementFields {
            if_token,
            l_paren_token,
            test,
            r_paren_token,
            consequent,
            else_clause,
        } = node.as_fields();

        let consequent = consequent?;

        write!(
            f,
            [
                if_token.format(),
                space_token(),
                format_delimited(&l_paren_token?, &test.format(), &r_paren_token?)
                    .soft_block_indent(),
                FormatIfElseConsequentBlock::from(&consequent),
            ]
        )?;

        if let Some(else_clause) = else_clause {
            // Place the `else` clause on its own line if the consequent has any trailing line comments
            // or the else clause any leading comments
            // Used to format
            // ```
            // if (test) {
            //   ...
            // }
            // // comment
            // else {}
            // ```
            let comments = f.context().comments();
            if comments
                .trailing_comments(consequent.syntax())
                .iter()
                .chain(comments.leading_comments(else_clause.syntax()).iter())
                .any(|comment| comment.kind().is_line())
            {
                write!(f, [hard_line_break()])?;
            }

            write!(f, [else_clause.format()])?;
        }

        Ok(())
    }
}

pub struct FormatIfElseConsequentBlock<'a>(&'a JsAnyStatement);

impl<'a> From<&'a JsAnyStatement> for FormatIfElseConsequentBlock<'a> {
    fn from(stmt: &'a JsAnyStatement) -> Self {
        FormatIfElseConsequentBlock(stmt)
    }
}

impl Format<JsFormatContext> for FormatIfElseConsequentBlock<'_> {
    fn fmt(&self, f: &mut Formatter<JsFormatContext>) -> FormatResult<()> {
        let stmt = &self.0;

        match stmt {
            JsAnyStatement::JsBlockStatement(block) => {
                write!(f, [space_token(), block.format()])
            }
            // If the body is an empty statement, force a line break to ensure behavior
            // is coherent with `is_non_collapsable_empty_block`
            JsAnyStatement::JsEmptyStatement(stmt) => {
                write!(f, [stmt.format(), hard_line_break()])
            }
            stmt => {
                write!(
                    f,
                    [
                        space_token(),
                        token("{"),
                        block_indent(&stmt.format()),
                        token("}")
                    ]
                )
            }
        }
    }
}
