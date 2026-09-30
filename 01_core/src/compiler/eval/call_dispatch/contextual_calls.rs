//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/eval/call_dispatch/contextual_calls.md
//! @prompt-hash a03f5420
//! @layer L1
//! @updated 2026-09-30

use super::*;

/// Intercepta chamadas contextuais na ordem de medida e layout.
pub(super) fn try_contextual_call(
    call: FuncCallNode<'_>,
    scopes: &mut Scopes<'_>,
    ctx: &mut EvalContext,
    engine: &mut Engine<'_>,
) -> SourceResult<Option<Value>> {
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
                    Value::Length(crate::entities::layout_types::Length::pt(width_pt)),
                );
                dict.insert(
                    "height".into(),
                    Value::Length(crate::entities::layout_types::Length::pt(height_pt)),
                );
                dict.insert(
                    "baseline".into(),
                    Value::Length(crate::entities::layout_types::Length::pt(baseline_pt)),
                );
                return Ok(Some(Value::Dict(dict)));
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
                let avail_w = f64::max(0.0, page_width_pt - margin_left - margin_right);
                let avail_h = f64::max(0.0, page_height_pt - margin_top - margin_bottom);
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
                    Ok(Some(Value::Content(rules::intercept_content(c, ctx, engine)?)))
                } else {
                    Ok(Some(result))
                };
            }
        }
    }

    Ok(None)
}
