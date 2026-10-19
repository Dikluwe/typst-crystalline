//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/core/compiler/eval/bindings/value_methods/counter.md
//! @prompt-hash c67420cc
//! @layer L1
//! @updated 2026-09-30

use super::*;

/// **P640** — Faz parse e validação dos argumentos de `counter.display(...)`.
/// Devolve o argumento nomeado `at:` (se válido) e o argumento posicional
/// pattern/callback (se válido). Produz erro claro para tipos inválidos,
/// argumentos não reconhecidos ou argumentos posicionais a mais.
fn parse_counter_display_args(
    args: crate::entities::ast::expr::Args<'_>,
    scopes: &mut Scopes<'_>,
    ctx: &mut EvalContext,
    engine: &mut Engine<'_>,
) -> SourceResult<Args> {
    use crate::entities::ast::expr::Arg;

    let mut occurrences = Vec::new();
    let mut has_positional = false;

    for arg in args.items() {
        match arg {
            Arg::Named(named) if named.name().as_str() == "at" => {
                let value = match named.expr() {
                    Expr::Label(node) => Value::Label(crate::entities::label::Label(
                        node.get().to_string(),
                    )),
                    expr => eval_expr(expr, scopes, ctx, engine)?,
                };
                occurrences.push(ArgOccurrence {
                    name: Some("at".into()),
                    value,
                    span: named.span(),
                    value_span: named.expr().span(),
                });
            }
            Arg::Named(named) if named.name().as_str() == "both" => {
                let value = eval_expr(named.expr(), scopes, ctx, engine)?;
                occurrences.push(ArgOccurrence {
                    name: Some("both".into()),
                    value,
                    span: named.span(),
                    value_span: named.expr().span(),
                });
            }
            Arg::Named(named) => {
                return Err(vec![SourceDiagnostic::error(
                    named.span(),
                    format!("unexpected argument: {}", named.name().as_str()),
                )]);
            }
            Arg::Pos(expr) if !has_positional => {
                let value = eval_expr(expr, scopes, ctx, engine)?;
                match value {
                    Value::Str(_) | Value::Func(_) | Value::Auto => {
                        occurrences.push(ArgOccurrence {
                            name: None,
                            value,
                            span: expr.span(),
                            value_span: expr.span(),
                        });
                        has_positional = true;
                    }
                    other => {
                        return Err(vec![SourceDiagnostic::error(
                            expr.span(),
                            format!(
                                "expected string, function, or auto, found {}",
                                other.type_name()
                            ),
                        )]);
                    }
                }
            }
            Arg::Pos(expr) => {
                return Err(vec![SourceDiagnostic::error(
                    expr.span(),
                    "counter.display() takes at most one positional argument".to_string(),
                )]);
            }
            Arg::Spread(spread) => {
                return Err(vec![SourceDiagnostic::error(
                    spread.span(),
                    "spread not allowed in counter.display()".to_string(),
                )]);
            }
        }
    }

    Ok(Args::from_occurrences(args.span(), occurrences))
}

/// **P506** — Despacha métodos de `Value::Counter`: `.update()`, `.step()`,
/// `.get()`, `.display()`, `.at()`.
pub(in crate::compiler::eval) fn eval_counter_method_value(
    counter: &Counter,
    method: &str,
    args: crate::entities::ast::expr::Args<'_>,
    scopes: &mut Scopes<'_>,
    ctx: &mut EvalContext,
    engine: &mut Engine<'_>,
) -> SourceResult<Value> {
    use crate::compiler::eval::call_dispatch::eval_args;
    let span = args.span();
    match method {
        "update" => {
            let args = eval_args(args, scopes, ctx, engine)?;
            counter_update(
                counter.key.clone(),
                args.items.into_iter().next().unwrap_or(Value::None),
            )
        }
        "step" => {
            let args = eval_args(args, scopes, ctx, engine)?;
            let level = args
                .named
                .get("level")
                .cloned()
                .or_else(|| args.items.first().cloned());
            let level = match level.unwrap_or(Value::Int(1)) {
                Value::Int(i) if i > 0 => {
                    std::num::NonZeroUsize::new(i as usize).unwrap()
                }
                other => {
                    return Err(vec![SourceDiagnostic::error(
                        span,
                        format!(
                            "expected positive integer, found {}",
                            vanilla_type_name(&other)
                        ),
                    )]);
                }
            };
            Ok(counter_step(counter.key.clone(), level))
        }
        "get" => {
            let _ = eval_args(args, scopes, ctx, engine)?;
            counter_get(counter, ctx, scopes, engine, span)
        }
        "display" => {
            // P640 — parse unificado e validação estrita dos argumentos.
            let args = parse_counter_display_args(args, scopes, ctx, engine)?;
            counter_display(counter, &args, scopes, ctx, engine, span)
        }
        "at" => {
            // P506 — counter.at(label): o parser cristalino avalia `<label>`
            // como Value::None; extraímos a string directamente do nó AST
            // para suportar a sintaxe vanilla.
            //
            // **P844** (achado #50 de P831) — aceita também `Location`
            // directa (ex.: `here()`) — paridade vanilla `Counter::at`
            // (`introspection/counter.rs:452`). Mensagens verbatim medidas
            // no vanilla 0.15.0: `counter.at()` →
            // `missing argument: selector`; `counter.at(1)` →
            // `expected label, function, location, or selector, found integer`.
            let mut items = args.items();
            let Some(first) = items.next() else {
                return Err(vec![SourceDiagnostic::error(
                    span,
                    "missing argument: selector".to_string(),
                )]);
            };
            if items.next().is_some() {
                return Err(vec![SourceDiagnostic::error(
                    span,
                    "unexpected argument".to_string(),
                )]);
            }
            match first {
                Arg::Pos(Expr::Label(node)) => {
                    let label = crate::entities::label::Label(node.get().to_string());
                    counter_at(counter, label, ctx, scopes, engine, span)
                }
                Arg::Pos(expr) => {
                    let value = eval_expr(expr, scopes, ctx, engine)?;
                    match value {
                        Value::Str(s) => counter_at(
                            counter,
                            crate::entities::label::Label(s.to_string()),
                            ctx,
                            scopes,
                            engine,
                            span,
                        ),
                        Value::Content(crate::entities::content::Content::Label(e)) => {
                            counter_at(
                                counter,
                                crate::entities::label::Label(e.name.to_string()),
                                ctx,
                                scopes,
                                engine,
                                span,
                            )
                        }
                        Value::Label(l) => {
                            counter_at(counter, l.clone(), ctx, scopes, engine, span)
                        }
                        Value::Location(loc) => {
                            counter_at_location(counter, loc, ctx, scopes, engine, span)
                        }
                        other => Err(vec![SourceDiagnostic::error(
                            span,
                            format!(
                                "expected label, function, location, or selector, found {}",
                                vanilla_type_name(&other)
                            ),
                        )]),
                    }
                }
                Arg::Named(named) => Err(vec![SourceDiagnostic::error(
                    named.span(),
                    format!("unexpected argument: {}", named.name().as_str()),
                )]),
                Arg::Spread(spread) => Err(vec![SourceDiagnostic::error(
                    spread.span(),
                    "unexpected argument".to_string(),
                )]),
            }
        }
        // **P844** (achado #49 de P831) — `counter.final()` existia como
        // nativa (`native_counter_final`, foundations.rs) mas não estava
        // ligado no dispatch de métodos. Mensagem verbatim medida no
        // vanilla 0.15.0: `counter.final(1)` → `unexpected argument`.
        "final" => {
            let args = eval_args(args, scopes, ctx, engine)?;
            if !args.items.is_empty() {
                return Err(vec![SourceDiagnostic::error(
                    span,
                    "unexpected argument".to_string(),
                )]);
            }
            counter_final(counter, ctx, scopes, engine, span)
        }
        _ => Err(vec![SourceDiagnostic::error(
            span,
            format!("counter não tem método '{}'", method),
        )]),
    }
}

/// P1149 — formas estáticas que precisam preservar literal label antes da
/// avaliação genérica dos argumentos.
pub(in crate::compiler::eval) fn eval_counter_static_method_value(
    method: &str,
    args: crate::entities::ast::expr::Args<'_>,
    scopes: &mut Scopes<'_>,
    ctx: &mut EvalContext,
    engine: &mut Engine<'_>,
) -> SourceResult<Value> {
    use crate::entities::ast::expr::Arg;

    let span = args.span();
    let mut items = args.items();
    let Some(Arg::Pos(receiver_expr)) = items.next() else {
        return Err(vec![SourceDiagnostic::error(span, "missing counter receiver")]);
    };
    let receiver = eval_expr(receiver_expr, scopes, ctx, engine)?;
    let Value::Counter(counter) = receiver else {
        return Err(vec![SourceDiagnostic::error(span, "expected counter receiver")]);
    };

    match method {
        "at" => {
            let Some(arg) = items.next() else {
                return Err(vec![SourceDiagnostic::error(
                    span,
                    "missing argument: selector",
                )]);
            };
            if items.next().is_some() {
                return Err(vec![SourceDiagnostic::error(span, "unexpected argument")]);
            }
            let value = match arg {
                Arg::Pos(Expr::Label(node)) => {
                    Value::Label(crate::entities::label::Label(node.get().to_string()))
                }
                Arg::Pos(expr) => eval_expr(expr, scopes, ctx, engine)?,
                Arg::Named(named) => {
                    return Err(vec![SourceDiagnostic::error(
                        named.span(),
                        format!("unexpected argument: {}", named.name().as_str()),
                    )]);
                }
                Arg::Spread(spread) => {
                    return Err(vec![SourceDiagnostic::error(
                        spread.span(),
                        "unexpected argument",
                    )]);
                }
            };
            match value {
                Value::Label(label) => {
                    counter_at(&counter, label, ctx, scopes, engine, span)
                }
                Value::Str(label) => counter_at(
                    &counter,
                    crate::entities::label::Label(label.to_string()),
                    ctx,
                    scopes,
                    engine,
                    span,
                ),
                Value::Location(location) => {
                    counter_at_location(&counter, location, ctx, scopes, engine, span)
                }
                other => Err(vec![SourceDiagnostic::error(
                    span,
                    format!(
                        "expected label, function, location, or selector, found {}",
                        vanilla_type_name(&other)
                    ),
                )]),
            }
        }
        "display" => {
            let mut occurrences = Vec::new();
            let mut has_positional = false;
            for arg in items {
                match arg {
                    Arg::Pos(expr) if !has_positional => {
                        occurrences.push(ArgOccurrence {
                            name: None,
                            value: eval_expr(expr, scopes, ctx, engine)?,
                            span: expr.span(),
                            value_span: expr.span(),
                        });
                        has_positional = true;
                    }
                    Arg::Pos(expr) => {
                        return Err(vec![SourceDiagnostic::error(
                            expr.span(),
                            "counter.display() takes at most one positional argument",
                        )]);
                    }
                    Arg::Named(named) if named.name().as_str() == "at" => {
                        let value = match named.expr() {
                            Expr::Label(node) => Value::Label(
                                crate::entities::label::Label(node.get().to_string()),
                            ),
                            expr => eval_expr(expr, scopes, ctx, engine)?,
                        };
                        occurrences.push(ArgOccurrence {
                            name: Some("at".into()),
                            value,
                            span: named.span(),
                            value_span: named.expr().span(),
                        });
                    }
                    Arg::Named(named) if named.name().as_str() == "both" => {
                        let value = eval_expr(named.expr(), scopes, ctx, engine)?;
                        occurrences.push(ArgOccurrence {
                            name: Some("both".into()),
                            value,
                            span: named.span(),
                            value_span: named.expr().span(),
                        });
                    }
                    Arg::Named(named) => {
                        return Err(vec![SourceDiagnostic::error(
                            named.span(),
                            format!("unexpected argument: {}", named.name().as_str()),
                        )]);
                    }
                    Arg::Spread(spread) => {
                        return Err(vec![SourceDiagnostic::error(
                            spread.span(),
                            "spread not allowed in counter.display()",
                        )]);
                    }
                }
            }
            let parsed = Args::from_occurrences(span, occurrences);
            counter_display(&counter, &parsed, scopes, ctx, engine, span)
        }
        _ => unreachable!("caller restringe métodos estáticos contextuais"),
    }
}
