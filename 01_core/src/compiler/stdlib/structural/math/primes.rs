//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/stdlib/structural/math/primes.md
//! @prompt-hash 4e351bfc
//! @layer L1
//! @updated 2026-09-30
//!
//! Comportamento de `math.primes` na camada stdlib.

use super::casts::vanilla_type_name_class;
use crate::compiler::eval::EvalContext;
use crate::entities::args::Args;
use crate::entities::content::Content;
use crate::entities::file_id::FileId;
use crate::entities::source_result::{SourceDiagnostic, SourceResult};
use crate::entities::value::Value;

/// `math.primes(count)` — grupo semântico de apóstrofos matemáticos.
pub fn native_math_primes(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    if args.named.contains_key("count") {
        return Err(vec![SourceDiagnostic::error(
            args.span,
            "the argument `count` is positional".to_string(),
        )]);
    }
    if let Some(name) = args.named.keys().next() {
        return Err(vec![SourceDiagnostic::error(
            args.span,
            format!("unexpected argument: {name}"),
        )]);
    }
    let count = match args.items.as_slice() {
        [] => {
            return Err(vec![SourceDiagnostic::error(
                args.span,
                "missing argument: count".to_string(),
            )])
        }
        [Value::Int(count)] if *count < 0 => {
            return Err(vec![SourceDiagnostic::error(
                args.span,
                "number must be at least zero".to_string(),
            )])
        }
        [Value::Int(count)] => usize::try_from(*count).map_err(|_| {
            vec![SourceDiagnostic::error(args.span, "number is too large".to_string())]
        })?,
        [other] => {
            return Err(vec![SourceDiagnostic::error(
                args.span,
                format!("expected integer, found {}", vanilla_type_name_class(other)),
            )])
        }
        [_, _, ..] => {
            return Err(vec![SourceDiagnostic::error(
                args.span,
                "unexpected argument".to_string(),
            )])
        }
    };
    Ok(Value::Content(Content::math_primes(count)))
}
