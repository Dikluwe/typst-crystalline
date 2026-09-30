//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/eval/call_dispatch/call.md
//! @prompt-hash 8091039f
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

        // **P466** — Métodos de instância para `array`, `dict` e `str`.
        if let Expr::FieldAccess(access) = call.callee() {
            let target = eval_expr(access.target(), scopes, ctx, engine)?;
            let method = access.field().as_str();
            let trace_arguments_callback =
                matches!(&target, Value::Args(_)) && matches!(method, "filter" | "map");
            let mut positional = Vec::new();
            let mut named: IndexMap<EcoString, Span, FxBuildHasher> = IndexMap::default();
            for arg in call.args().items() {
                match arg {
                    Arg::Pos(expr) => positional.push(expr.span()),
                    Arg::Named(named_arg) => {
                        named.insert(named_arg.name().as_str().into(), named_arg.span());
                    }
                    Arg::Spread(_) => {}
                }
            }
            let call_spans = CollectionCallSpans { call: call.span(), positional, named };
            let args = eval_args(call.args(), scopes, ctx, engine)?;
            if let Some(result) = try_dispatch_collection_method(
                target,
                method,
                args,
                Some(&call_spans),
                scopes,
                ctx,
                engine,
            ) {
                return if trace_arguments_callback {
                    trace_call(result, Some(method), call.span(), engine)
                } else {
                    result
                };
            }
        }

        // **P702** — `f.with(...)`: aplicação parcial de argumentos, disponível
        // em qualquer `Value::Func` (nativa com/sem namespace, closure, elemento,
        // plugin, ou já parcialmente aplicada). Mesmo padrão de intercepção das
        // secções acima; se o alvo não for `Value::Func`, não intercepta — cai
        // no field access genérico, sem mudança de comportamento para
        // não-funções (ver `entities/func.md` §"Variante `With`").
        if let Expr::FieldAccess(access) = call.callee() {
            if access.field().as_str() == "with" {
                let target = eval_expr(access.target(), scopes, ctx, engine)?;
                if let Value::Func(f) = target {
                    let args = eval_args(call.args(), scopes, ctx, engine)?;
                    return Ok(Value::Func(f.with(args)));
                }
            }
        }

        // **P712** — `measure(body)` / `std.measure(body)`: precisa de
        // `engine.styles` (o tamanho medido depende do `#set text(size:)`
        // activo, paridade com o vanilla `context.styles()`) e do gate
        // `ctx.in_context` (mesma convenção já usada por `counter.get()`/
        // `state.get()`, `stdlib/counter.rs:144`/`stdlib/state.rs:63`) —
        // nenhum dos dois acessível pela assinatura genérica
        // `NativeFn(ctx, args, world, file)`. Verifica primeiro a forma
        // sintáctica do callee (barato, sem side-effects) para não avaliar
        // nada quando não é sequer chamado "measure"; só depois avalia e
        // compara identidade de fn-ptr (`native_fn_addr`, não o nome — mesmo
        // padrão de `bindings::eval_element_where`, `bindings.rs:354`) para
        // não capturar um `measure` sombreado pelo utilizador.
        let measure_name_matches = match call.callee() {
            Expr::Ident(ident) => ident.as_str() == "measure",
            Expr::FieldAccess(access) => access.field().as_str() == "measure",
            _ => false,
        };
        if measure_name_matches {
            let target = eval_expr(call.callee(), scopes, ctx, engine)?;
            if let Value::Func(ref f) = target {
                if f.native_fn_addr().is_some_and(|addr| {
                    std::ptr::fn_addr_eq(addr, native_measure as fn(_, _, _, _) -> _)
                }) {
                    if !ctx.in_context {
                        return Err(vec![SourceDiagnostic::error(
                            call.callee().span(),
                            "measure() can only be used inside context".to_string(),
                        )]);
                    }
                    let args = eval_args(call.args(), scopes, ctx, engine)?;
                    let body = extract_measure_body(&args)?;
                    let (width_pt, height_pt, baseline_pt) =
                        crate::compiler::layout::measure_content_real_with_baseline(
                            &body,
                            engine.styles,
                            engine.font_metrics,
                        );
                    let mut dict: IndexMap<EcoString, Value, FxBuildHasher> =
                        IndexMap::default();
                    dict.insert(
                        "width".into(),
                        Value::Length(crate::entities::layout_types::Length::pt(
                            width_pt,
                        )),
                    );
                    dict.insert(
                        "height".into(),
                        Value::Length(crate::entities::layout_types::Length::pt(
                            height_pt,
                        )),
                    );
                    dict.insert(
                        "baseline".into(),
                        Value::Length(crate::entities::layout_types::Length::pt(
                            baseline_pt,
                        )),
                    );
                    return Ok(Value::Dict(dict));
                }
            }
        }

        // **P792** — `layout(func)`: paridade vanilla `layout/layout.rs:66`.
        // Chama a callback com as dimensões disponíveis (single-pass graded:
        // `available_width`/`available_height` calculadas a partir de
        // `engine.styles`). Mesmo padrão de intercepção que `measure`/P712:
        // verifica nome sintáctico → avalia o callee → compara fn-ptr
        // `native_layout` para não capturar um `layout` sombreado.
        let layout_name_matches = match call.callee() {
            Expr::Ident(ident) => ident.as_str() == "layout",
            Expr::FieldAccess(access) => access.field().as_str() == "layout",
            _ => false,
        };
        if layout_name_matches {
            let target = eval_expr(call.callee(), scopes, ctx, engine)?;
            if let Value::Func(ref f) = target {
                if f.native_fn_addr().is_some_and(|addr| {
                    std::ptr::fn_addr_eq(addr, native_layout as fn(_, _, _, _) -> _)
                }) {
                    let args = eval_args(call.args(), scopes, ctx, engine)?;
                    // Extrair a callback — único argumento posicional obrigatório.
                    let func = match args.items.as_slice() {
                        [Value::Func(f)] => f.clone(),
                        [other] => {
                            return Err(vec![SourceDiagnostic::error(
                                call.callee().span(),
                                format!(
                                    "layout() requer uma função, recebeu {}",
                                    other.type_name()
                                ),
                            )]);
                        }
                        _ => {
                            return Err(vec![SourceDiagnostic::error(
                                call.callee().span(),
                                "layout() requer exatamente 1 argumento".to_string(),
                            )]);
                        }
                    };
                    // Dimensões do container (single-pass graded — P792 scope-out two-pass).
                    // Lidas dinamicamente da StyleChain caso configuradas por um `#set page`
                    // anterior no escopo; caso contrário usa defaults da página A4.
                    let page_width_pt = match engine.styles.custom("page.width") {
                        Some(Value::Float(f)) => *f,
                        Some(Value::Int(i)) => *i as f64,
                        _ => crate::entities::page_geometry::Paper::A4.width_pt(),
                    };
                    let page_height_pt = match engine.styles.custom("page.height") {
                        Some(Value::Float(f)) => *f,
                        Some(Value::Int(i)) => *i as f64,
                        _ => crate::entities::page_geometry::Paper::A4.height_pt(),
                    };
                    let margin_left = match engine.styles.custom("page.margin-left") {
                        Some(Value::Float(f)) => *f,
                        Some(Value::Int(i)) => *i as f64,
                        _ => 56.69f64,
                    };
                    let margin_right = match engine.styles.custom("page.margin-right") {
                        Some(Value::Float(f)) => *f,
                        Some(Value::Int(i)) => *i as f64,
                        _ => 56.69f64,
                    };
                    let margin_top = match engine.styles.custom("page.margin-top") {
                        Some(Value::Float(f)) => *f,
                        Some(Value::Int(i)) => *i as f64,
                        _ => 56.69f64,
                    };
                    let margin_bottom = match engine.styles.custom("page.margin-bottom") {
                        Some(Value::Float(f)) => *f,
                        Some(Value::Int(i)) => *i as f64,
                        _ => 56.69f64,
                    };
                    let avail_w =
                        f64::max(0.0, page_width_pt - margin_left - margin_right);
                    let avail_h =
                        f64::max(0.0, page_height_pt - margin_top - margin_bottom);
                    let mut size_dict: IndexMap<EcoString, Value, FxBuildHasher> =
                        IndexMap::default();
                    size_dict.insert(
                        "width".into(),
                        Value::Length(crate::entities::layout_types::Length::pt(avail_w)),
                    );
                    size_dict.insert(
                        "height".into(),
                        Value::Length(crate::entities::layout_types::Length::pt(avail_h)),
                    );
                    let size_arg = Args::from_parts(
                        vec![Value::Dict(size_dict)],
                        indexmap::IndexMap::default(),
                        call.span(),
                    );
                    let result = apply_func(func, size_arg, scopes, ctx, engine)?;
                    return if let Value::Content(c) = result {
                        Ok(Value::Content(rules::intercept_content(c, ctx, engine)?))
                    } else {
                        Ok(result)
                    };
                }
            }
        }

        // **P792** — Métodos de `Location`: `loc.page()`, `loc.position()`,
        // `loc.page-numbering()` — paridade vanilla `location.rs #[scope]`.
        if let Expr::FieldAccess(access) = call.callee() {
            let method = access.field().as_str();
            if matches!(method, "page" | "position" | "page-numbering") {
                let target = eval_expr(access.target(), scopes, ctx, engine)?;
                if let Value::Location(loc) = target {
                    let _args = eval_args(call.args(), scopes, ctx, engine)?;
                    return eval_location_method(loc, method, ctx, call.span());
                }
            }
        }

        // **P829-B** — Métodos de instância de `Value::Content`:
        // `func`/`has`/`at`/`fields`/`location` — paridade vanilla `Content`
        // `#[scope]` (`foundations/content/mod.rs:510-590`). Corre depois de
        // todos os despachos legítimos acima (nenhum intercepta estes nomes para
        // Content) e antes do fallback P815, que de outra forma reportaria
        // `element strong has no method `func`` (medido m14/b1).
        if let Expr::FieldAccess(access) = call.callee() {
            let method = access.field().as_str();
            if matches!(method, "func" | "has" | "at" | "fields" | "location") {
                let target = eval_expr(access.target(), scopes, ctx, engine)?;
                if let Value::Content(c) = target {
                    let args = eval_args(call.args(), scopes, ctx, engine)?;
                    return bindings::eval_content_method(&c, method, args, call.span());
                }
                if let Value::LocatedContent(c, loc) = target {
                    let args = eval_args(call.args(), scopes, ctx, engine)?;
                    return bindings::eval_introspected_content_method_at(
                        &c,
                        loc,
                        method,
                        args,
                        call.span(),
                    );
                }
            }
        }

        // P1402/P1403 — métodos próprios da entry.
        if let Expr::FieldAccess(access) = call.callee() {
            let method = access.field().as_str();
            if matches!(method, "body" | "prefix" | "page" | "inner" | "indented") {
                let target = eval_expr(access.target(), scopes, ctx, engine)?;
                let content = match &target {
                    Value::Content(content) => Some(content),
                    Value::LocatedContent(content, _) => Some(content.content()),
                    _ => None,
                };
                if let Some(
                    content @ crate::entities::content::Content::OutlineEntry(_),
                ) = content
                {
                    let mut args = eval_args(call.args(), scopes, ctx, engine)?;
                    if method == "body" {
                        return bindings::eval_outline_entry_body_method(
                            content,
                            args,
                            call.span(),
                        );
                    }
                    args.items.insert(0, Value::Content(content.clone()));
                    args.occurrences = None;
                    let world = engine.world;
                    let current_file = engine.current_file;
                    return match method {
                        "prefix" => {
                            native_outline_entry_prefix(ctx, &args, world, current_file)
                        }
                        "page" => native_outline_entry_page(
                            ctx,
                            &args,
                            world,
                            current_file,
                            scopes,
                            engine,
                        ),
                        "inner" => native_outline_entry_inner(
                            ctx,
                            &args,
                            world,
                            current_file,
                            scopes,
                            engine,
                        ),
                        "indented" => native_outline_entry_indented(
                            ctx,
                            &args,
                            world,
                            current_file,
                            scopes,
                            engine,
                        ),
                        _ => unreachable!("outline entry method already checked"),
                    };
                }
            }
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
