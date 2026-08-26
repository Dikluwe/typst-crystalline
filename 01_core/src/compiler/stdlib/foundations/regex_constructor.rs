//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/stdlib/foundations/regex.md
//! @prompt-hash d718f5ae
//! @layer L1
//! @updated 2026-08-23

use crate::compiler::eval::EvalContext;
use crate::compiler::stdlib::{err, expect_no_named};
use crate::entities::args::Args;
use crate::entities::file_id::FileId;
use crate::entities::regex::Regex;
use crate::entities::source_result::{SourceDiagnostic, SourceResult};
use crate::entities::value::Value;

/// `regex(pattern)` → `Value::Regex`.
pub fn native_regex(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    expect_no_named(&args.named)?;
    match args.items.as_slice() {
        [Value::Str(pattern)] => match Regex::new(pattern.as_str()) {
            Ok(re) => Ok(Value::Regex(re)),
            Err(e) => Err(vec![SourceDiagnostic::error(
                args.span,
                format!("regex inválida: {}", e),
            )]),
        },
        [other] => err(format!("regex() espera string, recebeu {}", other.type_name())),
        _ => err("regex() requer 1 argumento (pattern)".to_string()),
    }
}
