use crate::prelude::*;
use rome_formatter::cst_builders::format_dangling_trivia;

use rome_formatter::write;
use rome_js_syntax::TsModuleBlock;
use rome_js_syntax::TsModuleBlockFields;

#[derive(Debug, Clone, Default)]
pub struct FormatTsModuleBlock;

impl FormatNodeRule<TsModuleBlock> for FormatTsModuleBlock {
    fn fmt_fields(&self, node: &TsModuleBlock, f: &mut JsFormatter) -> FormatResult<()> {
        let TsModuleBlockFields {
            l_curly_token,
            items,
            r_curly_token,
        } = node.as_fields();

        let r_curly_token = r_curly_token?;

        write!(f, [l_curly_token.format()])?;

        if items.is_empty() {
            write!(f, [format_dangling_trivia(&r_curly_token).indented()])?;
        } else {
            write!(f, [block_indent(&items.format())])?;
        }

        write!(f, [r_curly_token.format()])
    }
}
