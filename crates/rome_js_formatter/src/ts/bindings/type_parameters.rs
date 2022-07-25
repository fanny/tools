use crate::prelude::*;

use rome_formatter::cst_builders::format_dangling_trivia;
use rome_formatter::{write, CstFormatContext, DanglingTrivia, FormatRuleWithOptions, GroupId};
use rome_js_syntax::{TsTypeParameters, TsTypeParametersFields};

#[derive(Debug, Clone, Default)]
pub struct FormatTsTypeParameters {
    group_id: Option<GroupId>,
}

impl FormatRuleWithOptions<TsTypeParameters> for FormatTsTypeParameters {
    type Options = Option<GroupId>;

    fn with_options(mut self, options: Self::Options) -> Self {
        self.group_id = options;
        self
    }
}

impl FormatNodeRule<TsTypeParameters> for FormatTsTypeParameters {
    fn fmt_fields(&self, node: &TsTypeParameters, f: &mut JsFormatter) -> FormatResult<()> {
        let TsTypeParametersFields {
            items,
            r_angle_token,
            l_angle_token,
        } = node.as_fields();

        let r_angle_token = r_angle_token?;

        write!(f, [l_angle_token.format()])?;

        if items.is_empty() {
            let has_only_block_comments = f
                .context()
                .comments()
                .dangling_trivia(&r_angle_token)
                .iter()
                .any(|trivia| match trivia {
                    DanglingTrivia::Comment(comment) => !comment.kind().is_line(),
                    DanglingTrivia::SkippedToken(_) => false,
                });

            if has_only_block_comments {
                write!(f, [&format_dangling_trivia(&r_angle_token)])?;
            } else {
                write!(
                    f,
                    [
                        &format_dangling_trivia(&r_angle_token).indented(),
                        hard_line_break()
                    ]
                )?;
            }
        } else {
            write!(
                f,
                [
                    group_elements(&soft_block_indent(&items.format()))
                        .with_group_id(self.group_id)
                ]
            )?;
        }

        write!(f, [r_angle_token.format()])
    }
}
