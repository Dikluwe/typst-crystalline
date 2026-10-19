//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/core/compiler/eval/call_dispatch/instance_methods.md
//! @prompt-hash ca9b5cd4
//! @layer L1
//! @updated 2026-10-01

use super::*;

/// Despacha métodos de instância conhecidos após os métodos mutantes.
pub(super) fn try_instance_method(
    call: FuncCallNode<'_>,
    scopes: &mut Scopes<'_>,
    ctx: &mut EvalContext,
    engine: &mut Engine<'_>,
    cached_receiver: &mut Option<Value>,
) -> SourceResult<Option<Value>> {
    // **P506** — Métodos de instância para `state` e `counter`.
    if let Expr::FieldAccess(access) = call.callee() {
        let target =
            super::call::resolve_receiver(access, cached_receiver, scopes, ctx, engine)?;
        let method = access.field().as_str();
        match target {
            Value::State(state) => {
                return (super::bindings::eval_state_method(
                    state,
                    method,
                    call.args(),
                    scopes,
                    ctx,
                    engine,
                ))
                .map(Some);
            }
            Value::Counter(counter) => {
                return (super::bindings::eval_counter_method_value(
                    counter,
                    method,
                    call.args(),
                    scopes,
                    ctx,
                    engine,
                ))
                .map(Some);
            }
            Value::Type(crate::entities::value::Type::Counter)
                if matches!(method, "at" | "display") =>
            {
                return (super::bindings::eval_counter_static_method_value(
                    method,
                    call.args(),
                    scopes,
                    ctx,
                    engine,
                ))
                .map(Some);
            }
            // **P742** — Métodos de instância de `Value::Color` (9, padrão
            // P506). Só intercepta os métodos conhecidos; os restantes caem
            // no caminho genérico (erro de field access pré-P742).
            Value::SpotColorant(colorant) if method == "tint" => {
                let args = eval_args(call.args(), scopes, ctx, engine)?;
                return (crate::compiler::stdlib::color::dispatch_spot_colorant_tint(
                    colorant,
                    args,
                    ctx,
                    engine.world,
                    engine.current_file,
                ))
                .map(Some);
            }
            Value::Color(color) => {
                if crate::compiler::stdlib::color::is_color_instance_method(method) {
                    return (super::bindings::eval_color_method(
                        color,
                        method,
                        call.args(),
                        scopes,
                        ctx,
                        engine,
                    ))
                    .map(Some);
                }
            }
            Value::Gradient(gradient) => {
                if crate::compiler::stdlib::gradients::is_gradient_instance_method(method)
                {
                    let args = eval_args(call.args(), scopes, ctx, engine)?;
                    return (crate::compiler::stdlib::gradients::dispatch_gradient_method(
                        gradient,
                        method,
                        args,
                        ctx,
                        engine.world,
                        engine.current_file,
                    )).map(Some);
                }
            }
            Value::Int(value) => {
                if crate::compiler::stdlib::is_int_instance_method(method) {
                    let args = eval_args(call.args(), scopes, ctx, engine)?;
                    return (crate::compiler::stdlib::dispatch_int_method(
                        *value,
                        method,
                        args,
                        ctx,
                        engine.world,
                        engine.current_file,
                    ))
                    .map(Some);
                }
            }
            Value::Float(value) => {
                if crate::compiler::stdlib::is_float_instance_method(method) {
                    let spans =
                        (method == "is-nan").then(|| FloatIsNanCallSpans::capture(call));
                    let mut args = eval_args(call.args(), scopes, ctx, engine)?;
                    if let Some(spans) = spans {
                        args.span = spans.anchor(&args, true);
                    }
                    if matches!(method, "signum" | "from-bytes" | "to-bytes") {
                        args.span = call.span();
                        let result =
                            crate::compiler::stdlib::dispatch_float_method_spanned(
                                *value,
                                access.target().span(),
                                method,
                                args,
                                ctx,
                                engine.world,
                                engine.current_file,
                            );
                        return (trace_call(result, Some(method), call.span(), engine))
                            .map(Some);
                    }
                    return (crate::compiler::stdlib::dispatch_float_method(
                        *value,
                        method,
                        args,
                        ctx,
                        engine.world,
                        engine.current_file,
                    ))
                    .map(Some);
                }
            }
            Value::Angle(angle) if matches!(method, "deg" | "rad") => {
                let args = eval_args(call.args(), scopes, ctx, engine)?;
                let mut occurrences = args.occurrence_sequence();
                occurrences.insert(
                    0,
                    ArgOccurrence {
                        name: None,
                        value: Value::Angle(*angle),
                        span: access.target().span(),
                        value_span: access.target().span(),
                    },
                );
                let result = angle_conversion(
                    &Args::from_occurrences(call.span(), occurrences),
                    method == "deg",
                );
                return (trace_call(result, Some(method), call.span(), engine)).map(Some);
            }
            Value::Dir(_)
                if matches!(method, "axis" | "end" | "inv" | "sign" | "start") =>
            {
                let args = eval_args(call.args(), scopes, ctx, engine)?;
                return (dispatch_p1284_value_method(
                    target.clone(),
                    method,
                    args,
                    scopes,
                    ctx,
                    engine,
                ))
                .map(Some);
            }
            Value::Align(_) if matches!(method, "axis" | "inv") => {
                let args = eval_args(call.args(), scopes, ctx, engine)?;
                return (dispatch_p1284_value_method(
                    target.clone(),
                    method,
                    args,
                    scopes,
                    ctx,
                    engine,
                ))
                .map(Some);
            }
            Value::Duration(_)
                if matches!(
                    method,
                    "days" | "hours" | "minutes" | "seconds" | "weeks"
                ) =>
            {
                let args = eval_args(call.args(), scopes, ctx, engine)?;
                return (dispatch_p1284_value_method(
                    target.clone(),
                    method,
                    args,
                    scopes,
                    ctx,
                    engine,
                ))
                .map(Some);
            }
            Value::Length(_)
                if matches!(method, "cm" | "inches" | "mm" | "pt" | "to-absolute") =>
            {
                let args = eval_args(call.args(), scopes, ctx, engine)?;
                return (dispatch_p1284_value_method(
                    target.clone(),
                    method,
                    args,
                    scopes,
                    ctx,
                    engine,
                ))
                .map(Some);
            }
            // **P796** — `.at(index)` em `Value::Version` (único método de
            // instância; ver `entities/version.md` §8a). Só intercepta
            // "at"; outros métodos caem no caminho genérico existente.
            Value::Version(v) => {
                if method == "at" {
                    return (super::bindings::eval_version_method_value(
                        v,
                        method,
                        call.args(),
                        call.span(),
                        scopes,
                        ctx,
                        engine,
                    ))
                    .map(Some);
                }
            }
            _ => {}
        }
    }

    Ok(None)
}
