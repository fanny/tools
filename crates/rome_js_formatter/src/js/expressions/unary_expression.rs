use crate::prelude::*;
use crate::utils::is_simple_expression;
use rome_formatter::{write, CstFormatContext};

use rome_js_syntax::JsPreUpdateOperator;
use rome_js_syntax::{JsAnyExpression, JsUnaryExpression};
use rome_js_syntax::{JsUnaryExpressionFields, JsUnaryOperator};

#[derive(Debug, Clone, Default)]
pub struct FormatJsUnaryExpression;

impl FormatNodeRule<JsUnaryExpression> for FormatJsUnaryExpression {
    fn fmt_fields(&self, node: &JsUnaryExpression, f: &mut JsFormatter) -> FormatResult<()> {
        let JsUnaryExpressionFields {
            operator_token,
            argument,
        } = node.as_fields();

        let operation = node.operator()?;
        let operator_token = operator_token?;
        let argument = argument?;

        // Insert a space between the operator and argument if its a keyword
        let is_keyword_operator = matches!(
            operation,
            JsUnaryOperator::Delete | JsUnaryOperator::Void | JsUnaryOperator::Typeof
        );

        if is_keyword_operator {
            return write![
                f,
                [operator_token.format(), space_token(), argument.format(),]
            ];
        }

        // Parenthesize the inner expression if it's a binary or pre-update
        // operation with an ambiguous operator (+ and ++ or - and --)
        let is_ambiguous_expression = match &argument {
            JsAnyExpression::JsUnaryExpression(expr) => {
                let inner_op = expr.operator()?;
                matches!(
                    (operation, inner_op),
                    (JsUnaryOperator::Plus, JsUnaryOperator::Plus)
                        | (JsUnaryOperator::Minus, JsUnaryOperator::Minus)
                )
            }
            JsAnyExpression::JsPreUpdateExpression(expr) => {
                let inner_op = expr.operator()?;
                matches!(
                    (operation, inner_op),
                    (JsUnaryOperator::Plus, JsPreUpdateOperator::Increment)
                        | (JsUnaryOperator::Minus, JsPreUpdateOperator::Decrement)
                )
            }
            _ => false,
        };

        if is_ambiguous_expression {
            operator_token.format().fmt(f)?;

            let format_argument = argument.format();

            let parenthesize = format_parenthesize(&format_argument);

            if is_simple_expression(&argument, &f.context().comments())? {
                parenthesize.fmt(f)
            } else {
                parenthesize.grouped_with_soft_block_indent().fmt(f)
            }
        } else {
            write![f, [operator_token.format(), argument.format(),]]
        }
    }
}
