//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/eval/call_dispatch/value_methods.md
//! @prompt-hash 09c6fe39
//! @layer L1
//! @updated 2026-09-30

use super::*;

pub(crate) fn take_p1339_self(args: &Args) -> SourceResult<(Value, Span, Args)> {
    let occurrences = args.occurrence_sequence();
    if let Some(arg) = occurrences.iter().find(|arg| arg.name.is_none()) {
        let mut rest = args.clone();
        rest.remove_positional(0);
        let span = if arg.value_span.is_detached() { args.span } else { arg.value_span };
        return Ok((arg.value.clone(), span, rest));
    }
    if let Some(arg) = occurrences.iter().find(|arg| arg.name.as_deref() == Some("self"))
    {
        let span = if arg.span.is_detached() { args.span } else { arg.span };
        return Err(vec![SourceDiagnostic::error(
            span,
            "the argument `self` is positional",
        )
        .with_hint("try removing `self:`")]);
    }
    Err(vec![SourceDiagnostic::error(args.span, "missing argument: self")])
}

pub(super) fn angle_conversion(args: &Args, degrees: bool) -> SourceResult<Value> {
    let (receiver, span, rest) = take_p1339_self(args)?;
    let Value::Angle(angle) = receiver else {
        return Err(vec![SourceDiagnostic::error(
            span,
            format!("expected angle, found {}", vanilla_type_name(&receiver)),
        )]);
    };
    if let Some(arg) = rest.occurrence_sequence().first() {
        let message = arg
            .name
            .as_ref()
            .map(|name| format!("unexpected argument: {name}"))
            .unwrap_or_else(|| "unexpected argument".into());
        let span = if arg.span.is_detached() { args.span } else { arg.span };
        return Err(vec![SourceDiagnostic::error(span, message)]);
    }
    Ok(Value::Float(if degrees { angle.to_deg() } else { angle.to_rad() }))
}

pub(super) fn angle_deg_static(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _file: crate::entities::file_id::FileId,
) -> SourceResult<Value> {
    angle_conversion(args, true)
}

pub(super) fn angle_rad_static(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _file: crate::entities::file_id::FileId,
) -> SourceResult<Value> {
    angle_conversion(args, false)
}

pub(super) fn function_with_static(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _file: crate::entities::file_id::FileId,
) -> SourceResult<Value> {
    let (receiver, span, rest) = take_p1339_self(args)?;
    let Value::Func(function) = receiver else {
        return Err(vec![SourceDiagnostic::error(
            span,
            format!("expected function, found {}", vanilla_type_name(&receiver)),
        )]);
    };
    Ok(function_with(function, rest))
}

pub(super) fn function_with(function: Func, args: Args) -> Value {
    Value::Func(function.with(args))
}

pub(super) fn p1284_no_args(args: &Args) -> SourceResult<()> {
    if !args.items.is_empty() || !args.named.is_empty() {
        return Err(vec![SourceDiagnostic::error(args.span, "unexpected argument")]);
    }
    Ok(())
}

pub(super) fn p1284_static_call(
    method: &str,
    args: &Args,
    scopes: &mut Scopes<'_>,
    ctx: &mut EvalContext,
    engine: &mut Engine<'_>,
) -> SourceResult<Value> {
    let Some(target) = args.items.first() else {
        return Err(vec![SourceDiagnostic::error(args.span, "missing argument: self")]);
    };
    let mut rest = args.clone();
    rest.remove_positional(0);
    dispatch_p1284_value_method(target.clone(), method, rest, scopes, ctx, engine)
}

macro_rules! p1284_static {
    ($rust:ident, $method:literal) => {
        pub(super) fn $rust(
            ctx: &mut EvalContext,
            args: &Args,
            _world: &dyn crate::contracts::world::World,
            _file: crate::entities::file_id::FileId,
            scopes: &mut Scopes<'_>,
            engine: &mut Engine<'_>,
        ) -> SourceResult<Value> {
            p1284_static_call($method, args, scopes, ctx, engine)
        }
    };
}

p1284_static!(direction_axis_static, "axis");
p1284_static!(direction_end_static, "end");
p1284_static!(direction_inv_static, "inv");
p1284_static!(direction_sign_static, "sign");
p1284_static!(direction_start_static, "start");
p1284_static!(alignment_axis_static, "axis");
p1284_static!(alignment_inv_static, "inv");
p1284_static!(duration_days_static, "days");
p1284_static!(duration_hours_static, "hours");
p1284_static!(duration_minutes_static, "minutes");
p1284_static!(duration_seconds_static, "seconds");
p1284_static!(duration_weeks_static, "weeks");
p1284_static!(length_pt_static, "pt");
p1284_static!(length_mm_static, "mm");
p1284_static!(length_cm_static, "cm");
p1284_static!(length_inches_static, "inches");
p1284_static!(length_to_absolute_static, "to-absolute");
p1284_static!(selector_and_static, "and");
p1284_static!(selector_or_static, "or");
p1284_static!(selector_within_static, "within");
p1284_static!(selector_before_static, "before");
p1284_static!(selector_after_static, "after");
p1284_static!(state_at_static, "at");
p1284_static!(state_final_static, "final");
p1284_static!(state_get_static, "get");
p1284_static!(state_update_static, "update");
p1284_static!(location_page_static, "page");
p1284_static!(location_page_numbering_static, "page-numbering");
p1284_static!(location_position_static, "position");

pub(super) fn direction_from(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _file: crate::entities::file_id::FileId,
    _scopes: &mut Scopes<'_>,
    _engine: &mut Engine<'_>,
) -> SourceResult<Value> {
    direction_from_to(args, false)
}

pub(super) fn direction_to(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _file: crate::entities::file_id::FileId,
    _scopes: &mut Scopes<'_>,
    _engine: &mut Engine<'_>,
) -> SourceResult<Value> {
    direction_from_to(args, true)
}

pub(super) fn direction_from_to(args: &Args, to: bool) -> SourceResult<Value> {
    use crate::entities::dir::Dir;
    use crate::entities::layout_types::{HAlign, VAlign};
    if !args.named.is_empty() || args.items.len() != 1 {
        return Err(vec![SourceDiagnostic::error(args.span, "expected one side")]);
    }
    let direction = match &args.items[0] {
        Value::Align(align) => match (align.h, align.v, to) {
            (Some(HAlign::Left), None, false) => Dir::LTR,
            (Some(HAlign::Right), None, false) => Dir::RTL,
            (None, Some(VAlign::Top), false) => Dir::TTB,
            (None, Some(VAlign::Bottom), false) => Dir::BTT,
            (Some(HAlign::Right), None, true) => Dir::LTR,
            (Some(HAlign::Left), None, true) => Dir::RTL,
            (None, Some(VAlign::Bottom), true) => Dir::TTB,
            (None, Some(VAlign::Top), true) => Dir::BTT,
            _ => {
                return Err(vec![SourceDiagnostic::error(
                    args.span,
                    "expected side alignment",
                )]);
            }
        },
        other => {
            return Err(vec![SourceDiagnostic::error(
                args.span,
                format!("expected alignment, found {}", vanilla_type_name(other)),
            )]);
        }
    };
    Ok(Value::Dir(direction))
}

pub(super) fn dispatch_p1284_value_method(
    target: Value,
    method: &str,
    args: Args,
    _scopes: &mut Scopes<'_>,
    ctx: &mut EvalContext,
    engine: &mut Engine<'_>,
) -> SourceResult<Value> {
    use crate::entities::dir::Dir;
    use crate::entities::layout_types::{Abs, Align2D, HAlign, Length, VAlign};

    let inverse_h = |h| match h {
        HAlign::Left => HAlign::Right,
        HAlign::Right => HAlign::Left,
        HAlign::Start => HAlign::End,
        HAlign::End => HAlign::Start,
        HAlign::Center => HAlign::Center,
    };
    let inverse_v = |v| match v {
        VAlign::Top => VAlign::Bottom,
        VAlign::Bottom => VAlign::Top,
        VAlign::Horizon => VAlign::Horizon,
    };

    match target {
        Value::Dir(direction) => {
            p1284_no_args(&args)?;
            Ok(match method {
                "axis" => Value::Str(
                    if direction.is_horizontal() { "horizontal" } else { "vertical" }
                        .into(),
                ),
                "sign" => Value::Int(if direction.is_reverse() { -1 } else { 1 }),
                "inv" => Value::Dir(match direction {
                    Dir::LTR => Dir::RTL,
                    Dir::RTL => Dir::LTR,
                    Dir::TTB => Dir::BTT,
                    Dir::BTT => Dir::TTB,
                }),
                "start" | "end" => {
                    let start = method == "start";
                    let alignment = match (direction, start) {
                        (Dir::LTR, true) | (Dir::RTL, false) => {
                            Align2D { h: Some(HAlign::Left), v: None }
                        }
                        (Dir::LTR, false) | (Dir::RTL, true) => {
                            Align2D { h: Some(HAlign::Right), v: None }
                        }
                        (Dir::TTB, true) | (Dir::BTT, false) => {
                            Align2D { h: None, v: Some(VAlign::Top) }
                        }
                        (Dir::TTB, false) | (Dir::BTT, true) => {
                            Align2D { h: None, v: Some(VAlign::Bottom) }
                        }
                    };
                    Value::Align(alignment)
                }
                _ => {
                    return Err(vec![SourceDiagnostic::error(
                        args.span,
                        "unknown method",
                    )]);
                }
            })
        }
        Value::Align(alignment) => {
            p1284_no_args(&args)?;
            Ok(match method {
                "axis" => match (alignment.h, alignment.v) {
                    (Some(_), None) => Value::Str("horizontal".into()),
                    (None, Some(_)) => Value::Str("vertical".into()),
                    _ => Value::None,
                },
                "inv" => Value::Align(Align2D {
                    h: alignment.h.map(inverse_h),
                    v: alignment.v.map(inverse_v),
                }),
                _ => {
                    return Err(vec![SourceDiagnostic::error(
                        args.span,
                        "unknown method",
                    )]);
                }
            })
        }
        Value::Duration(duration) => {
            p1284_no_args(&args)?;
            let seconds = duration.nanos as f64 / 1_000_000_000.0;
            Ok(Value::Float(match method {
                "seconds" => seconds,
                "minutes" => seconds / 60.0,
                "hours" => seconds / 3_600.0,
                "days" => seconds / 86_400.0,
                "weeks" => seconds / 604_800.0,
                _ => {
                    return Err(vec![SourceDiagnostic::error(
                        args.span,
                        "unknown method",
                    )]);
                }
            }))
        }
        Value::Length(length) => {
            p1284_no_args(&args)?;
            if method == "to-absolute" {
                if !ctx.in_context {
                    return Err(vec![SourceDiagnostic::error(
                        args.span,
                        "can only be used when context is known",
                    )]);
                }
                let points = length.abs.to_pt() + length.em * engine.styles.size();
                return Ok(Value::Length(Length { abs: Abs(points), em: 0.0 }));
            }
            if length.em != 0.0 {
                return Err(vec![SourceDiagnostic::error(
                    args.span,
                    "relative length cannot be converted without context",
                )]);
            }
            let points = length.abs.to_pt();
            Ok(Value::Float(match method {
                "pt" => points,
                "mm" => points * 25.4 / 72.0,
                "cm" => points * 2.54 / 72.0,
                "inches" => points / 72.0,
                _ => {
                    return Err(vec![SourceDiagnostic::error(
                        args.span,
                        "unknown method",
                    )]);
                }
            }))
        }
        Value::Selector(base) => {
            if method == "before" || method == "after" {
                return selector_before_after(base, method, args).map(Value::Selector);
            }
            if !args.named.is_empty() {
                return Err(vec![SourceDiagnostic::error(
                    args.span,
                    "unexpected argument",
                )]);
            }
            match method {
                "and" | "or" => {
                    let mut selectors = vec![base];
                    for value in &args.items {
                        let selector = bindings::value_to_query_selector(value)
                            .ok_or_else(|| {
                                vec![SourceDiagnostic::error(
                                    args.span,
                                    format!(
                                        "expected selector, found {}",
                                        value.type_name()
                                    ),
                                )]
                            })?;
                        selectors.push(selector);
                    }
                    Ok(Value::Selector(if method == "and" {
                        crate::entities::selector::Selector::And(EcoVec::from(selectors))
                    } else {
                        crate::entities::selector::Selector::Or(EcoVec::from(selectors))
                    }))
                }
                "within" => {
                    let [ancestor] = args.items.as_slice() else {
                        return Err(vec![SourceDiagnostic::error(
                            args.span,
                            "within requires one ancestor",
                        )]);
                    };
                    let ancestor = bindings::value_to_query_selector(ancestor)
                        .ok_or_else(|| {
                            vec![SourceDiagnostic::error(
                                args.span,
                                format!(
                                    "expected selector, found {}",
                                    ancestor.type_name()
                                ),
                            )]
                        })?;
                    Ok(Value::Selector(crate::entities::selector::Selector::Within {
                        base: Box::new(base),
                        ancestor: Box::new(ancestor),
                    }))
                }
                _ => Err(vec![SourceDiagnostic::error(args.span, "unknown method")]),
            }
        }
        Value::State(state) => {
            use crate::compiler::stdlib::state::{
                state_at_location, state_final, state_get,
            };
            match method {
                "update" => match args.items.as_slice() {
                    [value] if args.named.is_empty() => {
                        Ok(crate::compiler::stdlib::state::state_method_update(
                            state.key.clone(),
                            value.clone(),
                        ))
                    }
                    _ => Err(vec![SourceDiagnostic::error(
                        args.span,
                        "state.update() requires one argument",
                    )]),
                },
                "get" => {
                    p1284_no_args(&args)?;
                    state_get(&state, ctx, engine, args.span)
                }
                "final" => {
                    p1284_no_args(&args)?;
                    state_final(&state, ctx, engine, args.span)
                }
                "at" => match args.items.as_slice() {
                    [Value::Location(location)] if args.named.is_empty() => {
                        state_at_location(&state, *location, ctx, engine, args.span)
                    }
                    [other] => Err(vec![SourceDiagnostic::error(
                        args.span,
                        format!("expected location, found {}", other.type_name()),
                    )]),
                    _ => Err(vec![SourceDiagnostic::error(
                        args.span,
                        "state.at() requires one selector",
                    )]),
                },
                _ => Err(vec![SourceDiagnostic::error(args.span, "unknown method")]),
            }
        }
        Value::Location(location) => {
            p1284_no_args(&args)?;
            eval_location_method(location, method, ctx, args.span)
        }
        other => Err(vec![SourceDiagnostic::error(
            args.span,
            format!("type {} has no method `{method}`", other.type_name()),
        )]),
    }
}

pub(in crate::compiler::eval) fn selector_before_after(
    base: crate::entities::selector::Selector,
    method: &str,
    mut args: Args,
) -> SourceResult<crate::entities::selector::Selector> {
    let inclusive = match args.remove_named("inclusive") {
        None => true,
        Some(Value::Bool(value)) => value,
        Some(other) => {
            return Err(vec![SourceDiagnostic::error(
                args.span,
                format!("expected boolean, found {}", other.type_name()),
            )]);
        }
    };
    if let Some(name) = args.named.keys().next() {
        return Err(vec![SourceDiagnostic::error(
            args.span,
            format!("unexpected argument: {name}"),
        )]);
    }
    let [boundary] = args.items.as_slice() else {
        return Err(vec![SourceDiagnostic::error(
            args.span,
            format!("{method} requires one selector"),
        )]);
    };
    let boundary = bindings::value_to_query_selector(boundary).ok_or_else(|| {
        vec![SourceDiagnostic::error(
            args.span,
            format!("expected selector, found {}", boundary.type_name()),
        )]
    })?;
    let selector = if method == "before" {
        crate::entities::selector::Selector::Before {
            base: Box::new(base),
            end: Box::new(boundary),
            inclusive,
        }
    } else {
        crate::entities::selector::Selector::After {
            base: Box::new(base),
            start: Box::new(boundary),
            inclusive,
        }
    };
    Ok(selector)
}
