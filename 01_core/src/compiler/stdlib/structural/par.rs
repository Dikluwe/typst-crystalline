//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/stdlib/structural/par.md
//! @prompt-hash f9d6d7e4
//! @layer L1
//! @updated 2026-09-30
//!
//! `par` e `parbreak` — elemento e fronteira de parágrafo no fluxo.
//!
//! Separado de `structural/flow.rs` em 2026-08-13: a fronteira anterior
//! estava sustentada por um cluster de co-mudança que não existia (artefacto
//! de atribuição da ferramenta). Fronteira nova medida no L0 deste nó.

use crate::compiler::eval::EvalContext;
use crate::entities::args::Args;
use crate::entities::content::Content;
use crate::entities::file_id::FileId;
use crate::entities::layout_types::Length;
use crate::entities::source_result::{SourceDiagnostic, SourceResult};
use crate::entities::span::Span;
use crate::entities::value::Value;

/// `par.line` is a settable element-like value, but cannot be constructed.
///
/// Line numbers are materialized by the physical line closer. Calling this
/// function directly must therefore fail before producing any `Content`.
pub fn native_par_line(
    _ctx: &mut EvalContext,
    _args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    Err(vec![SourceDiagnostic::error(Span::detached(), "cannot be constructed manually")])
}

/// `parbreak()` → marker de quebra de parágrafo já usado pela sintaxe markup.
///
/// P1140.17 restaura somente a superfície chamável ratificada. A realização e
/// o colapso de quebras consecutivas permanecem nos consumers existentes.
pub fn native_parbreak(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    if !args.items.is_empty() {
        return Err(vec![SourceDiagnostic::error(
            args.span,
            "unexpected argument".to_string(),
        )]);
    }

    if let Some(name) = args.named.keys().next() {
        return Err(vec![SourceDiagnostic::error(
            args.span,
            format!("unexpected argument: {name}"),
        )]);
    }

    Ok(Value::Content(Content::Parbreak))
}

/// Build a paragraph element with validated lexical paragraph options.
/// Global optimized breaking remains incomplete.
pub fn native_par(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    let body = match args.items.first() {
        Some(Value::Content(c)) => c.clone(),
        Some(Value::Str(s)) => Content::text(s.as_str()),
        Some(other) => {
            return Err(vec![SourceDiagnostic::error(
                Span::detached(),
                format!("expected content, found {}", other.type_name()),
            )])
        }
        None => {
            // Literal vanilla (`typst-library/src/model/par.rs` — medido:
            // `#par()` → "missing argument: body").
            return Err(vec![SourceDiagnostic::error(
                Span::detached(),
                "missing argument: body".to_string(),
            )]);
        }
    };

    let mut leading: Option<Length> = None;
    let mut styles = crate::entities::style::Styles::new();
    for (key, value) in args.named.iter() {
        match key.as_str() {
            "leading" => match value {
                Value::Length(l) => leading = Some(*l),
                other => {
                    return Err(vec![SourceDiagnostic::error(
                        Span::detached(),
                        format!(
                            "par(leading:) espera length, recebeu {}",
                            other.type_name()
                        ),
                    )])
                }
            },
            "justify" => {
                let span = args
                    .occurrences
                    .as_ref()
                    .and_then(|items| {
                        items
                            .iter()
                            .rev()
                            .find(|item| item.name.as_deref() == Some("justify"))
                    })
                    .map_or(args.span, |item| item.value_span);
                match value {
                    Value::Bool(_) => {
                        styles = styles.push_custom("par.justify", value.clone())
                    }
                    other => {
                        return Err(vec![break_policy_error(
                            "expected boolean",
                            other,
                            span,
                        )])
                    }
                }
            }
            "linebreaks" => {
                let span = args
                    .occurrences
                    .as_ref()
                    .and_then(|items| {
                        items
                            .iter()
                            .rev()
                            .find(|item| item.name.as_deref() == Some("linebreaks"))
                    })
                    .map_or(args.span, |item| item.value_span);
                let expected = "expected \"simple\", \"optimized\", or auto";
                match value {
                    Value::Auto => {}
                    Value::Str(name)
                        if matches!(name.as_str(), "simple" | "optimized") => {}
                    Value::Str(_) => {
                        return Err(vec![SourceDiagnostic::error(span, expected)])
                    }
                    other => return Err(vec![break_policy_error(expected, other, span)]),
                }
                styles = styles.push_custom("par.linebreaks", value.clone());
            }
            // Known properties awaiting their own consumers.
            "spacing"
            | "first-line-indent"
            | "hanging-indent"
            | "justification-limits" => {}
            other => {
                return Err(vec![SourceDiagnostic::error(
                    Span::detached(),
                    format!("argumento nomeado inesperado em par(): '{}'", other),
                )])
            }
        }
    }

    if let Some(l) = leading {
        styles = styles.push_custom("par.leading", Value::Length(l));
    }
    let body = Content::par(body);
    Ok(Value::Content(if styles.is_empty() {
        body
    } else {
        Content::styled(Box::new(body), styles)
    }))
}

/// Diagnostic vocabulary differs from the public short names of types.
fn break_policy_error(expected: &str, found: &Value, span: Span) -> SourceDiagnostic {
    let name = match found.type_name() {
        "int" => "integer",
        "str" => "string",
        "bool" => "boolean",
        other => other,
    };
    SourceDiagnostic::error(span, format!("{expected}, found {name}"))
}
