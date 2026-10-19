//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/stdlib/structural/math/frac.md
//! @prompt-hash ed8dfae2
//! @layer L1
//! @updated 2026-09-30
//!
//! Cast e construção semântica de `math.frac`.

use super::vanilla_type_name_class;
use crate::compiler::eval::EvalContext;
use crate::entities::args::Args;
use crate::entities::content::Content;
use crate::entities::file_id::FileId;
use crate::entities::source_result::{SourceDiagnostic, SourceResult};
use crate::entities::span::Span;
use crate::entities::value::Value;

fn math_frac_style(
    value: &Value,
    span: Span,
) -> SourceResult<crate::entities::elements::math_frac::MathFracStyle> {
    use crate::entities::elements::math_frac::MathFracStyle;
    match value {
        Value::Str(value) if value.as_str() == "vertical" => Ok(MathFracStyle::Vertical),
        Value::Str(value) if value.as_str() == "skewed" => Ok(MathFracStyle::Skewed),
        Value::Str(value) if value.as_str() == "horizontal" => {
            Ok(MathFracStyle::Horizontal)
        }
        Value::Str(_) => Err(vec![SourceDiagnostic::error(
            span,
            "expected \"vertical\", \"skewed\", or \"horizontal\"".to_string(),
        )]),
        other => Err(vec![SourceDiagnostic::error(
            span,
            format!(
                "expected \"vertical\", \"skewed\", or \"horizontal\", found {}",
                vanilla_type_name_class(other)
            ),
        )]),
    }
}

fn math_frac_operand(value: &Value, span: Span) -> SourceResult<Content> {
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

/// Parser/constructor compartilhado por `math.frac` e pelo bare `frac`.
pub(crate) fn math_frac_content(args: &Args) -> SourceResult<Content> {
    for positional in ["num", "denom"] {
        if args.named.contains_key(positional) {
            return Err(vec![SourceDiagnostic::error(
                args.span,
                format!("the argument `{positional}` is positional"),
            )]);
        }
    }
    if let Some((name, _)) = args.named.iter().find(|(name, _)| name.as_str() != "style")
    {
        return Err(vec![SourceDiagnostic::error(
            args.span,
            format!("unexpected argument: {name}"),
        )]);
    }
    let style_count = args
        .occurrence_sequence()
        .iter()
        .filter(|occurrence| occurrence.name.as_deref() == Some("style"))
        .count();
    if style_count > 1 {
        return Err(vec![SourceDiagnostic::error(
            args.span,
            "duplicate argument: style".to_string(),
        )]);
    }
    let [num, den] = args.items.as_slice() else {
        return Err(vec![SourceDiagnostic::error(
            args.span,
            match args.items.len() {
                0 => "missing argument: num",
                1 => "missing argument: denom",
                _ => "unexpected argument",
            }
            .to_string(),
        )]);
    };
    let style = args
        .named
        .get("style")
        .map(|value| math_frac_style(value, args.span))
        .transpose()?;
    Ok(Content::math_frac_full(
        math_frac_operand(num, args.span)?,
        math_frac_operand(den, args.span)?,
        style,
        false,
        false,
        true,
    ))
}

/// `math.frac(num, denom, style: ...)`.
pub fn native_math_frac(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    math_frac_content(args).map(Value::Content)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entities::elements::math_frac::MathFracStyle;
    use crate::entities::value::Value;

    #[test]
    fn explicit_vertical_style_remains_distinct_from_omission() {
        let args = Args::positional(vec![
            Value::Content(Content::text("a")),
            Value::Content(Content::text("b")),
        ]);
        let omitted = math_frac_content(&args).unwrap();
        let mut explicit = args;
        explicit.named.insert("style".into(), Value::Str("vertical".into()));
        let explicit = math_frac_content(&explicit).unwrap();
        let Content::MathFrac(omitted) = omitted else { panic!("expected fraction") };
        let Content::MathFrac(explicit) = explicit else { panic!("expected fraction") };
        assert_eq!(omitted.style, None);
        assert_eq!(explicit.style, Some(MathFracStyle::Vertical));
    }
}
