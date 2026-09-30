//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/stdlib/structural/math/norm.md
//! @prompt-hash fa58b8da
//! @layer L1
//! @updated 2026-09-30
//!
//! Comportamento de `math.norm` na camada stdlib.

use super::casts::{math_lr_content_value, math_lr_size_from_args};
use crate::compiler::eval::EvalContext;
use crate::entities::args::Args;
use crate::entities::content::Content;
use crate::entities::file_id::FileId;
use crate::entities::source_result::{SourceDiagnostic, SourceResult};
use crate::entities::value::Value;

/// Validação e construção compartilhada de `math.norm` e `norm(...)` bare.
pub(crate) fn math_norm_from_args(args: &Args) -> SourceResult<Content> {
    if args.named.contains_key("body") {
        return Err(vec![SourceDiagnostic::error(
            args.span,
            "the argument `body` is positional".to_string(),
        )]);
    }
    let (size, size_explicit) = math_lr_size_from_args(args)?;
    let body = match args.items.as_slice() {
        [value] => math_lr_content_value(value, args.span)?,
        [] => {
            return Err(vec![SourceDiagnostic::error(
                args.span,
                "missing argument: body".to_string(),
            )])
        }
        _ => {
            return Err(vec![SourceDiagnostic::error(
                args.span,
                "unexpected argument".to_string(),
            )])
        }
    };
    Ok(Content::math_lr(
        Content::MathSequence(
            vec![Content::MathText("‖".into()), body, Content::MathText("‖".into())]
                .into(),
        ),
        size,
        size_explicit,
    ))
}

pub fn native_math_norm(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    math_norm_from_args(args).map(Value::Content)
}
