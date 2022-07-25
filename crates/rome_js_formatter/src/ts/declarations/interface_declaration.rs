use crate::prelude::*;

use rome_formatter::cst_builders::format_dangling_trivia;
use rome_formatter::{format_args, write, CstFormatContext};
use rome_js_syntax::{TsInterfaceDeclaration, TsInterfaceDeclarationFields};

#[derive(Debug, Clone, Default)]
pub struct FormatTsInterfaceDeclaration;

impl FormatNodeRule<TsInterfaceDeclaration> for FormatTsInterfaceDeclaration {
    fn fmt_fields(&self, node: &TsInterfaceDeclaration, f: &mut JsFormatter) -> FormatResult<()> {
        let TsInterfaceDeclarationFields {
            interface_token,
            id,
            type_parameters,
            extends_clause,
            members,
            l_curly_token,
            r_curly_token,
        } = node.as_fields();

        let l_curly_token = l_curly_token?;
        let r_curly_token = r_curly_token?;
        let id = id?;

        write!(
            f,
            [group_elements(&format_with(|f: &mut JsFormatter| {
                let type_parameter_group = type_parameters
                    .as_ref()
                    .map(|_| f.group_id("type-parameters"));

                let format_id = format_with(|f| {
                    write!(f, [id.format(),])?;

                    if let Some(type_parameters) = &type_parameters {
                        write!(
                            f,
                            [type_parameters.format().with_options(type_parameter_group)]
                        )?;
                    }

                    Ok(())
                });

                let should_indent_extends_only = type_parameters.as_ref().map_or(false, |params| {
                    !f.context()
                        .comments()
                        .trailing_comments(params.syntax())
                        .iter()
                        .any(|comment| comment.kind().is_line())
                });

                let format_extends = format_with(|f| {
                    if let Some(extends_clause) = &extends_clause {
                        if should_indent_extends_only {
                            write!(
                                f,
                                [
                                    if_group_breaks(&space_token())
                                        .with_group_id(type_parameter_group),
                                    if_group_fits_on_line(&soft_line_break_or_space())
                                        .with_group_id(type_parameter_group),
                                ]
                            )?;
                        } else {
                            write!(f, [soft_line_break_or_space()])?;
                        }

                        write!(f, [extends_clause.format(), space_token()])?;
                    }

                    Ok(())
                });

                write![f, [interface_token.format(), space_token()]]?;

                let id_has_trailing_comments =
                    f.context().comments().has_trailing_comments(id.syntax());
                if id_has_trailing_comments || extends_clause.is_some() {
                    if should_indent_extends_only {
                        write!(
                            f,
                            [group_elements(&format_args!(
                                format_id,
                                indent(&format_extends)
                            ))]
                        )?;
                    } else {
                        write!(
                            f,
                            [group_elements(&indent(&format_args!(
                                format_id,
                                format_extends
                            )))]
                        )?;
                    }
                } else {
                    write!(f, [format_id, format_extends])?;
                }

                write!(f, [space_token()])?;

                if members.is_empty() {
                    write!(
                        f,
                        [
                            l_curly_token.format(),
                            &format_dangling_trivia(&r_curly_token).indented(),
                            r_curly_token.format()
                        ]
                    )
                } else {
                    write!(
                        f,
                        [
                            format_delimited(&l_curly_token, &members.format(), &r_curly_token)
                                .block_indent()
                        ]
                    )
                }
            }))]
        )
    }
}
