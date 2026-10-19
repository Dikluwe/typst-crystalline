//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/core/compiler/stdlib/structural/math/accent.md
//! @prompt-hash be214cbd
//! @layer L1
//! @updated 2026-09-30
//!
//! Aliases, casts e construção semântica de `math.accent`.

use crate::compiler::eval::EvalContext;
use crate::entities::args::Args;
use crate::entities::content::Content;
use crate::entities::elements::math_accent::MathAccentExplicit;
use crate::entities::file_id::FileId;
use crate::entities::layout_types::Length;
use crate::entities::rel::Rel;
use crate::entities::source_result::{SourceDiagnostic, SourceResult};
use crate::entities::span::Span;
use crate::entities::value::Value;

use super::vanilla_type_name_class;

const ACCENT_ALIASES: &[(char, &[&str])] = &[
    ('\u{0300}', &["`"]),
    ('\u{0301}', &["´"]),
    ('\u{0302}', &["^", "ˆ"]),
    ('\u{0303}', &["~", "∼", "˜"]),
    ('\u{0304}', &["¯"]),
    ('\u{0305}', &["-", "–", "‾", "−"]),
    ('\u{0306}', &["˘"]),
    ('\u{0307}', &[".", "˙", "⋅"]),
    ('\u{0308}', &["¨"]),
    ('\u{20db}', &[]),
    ('\u{20dc}', &[]),
    ('\u{030a}', &["∘", "○"]),
    ('\u{030b}', &["˝"]),
    ('\u{030c}', &["ˇ"]),
    ('\u{20d6}', &["←"]),
    ('\u{20d7}', &["→", "⟶"]),
    ('\u{20e1}', &["↔", "↔\u{fe0e}", "⟷"]),
    ('\u{20d0}', &["↼"]),
    ('\u{20d1}', &["⇀"]),
];

/// Only known combining marks/aliases are callable as symbol wrappers.
pub(crate) fn is_known_math_accent(value: &str) -> bool {
    let direct = value.parse::<char>().ok();
    ACCENT_ALIASES
        .iter()
        .any(|(accent, aliases)| Some(*accent) == direct || aliases.contains(&value))
}

pub(crate) fn normalize_math_accent(value: &str) -> Option<char> {
    let direct = value.parse::<char>().ok();
    ACCENT_ALIASES
        .iter()
        .find(|(accent, aliases)| Some(*accent) == direct || aliases.contains(&value))
        .map(|(accent, _)| *accent)
        .or(direct)
}

fn math_accent_base(value: &Value, span: Span) -> SourceResult<Content> {
    match value {
        Value::Content(content) => Ok(content.clone()),
        Value::LocatedContent(content, _) => Ok(content.content().clone()),
        Value::Str(text) => Ok(Content::text(text.as_str())),
        Value::None => Ok(Content::Empty),
        other => Err(vec![SourceDiagnostic::error(
            span,
            format!("expected content, found {}", vanilla_type_name_class(other)),
        )]),
    }
}

fn math_accent_char(value: &Value, span: Span) -> SourceResult<char> {
    match value {
        Value::Str(text) => normalize_math_accent(text.as_str()).ok_or_else(|| {
            vec![SourceDiagnostic::error(span, "expected exactly one character")]
        }),
        Value::Symbol(symbol) => {
            normalize_math_accent(symbol.value.as_str()).ok_or_else(|| {
                vec![SourceDiagnostic::error(span, "expected exactly one character")]
            })
        }
        Value::Content(Content::MathText(text)) => normalize_math_accent(text.as_str())
            .ok_or_else(|| {
                vec![SourceDiagnostic::error(span, "expected a single-codepoint symbol")]
            }),
        _ => {
            Err(vec![SourceDiagnostic::error(span, "expected a single-codepoint symbol")])
        }
    }
}

/// Parser/constructor compartilhado pelo binding qualificado e pelas rotas math.
pub(crate) fn math_accent_content(args: &Args) -> SourceResult<Content> {
    for positional in ["base", "accent"] {
        if args.named.contains_key(positional) {
            return Err(vec![SourceDiagnostic::error(
                args.span,
                format!("the argument `{positional}` is positional"),
            )]);
        }
    }
    if let Some((name, _)) = args
        .named
        .iter()
        .find(|(name, _)| !matches!(name.as_str(), "size" | "dotless"))
    {
        return Err(vec![SourceDiagnostic::error(
            args.span,
            format!("unexpected argument: {name}"),
        )]);
    }
    for name in ["size", "dotless"] {
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
    let [base, accent] = args.items.as_slice() else {
        return Err(vec![SourceDiagnostic::error(
            args.span,
            match args.items.len() {
                0 => "missing argument: base",
                1 => "missing argument: accent",
                _ => "unexpected argument",
            },
        )]);
    };

    let mut size = Rel::from_percent(100.0);
    let mut dotless = true;
    let mut explicit = MathAccentExplicit::default();
    if let Some(value) = args.named.get("size") {
        explicit.size = true;
        size = match value {
            Value::Relative(value) => *value,
            Value::Ratio(value) => Rel { rel: value.get(), abs: Length::ZERO },
            Value::Length(value) => Rel { rel: 0.0, abs: *value },
            other => {
                return Err(vec![SourceDiagnostic::error(
                    args.span,
                    format!(
                        "expected relative length, found {}",
                        vanilla_type_name_class(other)
                    ),
                )])
            }
        };
    }
    if let Some(value) = args.named.get("dotless") {
        explicit.dotless = true;
        dotless = match value {
            Value::Bool(value) => *value,
            other => {
                return Err(vec![SourceDiagnostic::error(
                    args.span,
                    format!("expected boolean, found {}", vanilla_type_name_class(other)),
                )])
            }
        };
    }
    Ok(Content::math_accent_full(
        math_accent_base(base, args.span)?,
        math_accent_char(accent, args.span)?,
        size,
        dotless,
        explicit,
    ))
}

pub fn native_accent(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    math_accent_content(args).map(Value::Content)
}

#[cfg(test)]
mod tests {
    use super::normalize_math_accent;

    #[test]
    fn alias_e_combining_direto_conservam_o_mesmo_acento() {
        assert_eq!(normalize_math_accent("^"), Some('\u{0302}'));
        assert_eq!(normalize_math_accent("\u{0302}"), Some('\u{0302}'));
    }
}
