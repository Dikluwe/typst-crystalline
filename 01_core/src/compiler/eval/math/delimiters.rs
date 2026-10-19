//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/core/compiler/eval/math/delimiters.md
//! @prompt-hash 224a7ca3
//! @layer L1
//! @updated 2026-10-16

use super::*;

pub(super) fn parse_delim_val(val: &Value) -> Option<(char, char)> {
    match val {
        Value::Str(s) => Some(parse_delim_str(s.as_str())),
        Value::Symbol(s) => Some(parse_delim_str(s.value.as_str())),
        Value::None => Some(('\0', '\0')),
        Value::Array(arr) => {
            let left = arr.first().and_then(parse_delim_char).unwrap_or('\0');
            let right = arr.get(1).and_then(parse_delim_char).unwrap_or(left);
            Some((left, right))
        }
        Value::Bool(_)
        | Value::Int(_)
        | Value::Float(_)
        | Value::Dict(_)
        | Value::Module(_)
        | Value::Datetime(_)
        | Value::Func(_)
        | Value::Content(_)
        | Value::LocatedContent(_, _)
        | Value::Auto
        | Value::Length(_)
        | Value::Relative(_)
        | Value::Ratio(_)
        | Value::Angle(_)
        | Value::Color(_)
        | Value::Stroke(_)
        | Value::Fraction(_)
        | Value::Align(_)
        | Value::Location(_)
        | Value::Gradient(_)
        | Value::Regex(_)
        | Value::Tiling(_)
        | Value::Bytes(_)
        | Value::Decimal(_)
        | Value::Duration(_)
        | Value::Version(_)
        | Value::Selector(_)
        | Value::Args(_)
        | Value::State(_)
        | Value::Counter(_)
        | Value::Label(_)
        | Value::Dir(_)
        | Value::Path(_)
        | Value::Type(_)
        | Value::SpotColorant(_) => None, // neutro: N16[β] — projecção de delim: valores não-string/array retornam None (paridade vanilla math/matrix.rs)
    }
}

pub(super) fn parse_delim_char(val: &Value) -> Option<char> {
    match val {
        Value::Str(s) => s.chars().next(),
        Value::Symbol(s) => s.value.chars().next(),
        Value::None => Some('\0'),
        Value::Bool(_)
        | Value::Int(_)
        | Value::Float(_)
        | Value::Array(_)
        | Value::Dict(_)
        | Value::Module(_)
        | Value::Datetime(_)
        | Value::Func(_)
        | Value::Content(_)
        | Value::LocatedContent(_, _)
        | Value::Auto
        | Value::Length(_)
        | Value::Relative(_)
        | Value::Ratio(_)
        | Value::Angle(_)
        | Value::Color(_)
        | Value::Stroke(_)
        | Value::Fraction(_)
        | Value::Align(_)
        | Value::Location(_)
        | Value::Gradient(_)
        | Value::Regex(_)
        | Value::Tiling(_)
        | Value::Bytes(_)
        | Value::Decimal(_)
        | Value::Duration(_)
        | Value::Version(_)
        | Value::Selector(_)
        | Value::Args(_)
        | Value::State(_)
        | Value::Counter(_)
        | Value::Label(_)
        | Value::Dir(_)
        | Value::Path(_)
        | Value::Type(_)
        | Value::SpotColorant(_) => None, // neutro: N16[β] — projecção de delim-char: tipos não-texto retornam None (\0 default)
    }
}

#[allow(clippy::too_many_arguments)]
pub(super) fn apply_syntax_vec_option(
    delim: &mut (char, char),
    align: &mut crate::entities::layout_types::HAlign,
    gap: &mut crate::entities::rel::Rel<crate::entities::layout_types::Length>,
    presence: &mut crate::entities::elements::math_vec::MathVecExplicit,
    name: &str,
    value: &Value,
    span: Span,
    explicit: bool,
) -> SourceResult<()> {
    let type_name = |value: &Value| match value.type_name() {
        "int" => "integer",
        "str" => "string",
        "bool" => "boolean",
        other => other,
    };
    match name {
        "delim" => {
            *delim = parse_delim_val(value).ok_or_else(|| {
                vec![SourceDiagnostic::error(
                    span,
                    format!(
                        "expected array, none, symbol, or string, found {}",
                        type_name(value)
                    ),
                )]
            })?;
            presence.delim |= explicit;
        }
        "align" => {
            let Value::Align(value) = value else {
                return Err(vec![SourceDiagnostic::error(
                    span,
                    format!("expected alignment, found {}", type_name(value)),
                )]);
            };
            let Some(horizontal) = value.h.filter(|_| value.v.is_none()) else {
                return Err(vec![SourceDiagnostic::error(
                    span,
                    "expected horizontal alignment".to_string(),
                )]);
            };
            *align = horizontal;
            presence.align |= explicit;
        }
        "gap" => {
            *gap = match value {
                Value::Relative(value) => *value,
                Value::Ratio(value) => crate::entities::rel::Rel {
                    rel: value.get(),
                    abs: crate::entities::layout_types::Length::ZERO,
                },
                Value::Length(value) => {
                    crate::entities::rel::Rel { rel: 0.0, abs: *value }
                }
                other => {
                    return Err(vec![SourceDiagnostic::error(
                        span,
                        format!("expected relative length, found {}", type_name(other)),
                    )]);
                }
            };
            presence.gap |= explicit;
        }
        other => {
            return Err(vec![SourceDiagnostic::error(
                span,
                format!("unexpected argument: {other}"),
            )]);
        }
    }
    Ok(())
}

pub(super) fn parse_delim_str(s: &str) -> (char, char) {
    match s {
        "(" | ")" => ('(', ')'),
        "[" | "]" => ('[', ']'),
        "{" | "}" => ('{', '}'),
        "|" => ('|', '|'),
        "||" | "|||" => ('‖', '‖'),
        "⌊" | "floor" => ('⌊', '⌋'),
        "⌈" | "ceil" => ('⌈', '⌉'),
        "<" | ">" | "chevron" => ('⟨', '⟩'),
        "" | "none" => ('\0', '\0'),
        other => {
            let mut chars = other.chars();
            let first = chars.next().unwrap_or('\0');
            let second = chars.next().unwrap_or(first);
            (first, second)
        }
    }
}

#[cfg(test)]
mod v16_math_value_tests {
    use super::*;
    use crate::entities::symbol::Symbol;

    #[test]
    fn v16_math_value_delimiter_pairs_and_array_fallback_preserve_morphology() {
        for (value, expected) in [
            (Value::None, Some(('\0', '\0'))),
            (Value::Str("floor".into()), Some(('⌊', '⌋'))),
            (Value::Str("⟦⟧".into()), Some(('⟦', '⟧'))),
            (Value::Symbol(Symbol::new("⟦⟧", "test")), Some(('⟦', '⟧'))),
            (Value::Array(vec![]), Some(('\0', '\0'))),
            (
                Value::Array(vec![Value::Int(2), Value::Str("]".into())]),
                Some(('\0', ']')),
            ),
            (
                Value::Array(vec![Value::Str("[".into()), Value::Bool(false)]),
                Some(('[', '[')),
            ),
            (
                Value::Array(vec![
                    Value::Str("[".into()),
                    Value::None,
                    Value::Str("ignored".into()),
                ]),
                Some(('[', '\0')),
            ),
            (Value::Auto, None),
            (Value::Int(1), None),
            (Value::Bool(false), None),
        ] {
            assert_eq!(parse_delim_val(&value), expected, "{value:?}");
        }
    }

    #[test]
    fn v16_math_value_delimiter_char_keeps_first_unicode_scalar_and_rejections() {
        for (value, expected) in [
            (Value::None, Some('\0')),
            (Value::Str("".into()), None),
            (Value::Str("⟦⟧".into()), Some('⟦')),
            (Value::Symbol(Symbol::new("←→", "arrow")), Some('←')),
            (Value::Array(vec![]), None),
            (Value::Bool(false), None),
            (Value::Float(f64::NAN), None),
            (Value::Auto, None),
        ] {
            assert_eq!(parse_delim_char(&value), expected, "{value:?}");
        }
    }
}
