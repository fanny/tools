use crate::jsx::auxiliary::space::JsxSpace;
use crate::prelude::*;
use crate::prelude::{format_args, write};
use rome_formatter::{group_elements, CstFormatContext, FormatResult};
use rome_js_syntax::{
    JsAnyExpression, JsAnyLiteralExpression, JsxExpressionChild, JsxExpressionChildFields,
};

#[derive(Debug, Clone, Default)]
pub struct FormatJsxExpressionChild;

impl FormatNodeRule<JsxExpressionChild> for FormatJsxExpressionChild {
    fn fmt_fields(&self, node: &JsxExpressionChild, f: &mut JsFormatter) -> FormatResult<()> {
        let JsxExpressionChildFields {
            l_curly_token,
            expression,
            r_curly_token,
        } = node.as_fields();

        let l_curly_token = l_curly_token?;
        let r_curly_token = r_curly_token?;
        let comments = f.context().comments();

        let has_comments = expression
            .as_ref()
            .map(|expr| comments.has_comments(expr.syntax()))
            .unwrap_or_else(|| comments.has_dangling_trivia(&r_curly_token));

        // If the expression child is just a string literal with one space in it, it's a JSX space
        if let Some(JsAnyExpression::JsAnyLiteralExpression(
            JsAnyLiteralExpression::JsStringLiteralExpression(string_literal),
        )) = &expression
        {
            let str_token = string_literal.value_token()?;
            let trimmed_text = str_token.text_trimmed();

            let has_space_text = trimmed_text == "' '" || trimmed_text == "\" \"";
            let is_suppressed = f.context().is_suppressed(string_literal.syntax());

            if has_space_text && !has_comments && !is_suppressed {
                return write![
                    f,
                    [
                        format_removed(&l_curly_token),
                        format_replaced(&str_token, &JsxSpace::default()),
                        format_removed(&r_curly_token)
                    ]
                ];
            }
        }

        write!(
            f,
            [group_elements(&format_with(|f| {
                write!(f, [l_curly_token.format()])?;

                if has_comments {
                    write!(
                        f,
                        [soft_block_indent(&format_args![
                            expression.format(),
                            line_suffix_boundary()
                        ])]
                    )?;
                } else {
                    write!(f, [expression.format(), line_suffix_boundary()])?;
                }

                write!(f, [r_curly_token.format()])
            }))]
        )
    }
}
