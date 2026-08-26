//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/stdlib/structural/footnote.md
//! @prompt-hash 4fc1ced8
//! @layer L1
//! @updated 2026-08-13
//!
//! `footnote` — construção da nota (o render vive em `compiler/layout/footnote.rs`).
//!
//! Separado de `structural/flow.rs` em 2026-08-13: a fronteira anterior
//! estava sustentada por um cluster de co-mudança que não existia (artefacto
//! de atribuição da ferramenta). Fronteira nova medida no L0 deste nó.

use crate::compiler::eval::EvalContext;
use crate::entities::args::Args;
use crate::entities::content::Content;
use crate::entities::file_id::FileId;
use crate::entities::source_result::{SourceDiagnostic, SourceResult};
use crate::entities::span::Span;
use crate::entities::value::Value;
use ecow::EcoString;

// ── Passo 157A (ADR-0060 Fase 2 sub-passo 1) — table minimal ────────────────

/// `footnote(body, numbering:?)` — emite `Content::Footnote { body, numbering }`.
/// Body posicional obrigatório (content ou string).
/// P502: `numbering` named opcional; uso no marcador scope-out per ADR-0054.
pub fn native_footnote(
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
                format!(
                    "footnote() espera content ou string, recebeu {}",
                    other.type_name()
                ),
            )])
        }
        None => {
            return Err(vec![SourceDiagnostic::error(
                Span::detached(),
                "footnote() exige body como argumento posicional".to_string(),
            )])
        }
    };

    let mut numbering: Option<EcoString> = None;
    for (k, v) in args.named.iter() {
        match k.as_str() {
            "numbering" => {
                numbering = match v {
                    Value::Str(s) => Some(s.clone()),
                    Value::None => None,
                    _ => {
                        return Err(vec![SourceDiagnostic::error(
                            Span::detached(),
                            format!(
                                "footnote(numbering:): espera string, recebeu {}",
                                v.type_name()
                            ),
                        )])
                    }
                };
            }
            other => {
                return Err(vec![SourceDiagnostic::error(
                    Span::detached(),
                    format!("footnote(): argumento nomeado '{}' não suportado (cosméticos scope-out per ADR-0054 graded)", other),
                )]);
            }
        }
    }

    Ok(Value::Content(Content::footnote_with_numbering(body, numbering)))
}

// ── Passo 296 — `accent()` + `cancel()` math (P-math-accent-cancel) ──────
//
// **A.0.0 N=4 (HIV)**: features ausentes apesar de Tabela A.4
// marcar `parcial`. Materialização from-scratch. Padrão "variant
// rico" N=4 **preservado** (A.2 → (a) minimal; sem cosméticos
// `size`/`length`/`inverted`/`cross`/`angle`/`stroke`).
