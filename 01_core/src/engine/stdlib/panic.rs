//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/engine/stdlib/panic.md
//! @prompt-hash e236743a
//! @layer L1
//! @updated 2026-06-22
//!
//! Função nativa `panic`.

use crate::engine::eval::EvalContext;
use crate::entities::args::Args;
use crate::entities::file_id::FileId;
use crate::entities::source_result::{SourceDiagnostic, SourceResult};
use crate::entities::value::Value;

/// `panic(..)` → aborta a avaliação com erro.
///
/// **P843 (F6)** — paridade vanilla (`panic` em foundations/mod.rs:140-152,
/// medido em `temp/p843/f6_*.typ`): variádico; a mensagem é
/// `panicked with: {valores}` com separador `, ` — strings cruas,
/// não-strings via `repr`. Sem argumentos → `panicked` (sem `with:`).
pub fn native_panic(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    super::expect_no_named(&args.named)?;

    let mut msg = String::from("panicked");
    if !args.items.is_empty() {
        msg.push_str(" with: ");
        for (i, value) in args.items.iter().enumerate() {
            if i > 0 {
                msg.push_str(", ");
            }
            match value {
                Value::Str(s) => msg.push_str(s.as_str()),
                other => {
                    msg.push_str(&crate::engine::eval::repr::repr_value(other));
                }
            }
        }
    }
    Err(vec![SourceDiagnostic::error(args.span, msg)])
}
