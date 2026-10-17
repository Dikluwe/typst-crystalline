from pathlib import Path
p=Path('01_core/src/compiler/stdlib/transforms.rs');header='\n'.join(p.read_text().splitlines()[:5])+'\n\n';p.write_text(header+'''use std::sync::Arc;

use crate::compiler::eval::EvalContext;
use crate::compiler::eval::operators::error_formatting::vanilla_type_name;
use crate::entities::args::{ArgOccurrence, Args};
use crate::entities::content::Content;
use crate::entities::elements::transform::{TransformElem, TransformOperation, TransformScale};
use crate::entities::file_id::FileId;
use crate::entities::layout_types::{Align2D, Angle, Length};
use crate::entities::rel::Rel;
use crate::entities::source_result::{SourceDiagnostic, SourceResult};
use crate::entities::span::Span;
use crate::entities::value::Value;

fn type_error(arg: &ArgOccurrence, expected: &str) -> Vec<SourceDiagnostic> {
    vec![SourceDiagnostic::error(arg.value_span,
        format!("expected {expected}, found {}", vanilla_type_name(&arg.value)))]
}

fn named(args: &mut Vec<ArgOccurrence>, name: &str) -> Option<ArgOccurrence> {
    let last = args.iter().rev().find(|a| a.name.as_deref() == Some(name)).cloned();
    args.retain(|a| a.name.as_deref() != Some(name));
    last
}

fn angle(arg: ArgOccurrence) -> SourceResult<Angle> {
    match arg.value { Value::Angle(a) => Ok(a), _ => Err(type_error(&arg, "angle")) }
}

fn origin(args: &mut Vec<ArgOccurrence>) -> SourceResult<Option<Align2D>> {
    named(args, "origin").map(|a| match a.value {
        Value::Align(v) => Ok(v), _ => Err(type_error(&a, "alignment")),
    }).transpose()
}

fn reflow(args: &mut Vec<ArgOccurrence>) -> SourceResult<Option<bool>> {
    named(args, "reflow").map(|a| match a.value {
        Value::Bool(v) => Ok(v), _ => Err(type_error(&a, "boolean")),
    }).transpose()
}

fn relative(arg: ArgOccurrence) -> SourceResult<Rel<Length>> {
    match arg.value {
        Value::Relative(r) => Ok(r),
        Value::Length(l) => Ok(Rel { rel: 0.0, abs: l }),
        Value::Ratio(r) => Ok(Rel { rel: r.0, abs: Length::ZERO }),
        _ => Err(type_error(&arg, "relative length")),
    }
}

fn scale(arg: ArgOccurrence) -> SourceResult<TransformScale> {
    match arg.value {
        Value::Auto => Ok(TransformScale::Auto),
        Value::Ratio(r) => Ok(TransformScale::Ratio(r)),
        Value::Length(l) => Ok(TransformScale::Length(l)),
        _ => Err(type_error(&arg, "ratio, length, or auto")),
    }
}

fn finish(operation: TransformOperation, mut args: Vec<ArgOccurrence>, span: Span) -> SourceResult<Value> {
    let body = if let Some(i) = args.iter().position(|a| a.name.is_none()) {
        let a = args.remove(i);
        match a.value {
            Value::Content(c) => c,
            Value::LocatedContent(c, _) => c.content().clone(),
            Value::Str(s) => Content::text(s),
            Value::None => Content::Empty,
            _ => return Err(type_error(&a, "content")),
        }
    } else {
        let message = if args.iter().any(|a| a.name.as_deref() == Some("body")) {
            "the argument `body` is positional"
        } else { "missing argument: body" };
        return Err(vec![SourceDiagnostic::error(span, message)]);
    };
    if let Some(a) = args.first() {
        return Err(vec![SourceDiagnostic::error(a.span, match &a.name {
            Some(n) => format!("unexpected argument: {n}"),
            None => "unexpected argument".into(),
        })]);
    }
    Ok(Value::Content(Content::Transform(Arc::new(TransformElem { operation, body }))))
}

fn occurrences(args: &Args) -> Vec<ArgOccurrence> {
    args.occurrence_sequence().into_iter().map(|mut a| {
        a.span = a.span.or(args.span);
        a.value_span = a.value_span.or(a.span);
        a
    }).collect()
}

/// Move retains relative offsets until the region and font are known.
pub fn native_move(_ctx: &mut EvalContext, args: &Args,
    _world: &dyn crate::contracts::world::World, _current_file: FileId) -> SourceResult<Value> {
    let mut rest = occurrences(args);
    let dx = named(&mut rest, "dx").map(relative).transpose()?;
    let dy = named(&mut rest, "dy").map(relative).transpose()?;
    finish(TransformOperation::Move { dx, dy }, rest, args.span)
}

/// The optional positional angle is found by type, including after body.
pub fn native_rotate(_ctx: &mut EvalContext, args: &Args,
    _world: &dyn crate::contracts::world::World, _current_file: FileId) -> SourceResult<Value> {
    let mut rest = occurrences(args);
    let angle = rest.iter().position(|a| a.name.is_none() && matches!(a.value, Value::Angle(_)))
        .map(|i| angle(rest.remove(i))).transpose()?;
    let origin = origin(&mut rest)?;
    let reflow = reflow(&mut rest)?;
    finish(TransformOperation::Rotate { angle, origin, reflow }, rest, args.span)
}

/// A positional factor sets both axes; named values override independently.
pub fn native_scale(_ctx: &mut EvalContext, args: &Args,
    _world: &dyn crate::contracts::world::World, _current_file: FileId) -> SourceResult<Value> {
    let mut rest = occurrences(args);
    let factor = rest.iter().position(|a| a.name.is_none() && matches!(a.value, Value::Auto | Value::Ratio(_) | Value::Length(_)))
        .map(|i| scale(rest.remove(i))).transpose()?;
    let x = named(&mut rest, "x").map(scale).transpose()?.or(factor);
    let y = named(&mut rest, "y").map(scale).transpose()?.or(factor);
    let origin = origin(&mut rest)?;
    let reflow = reflow(&mut rest)?;
    finish(TransformOperation::Scale { x, y, origin, reflow }, rest, args.span)
}

/// Skew accepts named angles without a construction-time near-90-degree cutoff.
pub fn native_skew(_ctx: &mut EvalContext, args: &Args,
    _world: &dyn crate::contracts::world::World, _current_file: FileId) -> SourceResult<Value> {
    let mut rest = occurrences(args);
    let ax = named(&mut rest, "ax").map(angle).transpose()?;
    let ay = named(&mut rest, "ay").map(angle).transpose()?;
    let origin = origin(&mut rest)?;
    let reflow = reflow(&mut rest)?;
    finish(TransformOperation::Skew { ax, ay, origin, reflow }, rest, args.span)
}
''')
p=Path('01_core/src/compiler/eval/repr/content.rs');s=p.read_text();s=s.replace('''let fields = crate::compiler::eval::bindings::field_access::content_methods::content_fields(c)
                    .into_iter().map(|(name, value)| format!("{name}: {}", repr_value(&value))).collect::<Vec<_>>();''','''let names: &[&str] = match t.operation {
                    crate::entities::elements::transform::TransformOperation::Move { .. } => &["dx", "dy", "body"],
                    crate::entities::elements::transform::TransformOperation::Rotate { .. } => &["angle", "origin", "reflow", "body"],
                    crate::entities::elements::transform::TransformOperation::Scale { .. } => &["x", "y", "origin", "reflow", "body"],
                    crate::entities::elements::transform::TransformOperation::Skew { .. } => &["ax", "ay", "origin", "reflow", "body"],
                    crate::entities::elements::transform::TransformOperation::Matrix(_) => &[],
                };
                let fields = names.iter().filter_map(|name| c.get_field(name).map(|value| format!("{name}: {}", repr_value(&value)))).collect::<Vec<_>>();''');p.write_text(s)
