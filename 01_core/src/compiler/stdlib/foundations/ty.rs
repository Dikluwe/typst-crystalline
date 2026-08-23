//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/stdlib/foundations/ty.md
//! @prompt-hash 11a6302c
//! @layer L1
//! @updated 2026-08-13
//!
//! `type(v)` — valor-tipo do argumento.
//! Fatiado de `foundations.rs` no Passo 1032.

use crate::compiler::eval::EvalContext;
use crate::entities::args::Args;
use crate::entities::file_id::FileId;
use crate::entities::source_result::SourceResult;
use crate::entities::value::Value;

use crate::compiler::stdlib::{err, expect_no_named};

/// `type(v)` → valor-tipo do argumento (`Value::Type`). P685: paridade vanilla —
/// `type(1) == int` funciona por comparação directa de valores de tipo.
pub fn native_type(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    expect_no_named(&args.named)?;
    match args.items.as_slice() {
        [v] => Ok(Value::Type(v.type_of())),
        _ => err(format!("type() requer 1 argumento, recebeu {}", args.items.len())),
    }
}
