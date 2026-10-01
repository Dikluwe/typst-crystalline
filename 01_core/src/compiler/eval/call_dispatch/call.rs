//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/eval/call_dispatch/call.md
//! @prompt-hash 4288a7d6
//! @layer L1
//! @updated 2026-09-30

use super::*;

pub(in crate::compiler::eval) fn eval_func_call(
    call: FuncCallNode<'_>,
    scopes: &mut Scopes<'_>,
    ctx: &mut EvalContext,
    engine: &mut Engine<'_>,
) -> SourceResult<Value> {
    // As duas rotas preservam o receiver resolvido mesmo no fallback.
    let mut resolved_callee = None;
    if let Expr::FieldAccess(access) = call.callee() {
        let method = access.field().as_str();
        if matches!(method, "where" | "with") {
            let target = eval_expr(access.target(), scopes, ctx, engine)?;
            if let Value::Func(function) = target {
                let mut args = eval_args(call.args(), scopes, ctx, engine)?;
                args.span = call.span();
                let result = if method == "where" {
                    bindings::eval_element_where(&function, &args).map(Value::Selector)
                } else {
                    Ok(function_with(function, args))
                };
                return trace_call(result, Some(method), call.span(), engine);
            }
            if let Some(error) = bindings::field_callee_error(&target, access) {
                return Err(error);
            }
            resolved_callee = Some(bindings::eval_value_field_access(
                target,
                method,
                access.field().span(),
            )?);
        }
    }

    if resolved_callee.is_none() {
        if let Some(value) =
            super::selector_intercepts::try_selector_call(call, scopes, ctx, engine)?
        {
            return Ok(value);
        }

        // **P717** — Métodos mutantes (`push`/`pop`/`insert`/`remove`): mirror
        // de `maybe_resolve_mutating` (vanilla `call.rs:189-212`), sobre o
        // `access()` de P716. Tem de correr **antes** do bloco P466, que avalia
        // o target como valor (clone) — mutação exige o local. `Ok(None)` =
        // fall-through para a cadeia normal (módulos/funcs com campos com estes
        // nomes continuam a resolver abaixo).
        if let Expr::FieldAccess(access) = call.callee() {
            if bindings::is_mutating_method(access.field().as_str()) {
                if let Some(result) = bindings::try_eval_mutating_method(
                    access,
                    call.args(),
                    call.span(),
                    scopes,
                    ctx,
                    engine,
                )? {
                    return Ok(result);
                }
            }
        }

        // **P506** — Métodos de instância para `state` e `counter`.
        if let Expr::FieldAccess(access) = call.callee() {
            let target = eval_expr(access.target(), scopes, ctx, engine)?;
            let method = access.field().as_str();
            match target {
                Value::State(ref state) => {
                    return super::bindings::eval_state_method(
                        state,
                        method,
                        call.args(),
                        scopes,
                        ctx,
                        engine,
                    );
                }
                Value::Counter(ref counter) => {
                    return super::bindings::eval_counter_method_value(
                        counter,
                        method,
                        call.args(),
                        scopes,
                        ctx,
                        engine,
                    );
                }
                Value::Type(crate::entities::value::Type::Counter)
                    if matches!(method, "at" | "display") =>
                {
                    return super::bindings::eval_counter_static_method_value(
                        method,
                        call.args(),
                        scopes,
                        ctx,
                        engine,
                    );
                }
                // **P742** — Métodos de instância de `Value::Color` (9, padrão
                // P506). Só intercepta os métodos conhecidos; os restantes caem
                // no caminho genérico (erro de field access pré-P742).
                Value::SpotColorant(ref colorant) if method == "tint" => {
                    let args = eval_args(call.args(), scopes, ctx, engine)?;
                    return crate::compiler::stdlib::color::dispatch_spot_colorant_tint(
                        colorant,
                        args,
                        ctx,
                        engine.world,
                        engine.current_file,
                    );
                }
                Value::Color(ref color) => {
                    if crate::compiler::stdlib::color::is_color_instance_method(method) {
                        return super::bindings::eval_color_method(
                            color,
                            method,
                            call.args(),
                            scopes,
                            ctx,
                            engine,
                        );
                    }
                }
                Value::Gradient(ref gradient) => {
                    if crate::compiler::stdlib::gradients::is_gradient_instance_method(
                        method,
                    ) {
                        let args = eval_args(call.args(), scopes, ctx, engine)?;
                        return crate::compiler::stdlib::gradients::dispatch_gradient_method(
                        gradient,
                        method,
                        args,
                        ctx,
                        engine.world,
                        engine.current_file,
                    );
                    }
                }
                Value::Int(value) => {
                    if crate::compiler::stdlib::is_int_instance_method(method) {
                        let args = eval_args(call.args(), scopes, ctx, engine)?;
                        return crate::compiler::stdlib::dispatch_int_method(
                            value,
                            method,
                            args,
                            ctx,
                            engine.world,
                            engine.current_file,
                        );
                    }
                }
                Value::Float(value) => {
                    if crate::compiler::stdlib::is_float_instance_method(method) {
                        let spans = (method == "is-nan")
                            .then(|| FloatIsNanCallSpans::capture(call));
                        let mut args = eval_args(call.args(), scopes, ctx, engine)?;
                        if let Some(spans) = spans {
                            args.span = spans.anchor(&args, true);
                        }
                        if matches!(method, "signum" | "from-bytes" | "to-bytes") {
                            args.span = call.span();
                            let result =
                                crate::compiler::stdlib::dispatch_float_method_spanned(
                                    value,
                                    access.target().span(),
                                    method,
                                    args,
                                    ctx,
                                    engine.world,
                                    engine.current_file,
                                );
                            return trace_call(result, Some(method), call.span(), engine);
                        }
                        return crate::compiler::stdlib::dispatch_float_method(
                            value,
                            method,
                            args,
                            ctx,
                            engine.world,
                            engine.current_file,
                        );
                    }
                }
                Value::Angle(angle) if matches!(method, "deg" | "rad") => {
                    let args = eval_args(call.args(), scopes, ctx, engine)?;
                    let mut occurrences = args.occurrence_sequence();
                    occurrences.insert(
                        0,
                        ArgOccurrence {
                            name: None,
                            value: Value::Angle(angle),
                            span: access.target().span(),
                            value_span: access.target().span(),
                        },
                    );
                    let result = angle_conversion(
                        &Args::from_occurrences(call.span(), occurrences),
                        method == "deg",
                    );
                    return trace_call(result, Some(method), call.span(), engine);
                }
                value @ Value::Dir(_)
                    if matches!(method, "axis" | "end" | "inv" | "sign" | "start") =>
                {
                    let args = eval_args(call.args(), scopes, ctx, engine)?;
                    return dispatch_p1284_value_method(
                        value, method, args, scopes, ctx, engine,
                    );
                }
                value @ Value::Align(_) if matches!(method, "axis" | "inv") => {
                    let args = eval_args(call.args(), scopes, ctx, engine)?;
                    return dispatch_p1284_value_method(
                        value, method, args, scopes, ctx, engine,
                    );
                }
                value @ Value::Duration(_)
                    if matches!(
                        method,
                        "days" | "hours" | "minutes" | "seconds" | "weeks"
                    ) =>
                {
                    let args = eval_args(call.args(), scopes, ctx, engine)?;
                    return dispatch_p1284_value_method(
                        value, method, args, scopes, ctx, engine,
                    );
                }
                value @ Value::Length(_)
                    if matches!(
                        method,
                        "cm" | "inches" | "mm" | "pt" | "to-absolute"
                    ) =>
                {
                    let args = eval_args(call.args(), scopes, ctx, engine)?;
                    return dispatch_p1284_value_method(
                        value, method, args, scopes, ctx, engine,
                    );
                }
                // **P796** — `.at(index)` em `Value::Version` (único método de
                // instância; ver `entities/version.md` §8a). Só intercepta
                // "at"; outros métodos caem no caminho genérico existente.
                Value::Version(ref v) => {
                    if method == "at" {
                        return super::bindings::eval_version_method_value(
                            v,
                            method,
                            call.args(),
                            call.span(),
                            scopes,
                            ctx,
                            engine,
                        );
                    }
                }
                _ => {}
            }
        }

        if let Some(result) =
            super::collection_methods::try_collection_or_with(call, scopes, ctx, engine)?
        {
            return Ok(result);
        }

        if let Some(result) =
            super::contextual_calls::try_contextual_call(call, scopes, ctx, engine)?
        {
            return Ok(result);
        }

        if let Some(result) =
            super::content_methods::try_content_methods(call, scopes, ctx, engine)?
        {
            return Ok(result);
        }

        // **P815** — `eval_field_callee` do vanilla (`call.rs:239-345`): depois
        // de todos os despachos de método legítimos acima, um callee
        // `target.field` chamado como função cujo alvo não é
        // Symbol/Func/Type/Module produz os erros verbatim do vanilla —
        // método inexistente (`type integer has no method `foo``), dict-key-call
        // com hints, "not a valid method". Sem isto, o caminho genérico avaliava
        // o field access e errava com mensagens divergentes (ou, pior, chamava
        // funções guardadas em dict keys — medido em P815).
        if let Expr::FieldAccess(access) = call.callee() {
            let target = eval_expr(access.target(), scopes, ctx, engine)?;
            if let Some(err) = bindings::field_callee_error(&target, access) {
                return Err(err);
            }
        }
    }
    super::invoke::invoke_resolved_call(resolved_callee, call, scopes, ctx, engine)
}
