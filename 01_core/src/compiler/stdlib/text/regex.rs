//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/stdlib/text/regex.md
//! @prompt-hash fba91b17
//! @layer L1
//! @updated 2026-08-13
//!
//! `regex` — constructor de `Value::Regex`.
//!
//! Extraído de `stdlib/text.rs` (2026-08-13) conforme ADR-0109 — atomização
//! forma B: a lógica vive no ficheiro da unidade, o hub é só fronteira.

use crate::compiler::eval::EvalContext;
use crate::compiler::stdlib::err;
use crate::compiler::stdlib::expect_no_named;
use crate::entities::args::Args;
use crate::entities::file_id::FileId;
use crate::entities::regex::Regex;
use crate::entities::source_result::{SourceDiagnostic, SourceResult};
use crate::entities::value::Value;


/// `regex(pattern)` → `Value::Regex` (P393).
///
/// Recebe um único argumento posicional `Str` com uma pattern regex válida.
/// Argumentos nomeados são rejeitados. Pattern inválida → erro de eval.
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
