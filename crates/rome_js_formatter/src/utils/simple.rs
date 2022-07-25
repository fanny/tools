//! This module exposes utility functions for detecting "simple" expressions
//!
//! Simple expressions are expressions that are going to create a single group
//! anyway, so they don't need to be wrapped in a second one: this includes
//! object, array or parenthesized expressions, as well as function
//! declarations that have a high probability of breaking only in their body
//! group.
//! This last bit is defined recursively in [is_simple_function_expression] as
//! functions that only have a few (less than 3) identifier parameters, no type
//! parameter or return type and a block body: technically such a function
//! expression can still break in both the parameters and body group but the
//! small number of parameters makes it unlikely.
//!
//! The use case for detecting these "simple" expressions is to avoid creating
//! redundant groups in nested delimited expressions when only one would
//! suffice, for instance in call expressions:
//!
//! ```js
//! // Formatter output without handling of simple expressions
//! new Promise(
//!   (resolve, reject) => {
//!     resolve();
//!   },
//! );
//!
//! func(
//!   {
//!     key: 'value',
//!   },
//! );
//!
//! // Formatter output with handling of simple expressions
//! new Promise((resolve, reject) => {
//!   resolve();
//! });
//!
//! func({
//!   key: 'value',
//! });
//! ```

use rome_formatter::Comments;
use rome_js_syntax::{
    JsAnyArrowFunctionParameters, JsAnyBinding, JsAnyBindingPattern, JsAnyExpression,
    JsAnyFormalParameter, JsAnyFunction, JsAnyParameter, JsFormalParameter,
    JsFormalParameterFields, JsLanguage, JsObjectExpression, JsObjectExpressionFields,
    JsParameters, JsParametersFields, JsSyntaxToken,
};
use rome_rowan::{AstNode, AstSeparatedList, SyntaxResult};

/// Returns true is the passed [JsAnyExpression] is a simple function, array or object expression
pub(crate) fn is_simple_expression(
    node: &JsAnyExpression,
    comments: &Comments<JsLanguage>,
) -> SyntaxResult<bool> {
    match node {
        JsAnyExpression::JsArrayExpression(_) => Ok(true),
        JsAnyExpression::JsObjectExpression(object) => Ok(true),
        node => {
            if let Some(func) = JsAnyFunction::cast(node.syntax().clone()) {
                is_simple_function_expression(func, comments)
            } else {
                Ok(false)
            }
        }
    }
}

/// Returns true if the passed [JsAnyFunction] does not have any comment, type
/// parameters, return type annotation and simple parameters (see [is_simple_function_parameters])
pub(crate) fn is_simple_function_expression(
    func: JsAnyFunction,
    comments: &Comments<JsLanguage>,
) -> SyntaxResult<bool> {
    if let Some(token) = func.function_token()? {
        if comments.has_dangling_trivia(&token) {
            return Ok(false);
        }
    }

    if let Some(token) = func.star_token() {
        if comments.has_dangling_trivia(&token) {
            return Ok(false);
        }
    }

    if func.type_parameters().is_some() {
        return Ok(false);
    }

    match func.parameters()? {
        JsAnyArrowFunctionParameters::JsAnyBinding(JsAnyBinding::JsIdentifierBinding(_)) => {}
        JsAnyArrowFunctionParameters::JsParameters(parameters) => {
            if !is_simple_function_parameters(parameters, comments)? {
                return Ok(false);
            }
        }
        _ => {
            return Ok(false);
        }
    }

    if func.return_type_annotation().is_some() {
        return Ok(false);
    }

    Ok(true)
}

/// Returns true if the passed [JsParameters] has 2 or less parameters that are
/// all simple parameters (see [is_simple_parameter]) with no comments trivia
fn is_simple_function_parameters(
    node: JsParameters,
    comments: &Comments<JsLanguage>,
) -> SyntaxResult<bool> {
    if comments.has_comments(node.syntax()) {
        return Ok(false);
    }

    let items = node.items();
    if items.len() >= 3 {
        return Ok(false);
    }

    for item in &items {
        match item? {
            JsAnyParameter::JsAnyFormalParameter(JsAnyFormalParameter::JsFormalParameter(node)) => {
                if !is_simple_parameter(node, comments)? {
                    return Ok(false);
                }
            }
            _ => return Ok(false),
        }
    }

    Ok(true)
}

/// Returns true if the passed [JsFormalParameter] is a single identifier
/// with no question mark token, type annotation or initializer
fn is_simple_parameter(
    node: JsFormalParameter,
    comments: &Comments<JsLanguage>,
) -> SyntaxResult<bool> {
    let JsFormalParameterFields {
        binding,
        question_mark_token,
        type_annotation,
        initializer,
    } = node.as_fields();

    match binding? {
        JsAnyBindingPattern::JsAnyBinding(JsAnyBinding::JsIdentifierBinding(ident)) => {
            if comments.has_comments(ident.syntax()) {
                return Ok(false);
            }
        }
        _ => return Ok(false),
    }

    if question_mark_token.is_some() || type_annotation.is_some() || initializer.is_some() {
        return Ok(false);
    }

    Ok(true)
}
