//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/stdlib/foundations/repr.md
//! @prompt-hash f7d2663f
//! @layer L1
//! @updated 2026-08-13
//!
//! `repr(v)` — representação textual reconhecível do valor.
//! Fatiado de `foundations.rs` no Passo 1032.

use ecow::EcoString;

use crate::compiler::eval::repr::repr_value;
use crate::compiler::eval::EvalContext;
use crate::entities::args::Args;
use crate::entities::file_id::FileId;
use crate::entities::source_result::SourceResult;
use crate::entities::value::Value;

use crate::compiler::stdlib::{err, expect_no_named};

/// `repr(v)` → representação textual reconhecível do valor (P421).
pub fn native_repr(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    expect_no_named(&args.named)?;
    match args.items.as_slice() {
        [v] => Ok(Value::Str(EcoString::from(repr_value(v)))),
        _ => err(format!("repr() requer 1 argumento, recebeu {}", args.items.len())),
    }
}
