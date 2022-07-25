use crate::prelude::*;
use crate::utils::FormatTypeMemberSeparator;

use rome_formatter::{write, Memoized};
use rome_js_syntax::{
    TsAnyReturnType, TsMethodSignatureTypeMember, TsMethodSignatureTypeMemberFields, TsType,
};

#[derive(Debug, Clone, Default)]
pub struct FormatTsMethodSignatureTypeMember;

impl FormatNodeRule<TsMethodSignatureTypeMember> for FormatTsMethodSignatureTypeMember {
    fn fmt_fields(
        &self,
        node: &TsMethodSignatureTypeMember,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        let TsMethodSignatureTypeMemberFields {
            name,
            optional_token,
            type_parameters,
            parameters,
            return_type_annotation,
            separator_token,
        } = node.as_fields();

        let mut format_return_type = return_type_annotation.format().memoized();
        let should_group_params =
            should_group_function_parameters(node, &mut format_return_type, f)?;

        let content = format_with(|f| {
            write!(
                f,
                [
                    name.format(),
                    optional_token.format(),
                    type_parameters.format(),
                ]
            )?;

            if should_group_params {
                write!(f, [group_elements(&parameters.format())])?;
            } else {
                write!(f, [parameters.format()])?;
            }

            write!(f, [format_return_type])
        });

        write![
            f,
            [
                group_elements(&content),
                FormatTypeMemberSeparator::new(separator_token.as_ref())
            ]
        ]
    }
}

fn should_group_function_parameters<F>(
    node: &TsMethodSignatureTypeMember,
    return_type_doc: &mut Memoized<F, JsFormatContext>,
    f: &mut JsFormatter,
) -> FormatResult<bool>
where
    F: Format<JsFormatContext>,
{
    let return_type_annotation = match node.return_type_annotation() {
        Some(return_type_annotation) => return_type_annotation,
        None => return Ok(false),
    };

    if let Some(type_parameters) = node.type_parameters() {
        match type_parameters.items().len() {
            0 => {
                // Fall through
            }
            1 => {
                let parameter = type_parameters.items().first().unwrap()?;
                if parameter.constraint().is_some() || parameter.default().is_some() {
                    return Ok(false);
                }
            }
            _ => {
                return Ok(false);
            }
        }
    }

    let parameters = node.parameters()?;

    let is_object_return_type = matches!(
        return_type_annotation.ty()?,
        TsAnyReturnType::TsType(TsType::TsObjectType(_))
    );

    return Ok(parameters.items().len() == 1
        && (is_object_return_type || return_type_doc.inspect(f)?.will_break()));
}
