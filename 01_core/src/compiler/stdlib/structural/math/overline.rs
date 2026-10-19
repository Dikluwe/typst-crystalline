//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/core/compiler/stdlib/structural/math/overline.md
//! @prompt-hash d2ccdc76
//! @layer L1
//! @updated 2026-09-30
//!
//! Comportamento de `math.overline` na camada stdlib.

use super::casts::math_lr_content_value;
use crate::compiler::eval::EvalContext;
use crate::entities::args::Args;
use crate::entities::content::Content;
use crate::entities::file_id::FileId;
use crate::entities::source_result::{SourceDiagnostic, SourceResult};
use crate::entities::value::Value;

/// Validação e construção compartilhada de `math.overline` e da forma bare.
pub(crate) fn math_overline_from_args(args: &Args) -> SourceResult<Content> {
    if args.named.contains_key("body") {
        return Err(vec![SourceDiagnostic::error(
            args.span,
            "the argument `body` is positional".to_string(),
        )]);
    }
    if let Some(name) = args.named.keys().next() {
        return Err(vec![SourceDiagnostic::error(
            args.span,
            format!("unexpected argument: {name}"),
        )]);
    }
    let body = match args.items.as_slice() {
        [body] => math_lr_content_value(body, args.span)?,
        [] => {
            return Err(vec![SourceDiagnostic::error(
                args.span,
                "missing argument: body".to_string(),
            )])
        }
        [_, _, ..] => {
            return Err(vec![SourceDiagnostic::error(
                args.span,
                "unexpected argument".to_string(),
            )])
        }
    };
    Ok(Content::math_overline(body))
}

pub fn native_math_overline(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    math_overline_from_args(args).map(Value::Content)
}
