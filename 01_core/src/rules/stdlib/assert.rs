//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/rules/stdlib.md
//! @prompt-hash 68fc3823
//! @layer L1
//! @updated 2026-04-23
//!
//! Função nativa `assert`.
//! Extraído de `stdlib.rs` no Passo 96.5 conforme ADR-0037.

use crate::entities::args::Args;
use crate::entities::file_id::FileId;
use crate::entities::source_result::{SourceDiagnostic, SourceResult};
use crate::entities::span::Span;
use crate::entities::value::Value;
use crate::rules::eval::EvalContext;

// ── `assert()` — prova de fogo dos named args (Passo 66, DEBT-16) ───────────

/// `assert(condition, message: ...)` → sem output; erro se condição for falsa.
///
/// Primeira função com named arg documentado (não apenas tolerado).
/// Prova de que o mecanismo de named args (DEBT-16) funciona de ponta a ponta.
pub fn native_assert(_ctx: &mut EvalContext, args: &Args, _world: &dyn crate::contracts::world::World, _current_file: FileId, _figure_numbering: Option<&str>) -> SourceResult<Value> {
    // Validar named args: apenas "message" é aceite.
    for key in args.named.keys() {
        if key.as_str() != "message" {
            return Err(vec![SourceDiagnostic::error(
                Span::detached(),
                format!("argumento nomeado inesperado: '{}'", key),
            )]);
        }
    }

    // Argumento posicional: condição (obrigatório).
    let condition = match args.items.first() {
        Some(Value::Bool(b)) => *b,
        Some(other) => return Err(vec![SourceDiagnostic::error(
            Span::detached(),
            format!("assert() requer condição booleana, recebeu {}", other.type_name()),
        )]),
        None => return Err(vec![SourceDiagnostic::error(
            Span::detached(),
            "assert() requer 1 argumento posicional (condição)".to_string(),
        )]),
    };

    // Argumento nomeado: message (opcional).
    let message = args.named.get("message")
        .map(|v| match v {
            Value::Str(s)     => s.to_string(),
            Value::Content(c) => c.plain_text(),
            other             => other.type_name().to_string(),
        })
        .unwrap_or_else(|| "Asserção falhou".to_string());

    if !condition {
        return Err(vec![SourceDiagnostic::error(Span::detached(), message)]);
    }

    Ok(Value::None)
}
