use crate::prelude::*;
use rome_formatter::cst_builders::format_dangling_trivia;
use rome_formatter::{write, Comments, CstFormatContext};
use rome_js_syntax::{JsAnyClass, JsLanguage};

pub struct FormatClass<'a> {
    class: &'a JsAnyClass,
}

impl FormatClass<'_> {
    fn should_group(&self, comments: &Comments<JsLanguage>) -> FormatResult<bool> {
        if let Some(id) = self.class.id()? {
            if comments.has_trailing_comments(id.syntax()) {
                return Ok(true);
            }
        }

        if let Some(type_parameters) = self.class.type_parameters() {
            if comments.has_trailing_comments(type_parameters.syntax()) {
                return Ok(true);
            }
        }

        if let Some(extends) = self.class.extends_clause() {
            if comments.has_trailing_comments(extends.syntax()) {
                return Ok(true);
            }
        }

        if self.class.implements_clause().is_some() {
            return Ok(true);
        }

        Ok(false)
    }
}

impl<'a> From<&'a JsAnyClass> for FormatClass<'a> {
    fn from(class: &'a JsAnyClass) -> Self {
        Self { class }
    }
}

impl Format<JsFormatContext> for FormatClass<'_> {
    fn fmt(&self, f: &mut Formatter<JsFormatContext>) -> FormatResult<()> {
        let abstract_token = self.class.abstract_token();
        let id = self.class.id()?;
        let extends = self.class.extends_clause();
        let implements_clause = self.class.implements_clause();
        let type_parameters = self.class.type_parameters();
        let class_token = self.class.class_token()?;
        let members = self.class.members();

        let group_mode = self.should_group(&f.context().comments())?;

        if let Some(abstract_token) = abstract_token {
            write!(f, [abstract_token.format(), space_token()])?;
        }

        write!(f, [class_token.format()])?;

        let head = format_with(|f| {
            if let Some(id) = &id {
                write!(f, [space_token(), id.format()])?;
            }

            write!(f, [type_parameters.format()])?;

            if let Some(extends) = &extends {
                if group_mode {
                    write!(f, [soft_line_break_or_space()])?;
                } else {
                    write!(f, [space_token()])?;
                }

                write!(f, [extends.format()])?;
            }

            if let Some(implements_clause) = &implements_clause {
                write!(f, [soft_line_break_or_space(), implements_clause.format()])?;
            }

            Ok(())
        });

        if group_mode {
            let heritage_id = f.group_id("heritageGroup");

            write!(
                f,
                [
                    group_elements(&indent(&head)).with_group_id(Some(heritage_id)),
                    space_token(),
                ]
            )?;

            if !members.is_empty() {
                write!(
                    f,
                    [if_group_breaks(&hard_line_break()).with_group_id(Some(heritage_id))]
                )?;
            }
        } else {
            write!(f, [head, space_token()])?;
        }

        if members.is_empty() {
            let r_curly_token = self.class.r_curly_token()?;

            write!(
                f,
                [
                    self.class.l_curly_token().format(),
                    format_dangling_trivia(&r_curly_token).indented(),
                    r_curly_token.format()
                ]
            )
        } else {
            write![
                f,
                [format_delimited(
                    &self.class.l_curly_token()?,
                    &members.format(),
                    &self.class.r_curly_token()?
                )
                .block_indent()]
            ]
        }
    }
}
