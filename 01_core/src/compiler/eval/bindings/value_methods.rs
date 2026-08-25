//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/eval/bindings/value_methods.md
//! @prompt-hash a88fe70d
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
    counter_at, counter_at_location, counter_display, counter_final, counter_get,
    counter_step, counter_update,
};
use crate::compiler::stdlib::state::{
    state_at_location, state_display, state_final, state_get, state_update,
};
use crate::entities::args::Args;
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
        ctx.introspector.query_by_label(label).ok_or_else(|| {
            vec![SourceDiagnostic::error(
                span,
                format!("label `<{}>` does not exist in the document", label.0),
            )]
        })
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
    let mut synth = eval_args(args, scopes, ctx, engine)?;
    synth.items.insert(0, Value::Color(*color));
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

    let mut parsed = Args::positional(vec![]);
    parsed.span = args.span();

    for arg in args.items() {
        match arg {
            Arg::Named(named) if named.name().as_str() == "at" => {
                let value = match named.expr() {
                    Expr::Label(node) => Value::Label(crate::entities::label::Label(
                        node.get().to_string(),
                    )),
                    expr => eval_expr(expr, scopes, ctx, engine)?,
                };
                parsed.named.insert("at".into(), value);
            }
            Arg::Named(named) if named.name().as_str() == "both" => {
                let value = eval_expr(named.expr(), scopes, ctx, engine)?;
                parsed.named.insert("both".into(), value);
            }
            Arg::Named(named) => {
                return Err(vec![SourceDiagnostic::error(
                    named.span(),
                    format!("unexpected argument: {}", named.name().as_str()),
                )]);
            }
            Arg::Pos(expr) if parsed.items.is_empty() => {
                let value = eval_expr(expr, scopes, ctx, engine)?;
                match value {
                    Value::Str(_) | Value::Func(_) | Value::Auto => {
                        parsed.items.push(value)
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

    Ok(parsed)
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
            counter_get(counter, ctx, span)
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
                    counter_at(counter, label, ctx, span)
                }
                Arg::Pos(expr) => {
                    let value = eval_expr(expr, scopes, ctx, engine)?;
                    match value {
                        Value::Str(s) => counter_at(
                            counter,
                            crate::entities::label::Label(s.to_string()),
                            ctx,
                            span,
                        ),
                        Value::Content(crate::entities::content::Content::Label(e)) => {
                            counter_at(
                                counter,
                                crate::entities::label::Label(e.name.to_string()),
                                ctx,
                                span,
                            )
                        }
                        Value::Label(l) => counter_at(counter, l.clone(), ctx, span),
                        Value::Location(loc) => counter_at_location(counter, loc, ctx),
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
            counter_final(counter, ctx, span)
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
                Value::Label(label) => counter_at(&counter, label, ctx, span),
                Value::Str(label) => counter_at(
                    &counter,
                    crate::entities::label::Label(label.to_string()),
                    ctx,
                    span,
                ),
                Value::Location(location) => counter_at_location(&counter, location, ctx),
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
            let mut parsed = Args::positional(vec![]);
            parsed.span = span;
            for arg in items {
                match arg {
                    Arg::Pos(expr) if parsed.items.is_empty() => {
                        parsed.items.push(eval_expr(expr, scopes, ctx, engine)?);
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
                        parsed.named.insert("at".into(), value);
                    }
                    Arg::Named(named) if named.name().as_str() == "both" => {
                        let value = eval_expr(named.expr(), scopes, ctx, engine)?;
                        parsed.named.insert("both".into(), value);
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
    scopes: &mut Scopes<'_>,
    ctx: &mut EvalContext,
    engine: &mut Engine<'_>,
) -> SourceResult<Value> {
    use crate::compiler::eval::call_dispatch::eval_args;
    let span = args.span();
    match method {
        "at" => {
            let args = eval_args(args, scopes, ctx, engine)?;
            let index = match args.items.as_slice() {
                [Value::Int(i)] => *i,
                [other] => {
                    return Err(vec![SourceDiagnostic::error(
                        span,
                        format!("expected integer, found {}", vanilla_type_name(other)),
                    )])
                }
                _ => {
                    return Err(vec![SourceDiagnostic::error(
                        span,
                        "version.at() requires exactly one positional argument"
                            .to_string(),
                    )])
                }
            };
            version
                .at(index)
                .map(Value::Int)
                .map_err(|msg| vec![SourceDiagnostic::error(span, msg)])
        }
        _ => Err(vec![SourceDiagnostic::error(
            span,
            format!("version não tem método '{}'", method),
        )]),
    }
}

/// **P417 (M)** — Tenta avaliar `<elemento>.where(field: value)`.
///
/// Retorna `Ok(Some(Selector::Where { ... }))` se o target for uma função
/// nativa de elemento suportada (heading, figure, strong, emph, raw) e houver
/// exatamente um named argumento. Retorna `Ok(None)` se o target não for um
/// elemento nativo (deixa o caller continuar com field access normal).
/// Retorna `Err` se for elemento mas os argumentos forem inválidos.
pub(in crate::compiler::eval) fn eval_element_where<'a>(
    target_expr: Expr<'_>,
    args_node: crate::entities::ast::expr::Args<'a>,
    scopes: &mut Scopes<'_>,
    ctx: &mut EvalContext,
    engine: &mut Engine<'_>,
) -> SourceResult<Option<Selector>> {
    use crate::compiler::stdlib::{
        native_emph, native_figure, native_heading, native_raw, native_strong,
    };
    use std::ptr::fn_addr_eq;

    let target = eval_expr(target_expr, scopes, ctx, engine)?;
    let Value::Func(ref f) = target else {
        return Ok(None);
    };

    let kind = match f.native_fn_addr() {
        Some(addr) if fn_addr_eq(addr, native_heading as fn(_, _, _, _) -> _) => {
            ElementKind::Heading
        }
        Some(addr) if fn_addr_eq(addr, native_figure as fn(_, _, _, _) -> _) => {
            ElementKind::Figure
        }
        Some(addr) if fn_addr_eq(addr, native_strong as fn(_, _, _, _) -> _) => {
            return Err(vec![SourceDiagnostic::error(
                target_expr.span(),
                "selector where não suportado para strong".to_string(),
            )])
        }
        Some(addr) if fn_addr_eq(addr, native_emph as fn(_, _, _, _) -> _) => {
            return Err(vec![SourceDiagnostic::error(
                target_expr.span(),
                "selector where não suportado para emph".to_string(),
            )])
        }
        Some(addr) if fn_addr_eq(addr, native_raw as fn(_, _, _, _) -> _) => {
            return Err(vec![SourceDiagnostic::error(
                target_expr.span(),
                "selector where não suportado para raw".to_string(),
            )])
        }
        _ => return Ok(None),
    };

    let mut named_args: Vec<(EcoString, Value)> = Vec::new();
    for arg in args_node.items() {
        match arg {
            Arg::Pos(_) => {
                return Err(vec![SourceDiagnostic::error(
                    args_node.span(),
                    "heading.where() requer argumentos nomeados (ex.: level: 1)"
                        .to_string(),
                )]);
            }
            Arg::Named(named) => {
                let field = named.name().as_str().into();
                let value = eval_expr(named.expr(), scopes, ctx, engine)?;
                named_args.push((field, value));
            }
            Arg::Spread(_) => {}
        }
    }

    if named_args.is_empty() {
        return Err(vec![SourceDiagnostic::error(
            args_node.span(),
            "heading.where() requer pelo menos um argumento nomeado".to_string(),
        )]);
    }

    let mut selector = Selector::Kind(kind);
    for (field, value) in named_args {
        selector = Selector::Where {
            base: Box::new(selector),
            field,
            value: Box::new(value),
        };
    }
    Ok(Some(selector))
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
fn value_to_query_selector(value: &Value) -> Option<Selector> {
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
    let other = args.items.into_iter().next().ok_or_else(|| {
        vec![SourceDiagnostic::error(
            args_node.span(),
            format!("selector.{}() requer um argumento posicional", method),
        )]
    })?;
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

    match method {
        "or" => Ok(Some(Selector::Or(EcoVec::from(vec![base, other_sel])))),
        "and" => Ok(Some(Selector::And(EcoVec::from(vec![base, other_sel])))),
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
