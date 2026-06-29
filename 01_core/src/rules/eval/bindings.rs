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
use crate::entities::counter_update::CounterUpdate as CounterAction;
use crate::entities::element_kind::ElementKind;
use crate::entities::engine::Engine;
use crate::entities::selector::Selector;
use crate::entities::source_result::{SourceDiagnostic, SourceResult};
use crate::entities::value::Value;
use crate::rules::scopes::Scopes;

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

/// Retorna `None` se a expressão não for uma chamada a `counter`.
pub(super) fn extract_counter_key(expr: Expr<'_>) -> Option<String> {
    let call = match expr {
        Expr::FuncCall(c) => c,
        _ => return None,
    };
    // Verificar que o callee é o identificador "counter"
    let callee_name = match call.callee() {
        Expr::Ident(id) => id.as_str().to_string(),
        _ => return None,
    };
    if callee_name != "counter" {
        return None;
    }

    // Extrair o primeiro argumento posicional como chave string
    let first_arg = call.args().items().next()?;
    match first_arg {
        Arg::Pos(Expr::Ident(id)) => Some(id.as_str().to_string()),
        Arg::Pos(Expr::Str(s)) => Some(s.get().to_string()),
        _ => None,
    }
}

/// Avalia um método de contador: step(), update(), get(), display().
pub(super) fn eval_counter_method<'a>(
    key: &str,
    method: &str,
    args: crate::entities::ast::expr::Args<'a>,
    scopes: &mut Scopes<'_>,
    ctx: &mut EvalContext,
    engine: &mut Engine<'_>,
) -> SourceResult<Value> {
    match method {
        "step" => Ok(Value::Content(Content::counter_update(
            key.to_string(),
            CounterAction::Step,
        ))),

        "update" => {
            // Extrair o valor numérico do primeiro argumento.
            // Defensivo: se o argumento não for Int, usar 0 silenciosamente.
            let val = args
                .items()
                .next()
                .and_then(|arg| match arg {
                    Arg::Pos(expr) => {
                        if let Ok(Value::Int(n)) = eval_expr(expr, scopes, ctx, engine) {
                            Some(n.max(0) as usize)
                        } else {
                            None
                        }
                    }
                    _ => None,
                })
                .unwrap_or(0);
            Ok(Value::Content(Content::counter_update(
                key.to_string(),
                CounterAction::Update(val),
            )))
        }

        // get(), display() e outros — fallback até motor de introspecção completo
        _ => {
            // P504 — counter.display(pattern?, at: <label>?)
            let mut pattern: Option<String> = None;
            let mut at_label: Option<crate::entities::label::Label> = None;
            for arg in args.items() {
                match arg {
                    Arg::Pos(expr) if pattern.is_none() => {
                        if let Ok(Value::Str(s)) = eval_expr(expr, scopes, ctx, engine) {
                            pattern = Some(s.to_string());
                        }
                    }
                    Arg::Named(named) if named.name().as_str() == "at" => {
                        // Typst 0.15.0 aceita `at: <label>` — o cristalino ainda
                        // não tem `Value::Label`, pelo que extraímos a string do
                        // nó AST `Expr::Label` directamente.
                        match named.expr() {
                            Expr::Label(label_node) => {
                                at_label = Some(crate::entities::label::Label(
                                    label_node.get().to_string().into(),
                                ));
                            }
                            other_expr => {
                                if let Ok(Value::Str(s)) =
                                    eval_expr(other_expr, scopes, ctx, engine)
                                {
                                    at_label = Some(crate::entities::label::Label(s.to_string()));
                                }
                            }
                        }
                    }
                    _ => {}
                }
            }

            if let Some(label) = at_label {
                use crate::entities::introspector::Introspector;
                let text = ctx
                    .introspector
                    .query_by_label(&label)
                    .and_then(|loc| ctx.introspector.formatted_counter_at(key, loc))
                    .unwrap_or_default();
                // Aplicar pattern minimal: "1." → "1." (append).
                let rendered = match pattern {
                    Some(p) if !p.is_empty() => {
                        let mut out = text.clone();
                        // Se o pattern termina com separador, anexá-lo ao
                        // valor hierárquico já formatado.
                        if p.ends_with('.') {
                            out.push('.');
                        } else {
                            out = format!("{p}{text}");
                        }
                        out
                    }
                    _ => text,
                };
                Ok(Value::Content(Content::text(rendered)))
            } else {
                Ok(Value::Content(Content::counter_display(key.to_string())))
            }
        }
    }
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
