//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/stdlib/structural/math/annotations.md
//! @prompt-hash 41a5bafd
//! @layer L1
//! @updated 2026-09-30
//!
//! Comportamento de `math.annotations` na camada stdlib.

use super::casts::math_lr_content_value;
use crate::compiler::eval::EvalContext;
use crate::entities::args::Args;
use crate::entities::content::Content;
use crate::entities::file_id::FileId;
use crate::entities::source_result::{SourceDiagnostic, SourceResult};
use crate::entities::value::Value;

/// Validação e construção compartilhada de `math.overbrace` e da forma bare.
pub(crate) fn math_overbrace_from_args(args: &Args) -> SourceResult<Content> {
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
    let (body, annotation, annotation_explicit) = match args.items.as_slice() {
        [body] => (math_lr_content_value(body, args.span)?, None, false),
        [body, Value::None] => (math_lr_content_value(body, args.span)?, None, true),
        [body, annotation @ (Value::Content(_)
        | Value::LocatedContent(_, _)
        | Value::Str(_)
        | Value::Symbol(_))] => (
            math_lr_content_value(body, args.span)?,
            Some(math_lr_content_value(annotation, args.span)?),
            true,
        ),
        [] => {
            return Err(vec![SourceDiagnostic::error(
                args.span,
                "missing argument: body".to_string(),
            )])
        }
        [_, _] | [_, _, ..] => {
            return Err(vec![SourceDiagnostic::error(
                args.span,
                "unexpected argument".to_string(),
            )])
        }
    };
    Ok(Content::math_overbrace(body, annotation, annotation_explicit))
}

pub fn native_math_overbrace(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    math_overbrace_from_args(args).map(Value::Content)
}

/// Validação e construção compartilhada de `math.underbrace` e da forma bare.
pub(crate) fn math_underbrace_from_args(args: &Args) -> SourceResult<Content> {
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
    let (body, annotation, annotation_explicit) = match args.items.as_slice() {
        [body] => (math_lr_content_value(body, args.span)?, None, false),
        [body, Value::None] => (math_lr_content_value(body, args.span)?, None, true),
        [body, annotation @ (Value::Content(_)
        | Value::LocatedContent(_, _)
        | Value::Str(_)
        | Value::Symbol(_))] => (
            math_lr_content_value(body, args.span)?,
            Some(math_lr_content_value(annotation, args.span)?),
            true,
        ),
        [] => {
            return Err(vec![SourceDiagnostic::error(
                args.span,
                "missing argument: body".to_string(),
            )])
        }
        [_, _] | [_, _, ..] => {
            return Err(vec![SourceDiagnostic::error(
                args.span,
                "unexpected argument".to_string(),
            )])
        }
    };
    Ok(Content::math_underbrace(body, annotation, annotation_explicit))
}

pub fn native_math_underbrace(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    math_underbrace_from_args(args).map(Value::Content)
}

/// Validação e construção compartilhada de `math.underbracket` e da forma bare.
pub(crate) fn math_underbracket_from_args(args: &Args) -> SourceResult<Content> {
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
    let (body, annotation, annotation_explicit) = match args.items.as_slice() {
        [body] => (math_lr_content_value(body, args.span)?, None, false),
        [body, Value::None] => (math_lr_content_value(body, args.span)?, None, true),
        [body, annotation @ (Value::Content(_)
        | Value::LocatedContent(_, _)
        | Value::Str(_)
        | Value::Symbol(_))] => (
            math_lr_content_value(body, args.span)?,
            Some(math_lr_content_value(annotation, args.span)?),
            true,
        ),
        [] => {
            return Err(vec![SourceDiagnostic::error(
                args.span,
                "missing argument: body".to_string(),
            )])
        }
        [_, _] | [_, _, ..] => {
            return Err(vec![SourceDiagnostic::error(
                args.span,
                "unexpected argument".to_string(),
            )])
        }
    };
    Ok(Content::math_underbracket(body, annotation, annotation_explicit))
}

pub fn native_math_underbracket(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    math_underbracket_from_args(args).map(Value::Content)
}

/// Validação e construção compartilhada de `math.underparen` e da forma bare.
pub(crate) fn math_underparen_from_args(args: &Args) -> SourceResult<Content> {
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
    let (body, annotation, annotation_explicit) = match args.items.as_slice() {
        [body] => (math_lr_content_value(body, args.span)?, None, false),
        [body, Value::None] => (math_lr_content_value(body, args.span)?, None, true),
        [body, annotation @ (Value::Content(_)
        | Value::LocatedContent(_, _)
        | Value::Str(_)
        | Value::Symbol(_))] => (
            math_lr_content_value(body, args.span)?,
            Some(math_lr_content_value(annotation, args.span)?),
            true,
        ),
        [] => {
            return Err(vec![SourceDiagnostic::error(
                args.span,
                "missing argument: body".to_string(),
            )])
        }
        [_, _] | [_, _, ..] => {
            return Err(vec![SourceDiagnostic::error(
                args.span,
                "unexpected argument".to_string(),
            )])
        }
    };
    Ok(Content::math_underparen(body, annotation, annotation_explicit))
}

pub fn native_math_underparen(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    math_underparen_from_args(args).map(Value::Content)
}

/// Validação e construção compartilhada de `math.undershell` e da forma bare.
pub(crate) fn math_undershell_from_args(args: &Args) -> SourceResult<Content> {
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
    let (body, annotation, annotation_explicit) = match args.items.as_slice() {
        [body] => (math_lr_content_value(body, args.span)?, None, false),
        [body, Value::None] => (math_lr_content_value(body, args.span)?, None, true),
        [body, annotation @ (Value::Content(_)
        | Value::LocatedContent(_, _)
        | Value::Str(_)
        | Value::Symbol(_))] => (
            math_lr_content_value(body, args.span)?,
            Some(math_lr_content_value(annotation, args.span)?),
            true,
        ),
        [] => {
            return Err(vec![SourceDiagnostic::error(
                args.span,
                "missing argument: body".to_string(),
            )])
        }
        [_, _] | [_, _, ..] => {
            return Err(vec![SourceDiagnostic::error(
                args.span,
                "unexpected argument".to_string(),
            )])
        }
    };
    Ok(Content::math_undershell(body, annotation, annotation_explicit))
}

pub fn native_math_undershell(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    math_undershell_from_args(args).map(Value::Content)
}

/// Validação e construção compartilhada de `math.overbracket` e da forma bare.
pub(crate) fn math_overbracket_from_args(args: &Args) -> SourceResult<Content> {
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
    let (body, annotation, annotation_explicit) = match args.items.as_slice() {
        [body] => (math_lr_content_value(body, args.span)?, None, false),
        [body, Value::None] => (math_lr_content_value(body, args.span)?, None, true),
        [body, annotation @ (Value::Content(_)
        | Value::LocatedContent(_, _)
        | Value::Str(_)
        | Value::Symbol(_))] => (
            math_lr_content_value(body, args.span)?,
            Some(math_lr_content_value(annotation, args.span)?),
            true,
        ),
        [] => {
            return Err(vec![SourceDiagnostic::error(
                args.span,
                "missing argument: body".to_string(),
            )])
        }
        [_, _] | [_, _, ..] => {
            return Err(vec![SourceDiagnostic::error(
                args.span,
                "unexpected argument".to_string(),
            )])
        }
    };
    Ok(Content::math_overbracket(body, annotation, annotation_explicit))
}

pub fn native_math_overbracket(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    math_overbracket_from_args(args).map(Value::Content)
}

/// Validação e construção compartilhada de `math.overparen` e da forma bare.
pub(crate) fn math_overparen_from_args(args: &Args) -> SourceResult<Content> {
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
    let (body, annotation, annotation_explicit) = match args.items.as_slice() {
        [body] => (math_lr_content_value(body, args.span)?, None, false),
        [body, Value::None] => (math_lr_content_value(body, args.span)?, None, true),
        [body, annotation @ (Value::Content(_)
        | Value::LocatedContent(_, _)
        | Value::Str(_)
        | Value::Symbol(_))] => (
            math_lr_content_value(body, args.span)?,
            Some(math_lr_content_value(annotation, args.span)?),
            true,
        ),
        [] => {
            return Err(vec![SourceDiagnostic::error(
                args.span,
                "missing argument: body".to_string(),
            )])
        }
        [_, _] | [_, _, ..] => {
            return Err(vec![SourceDiagnostic::error(
                args.span,
                "unexpected argument".to_string(),
            )])
        }
    };
    Ok(Content::math_overparen(body, annotation, annotation_explicit))
}

pub fn native_math_overparen(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    math_overparen_from_args(args).map(Value::Content)
}

/// Validação e construção compartilhada de `math.overshell` e da forma bare.
pub(crate) fn math_overshell_from_args(args: &Args) -> SourceResult<Content> {
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
    let (body, annotation, annotation_explicit) = match args.items.as_slice() {
        [body] => (math_lr_content_value(body, args.span)?, None, false),
        [body, Value::None] => (math_lr_content_value(body, args.span)?, None, true),
        [body, annotation @ (Value::Content(_)
        | Value::LocatedContent(_, _)
        | Value::Str(_)
        | Value::Symbol(_))] => (
            math_lr_content_value(body, args.span)?,
            Some(math_lr_content_value(annotation, args.span)?),
            true,
        ),
        [] => {
            return Err(vec![SourceDiagnostic::error(
                args.span,
                "missing argument: body".to_string(),
            )])
        }
        [_, _] | [_, _, ..] => {
            return Err(vec![SourceDiagnostic::error(
                args.span,
                "unexpected argument".to_string(),
            )])
        }
    };
    Ok(Content::math_overshell(body, annotation, annotation_explicit))
}

pub fn native_math_overshell(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    math_overshell_from_args(args).map(Value::Content)
}
