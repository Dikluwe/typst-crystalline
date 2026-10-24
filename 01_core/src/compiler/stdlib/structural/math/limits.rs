//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/core/compiler/stdlib/structural/math/limits.md
//! @prompt-hash a752e4b5
//! @layer L1
//! @updated 2026-09-30
//!
//! Comportamento de `math.limits` na camada stdlib.

use super::casts::vanilla_type_name_class;
use crate::compiler::eval::EvalContext;
use crate::entities::args::Args;
use crate::entities::content::Content;
use crate::entities::file_id::FileId;
use crate::entities::source_result::{SourceDiagnostic, SourceResult};
use crate::entities::value::Value;

/// `scripts(body)` — força attachments laterais reutilizando o constructor
/// canónico de `MathLimitsOverride`.
pub(in crate::compiler) fn native_math_scripts(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    if let Some((name, _)) = args.named.iter().next() {
        return Err(vec![SourceDiagnostic::error(
            args.span,
            format!("unexpected argument: {name}"),
        )]);
    }

    match args.items.as_slice() {
        [body] => Ok(Value::Content(Content::math_limits_override(
            super::casts::math_lr_content_value(body, args.span)?,
            false,
            true,
            false,
        ))),
        [] => Err(vec![SourceDiagnostic::error(
            args.span,
            "missing argument: body".to_string(),
        )]),
        _ => Err(vec![SourceDiagnostic::error(
            args.span,
            "unexpected argument".to_string(),
        )]),
    }
}

/// `math.limits(body, inline: true)` — constructor qualificado convergente
/// com a sintaxe bare, inclusive na presença causal do named `inline`.
pub(in crate::compiler) fn native_math_limits(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    if args.named.contains_key("body") {
        return Err(vec![SourceDiagnostic::error(
            args.span,
            "the argument `body` is positional".to_string(),
        )]);
    }

    let inline_explicit = args.named.contains_key("inline");
    let inline = match args.named.get("inline") {
        Some(Value::Bool(value)) => *value,
        Some(other) => {
            return Err(vec![SourceDiagnostic::error(
                args.span,
                format!("expected boolean, found {}", vanilla_type_name_class(other)),
            )])
        }
        None => true,
    };

    if let Some((name, _)) = args.named.iter().find(|(name, _)| name.as_str() != "inline")
    {
        return Err(vec![SourceDiagnostic::error(
            args.span,
            format!("unexpected argument: {name}"),
        )]);
    }

    let body = match args.items.as_slice() {
        [body] => super::casts::math_lr_content_value(body, args.span)?,
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

    Ok(Value::Content(Content::math_limits_override(body, true, inline, inline_explicit)))
}
