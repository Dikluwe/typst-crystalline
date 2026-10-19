//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/core/compiler/stdlib/structural/math/lr.md
//! @prompt-hash 785de4f3
//! @layer L1
//! @updated 2026-09-30
//!
//! Comportamento de `math.lr` na camada stdlib.

use super::casts::{math_lr_content_value, math_lr_size_from_args};
use crate::compiler::eval::EvalContext;
use crate::entities::args::Args;
use crate::entities::content::Content;
use crate::entities::file_id::FileId;
use crate::entities::source_result::SourceResult;
use crate::entities::value::Value;

/// Validação e construção compartilhada de `math.lr` e `lr(...)` bare.
pub(crate) fn math_lr_from_args(args: &Args) -> SourceResult<Content> {
    let (size, size_explicit) = math_lr_size_from_args(args)?;
    let mut body =
        Vec::with_capacity(args.items.len().saturating_mul(2).saturating_sub(1));
    for (index, value) in args.items.iter().enumerate() {
        if index > 0 {
            body.push(Content::text(","));
        }
        body.push(math_lr_content_value(value, args.span)?);
    }
    let body = match body.len() {
        0 => Content::Empty,
        1 => body.pop().unwrap(),
        _ => Content::MathSequence(body.into()),
    };
    Ok(Content::math_lr(body, size, size_explicit))
}

pub fn native_math_lr(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    math_lr_from_args(args).map(Value::Content)
}

/// Classify the pinned left/right symbol wrappers without calling them.
pub(crate) fn is_callable_lr_symbol(value: &str) -> bool {
    value.chars().count() == 1 && "⌈⌊(⟮⦇⦅⦓⦕{⦃[⦍⦏⟦⦋❲⟬⦗⟅⎰⎱⧘⧚⟨⧼⦑⦉⟪⌜⌞|‖⦀⦙⦚".contains(value)
}
