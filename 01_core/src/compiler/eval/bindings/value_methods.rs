//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/eval/bindings/value_methods.md
//! @prompt-hash cde578e8
//! @layer L1
//! @updated 2026-08-12
//!
//! Métodos de instância avaliados sobre valores, com argumentos ainda em AST:
//! `state`, `counter` (incl. `display`/`at`), `color`, `version`, e os
//! combinadores de `selector` (`where`, `and`/`or`, `within`).
//!
//! Extraído de `compiler/eval/bindings.rs` no Passo 1013 conforme ADR-0109
//! (atomização — forma B, free function no arquivo da unidade).

use ecow::{EcoString, EcoVec};

use crate::compiler::scopes::Scopes;
use crate::compiler::stdlib::counter::{
    counter_at_location_owned as counter_at_location, counter_at_owned as counter_at,
    counter_display, counter_final_owned as counter_final,
    counter_get_owned as counter_get, counter_step, counter_update,
};
use crate::compiler::stdlib::state::{
    state_at_location, state_display, state_final, state_get, state_update,
};
use crate::entities::args::{ArgOccurrence, Args};
use crate::entities::ast::expr::{Arg, Expr};
use crate::entities::ast::AstNode;
use crate::entities::counter::Counter;
use crate::entities::element_kind::ElementKind;
use crate::entities::engine::Engine;
use crate::entities::selector::Selector;
use crate::entities::source_result::{SourceDiagnostic, SourceResult};
use crate::entities::span::Span;
use crate::entities::state::State;
use crate::entities::value::Value;

use crate::compiler::eval::operators::error_formatting::vanilla_type_name;
use crate::compiler::eval::{eval_expr, EvalContext};

/// **P506** — Despacha métodos de `Value::State`: `.update()`, `.get()`,
/// `.display()`.
pub(in crate::compiler::eval) fn eval_state_method(
    state: &State,
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
            match args.items.as_slice() {
                [value] => Ok(state_update(state.key.clone(), value.clone())),
                _ => Err(vec![SourceDiagnostic::error(
                    span,
                    "state.update() requer 1 argumento".to_string(),
                )]),
            }
        }
        "get" => {
            let _ = eval_args(args, scopes, ctx, engine)?;
            state_get(state, ctx, span)
        }
        "display" => {
            let args = eval_args(args, scopes, ctx, engine)?;
            state_display(state, &args, scopes, ctx, engine, span)
        }
        // **P844** (achado #49 de P831) — `.at()`/`.final()` existiam como
        // nativas (`native_state_at`/`native_state_final`, foundations.rs)
        // mas não estavam ligados no dispatch de métodos.
        "at" => state_at_dispatch(state, args, scopes, ctx, engine, span),
        "final" => {
            let args = eval_args(args, scopes, ctx, engine)?;
            // Mensagens verbatim medidas no vanilla 0.15.0:
            // `s.final(1)` → "unexpected argument".
            if !args.items.is_empty() {
                return Err(vec![SourceDiagnostic::error(
                    span,
                    "unexpected argument".to_string(),
                )]);
            }
            state_final(state, ctx, span)
        }
        _ => Err(vec![SourceDiagnostic::error(
            span,
            format!("state não tem método '{}'", method),
        )]),
    }
}

/// **P844** (achado #49 de P831) — Despacha `state.at(selector)`.
/// Aceita `Location` directa (ex.: `here()`) ou `<label>` resolvida via
/// introspector — paridade vanilla `State::at` (`LocatableSelector`).
/// Mensagens verbatim medidas no vanilla 0.15.0:
/// - `s.at()` → `missing argument: selector`
/// - `s.at(1)` → `expected label, function, location, or selector, found integer`
/// - `s.at("x")` → `text is not locatable`
/// - `s.at(<inexistente>)` → ``label `<inexistente>` does not exist in the document``
/// A validação de argumentos precede o gate de contexto (ordem medida:
/// `s.at(1)` fora de contexto → erro de tipo, não gate).
fn state_at_dispatch(
    state: &State,
    args: crate::entities::ast::expr::Args<'_>,
    scopes: &mut Scopes<'_>,
    ctx: &mut EvalContext,
    engine: &mut Engine<'_>,
    span: Span,
) -> SourceResult<Value> {
    use crate::entities::introspector::Introspector;

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

    // Resolve o selector para uma Location (label → lookup no introspector).
    let resolve_label = |label: &crate::entities::label::Label,
                         ctx: &EvalContext|
     -> SourceResult<crate::entities::location::Location> {
        resolve_state_label(label, ctx, span)
    };

    match first {
        Arg::Pos(Expr::Label(node)) => {
            let label = crate::entities::label::Label(node.get().to_string());
            let loc = resolve_label(&label, ctx)?;
            state_at_location(state, loc, ctx, span)
        }
        Arg::Pos(expr) => {
            let value = eval_expr(expr, scopes, ctx, engine)?;
            match value {
                Value::Location(loc) => state_at_location(state, loc, ctx, span),
                Value::Label(label) => {
                    let loc = resolve_label(&label, ctx)?;
                    state_at_location(state, loc, ctx, span)
                }
                Value::Content(crate::entities::content::Content::Label(e)) => {
                    let label = crate::entities::label::Label(e.name.to_string());
                    let loc = resolve_label(&label, ctx)?;
                    state_at_location(state, loc, ctx, span)
                }
                Value::Str(_) => Err(vec![SourceDiagnostic::error(
                    span,
                    "text is not locatable".to_string(),
                )]),
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

/// **P742** — Despacha métodos de `Value::Color` (padrão P506).
///
/// Sintetiza `Args` com a cor como primeiro posicional e delega nas
/// nativas estáticas de `rules/stdlib/color.rs` — validação, semântica e
/// mensagens idênticas nos dois caminhos (medido: `red.lighten(20%)` ≡
/// `color.lighten(red, 20%)`). Só é chamado para os 9 métodos de
/// `is_color_instance_method`; o braço em `closures.rs` filtra os restantes
/// (que caem no caminho genérico de field access).
pub(in crate::compiler::eval) fn eval_color_method(
    color: &crate::entities::layout_types::Color,
    method: &str,
    args: crate::entities::ast::expr::Args<'_>,
    scopes: &mut Scopes<'_>,
    ctx: &mut EvalContext,
    engine: &mut Engine<'_>,
) -> SourceResult<Value> {
    use crate::compiler::eval::call_dispatch::eval_args;
    use crate::compiler::stdlib::color as color_rules;
    let span = args.span();
    let args = eval_args(args, scopes, ctx, engine)?;
    let synth = if args.occurrences.is_some() {
        let mut occurrences = args.occurrence_sequence();
        occurrences.insert(
            0,
            ArgOccurrence {
                name: None,
                value: Value::Color(*color),
                span: Span::detached(),
                value_span: Span::detached(),
            },
        );
        Args::from_occurrences(args.span, occurrences)
    } else {
        let mut items = args.items;
        items.insert(0, Value::Color(*color));
        Args::from_parts(items, args.named, args.span)
    };
    let world = engine.world;
    let current_file = engine.current_file;
    match method {
        "lighten" => color_rules::native_color_lighten(ctx, &synth, world, current_file),
        "darken" => color_rules::native_color_darken(ctx, &synth, world, current_file),
        "mix" => {
            // **P744** — o método de instância `red.mix(blue)` não aceita
            // `weight:` (só `space:`); o peso é sempre 0.5. A estática
            // `color.mix(red, blue, weight: ...)` mantém o argumento.
            if synth.named.contains_key("weight") {
                return Err(vec![SourceDiagnostic::error(
                    span,
                    "unexpected argument: weight".to_string(),
                )]);
            }
            color_rules::native_color_mix(ctx, &synth, world, current_file)
        }
        "negate" => color_rules::native_color_negate(ctx, &synth, world, current_file),
        "saturate" => {
            color_rules::native_color_saturate(ctx, &synth, world, current_file)
        }
        "desaturate" => {
            color_rules::native_color_desaturate(ctx, &synth, world, current_file)
        }
        "rotate" => color_rules::native_color_rotate(ctx, &synth, world, current_file),
        "components" => {
            color_rules::native_color_components(ctx, &synth, world, current_file)
        }
        "space" => color_rules::native_color_space(ctx, &synth, world, current_file),
        "to-hex" => color_rules::native_color_to_hex(ctx, &synth, world, current_file),
        "transparentize" => {
            color_rules::native_color_transparentize(ctx, &synth, world, current_file)
        }
        "opacify" => color_rules::native_color_opacify(ctx, &synth, world, current_file),
        _ => Err(vec![SourceDiagnostic::error(
            span,
            format!("color não tem método '{method}'"),
        )]),
    }
}

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
                    )])
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
                        Value::Label(l) => counter_at(counter, l.clone(), ctx, scopes, engine, span),
                        Value::Location(loc) => counter_at_location(counter, loc, ctx, scopes, engine, span),
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
                    )])
                }
                Arg::Spread(spread) => {
                    return Err(vec![SourceDiagnostic::error(
                        spread.span(),
                        "unexpected argument",
                    )])
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
                        )])
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
                        )])
                    }
                    Arg::Spread(spread) => {
                        return Err(vec![SourceDiagnostic::error(
                            spread.span(),
                            "spread not allowed in counter.display()",
                        )])
                    }
                }
            }
            let parsed = Args::from_occurrences(span, occurrences);
            counter_display(&counter, &parsed, scopes, ctx, engine, span)
        }
        _ => unreachable!("caller restringe métodos estáticos contextuais"),
    }
}

/// **P796** — Despacha `.at(index)`, o único método de instância de
/// `Value::Version` (`entities/version.md` §8a). Mesmo padrão de
/// `eval_counter_method_value`/`eval_color_method`: recebe os argumentos AST
/// não avaliados, avalia-os aqui.
pub(in crate::compiler::eval) fn eval_version_method_value(
    version: &crate::entities::version::Version,
    method: &str,
    args: crate::entities::ast::expr::Args<'_>,
    call_span: Span,
    scopes: &mut Scopes<'_>,
    ctx: &mut EvalContext,
    engine: &mut Engine<'_>,
) -> SourceResult<Value> {
    let mut args =
        crate::compiler::eval::call_dispatch::eval_args(args, scopes, ctx, engine)?;
    args.span = call_span;
    crate::compiler::stdlib::dispatch_version_method(
        version,
        method,
        args,
        ctx,
        engine.world,
        engine.current_file,
    )
}

pub(crate) fn resolve_state_label(
    label: &crate::entities::label::Label,
    ctx: &EvalContext,
    span: Span,
) -> SourceResult<crate::entities::location::Location> {
    use crate::entities::introspector::Introspector;
    let result = ctx
        .introspector
        .query_by_label(label)
        .map(Value::Location)
        .ok_or_else(|| {
            vec![SourceDiagnostic::error(
                span,
                format!("label `<{}>` does not exist in the document", label.0),
            )]
        });
    match ctx.observe_context_read(
        crate::compiler::eval::ContextReadRequest::StateResolveLabel {
            label: label.clone(),
        },
        span,
        result,
    )? {
        Value::Location(location) => Ok(location),
        _ => unreachable!("label resolution only produces locations"),
    }
}

/// Constrói um grupo público sem avaliar novamente receiver ou argumentos.
pub(crate) fn eval_element_where(
    function: &crate::entities::func::Func,
    args: &Args,
) -> SourceResult<Selector> {
    let Some((name, valid, _)) = native_element_fields(function) else {
        return Err(vec![SourceDiagnostic::error(
            args.span,
            "`where()` can only be called on element functions",
        )]);
    };
    // A view mantém a primeira posição e o último valor de cada chave.
    for field in args.named.keys() {
        if field.as_str() != "label" && !valid.contains(&field.as_str()) {
            return Err(vec![SourceDiagnostic::error(
                args.span,
                format!("element `{name}` does not have field `{field}`"),
            )]);
        }
    }
    if let Some(extra) = args.occurrence_sequence().iter().find(|arg| arg.name.is_none())
    {
        return Err(vec![SourceDiagnostic::error(extra.span, "unexpected argument")]);
    }
    Ok(Selector::Element {
        function: function.clone(),
        fields: args
            .named
            .iter()
            .map(|(name, value)| (name.clone(), value.clone()))
            .collect(),
    })
}

pub(crate) fn native_function_where(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _file: crate::entities::file_id::FileId,
) -> SourceResult<Value> {
    let (receiver, span, rest) =
        crate::compiler::eval::call_dispatch::take_p1339_self(args)?;
    let Value::Func(function) = receiver else {
        return Err(vec![SourceDiagnostic::error(
            span,
            format!("expected function, found {}", vanilla_type_name(&receiver)),
        )]);
    };
    eval_element_where(&function, &rest).map(Value::Selector)
}

/// Tabela estática dos campos de linguagem, excluindo #[internal]/#[external].
/// Fonte: typst-library e field_id de typst-macros::elem (a51e02804).
/// O nome serve ao diagnóstico; só o ponteiro tipado reconhece a função.
pub(crate) fn native_element_fields(
    function: &crate::entities::func::Func,
) -> Option<(&'static str, &'static [&'static str], bool)> {
    use crate::compiler::stdlib::*;
    if let crate::entities::func::FuncRepr::NativeWithEngine(native) = function.repr() {
        if std::ptr::fn_addr_eq(native.call, native_page as fn(_, _, _, _, _, _) -> _) {
            return Some((
                "page",
                &[
                    "width",
                    "height",
                    "flipped",
                    "margin",
                    "bleed",
                    "binding",
                    "columns",
                    "fill",
                    "numbering",
                    "supplement",
                    "number-align",
                    "header",
                    "header-ascent",
                    "footer",
                    "footer-descent",
                    "background",
                    "foreground",
                ],
                false,
            ));
        }
    }
    let addr = function.native_fn_addr()?;
    macro_rules! element {
        ($native:path, $name:literal, $locatable:literal, [$($field:literal),*]) => {
            if std::ptr::fn_addr_eq(addr, $native as fn(_, _, _, _) -> _) {
                return Some(($name, &[$($field),*], $locatable));
            }
        };
    }
    element!(native_strong, "strong", true, ["delta", "body"]);
    element!(native_emph, "emph", true, ["body"]);
    element!(
        native_heading,
        "heading",
        true,
        [
            "level",
            "depth",
            "offset",
            "numbering",
            "supplement",
            "outlined",
            "bookmarked",
            "hanging-indent",
            "body"
        ]
    );
    element!(
        native_figure,
        "figure",
        true,
        [
            "body",
            "alt",
            "placement",
            "scope",
            "caption",
            "kind",
            "supplement",
            "numbering",
            "gap",
            "outlined",
            "counter"
        ]
    );
    element!(
        native_text,
        "text",
        false,
        [
            "font",
            "fallback",
            "style",
            "weight",
            "stretch",
            "size",
            "fill",
            "stroke",
            "tracking",
            "spacing",
            "cjk-latin-spacing",
            "baseline",
            "overhang",
            "top-edge",
            "bottom-edge",
            "lang",
            "region",
            "script",
            "dir",
            "hyphenate",
            "costs",
            "kerning",
            "alternates",
            "stylistic-set",
            "ligatures",
            "discretionary-ligatures",
            "historical-ligatures",
            "number-type",
            "number-width",
            "slashed-zero",
            "fractions",
            "features",
            "variations",
            "text"
        ]
    );
    element!(
        native_raw,
        "raw",
        true,
        ["text", "block", "lang", "align", "syntaxes", "theme", "tab-size", "lines"]
    );
    element!(
        native_table,
        "table",
        true,
        [
            "columns",
            "rows",
            "column-gutter",
            "row-gutter",
            "inset",
            "align",
            "fill",
            "stroke",
            "children"
        ]
    );
    element!(
        native_grid,
        "grid",
        false,
        [
            "columns",
            "rows",
            "column-gutter",
            "row-gutter",
            "inset",
            "align",
            "fill",
            "stroke",
            "children"
        ]
    );
    element!(native_link, "link", true, ["dest", "body"]);
    element!(native_quote, "quote", true, ["block", "quotes", "attribution", "body"]);
    element!(native_footnote, "footnote", true, ["numbering", "body"]);
    element!(
        native_list,
        "list",
        true,
        [
            "tight",
            "marker",
            "indent",
            "body-indent",
            "spacing",
            "marker-align",
            "children"
        ]
    );
    element!(
        native_enum,
        "enum",
        true,
        [
            "tight",
            "numbering",
            "start",
            "full",
            "reversed",
            "indent",
            "body-indent",
            "spacing",
            "number-align",
            "children"
        ]
    );
    element!(
        native_terms,
        "terms",
        true,
        ["tight", "separator", "indent", "hanging-indent", "spacing", "children"]
    );
    element!(native_outline, "outline", true, ["title", "target", "depth", "indent"]);
    element!(native_metadata, "metadata", true, ["value"]);
    element!(native_cite, "cite", true, ["key", "supplement", "form", "style"]);
    element!(
        native_bibliography,
        "bibliography",
        true,
        ["sources", "title", "full", "style", "target", "group"]
    );
    element!(
        native_ref,
        "ref",
        true,
        ["target", "supplement", "form", "citation", "element"]
    );
    element!(
        native_image,
        "image",
        true,
        ["source", "format", "width", "height", "alt", "page", "fit", "scaling", "icc"]
    );
    element!(native_align, "align", false, ["alignment", "body"]);
    element!(
        native_block,
        "block",
        false,
        [
            "width",
            "height",
            "breakable",
            "fill",
            "stroke",
            "radius",
            "inset",
            "outset",
            "above",
            "below",
            "clip",
            "sticky",
            "body"
        ]
    );
    element!(
        native_box,
        "box",
        false,
        [
            "width", "height", "baseline", "fill", "stroke", "radius", "inset", "outset",
            "clip", "body"
        ]
    );
    element!(native_columns, "columns", false, ["count", "gutter", "balanced", "body"]);
    element!(native_pad, "pad", false, ["left", "top", "right", "bottom", "body"]);
    element!(
        native_place,
        "place",
        false,
        ["alignment", "scope", "float", "clearance", "dx", "dy", "body"]
    );
    element!(native_hide, "hide", false, ["body"]);
    element!(native_repeat, "repeat", false, ["body", "gap", "justify"]);
    element!(native_stack, "stack", false, ["dir", "spacing", "children"]);
    element!(native_h, "h", false, ["amount", "weak"]);
    element!(native_v, "v", false, ["amount", "weak"]);
    element!(native_pagebreak, "pagebreak", false, ["weak", "to"]);
    element!(native_colbreak, "colbreak", false, ["weak"]);
    element!(native_linebreak, "linebreak", false, ["justify"]);
    element!(
        native_par,
        "par",
        true,
        [
            "leading",
            "spacing",
            "justify",
            "justification-limits",
            "linebreaks",
            "first-line-indent",
            "hanging-indent",
            "body"
        ]
    );
    element!(native_parbreak, "parbreak", false, []);
    element!(
        native_rect,
        "rect",
        false,
        ["width", "height", "fill", "stroke", "radius", "inset", "outset", "body"]
    );
    element!(
        native_square,
        "square",
        false,
        ["width", "height", "fill", "stroke", "radius", "inset", "outset", "body"]
    );
    element!(
        native_circle,
        "circle",
        false,
        ["width", "height", "fill", "stroke", "inset", "outset", "body"]
    );
    element!(
        native_ellipse,
        "ellipse",
        false,
        ["width", "height", "fill", "stroke", "inset", "outset", "body"]
    );
    element!(native_line, "line", false, ["start", "end", "length", "angle", "stroke"]);
    element!(
        native_polygon,
        "polygon",
        false,
        ["fill", "fill-rule", "stroke", "vertices"]
    );
    element!(native_curve, "curve", false, ["fill", "fill-rule", "stroke", "components"]);
    element!(native_move, "move", false, ["dx", "dy", "body"]);
    element!(native_scale, "scale", false, ["x", "y", "origin", "reflow", "body"]);
    element!(native_rotate, "rotate", false, ["angle", "origin", "reflow", "body"]);
    element!(native_skew, "skew", false, ["ax", "ay", "origin", "reflow", "body"]);
    element!(
        native_underline,
        "underline",
        true,
        ["stroke", "offset", "extent", "evade", "background", "body"]
    );
    element!(
        native_overline,
        "overline",
        true,
        ["stroke", "offset", "extent", "evade", "background", "body"]
    );
    element!(
        native_strike,
        "strike",
        true,
        ["stroke", "offset", "extent", "background", "body"]
    );
    element!(
        native_highlight,
        "highlight",
        true,
        ["fill", "stroke", "top-edge", "bottom-edge", "extent", "radius", "body"]
    );
    element!(native_smallcaps, "smallcaps", false, ["all", "body"]);
    element!(native_subscript, "sub", false, ["typographic", "baseline", "size", "body"]);
    element!(
        native_superscript,
        "super",
        false,
        ["typographic", "baseline", "size", "body"]
    );
    element!(
        native_smartquote,
        "smartquote",
        false,
        ["double", "enabled", "alternative", "quotes"]
    );
    element!(native_table_header, "header", false, ["repeat", "level", "children"]);
    element!(native_table_footer, "footer", false, ["repeat", "children"]);
    element!(
        native_table_hline,
        "hline",
        false,
        ["y", "start", "end", "stroke", "position"]
    );
    element!(
        native_table_vline,
        "vline",
        false,
        ["x", "start", "end", "stroke", "position"]
    );
    element!(
        native_table_cell,
        "cell",
        false,
        [
            "body",
            "x",
            "y",
            "colspan",
            "rowspan",
            "inset",
            "align",
            "fill",
            "stroke",
            "breakable"
        ]
    );
    element!(native_grid_header, "header", false, ["repeat", "level", "children"]);
    element!(native_grid_footer, "footer", false, ["repeat", "children"]);
    element!(
        native_grid_hline,
        "hline",
        false,
        ["y", "start", "end", "stroke", "position"]
    );
    element!(
        native_grid_vline,
        "vline",
        false,
        ["x", "start", "end", "stroke", "position"]
    );
    element!(
        native_grid_cell,
        "cell",
        false,
        [
            "body",
            "x",
            "y",
            "colspan",
            "rowspan",
            "inset",
            "align",
            "fill",
            "stroke",
            "breakable"
        ]
    );
    None
}

/// Valida somente as folhas novas; ramos antigos não ganham regras novas.
pub(crate) fn validate_element_locatability(
    selector: &Selector,
    span: Span,
) -> SourceResult<()> {
    match selector {
        Selector::Element { function, .. } => {
            if let Some((name, _, false)) = native_element_fields(function) {
                return Err(vec![SourceDiagnostic::error(
                    span,
                    format!("{name} is not locatable"),
                )]);
            }
        }
        Selector::And(children) | Selector::Or(children) => {
            for child in children {
                validate_element_locatability(child, span)?;
            }
        }
        Selector::Where { base, .. } => validate_element_locatability(base, span)?,
        Selector::Within { base, ancestor } => {
            validate_element_locatability(base, span)?;
            validate_element_locatability(ancestor, span)?;
        }
        _ => {}
    }
    Ok(())
}

/// **P423 (S-M)** — Tenta avaliar `<selector>.or(other)` ou
/// `<selector>.and(other)`.
///
/// Retorna `Ok(Some(Selector::Or(...)))` ou `Ok(Some(Selector::And(...)))`
/// se o target avaliar para `Value::Selector` e houver exactamente um
/// argumento posicional que também avalie para `Value::Selector`.
/// Retorna `Ok(None)` se o target não for um selector (deixa o caller
/// continuar com field access normal).
/// Converte `Value::Func` nativo de elemento (heading, figure) ou
/// `Value::Selector` num `Selector` de query. Retorna `None` se o valor não
/// for convertível.
pub(in crate::compiler::eval) fn value_to_query_selector(
    value: &Value,
) -> Option<Selector> {
    match value {
        Value::Selector(s) => Some(s.clone()),
        Value::Func(f) => {
            use crate::compiler::stdlib::{native_figure, native_heading};
            use std::ptr::fn_addr_eq;
            f.native_fn_addr().and_then(|addr| {
                if fn_addr_eq(addr, native_heading as fn(_, _, _, _) -> _) {
                    Some(Selector::Kind(ElementKind::Heading))
                } else if fn_addr_eq(addr, native_figure as fn(_, _, _, _) -> _) {
                    Some(Selector::Kind(ElementKind::Figure))
                } else {
                    None
                }
            })
        }
        _ => None,
    }
}

pub(in crate::compiler::eval) fn eval_selector_or_and<'a>(
    target_expr: Expr<'_>,
    method: &str,
    args_node: crate::entities::ast::expr::Args<'a>,
    scopes: &mut Scopes<'_>,
    ctx: &mut EvalContext,
    engine: &mut Engine<'_>,
) -> SourceResult<Option<Selector>> {
    let target = eval_expr(target_expr, scopes, ctx, engine)?;
    let Some(base) = value_to_query_selector(&target) else {
        return Ok(None);
    };

    let args =
        crate::compiler::eval::call_dispatch::eval_args(args_node, scopes, ctx, engine)?;
    if !args.named.is_empty() {
        return Err(vec![SourceDiagnostic::error(
            args_node.span(),
            "unexpected argument",
        )]);
    }
    let mut selectors = vec![base];
    for other in args.items {
        let Some(other_sel) = value_to_query_selector(&other) else {
            return Err(vec![SourceDiagnostic::error(
                args_node.span(),
                format!(
                    "selector.{}() espera um selector, recebeu {}",
                    method,
                    other.type_name()
                ),
            )]);
        };
        selectors.push(other_sel);
    }

    match method {
        "or" => Ok(Some(Selector::Or(EcoVec::from(selectors)))),
        "and" => Ok(Some(Selector::And(EcoVec::from(selectors)))),
        _ => Ok(None),
    }
}

/// **P504** — Intercepta `selector(base).within(ancestor)`.
pub(in crate::compiler::eval) fn eval_selector_within<'a>(
    target_expr: Expr<'_>,
    args_node: crate::entities::ast::expr::Args<'a>,
    scopes: &mut Scopes<'_>,
    ctx: &mut EvalContext,
    engine: &mut Engine<'_>,
) -> SourceResult<Option<Selector>> {
    let target = eval_expr(target_expr, scopes, ctx, engine)?;
    let Some(base) = value_to_query_selector(&target) else {
        return Ok(None);
    };

    let args =
        crate::compiler::eval::call_dispatch::eval_args(args_node, scopes, ctx, engine)?;
    let other = args.items.into_iter().next().ok_or_else(|| {
        vec![SourceDiagnostic::error(
            args_node.span(),
            "selector.within() requer um argumento posicional".to_string(),
        )]
    })?;
    let Some(ancestor) = value_to_query_selector(&other) else {
        return Err(vec![SourceDiagnostic::error(
            args_node.span(),
            format!(
                "selector.within() espera um selector, recebeu {}",
                other.type_name()
            ),
        )]);
    };

    Ok(Some(Selector::Within { base: Box::new(base), ancestor: Box::new(ancestor) }))
}

// ── Dispatcher arms: FieldAccess (Passo 96.2, ADR-0037 Regra 4) ───────────
