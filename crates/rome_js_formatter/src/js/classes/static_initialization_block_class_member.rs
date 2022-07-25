use crate::prelude::*;
use rome_formatter::cst_builders::format_dangling_trivia;

use rome_formatter::write;
use rome_js_syntax::JsStaticInitializationBlockClassMember;
use rome_js_syntax::JsStaticInitializationBlockClassMemberFields;

#[derive(Debug, Clone, Default)]
pub struct FormatJsStaticInitializationBlockClassMember;

impl FormatNodeRule<JsStaticInitializationBlockClassMember>
    for FormatJsStaticInitializationBlockClassMember
{
    fn fmt_fields(
        &self,
        node: &JsStaticInitializationBlockClassMember,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        let JsStaticInitializationBlockClassMemberFields {
            static_token,
            l_curly_token,
            statements,
            r_curly_token,
        } = node.as_fields();

        let r_curly_token = r_curly_token?;

        write!(f, [static_token.format(), space_token()])?;

        if statements.is_empty() {
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
                [
                    format_delimited(&l_curly_token?, &statements.format(), &r_curly_token)
                        .block_indent()
                ]
            )
        }
    }
}
