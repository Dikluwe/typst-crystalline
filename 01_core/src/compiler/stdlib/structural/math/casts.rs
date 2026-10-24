//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/core/compiler/stdlib/structural/math/casts.md
//! @prompt-hash c6abcde6
//! @layer L1
//! @updated 2026-09-30
//!
//! Comportamento de `math.casts` na camada stdlib.

use crate::entities::args::Args;
use crate::entities::content::Content;
use crate::entities::layout_types::Length;
use crate::entities::rel::Rel;
use crate::entities::source_result::{SourceDiagnostic, SourceResult};
use crate::entities::span::Span;
use crate::entities::value::Value;

pub(super) fn math_lr_content_value(value: &Value, span: Span) -> SourceResult<Content> {
    match value {
        Value::Content(content) => Ok(content.clone()),
        Value::LocatedContent(content, _) => Ok(content.content().clone()),
        Value::Str(text) => Ok(Content::text(text.as_str())),
        Value::Symbol(symbol) => Ok(Content::Symbol(symbol.value.clone())),
        Value::None => Ok(Content::Empty),
        other => Err(vec![SourceDiagnostic::error(
            span,
            format!("expected content, found {}", vanilla_type_name_class(other)),
        )]),
    }
}

/// Validação e construção compartilhada de `math.lr` e `lr(...)` bare.
pub(super) fn math_lr_size_from_args(args: &Args) -> SourceResult<(Rel<Length>, bool)> {
    let size_occurrences = args
        .occurrence_sequence()
        .iter()
        .filter(|occurrence| occurrence.name.as_deref() == Some("size"))
        .count();
    if size_occurrences > 1 {
        return Err(vec![SourceDiagnostic::error(
            args.span,
            "duplicate argument: size".to_string(),
        )]);
    }
    for name in args.named.keys() {
        if name != "size" {
            return Err(vec![SourceDiagnostic::error(
                args.span,
                format!("unexpected argument: {name}"),
            )]);
        }
    }
    let (size, size_explicit) = match args.named.get("size") {
        None => (Rel::from_percent(100.0), false),
        Some(Value::Relative(size)) => (*size, true),
        Some(Value::Ratio(size)) => (Rel { rel: size.get(), abs: Length::ZERO }, true),
        Some(Value::Length(size)) => (Rel { rel: 0.0, abs: *size }, true),
        Some(other) => {
            return Err(vec![SourceDiagnostic::error(
                args.span,
                format!(
                    "expected relative length, found {}",
                    vanilla_type_name_class(other)
                ),
            )])
        }
    };
    Ok((size, size_explicit))
}

pub(super) fn p1293_math_content(value: &Value, span: Span) -> SourceResult<Content> {
    match value {
        Value::None => Ok(Content::Empty),
        Value::Content(content) => Ok(content.clone()),
        Value::LocatedContent(content, _) => Ok(content.content().clone()),
        Value::Str(text) => Ok(Content::MathText(text.clone())),
        Value::Symbol(symbol) => Ok(Content::Symbol(symbol.value.clone())),
        other => Err(vec![SourceDiagnostic::error(
            span,
            format!("expected content, found {}", vanilla_type_name_class(other)),
        )]),
    }
}

/// **P825** — nome do tipo no formato longo do vanilla para mensagens de
/// cast (`found integer` etc.): `str`→`string`, `int`→`integer`,
/// `bool`→`boolean`; os restantes coincidem com `type_name()` (mesma
/// convenção de `loading.rs:561`/`pdf.rs:88`).
pub(super) fn vanilla_type_name_class(v: &Value) -> &'static str {
    match v {
        Value::Str(_) => "string",
        Value::Int(_) => "integer",
        Value::Bool(_) => "boolean",
        other => other.type_name(),
    }
}
