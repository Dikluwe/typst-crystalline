//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/core/compiler/stdlib/structural/math/underline.md
//! @prompt-hash b4d77e20
//! @layer L1
//! @updated 2026-09-30
//!
//! Comportamento de `math.underline` na camada stdlib.

use crate::compiler::eval::EvalContext;
use crate::entities::args::Args;
use crate::entities::content::Content;
use crate::entities::file_id::FileId;
use crate::entities::source_result::{SourceDiagnostic, SourceResult};
use crate::entities::value::Value;

/// `math.underline(body)` — construtor body-only do elemento matemático.
pub fn native_math_underline(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    let body = match args.items.as_slice() {
        [body] => super::casts::math_lr_content_value(body, args.span)?,
        [] => {
            return Err(vec![SourceDiagnostic::error(
                args.span,
                "missing argument: body".to_string(),
            )])
        }
        _ => return Err(vec![SourceDiagnostic::error(args.span, "unexpected argument")]),
    };

    if let Some(name) = args.named.keys().next() {
        return Err(vec![SourceDiagnostic::error(
            args.span,
            format!("unexpected argument: {name}"),
        )]);
    }

    Ok(Value::Content(Content::math_underline(body)))
}
