//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/stdlib/structural/math/cases.md
//! @prompt-hash 801457c3
//! @layer L1
//! @updated 2026-09-30
//!
//! Delimitadores, opções e construção semântica de `math.cases`.

use crate::compiler::eval::EvalContext;
use crate::entities::args::Args;
use crate::entities::content::Content;
use crate::entities::elements::math_cases::MathCasesExplicit;
use crate::entities::file_id::FileId;
use crate::entities::layout_types::Length;
use crate::entities::rel::Rel;
use crate::entities::source_result::{SourceDiagnostic, SourceResult};
use crate::entities::span::Span;
use crate::entities::value::Value;

use super::vanilla_type_name_class;

#[derive(Debug, Clone, Copy)]
pub(crate) struct MathCasesOptions {
    pub delim: (Option<char>, Option<char>),
    pub reverse: bool,
    pub gap: Rel<Length>,
    pub explicit: MathCasesExplicit,
}

impl Default for MathCasesOptions {
    fn default() -> Self {
        Self {
            delim: (Some('{'), Some('}')),
            reverse: false,
            gap: Rel { rel: 0.0, abs: Length::em(0.2) },
            explicit: MathCasesExplicit::default(),
        }
    }
}

fn math_cases_delim_char(value: &Value, span: Span) -> SourceResult<Option<char>> {
    let text = match value {
        Value::None => return Ok(None),
        Value::Str(text) => text.as_str(),
        Value::Symbol(symbol) => symbol.value.as_str(),
        other => {
            return Err(vec![SourceDiagnostic::error(
                span,
                format!(
                    "expected none, symbol, or string, found {}",
                    vanilla_type_name_class(other)
                ),
            )])
        }
    };
    let mut chars = text.chars();
    let Some(character) = chars.next() else {
        return Err(vec![SourceDiagnostic::error(span, "expected a single delimiter")]);
    };
    if chars.next().is_some() {
        return Err(vec![SourceDiagnostic::error(span, "expected a single delimiter")]);
    }
    if !matches!(
        crate::entities::math_class::default_math_class(character),
        Some(
            crate::entities::math_class::MathClass::Opening
                | crate::entities::math_class::MathClass::Closing
                | crate::entities::math_class::MathClass::Fence
        )
    ) {
        return Err(vec![SourceDiagnostic::error(
            span,
            format!("invalid delimiter: \"{character}\""),
        )]);
    }
    Ok(Some(character))
}

fn math_cases_matching_delim(character: char) -> char {
    match character {
        '[' => ']',
        ']' => '[',
        '{' => '}',
        '}' => '{',
        '(' => ')',
        ')' => '(',
        '⌊' => '⌋',
        '⌋' => '⌊',
        '⌈' => '⌉',
        '⌉' => '⌈',
        '⟨' | '<' => '⟩',
        '⟩' | '>' => '⟨',
        other => other,
    }
}

pub(crate) fn math_cases_delim(
    value: &Value,
    span: Span,
) -> SourceResult<(Option<char>, Option<char>)> {
    match value {
        Value::None => Ok((None, None)),
        Value::Str(_) | Value::Symbol(_) => {
            let single = math_cases_delim_char(value, span)?;
            Ok(single.map_or((None, None), |character| {
                let matching = math_cases_matching_delim(character);
                match crate::entities::math_class::default_math_class(character) {
                    Some(crate::entities::math_class::MathClass::Closing) => {
                        (Some(matching), Some(character))
                    }
                    _ => (Some(character), Some(matching)),
                }
            }))
        }
        Value::Array(values) => {
            let [left, right] = values.as_slice() else {
                return Err(vec![SourceDiagnostic::error(
                    span,
                    "expected exactly two delimiters".to_string(),
                )]);
            };
            Ok((math_cases_delim_char(left, span)?, math_cases_delim_char(right, span)?))
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

pub(crate) fn apply_math_cases_option(
    options: &mut MathCasesOptions,
    name: &str,
    value: &Value,
    span: Span,
    explicit: bool,
) -> SourceResult<()> {
    match name {
        "delim" => {
            options.delim = math_cases_delim(value, span)?;
            options.explicit.delim |= explicit;
        }
        "reverse" => {
            let Value::Bool(reverse) = value else {
                return Err(vec![SourceDiagnostic::error(
                    span,
                    format!("expected boolean, found {}", vanilla_type_name_class(value)),
                )]);
            };
            options.reverse = *reverse;
            options.explicit.reverse |= explicit;
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

fn math_cases_child(value: &Value, span: Span) -> SourceResult<Content> {
    match value {
        Value::Content(Content::MathIdent(text)) => Ok(Content::MathText(text.clone())),
        Value::Content(content) => Ok(content.clone()),
        Value::LocatedContent(content, _) => match content.content() {
            Content::MathIdent(text) => Ok(Content::MathText(text.clone())),
            content => Ok(content.clone()),
        },
        Value::Str(text) => Ok(Content::text(text.as_str())),
        Value::Symbol(symbol) => Ok(Content::MathText(symbol.value.clone())),
        Value::None => Ok(Content::Empty),
        other => Err(vec![SourceDiagnostic::error(
            span,
            format!("expected content, found {}", vanilla_type_name_class(other)),
        )]),
    }
}

pub(crate) fn math_cases_content(args: &Args) -> SourceResult<Content> {
    for name in ["delim", "reverse", "gap"] {
        let count = args
            .occurrence_sequence()
            .iter()
            .filter(|occurrence| occurrence.name.as_deref() == Some(name))
            .count();
        if count > 1 {
            return Err(vec![SourceDiagnostic::error(
                args.span,
                format!("duplicate argument: {name}"),
            )]);
        }
    }
    let mut options = MathCasesOptions::default();
    for (name, value) in &args.named {
        apply_math_cases_option(&mut options, name, value, args.span, true)?;
    }
    let children = args
        .items
        .iter()
        .map(|value| math_cases_child(value, args.span))
        .collect::<SourceResult<Vec<_>>>()?;
    Ok(Content::math_cases_full(
        children,
        options.delim,
        options.reverse,
        options.gap,
        options.explicit,
    ))
}

pub fn native_math_cases(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    math_cases_content(args).map(Value::Content)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn closing_delimiter_preserves_side_and_infers_opening_partner() {
        let pair = math_cases_delim(&Value::Str(")".into()), Span::detached()).unwrap();
        assert_eq!(pair, (Some('('), Some(')')));
    }
}
