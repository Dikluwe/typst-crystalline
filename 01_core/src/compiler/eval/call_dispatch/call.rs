//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/eval/call_dispatch/call.md
//! @prompt-hash 2e3fb67a
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

        if let Some(result) =
            super::instance_methods::try_instance_method(call, scopes, ctx, engine)?
        {
            return Ok(result);
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
