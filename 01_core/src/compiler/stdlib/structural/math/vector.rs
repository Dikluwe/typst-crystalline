//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/core/compiler/stdlib/structural/math/vector.md
//! @prompt-hash 68649499
//! @layer L1
//! @updated 2026-10-16
//!
//! Constructor, casts e opções da nativa `math.vec`.

use crate::compiler::eval::EvalContext;
use crate::entities::args::Args;
use crate::entities::content::Content;
use crate::entities::elements::math_vec::MathVecExplicit;
use crate::entities::file_id::FileId;
use crate::entities::layout_types::{HAlign, Length};
use crate::entities::rel::Rel;
use crate::entities::source_result::{SourceDiagnostic, SourceResult};
use crate::entities::span::Span;
use crate::entities::value::Value;

use super::vanilla_type_name_class;

/// Valores cascatiáveis do construtor canónico de `math.vec`.
#[derive(Debug, Clone, Copy)]
pub(crate) struct MathVecOptions {
    pub delim: (char, char),
    pub align: HAlign,
    pub gap: Rel<Length>,
    pub explicit: MathVecExplicit,
}

impl Default for MathVecOptions {
    fn default() -> Self {
        Self {
            delim: ('(', ')'),
            align: HAlign::Center,
            gap: Rel { rel: 0.0, abs: Length::em(0.2) },
            explicit: MathVecExplicit::default(),
        }
    }
}

fn vec_delim_char(value: &Value) -> Option<char> {
    match value {
        Value::None => Some('\0'),
        Value::Str(text) => text.chars().next(),
        Value::Symbol(symbol) => symbol.value.chars().next(),
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
        | Value::SpotColorant(_) => None,
    }
}

fn vec_delim_string(text: &str) -> (char, char) {
    match text {
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
            let left = chars.next().unwrap_or('\0');
            (left, chars.next().unwrap_or(left))
        }
    }
}

pub(crate) fn math_vec_delim(value: &Value, span: Span) -> SourceResult<(char, char)> {
    match value {
        Value::None => Ok(('\0', '\0')),
        Value::Str(text) => Ok(vec_delim_string(text.as_str())),
        Value::Symbol(symbol) => Ok(vec_delim_string(symbol.value.as_str())),
        Value::Array(values) => {
            let left = values.first().and_then(vec_delim_char).unwrap_or('\0');
            let right = values.get(1).and_then(vec_delim_char).unwrap_or(left);
            Ok((left, right))
        }
        other => Err(vec![SourceDiagnostic::error(
            span,
            format!(
                "expected array, none, symbol, or string, found {}",
                vanilla_type_name_class(other)
            ),
        )]),
    }
}

/// Aplica um named de `math.vec`; set rules usam `explicit = false`.
pub(crate) fn apply_math_vec_option(
    options: &mut MathVecOptions,
    name: &str,
    value: &Value,
    span: Span,
    explicit: bool,
) -> SourceResult<()> {
    match name {
        "delim" => {
            options.delim = math_vec_delim(value, span)?;
            options.explicit.delim |= explicit;
        }
        "align" => {
            let Value::Align(align) = value else {
                return Err(vec![SourceDiagnostic::error(
                    span,
                    format!(
                        "expected alignment, found {}",
                        vanilla_type_name_class(value)
                    ),
                )]);
            };
            let Some(horizontal) = align.h.filter(|_| align.v.is_none()) else {
                return Err(vec![SourceDiagnostic::error(
                    span,
                    "expected horizontal alignment".to_string(),
                )]);
            };
            options.align = horizontal;
            options.explicit.align |= explicit;
        }
        "gap" => {
            options.gap = match value {
                Value::Relative(gap) => *gap,
                Value::Ratio(gap) => Rel { rel: gap.get(), abs: Length::ZERO },
                Value::Length(gap) => Rel { rel: 0.0, abs: *gap },
                other => {
                    return Err(vec![SourceDiagnostic::error(
                        span,
                        format!(
                            "expected relative length, found {}",
                            vanilla_type_name_class(other)
                        ),
                    )])
                }
            };
            options.explicit.gap |= explicit;
        }
        other => {
            return Err(vec![SourceDiagnostic::error(
                span,
                format!("unexpected argument: {other}"),
            )])
        }
    }
    Ok(())
}

pub(crate) fn math_vec_content(
    children: Vec<Content>,
    options: MathVecOptions,
) -> Content {
    Content::math_vec_full(
        children,
        options.delim,
        options.align,
        options.gap,
        options.explicit,
    )
}

/// `math.vec(..children, delim:, align:, gap:)`.
pub fn native_math_vec(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    let mut children = Vec::with_capacity(args.items.len());
    for value in &args.items {
        let child = match value {
            Value::Content(content) => content.clone(),
            Value::LocatedContent(content, _) => content.content().clone(),
            Value::Str(text) => Content::text(text.as_str()),
            Value::Symbol(symbol) => Content::MathText(symbol.value.clone()),
            other => {
                return Err(vec![SourceDiagnostic::error(
                    args.span,
                    format!("expected content, found {}", vanilla_type_name_class(other)),
                )])
            }
        };
        children.push(child);
    }

    let mut options = MathVecOptions::default();
    for (name, value) in &args.named {
        apply_math_vec_option(&mut options, name, value, args.span, true)?;
    }
    Ok(Value::Content(math_vec_content(children, options)))
}

#[cfg(test)]
mod p1292_c_vec_tests {
    use super::*;
    use crate::entities::layout_types::Ratio;

    #[test]
    fn pure_ratio_gap_converte_zero_e_dez_porcento() {
        for (percent, expected) in [(0.0, 0.0), (10.0, 0.1)] {
            let mut options = MathVecOptions::default();
            apply_math_vec_option(
                &mut options,
                "gap",
                &Value::Ratio(Ratio::from_percent(percent)),
                Span::detached(),
                true,
            )
            .expect("ratio puro deve converter para relative length");
            assert_eq!(options.gap.rel, expected);
            assert!(options.gap.abs.is_zero());
            assert!(options.explicit.gap);
        }
    }
}

#[cfg(test)]
mod v16_math_value_tests {
    use super::*;
    use crate::entities::symbol::Symbol;

    #[test]
    fn v16_math_value_vector_char_keeps_none_unicode_and_neutral_types() {
        for (value, expected) in [
            (Value::None, Some('\0')),
            (Value::Str("".into()), None),
            (Value::Str("⟦⟧".into()), Some('⟦')),
            (Value::Symbol(Symbol::new("←→", "arrow")), Some('←')),
            (Value::Array(vec![]), None),
            (Value::Bool(false), None),
            (Value::Int(1), None),
            (Value::Auto, None),
        ] {
            assert_eq!(vec_delim_char(&value), expected, "{value:?}");
        }
    }

    #[test]
    fn v16_math_value_vector_array_fallback_and_top_level_error_are_preserved() {
        for (values, expected) in [
            (vec![], ('\0', '\0')),
            (vec![Value::Str("[".into())], ('[', '[')),
            (vec![Value::Int(2), Value::Str("]".into())], ('\0', ']')),
            (vec![Value::Str("[".into()), Value::Bool(false)], ('[', '[')),
            (vec![Value::Str("[".into()), Value::None], ('[', '\0')),
        ] {
            assert_eq!(
                math_vec_delim(&Value::Array(values), Span::detached()).unwrap(),
                expected
            );
        }
        let errors = math_vec_delim(&Value::Int(1), Span::detached()).unwrap_err();
        assert_eq!(
            errors[0].message,
            "expected array, none, symbol, or string, found integer"
        );
        assert_eq!(errors[0].span, Span::detached());
    }
}
