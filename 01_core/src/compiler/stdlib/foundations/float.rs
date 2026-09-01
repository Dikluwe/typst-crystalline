//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/stdlib/foundations/float.md
//! @prompt-hash 5c5346b2
//! @layer L1
//! @updated 2026-08-31
//!
//! Superfície e semântica numérica de `float.is-infinite`.

use crate::compiler::eval::operators::error_formatting::vanilla_type_name;
use crate::compiler::eval::EvalContext;
use crate::contracts::world::World;
use crate::entities::args::Args;
use crate::entities::file_id::FileId;
use crate::entities::func::Func;
use crate::entities::source_result::{SourceDiagnostic, SourceResult};
use crate::entities::value::Value;

fn native_is_infinite(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn World,
    _current_file: FileId,
) -> SourceResult<Value> {
    if let Some((name, _)) = args.named.first() {
        return Err(vec![SourceDiagnostic::error(
            args.span,
            format!("unexpected argument: {name}"),
        )]);
    }

    let value = match args.items.as_slice() {
        [] => {
            return Err(vec![SourceDiagnostic::error(
                args.span,
                "missing argument: self".to_string(),
            )])
        }
        [Value::Float(value)] => *value,
        [Value::Int(value)] => *value as f64,
        [other] => {
            return Err(vec![SourceDiagnostic::error(
                args.span,
                format!("expected float, found {}", vanilla_type_name(other)),
            )])
        }
        [_, ..] => {
            return Err(vec![SourceDiagnostic::error(
                args.span,
                "unexpected argument".to_string(),
            )])
        }
    };

    Ok(Value::Bool(value.is_infinite()))
}

pub(crate) fn float_type_field(field: &str) -> Option<Value> {
    match field {
        "is-infinite" => {
            Some(Value::Func(Func::native("is-infinite", native_is_infinite)))
        }
        _ => None,
    }
}

pub(crate) fn is_float_instance_method(method: &str) -> bool {
    method == "is-infinite"
}

pub(crate) fn dispatch_float_method(
    receiver: f64,
    method: &str,
    mut args: Args,
    ctx: &mut EvalContext,
    world: &dyn World,
    current_file: FileId,
) -> SourceResult<Value> {
    match method {
        "is-infinite" => {
            args.items.insert(0, Value::Float(receiver));
            native_is_infinite(ctx, &args, world, current_file)
        }
        _ => unreachable!("dispatch_float_method called with unknown method: {method}"),
    }
}
