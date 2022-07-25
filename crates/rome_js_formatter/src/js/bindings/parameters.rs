use crate::prelude::*;

use rome_formatter::cst_builders::format_dangling_trivia;
use rome_formatter::write;
use rome_js_syntax::JsParameters;
use rome_js_syntax::JsParametersFields;

#[derive(Debug, Clone, Default)]
pub struct FormatJsParameters;

impl FormatNodeRule<JsParameters> for FormatJsParameters {
    fn fmt_fields(&self, node: &JsParameters, f: &mut JsFormatter) -> FormatResult<()> {
        let JsParametersFields {
            l_paren_token,
            items,
            r_paren_token,
        } = node.as_fields();

        let r_paren_token = r_paren_token?;

        write!(f, [l_paren_token.format()])?;

        if items.is_empty() {
            write!(f, [format_dangling_trivia(&r_paren_token)])?;
        } else {
            write!(
                f,
                [
                    soft_block_indent(&items.format()),
                    if_group_fits_on_line(&line_suffix_boundary())
                ]
            )?;
        }

        write!(f, [r_paren_token.format()])
    }
}
