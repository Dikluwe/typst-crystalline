//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/stdlib/structural/math/equation.md
//! @prompt-hash b71d547c
//! @layer L1
//! @updated 2026-09-30
//!
//! Casts e construção semântica de `math.equation`.

use crate::compiler::eval::EvalContext;
use crate::entities::args::Args;
use crate::entities::content::Content;
use crate::entities::file_id::FileId;
use crate::entities::layout_types::{Align2D, HAlign};
use crate::entities::source_result::{SourceDiagnostic, SourceResult};
use crate::entities::span::Span;
use crate::entities::style::Styles;
use crate::entities::value::Value;

/// Construtor puro compartilhado pela função pública e pela sintaxe `$...$`.
pub(crate) fn equation_content(body: Content, block: bool) -> Content {
    Content::equation(body, block)
}

/// Cast público de `math.equation.number-align`.
pub(crate) fn equation_number_align(value: &Value, span: Span) -> SourceResult<Align2D> {
    let Value::Align(align) = value else {
        let found = match value.type_name() {
            "int" => "integer",
            "str" => "string",
            "bool" => "boolean",
            other => other,
        };
        return Err(vec![SourceDiagnostic::error(
            span,
            format!("expected alignment, found {found}"),
        )]);
    };
    if matches!(align.h, Some(HAlign::Center)) {
        return Err(vec![SourceDiagnostic::error(
            span,
            "expected `start`, `left`, `right`, or `end`, found center".to_string(),
        )]);
    }
    Ok(*align)
}

pub(crate) fn equation_supplement(value: &Value, span: Span) -> SourceResult<Value> {
    match value {
        Value::Str(text) => Ok(Value::Content(Content::text(text.as_str()))),
        Value::Content(_) | Value::Func(_) | Value::None | Value::Auto => {
            Ok(value.clone())
        }
        other => {
            let found = match other.type_name() {
                "int" => "integer",
                "bool" => "boolean",
                "str" => "string",
                name => name,
            };
            Err(vec![SourceDiagnostic::error(
                span,
                format!("expected content, function, none, or auto, found {found}"),
            )])
        }
    }
}

pub(crate) fn equation_alt(value: &Value, span: Span) -> SourceResult<Value> {
    match value {
        Value::Str(_) | Value::None => Ok(value.clone()),
        other => {
            let found = match other.type_name() {
                "int" => "integer",
                "bool" => "boolean",
                "str" => "string",
                name => name,
            };
            Err(vec![SourceDiagnostic::error(
                span,
                format!("expected string or none, found {found}"),
            )])
        }
    }
}

/// `math.equation(body, block: false)` — primeira fase do elemento público.
pub fn native_math_equation(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    let body = match args.items.as_slice() {
        [Value::Content(body)] => body.clone(),
        [other] => {
            return Err(vec![SourceDiagnostic::error(
                args.span,
                format!("expected content, found {}", other.type_name()),
            )])
        }
        [] => {
            return Err(vec![SourceDiagnostic::error(
                args.span,
                "missing argument: body".to_string(),
            )])
        }
        _ => {
            return Err(vec![SourceDiagnostic::error(
                args.span,
                "unexpected argument".to_string(),
            )])
        }
    };

    let mut block = false;
    let mut numbering: Option<Value> = None;
    let mut number_align: Option<Value> = None;
    let mut supplement: Option<Value> = None;
    let mut alt: Option<Value> = None;
    for (name, value) in &args.named {
        match name.as_str() {
            "block" => match value {
                Value::Bool(value) => block = *value,
                other => {
                    return Err(vec![SourceDiagnostic::error(
                        args.span,
                        format!("expected bool, found {}", other.type_name()),
                    )])
                }
            },
            "numbering" => match value {
                Value::Str(_) | Value::Func(_) | Value::None => {
                    numbering = Some(value.clone());
                }
                other => {
                    return Err(vec![SourceDiagnostic::error(
                        args.span,
                        format!(
                            "expected string, function, or none, found {}",
                            other.type_name()
                        ),
                    )])
                }
            },
            "number-align" => {
                equation_number_align(value, args.span)?;
                number_align = Some(value.clone());
            }
            "supplement" => {
                supplement = Some(equation_supplement(value, args.span)?);
            }
            "alt" => {
                alt = Some(equation_alt(value, args.span)?);
            }
            other => {
                return Err(vec![SourceDiagnostic::error(
                    args.span,
                    format!("unexpected argument: {other}"),
                )])
            }
        }
    }

    let equation = equation_content(body, block);
    let mut styles = Styles::new();
    if let Some(value) = numbering {
        styles = styles.push_custom("equation.numbering", value);
    }
    if let Some(value) = number_align {
        styles = styles.push_custom("equation.number-align", value);
    }
    if let Some(value) = supplement {
        styles = styles.push_custom("equation.supplement", value);
    }
    if let Some(value) = alt {
        styles = styles.push_custom("equation.alt", value);
    }
    Ok(Value::Content(if styles.delta().custom.is_empty() {
        equation
    } else {
        Content::styled(Box::new(equation), styles)
    }))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entities::content::Content;

    #[test]
    fn supplement_string_cast_differs_from_alt_content_rejection() {
        let supplement =
            equation_supplement(&Value::Str("Eq".into()), Span::detached()).unwrap();
        assert!(matches!(supplement, Value::Content(Content::Text(_))));
        let error = equation_alt(&Value::Content(Content::text("Eq")), Span::detached())
            .unwrap_err();
        assert_eq!(error[0].message, "expected string or none, found content");
    }
}
