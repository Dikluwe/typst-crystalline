//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/engine/stdlib/panic.md
//! @prompt-hash 8d34d84b
//! @layer L1
//! @updated 2026-06-22
//!
//! Função nativa `panic`.

use crate::entities::args::Args;
use crate::entities::file_id::FileId;
use crate::entities::source_result::{SourceDiagnostic, SourceResult};
use crate::entities::value::Value;
use crate::engine::eval::EvalContext;

/// `panic(msg)` → aborta a avaliação com a mensagem dada.
///
/// Recebe um único argumento posicional `Str`. Zero tipo novo; reutiliza o
/// mecanismo de erro de eval (`SourceDiagnostic`).
pub fn native_panic(_ctx: &mut EvalContext, args: &Args, _world: &dyn crate::contracts::world::World, _current_file: FileId) -> SourceResult<Value> {
    super::expect_no_named(&args.named)?;

    match args.items.as_slice() {
        [Value::Str(msg)] => Err(vec![SourceDiagnostic::error(args.span, msg.as_str())]),
        [other] => Err(vec![SourceDiagnostic::error(
            args.span,
            format!("panic() espera string, recebeu {}", other.type_name()),
        )]),
        _ => Err(vec![SourceDiagnostic::error(
            args.span,
            "panic() requer 1 argumento (mensagem)",
        )]),
    }
}
