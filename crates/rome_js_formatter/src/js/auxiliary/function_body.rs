use crate::prelude::*;

use rome_formatter::cst_builders::format_dangling_trivia;
use rome_formatter::{format_args, write, CstFormatContext};
use rome_js_syntax::JsFunctionBody;
use rome_js_syntax::JsFunctionBodyFields;

#[derive(Debug, Clone, Default)]
pub struct FormatJsFunctionBody;

impl FormatNodeRule<JsFunctionBody> for FormatJsFunctionBody {
    fn fmt_fields(&self, node: &JsFunctionBody, f: &mut JsFormatter) -> FormatResult<()> {
        let JsFunctionBodyFields {
            l_curly_token,
            directives,
            statements,
            r_curly_token,
        } = node.as_fields();

        let r_curly_token = r_curly_token?;

        if directives.is_empty() && statements.is_empty() {
            write!(
                f,
                [
                    l_curly_token.format(),
                    format_dangling_trivia(&r_curly_token).indented(),
                    r_curly_token.format()
                ]
            )
        } else {
            write!(
                f,
                [format_delimited(
                    &l_curly_token?,
                    &format_args![directives.format(), statements.format()],
                    &r_curly_token,
                )
                .block_indent()]
            )
        }
    }
}
