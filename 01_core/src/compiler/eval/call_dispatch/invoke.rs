//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/eval/call_dispatch/invoke.md
//! @prompt-hash 95c555ba
//! @layer L1
//! @updated 2026-09-30

use super::*;

pub(super) fn invoke_resolved_call(
    resolved_callee: Option<Value>,
    call: FuncCallNode<'_>,
    scopes: &mut Scopes<'_>,
    ctx: &mut EvalContext,
    engine: &mut Engine<'_>,
) -> SourceResult<Value> {
    let callee = match resolved_callee {
        Some(value) => value,
        None => eval_expr(call.callee(), scopes, ctx, engine)?,
    };

    let float_is_nan_spans = matches!(
        &callee,
        Value::Func(f)
            if matches!(f.repr(), FuncRepr::Native(n) if n.name == "is-nan")
    )
    .then(|| FloatIsNanCallSpans::capture(call));

    let p1293_math_spans = match &callee {
        Value::Func(func) => P1293MathNative::from_func(func)
            .map(|identity| (identity, P1293MathCallSpans::capture(call))),
        _ => None,
    };
    let p1293_html_spans = match &callee {
        Value::Func(func) => P1293HtmlNative::from_func(func)
            .map(|kind| P1293HtmlCallSpans::capture(call, kind)),
        _ => None,
    };

    // **P846 (#56)** — `eval`: ancorar os erros da re-avaliação ao span do
    // **literal string** (primeiro argumento posicional), paridade com o
    // `SpanMode::Uniform` do vanilla (`foundations/mod.rs:267,318` — medido:
    // `span7.typ` van `4:2` vs cris `3:5`). Solução pontual só para `eval`
    // (override de `args.span` no call site), sem o débito estrutural de
    // span-por-argumento em `Args` (P772s). O cast error (`#eval(5)` →
    // `1:6`) fica igualmente corrigido — o vanilla ancora erros de validação
    // de argumento no span do argumento.
    let eval_anchor = match &callee {
        Value::Func(f) if matches!(f.repr(), FuncRepr::NativeWithEngine(n) if n.name == "eval") => {
            call.args().items().find_map(|arg| match arg {
                Arg::Pos(expr) => Some(expr.span()),
                _ => None,
            })
        }
        _ => None,
    };

    let is_mat_call = matches!(
        &callee,
        Value::Func(func)
            if func
                .name()
                .is_some_and(|name| name == "mat" || name.ends_with(".mat"))
    );
    let isolate_mat_args = is_mat_call && !ctx.apply_show_rules;
    let mut args = if isolate_mat_args {
        let array_lengths = scopes
            .iter_all()
            .filter_map(|(name, value)| match value {
                Value::Array(values) => Some((name.to_string(), values.len())),
                _ => None,
            })
            .collect::<Vec<_>>();
        let mut isolated = Scopes {
            top: scopes.top.clone(),
            scopes: scopes.scopes.clone(),
            captured: scopes
                .captured
                .as_ref()
                .map(|scope| std::sync::Arc::new((**scope).clone())),
            captured_by: scopes.captured_by,
            base: scopes.base,
        };
        let args = eval_args(call.args(), &mut isolated, ctx, engine)?;
        for (name, length) in array_lengths {
            if let Some(Value::Array(values)) = scopes.get_mut(&name) {
                values.truncate(length);
            }
        }
        args
    } else {
        let before = (is_mat_call && ctx.apply_show_rules).then(|| {
            scopes
                .iter_all()
                .filter_map(|(name, value)| match value {
                    Value::Array(values) => Some((name.to_string(), values.clone())),
                    _ => None,
                })
                .collect::<Vec<_>>()
        });
        let args = eval_args(call.args(), scopes, ctx, engine)?;
        if let Some(before) = before {
            for (name, prefix) in before {
                if prefix.is_empty() {
                    continue;
                }
                if let Some(Value::Array(values)) = scopes.get_mut(&name) {
                    if values.len() == prefix.len() * 2
                        && values[..prefix.len()] == prefix
                        && values[prefix.len()..] == prefix
                    {
                        values.truncate(prefix.len());
                    }
                }
            }
        }
        args
    };
    if let Some(spans) = float_is_nan_spans {
        args.span = spans.anchor(&args, false);
    } else if let Some((identity, spans)) = p1293_math_spans {
        args.span = spans.anchor(identity, &args);
    } else if let Some(anchor) = eval_anchor {
        args.span = anchor;
    }

    match callee {
        Value::Func(func) => {
            transport_native_call_span(&func, &mut args, call.span());
            let html_args = p1293_html_spans.as_ref().map(|_| args.clone());
            let mut result = apply_func(func.clone(), args, scopes, ctx, engine);
            if let (Some(spans), Some(args), Err(errors)) =
                (p1293_html_spans.as_ref(), html_args.as_ref(), &mut result)
            {
                if let Some(error) = errors.first_mut() {
                    error.span = spans.anchor(args, &error.message);
                }
            }
            // **P846 (#57)** — call trace: um `Tracepoint::Call` por chamada
            // cujo span não contém o erro (mirror de `call_func` +
            // `Trace::trace` do vanilla — `typst-eval/src/call.rs:166-180`,
            // `typst-library/src/diag.rs:464-479`).
            let result = trace_call(result, func.name(), call.span(), engine)?;
            // Intercepção eager — show rules aplicadas após apply_func (Passo 68).
            if let Value::Content(c) = result {
                Ok(Value::Content(rules::intercept_content(c, ctx, engine)?))
            } else {
                Ok(result)
            }
        }
        // P685 — nomes de tipo chamáveis: `int("5")`, `str(5)`, `float("3.5")`,
        // `type(1)`. Despacha para o construtor nativo; tipos não chamáveis
        // não cobertos conservam o diagnóstico de construtor ausente.
        // P1339 acrescenta somente a conversão autorizada Array ← Bytes.
        Value::Type(t) => {
            let world = engine.world;
            let current_file = engine.current_file;
            match t {
                Type::Int => native_int(ctx, &args, world, current_file),
                Type::Float => native_float(ctx, &args, world, current_file),
                Type::Str => native_str(ctx, &args, world, current_file),
                Type::Type => native_type(ctx, &args, world, current_file),
                Type::Path => native_path(ctx, &args, world, current_file),
                // P737 — `counter`/`state` são tipos chamáveis (paridade
                // vanilla — medido: type(counter)/type(state) → type;
                // counter("x")/state("y", 0) criam instâncias).
                Type::Counter => native_counter(ctx, &args, world, current_file),
                Type::State => native_state(ctx, &args, world, current_file),
                // P765a — `symbol(...)` constructor.
                Type::Symbol => native_symbol(ctx, &args, world, current_file),
                // P843 (F4/F5) — constructors `bytes(...)` e `datetime(...)`.
                Type::Bytes => native_bytes(ctx, &args, world, current_file),
                Type::Array if matches!(args.items.first(), Some(Value::Bytes(_))) => {
                    let result = native_array_bytes(ctx, &args, world, current_file);
                    trace_call(result, Some("array"), call.span(), engine)
                }
                // P1284 — `arguments(...)` preserva simultaneamente a ordem
                // dos posicionais e dos named na representação já existente.
                Type::Arguments => Ok(Value::Args(args)),
                Type::Datetime => native_datetime(ctx, &args, world, current_file),
                // P1140.1-B — tipos públicos chamáveis, delegados aos
                // mesmos construtores atomizados da fase estrutural.
                Type::Decimal => native_decimal(ctx, &args, world, current_file),
                Type::Duration => native_duration(ctx, &args, world, current_file),
                Type::Regex => native_regex(ctx, &args, world, current_file),
                Type::Selector => native_selector(ctx, &args, world, current_file),
                Type::Stroke => native_stroke(ctx, &args, world, current_file),
                Type::Tiling => native_tiling(ctx, &args, world, current_file),
                Type::Version => native_version(ctx, &args, world, current_file),
                // P1140.2 — `label` é tipo chamável e produz Value::Label.
                Type::Label => native_label(ctx, &args, world, current_file),
                // P1473 — `color.spot(name, fallback)` constructor.
                Type::SpotColorant => {
                    native_spot_colorant_construct(ctx, &args, world, current_file)
                }
                other => Err(vec![SourceDiagnostic::error(
                    call.callee().span(),
                    format!("type {} does not have a constructor", other.name()),
                )]),
            }
        }
        other => Err(vec![SourceDiagnostic::error(
            call.callee().span(),
            format!("não é possível chamar {}", other.type_name()),
        )]),
    }
}
