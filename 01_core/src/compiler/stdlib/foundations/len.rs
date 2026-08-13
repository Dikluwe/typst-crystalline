//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/stdlib/foundations/len.md
//! @prompt-hash 0d064c16
//! @layer L1
//! @updated 2026-08-13
//!
//! `len(v)` — comprimento de Str, Array ou Dict.
//! Fatiado de `foundations.rs` no Passo 1032.

use crate::compiler::eval::EvalContext;
use crate::entities::args::Args;
use crate::entities::file_id::FileId;
use crate::entities::source_result::{SourceResult};
use crate::entities::value::Value;

use crate::compiler::stdlib::{err, expect_no_named};

/// `len(v)` → comprimento de Str, Array ou Dict.
pub fn native_len(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    expect_no_named(&args.named)?;
    match args.items.as_slice() {
        [Value::Str(s)] => Ok(Value::Int(s.chars().count() as i64)),
        [Value::Array(a)] => Ok(Value::Int(a.len() as i64)),
        [Value::Dict(d)] => Ok(Value::Int(d.len() as i64)),
        [other] => err(format!("len() não suporta {}", other.type_name())),
        _ => err(format!("len() requer 1 argumento, recebeu {}", args.items.len())),
    }
}
