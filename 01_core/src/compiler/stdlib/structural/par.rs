//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/stdlib/structural/par.md
//! @prompt-hash 848b794d
//! @layer L1
//! @updated 2026-08-13
//!
//! `par` e `parbreak` — parágrafo implícito no fluxo.
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

// ── P806 — `par(body, leading:?)` ────────────────────────────────────────

/// `par(body, leading:?)` → body como `Content` (parágrafo implícito).
///
/// **P806** (achado #12 de P798) — o vanilla tem `ParElem` invocável
/// (`#par[...]`); o cristalino não tem `Content::Par` (parágrafos são texto
/// plano em `Sequence` — `element_kind.rs`), logo o body é devolvido
/// **directamente**: o caso standalone é idêntico ao vanilla. A quebra de
/// fluxo block-level do vanilla (mid-paragraph) é limitação registada no L0
/// (`stdlib/structural.md` §`native_par`).
///
/// `leading:` (Length) é aplicado via `Content::Styled` com o custom
/// `"par.leading"` (mesmo canal do `#set par(leading:)`, F-5b/P373).
/// Propriedades vanilla conhecidas mas não honradas (`justify`, `spacing`,
/// `linebreaks`, `first-line-indent`, `hanging-indent`,
/// `justification-limits`) são aceites e ignoradas em silêncio (funções
/// nativas não têm acesso ao `Sink` — limitação registada).
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
            // Aceites e ignoradas (ver docstring — limitação registada).
            "justify"
            | "spacing"
            | "linebreaks"
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

    match leading {
        Some(l) => Ok(Value::Content(Content::Styled(
            Box::new(body),
            crate::entities::style::Styles::new()
                .push_custom("par.leading", Value::Length(l)),
        ))),
        None => Ok(Value::Content(body)),
    }
}
