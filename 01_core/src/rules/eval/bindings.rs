//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/rules/eval.md
//! @prompt-hash 4f183cf8
//! @prompt 00_nucleo/prompts/rules/eval/field-access.md
//! @prompt-hash c822a5ed
//! @layer L1
//! @updated 2026-06-22
//!
//! Bindings: `#let`, `#show` counter, e field access (P411 — Version; P412 — Duration). Extraído de `eval.rs` no Passo 96.1
//! conforme ADR-0037 (coesão por domínio). Assinaturas simplificadas no
//! Passo 109 (ADR-0044) via `Engine<'_>`.

use ecow::{EcoString, EcoVec};

use crate::entities::ast::code::{LetBinding, LetBindingKind};
use crate::entities::ast::expr::{Arg, Expr};
use crate::entities::ast::AstNode;
use crate::entities::content::Content;
use crate::entities::counter::Counter;
use crate::entities::counter_update::CounterUpdate as CounterAction;
use crate::entities::element_kind::ElementKind;
use crate::entities::engine::Engine;
use crate::entities::selector::Selector;
use crate::entities::args::Args;
use crate::entities::source_result::{SourceDiagnostic, SourceResult};
use crate::entities::span::Span;
use crate::entities::state::State;
use crate::entities::value::Value;
use crate::rules::scopes::Scopes;
use crate::rules::stdlib::counter::{counter_at, counter_display, counter_get, counter_step, counter_update};
use crate::rules::stdlib::state::{state_display, state_get, state_update};

use super::{eval_expr, EvalContext};

pub(super) fn eval_let(
    binding: LetBinding<'_>,
    scopes: &mut Scopes<'_>,
    ctx: &mut EvalContext,
    engine: &mut Engine<'_>,
) -> SourceResult<Value> {
    let mut value = match binding.init() {
        Some(init) => eval_expr(init, scopes, ctx, engine)?,
        None => Value::None,
    };

    match binding.kind() {
        LetBindingKind::Normal(pattern) => {
            // Binding simples: #let x = ... → Pattern::Normal(Expr::Ident(x))
            let bindings = pattern.bindings();
            if let Some(ident) = bindings.into_iter().next() {
                let name = ident.as_str().to_string();
                // Se a closure ainda não tem nome, dar-lhe o nome da binding (para recursão)
                if let Value::Func(ref mut func) = value {
                    func.set_name(name.clone());
                }
                scopes.define(name, value);
            }
        }
        LetBindingKind::Closure(ident) => {
            // Sintaxe function shorthand: #let fib(n) = ...
            // O nó Closure já carrega o nome — apenas definir no scope.
            // set_name() não é necessário: o nome vem de closure_expr.name()
            // no arm Expr::Closure (ver eval_expr).
            let name = ident.as_str().to_string();
            scopes.define(name, value);
        }
    }

    Ok(Value::None)
}

/// **P506** — Despacha métodos de `Value::State`: `.update()`, `.get()`,
/// `.display()`.
pub(super) fn eval_state_method(
    state: &State,
    method: &str,
    args: crate::entities::ast::expr::Args<'_>,
    scopes: &mut Scopes<'_>,
    ctx: &mut EvalContext,
    engine: &mut Engine<'_>,
) -> SourceResult<Value> {
    use crate::rules::eval::closures::eval_args;
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
        _ => Err(vec![SourceDiagnostic::error(
            span,
            format!("state não tem método '{}'", method),
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
) -> SourceResult<(Option<crate::entities::label::Label>, Option<Value>)> {
    use crate::entities::ast::expr::Arg;

    let mut at_label: Option<crate::entities::label::Label> = None;
    let mut pattern: Option<Value> = None;

    for arg in args.items() {
        match arg {
            Arg::Named(named) if named.name().as_str() == "at" => {
                at_label = Some(extract_display_at_label(
                    named.expr(),
                    scopes,
                    ctx,
                    engine,
                )?);
            }
            Arg::Named(named) => {
                return Err(vec![SourceDiagnostic::error(
                    named.span(),
                    format!(
                        "unexpected argument: {}",
                        named.name().as_str()
                    ),
                )]);
            }
            Arg::Pos(expr) if pattern.is_none() => {
                let value = eval_expr(expr, scopes, ctx, engine)?;
                match value {
                    Value::Str(_) | Value::Func(_) => pattern = Some(value),
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
                    "counter.display() takes at most one positional argument"
                        .to_string(),
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

    Ok((at_label, pattern))
}

/// **P640** — Extrai uma label do argumento nomeado `at:` de `counter.display`.
/// Aceita `<label>` (nó AST), string, ou content label.
fn extract_display_at_label(
    expr: Expr<'_>,
    scopes: &mut Scopes<'_>,
    ctx: &mut EvalContext,
    engine: &mut Engine<'_>,
) -> SourceResult<crate::entities::label::Label> {
    let span = expr.span();
    match expr {
        Expr::Label(node) => Ok(crate::entities::label::Label(
            node.get().to_string(),
        )),
        other => {
            let value = eval_expr(other, scopes, ctx, engine)?;
            match value {
                Value::Str(s) => Ok(crate::entities::label::Label(s.to_string())),
                Value::Content(crate::entities::content::Content::Label(e)) => {
                    Ok(crate::entities::label::Label(e.name.to_string()))
                }
                other => Err(vec![SourceDiagnostic::error(
                    span,
                    format!(
                        "expected label, function, location, selector, or auto, found {}",
                        other.type_name()
                    ),
                )]),
            }
        }
    }
}

/// **P640** — Renderiza `counter.display(..., at: <label>)` para texto plano.
fn render_counter_at_label(
    key: &str,
    label: &crate::entities::label::Label,
    pattern: Option<&Value>,
    ctx: &EvalContext,
) -> SourceResult<Value> {
    use crate::entities::introspector::Introspector;
    let text = ctx
        .introspector
        .query_by_label(label)
        .and_then(|loc| ctx.introspector.formatted_counter_at(key, loc))
        .unwrap_or_default();

    let rendered = match pattern {
        Some(Value::Str(p)) if !p.is_empty() => {
            if p.as_str().ends_with('.') {
                let mut out = text.clone();
                out.push('.');
                out
            } else {
                format!("{p}{text}")
            }
        }
        Some(Value::Func(_)) => {
            // Callbacks com `at:` não são suportados nesta fase; ignorar o
            // pattern e devolver o texto formatado pelo introspector.
            text
        }
        _ => text,
    };

    Ok(Value::Content(Content::text(rendered)))
}

/// **P506** — Despacha métodos de `Value::Counter`: `.update()`, `.step()`,
/// `.get()`, `.display()`, `.at()`.
pub(super) fn eval_counter_method_value(
    counter: &Counter,
    method: &str,
    args: crate::entities::ast::expr::Args<'_>,
    scopes: &mut Scopes<'_>,
    ctx: &mut EvalContext,
    engine: &mut Engine<'_>,
) -> SourceResult<Value> {
    use crate::rules::eval::closures::eval_args;
    let span = args.span();
    match method {
        "update" => {
            let args = eval_args(args, scopes, ctx, engine)?;
            counter_update(counter.key.clone(), args.items.into_iter().next().unwrap_or(Value::None))
        }
        "step" => {
            let _ = eval_args(args, scopes, ctx, engine)?;
            Ok(counter_step(counter.key.clone()))
        }
        "get" => {
            let _ = eval_args(args, scopes, ctx, engine)?;
            counter_get(counter, ctx, span)
        }
        "display" => {
            // P640 — parse unificado e validação estrita dos argumentos.
            let (at_label, pattern) = parse_counter_display_args(args, scopes, ctx, engine)?;
            if let Some(label) = at_label {
                render_counter_at_label(counter.key.as_str(), &label, pattern.as_ref(), ctx)
            } else {
                let args = Args::positional(pattern.into_iter().collect());
                counter_display(counter, &args, scopes, ctx, engine, span)
            }
        }
        "at" => {
            // P506 — counter.at(label): o parser cristalino avalia `<label>`
            // como Value::None; extraímos a string directamente do nó AST
            // para suportar a sintaxe vanilla.
            let label = extract_label_from_args(args, scopes, ctx, engine, span)?;
            counter_at(counter, label, ctx, span)
        }
        _ => Err(vec![SourceDiagnostic::error(
            span,
            format!("counter não tem método '{}'", method),
        )]),
    }
}

/// **P506** — Extrai uma `Label` do primeiro argumento de `counter.at(label)`.
/// Suporta `<label>` (nó AST Label → Value::None no eval cristalino) e strings.
fn extract_label_from_args(
    args: crate::entities::ast::expr::Args<'_>,
    scopes: &mut Scopes<'_>,
    ctx: &mut EvalContext,
    engine: &mut Engine<'_>,
    span: Span,
) -> SourceResult<crate::entities::label::Label> {
    use crate::entities::ast::expr::Arg;
    if let Some(first) = args.items().next() {
        match first {
            Arg::Pos(Expr::Label(node)) => {
                return Ok(crate::entities::label::Label(node.get().to_string()));
            }
            Arg::Pos(expr) => {
                let value = eval_expr(expr, scopes, ctx, engine)?;
                match value {
                    Value::Str(s) => return Ok(crate::entities::label::Label(s.to_string())),
                    Value::Content(crate::entities::content::Content::Label(e)) => {
                        return Ok(crate::entities::label::Label(e.name.to_string()));
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    }
    Err(vec![SourceDiagnostic::error(
        span,
        "counter.at() requer label ou string como argumento".to_string(),
    )])
}

/// **P417 (M)** — Tenta avaliar `<elemento>.where(field: value)`.
///
/// Retorna `Ok(Some(Selector::Where { ... }))` se o target for uma função
/// nativa de elemento suportada (heading, figure, strong, emph, raw) e houver
/// exatamente um named argumento. Retorna `Ok(None)` se o target não for um
/// elemento nativo (deixa o caller continuar com field access normal).
/// Retorna `Err` se for elemento mas os argumentos forem inválidos.
pub(super) fn eval_element_where<'a>(
    target_expr: Expr<'_>,
    args_node: crate::entities::ast::expr::Args<'a>,
    scopes: &mut Scopes<'_>,
    ctx: &mut EvalContext,
    engine: &mut Engine<'_>,
) -> SourceResult<Option<Selector>> {
    use crate::rules::stdlib::{
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
            use crate::rules::stdlib::{
                native_figure, native_heading,
            };
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

pub(super) fn eval_selector_or_and<'a>(
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

    let args = super::closures::eval_args(args_node, scopes, ctx, engine)?;
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
pub(super) fn eval_selector_within<'a>(
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

    let args = super::closures::eval_args(args_node, scopes, ctx, engine)?;
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

    Ok(Some(Selector::Within {
        base: Box::new(base),
        ancestor: Box::new(ancestor),
    }))
}

// ── Dispatcher arms: FieldAccess (Passo 96.2, ADR-0037 Regra 4) ───────────

pub(super) fn eval_field_access(
    access: crate::entities::ast::expr::FieldAccess<'_>,
    scopes: &mut Scopes<'_>,
    ctx: &mut EvalContext,
    engine: &mut Engine<'_>,
) -> SourceResult<Value> {
    use crate::entities::ast::AstNode;
    use crate::entities::source_result::SourceDiagnostic;

    let target = eval_expr(access.target(), scopes, ctx, engine)?;
    let field = access.field().as_str().to_string();

    // P509 — field access em coleções despacha para métodos de instância.
    if let Some(result) = crate::rules::stdlib::try_dispatch_collection_method(
        target.clone(),
        field.as_str(),
        crate::entities::args::Args::positional(vec![]),
        scopes,
        ctx,
        engine,
    ) {
        return result;
    }

    match target {
        Value::Dict(d) => d.get(field.as_str()).cloned().ok_or_else(|| {
            vec![SourceDiagnostic::error(
                access.span(),
                format!("campo '{field}' não existe"),
            )]
        }),
        // Field access em elementos estruturados — usado por show rules (Passo 68).
        // Ex: `it.body` onde `it` é Content::Heading retorna Value::Content(body).
        Value::Content(c) => c.get_field(field.as_str()).ok_or_else(|| {
            vec![SourceDiagnostic::error(
                access.span(),
                format!("campo '{field}' não existe neste elemento de conteúdo"),
            )]
        }),
        // P411 — Field access em Value::Version (semver): major/minor/patch/pre/build.
        Value::Version(v) => match field.as_str() {
            "major" => Ok(Value::Int(v.major as i64)),
            "minor" => Ok(Value::Int(v.minor as i64)),
            "patch" => Ok(Value::Int(v.patch as i64)),
            "pre" => {
                Ok(Value::Array(v.pre.iter().map(|s| Value::Str(s.clone())).collect()))
            }
            "build" => {
                Ok(Value::Array(v.build.iter().map(|s| Value::Str(s.clone())).collect()))
            }
            _ => Err(vec![SourceDiagnostic::error(
                access.span(),
                format!("campo desconhecido em version: '{}'", field),
            )]),
        },
        // P412 — Field access em Value::Duration: seconds/minutes/hours/days (retorno Float).
        Value::Duration(d) => {
            const NANOS_PER_SECOND: f64 = 1_000_000_000.0;
            const NANOS_PER_MINUTE: f64 = 60_000_000_000.0;
            const NANOS_PER_HOUR: f64 = 3_600_000_000_000.0;
            const NANOS_PER_DAY: f64 = 86_400_000_000_000.0;
            match field.as_str() {
                "seconds" => Ok(Value::Float(d.nanos as f64 / NANOS_PER_SECOND)),
                "minutes" => Ok(Value::Float(d.nanos as f64 / NANOS_PER_MINUTE)),
                "hours" => Ok(Value::Float(d.nanos as f64 / NANOS_PER_HOUR)),
                "days" => Ok(Value::Float(d.nanos as f64 / NANOS_PER_DAY)),
                _ => Err(vec![SourceDiagnostic::error(
                    access.span(),
                    format!("campo desconhecido em duration: '{}'", field),
                )]),
            }
        }
        // P493a — Field access em Value::Array: len, first, last.
        // dedup/chunks/windows são despachados via try_dispatch_collection_method
        // em closures.rs (method call), não como field access.
        Value::Array(arr) => match field.as_str() {
            "len" => Ok(Value::Int(arr.len() as i64)),
            "first" => Ok(arr.first().cloned().unwrap_or(Value::None)),
            "last" => Ok(arr.last().cloned().unwrap_or(Value::None)),
            _ => Err(vec![SourceDiagnostic::error(
                access.span(),
                format!("campo desconhecido em array: '{}'", field),
            )]),
        },
        // P504 — Field access em Value::Args: `.named` e `.positional`.
        Value::Args(a) => match field.as_str() {
            "named" => Ok(Value::Dict(a.named.clone())),
            "positional" => Ok(Value::Array(a.items.clone())),
            _ => Err(vec![SourceDiagnostic::error(
                access.span(),
                format!("campo desconhecido em arguments: '{}'", field),
            )]),
        },
        // P493b — Field access em Value::Func com namespace anexado (table.header, etc.).
        Value::Func(f) => match f.namespace() {
            Some(ns) => ns.get(field.as_str()).cloned().ok_or_else(|| {
                vec![SourceDiagnostic::error(
                    access.span(),
                    format!("função não tem campo '{}'", field),
                )]
            }),
            None => Err(vec![SourceDiagnostic::error(
                access.span(),
                "esta função não tem campos".to_string(),
            )]),
        },
        other => Err(vec![SourceDiagnostic::error(
            access.span(),
            format!("field access não suportado em {}", other.type_name()),
        )]),
    }
}
