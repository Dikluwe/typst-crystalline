//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/rules/eval.md
//! @prompt-hash cce90241
//! @layer L1
//! @updated 2026-07-09
//!
//! Controlo de fluxo: `if`/`else`, `while`, `for`. Extraído de `eval.rs` no
//! Passo 96.1 conforme ADR-0037 (coesão por domínio). Assinaturas
//! simplificadas no Passo 109 (ADR-0044) via `Engine<'_>`.

use crate::entities::ast::code::{Conditional, ForLoop, WhileLoop};
use crate::entities::ast::AstNode;
use crate::entities::content::Content;
use crate::entities::engine::Engine;
use crate::entities::source_result::{SourceDiagnostic, SourceResult};
use crate::entities::value::Value;
use crate::rules::scopes::Scopes;

use super::{eval_expr, EvalContext, FlowEvent};

pub(super) fn eval_conditional(
    cond: Conditional<'_>,
    scopes: &mut Scopes<'_>,
    ctx: &mut EvalContext,
    engine: &mut Engine<'_>,
) -> SourceResult<Value> {
    let condition = eval_expr(cond.condition(), scopes, ctx, engine)?;
    let output = match condition {
        Value::Bool(true) => eval_expr(cond.if_body(), scopes, ctx, engine),
        Value::Bool(false) => match cond.else_body() {
            Some(else_body) => eval_expr(else_body, scopes, ctx, engine),
            None            => Ok(Value::None),
        },
        other => Err(vec![SourceDiagnostic::error(
            cond.condition().span(),
            format!("condição if deve ser bool, encontrado {}", other.type_name()),
        )]),
    };

    // P635 — marcar return como conditional (paridade com
    // `typst-eval/src/flow.rs:55-57`).
    if let Some(FlowEvent::Return(_, _, conditional)) = &mut ctx.flow {
        *conditional = true;
    }

    output
}

pub(super) fn eval_while(
    loop_expr: WhileLoop<'_>,
    scopes: &mut Scopes<'_>,
    ctx: &mut EvalContext,
    engine: &mut Engine<'_>,
) -> SourceResult<Value> {
    let flow = ctx.flow.take();
    loop {
        let cond = eval_expr(loop_expr.condition(), scopes, ctx, engine)?;
        match cond {
            Value::Bool(true) => {
                ctx.tick_loop(loop_expr.span())?;
                scopes.enter();
                eval_expr(loop_expr.body(), scopes, ctx, engine)?;
                scopes.exit();

                match ctx.flow {
                    Some(FlowEvent::Break(_)) => {
                        ctx.flow = None;
                        break;
                    }
                    Some(FlowEvent::Continue(_)) => ctx.flow = None,
                    Some(FlowEvent::Return(..)) => break,
                    None => {}
                }
            }
            Value::Bool(false) => break,
            other => return Err(vec![SourceDiagnostic::error(
                loop_expr.condition().span(),
                format!("condição while deve ser bool, encontrado {}", other.type_name()),
            )]),
        }
    }
    if flow.is_some() {
        ctx.flow = flow;
    }
    Ok(Value::None)
}

pub(super) fn eval_for(
    loop_expr: ForLoop<'_>,
    scopes: &mut Scopes<'_>,
    ctx: &mut EvalContext,
    engine: &mut Engine<'_>,
) -> SourceResult<Value> {
    let iterable = eval_expr(loop_expr.iterable(), scopes, ctx, engine)?;
    match iterable {
        Value::Array(items) => {
            let bindings = loop_expr.pattern().bindings();
            // **P540** — suportar destructuring de tuplo em #for. Se o padrão
            // tiver múltiplos bindings (ex: (i, x)), cada item do iterável deve
            // ser um array cujos elementos são atribuídos posicionalmente.
            let flow = ctx.flow.take();
            let mut parts = Vec::new();
            for item in items {
                ctx.tick_loop(loop_expr.span())?;
                scopes.enter();

                if bindings.is_empty() {
                    // Pattern vazio ou underscore — não definir variáveis.
                } else if bindings.len() == 1 {
                    scopes.define(bindings[0].as_str(), item);
                } else {
                    let tuple = match &item {
                        Value::Array(t) => t.clone(),
                        other => {
                            scopes.exit();
                            return Err(vec![SourceDiagnostic::error(
                                loop_expr.pattern().span(),
                                format!("cannot destructure value of type {}", other.type_name()),
                            )]);
                        }
                    };
                    if tuple.len() != bindings.len() {
                        scopes.exit();
                        return Err(vec![SourceDiagnostic::error(
                            loop_expr.pattern().span(),
                            format!(
                                "cannot destructure {} values into {} bindings",
                                tuple.len(), bindings.len()
                            ),
                        )]);
                    }
                    for (ident, value) in bindings.iter().zip(tuple.into_iter()) {
                        scopes.define(ident.as_str(), value);
                    }
                }

                match eval_expr(loop_expr.body(), scopes, ctx, engine)? {
                    Value::Content(c) => parts.push(c),
                    Value::Str(s) => parts.push(Content::text(s.as_str())),
                    Value::None => {}
                    other => {
                        scopes.exit();
                        return Err(vec![SourceDiagnostic::error(
                            loop_expr.body().span(),
                            format!("corpo do for deve ser content, encontrado {}", other.type_name()),
                        )]);
                    }
                }
                scopes.exit();

                match ctx.flow {
                    Some(FlowEvent::Break(_)) => {
                        ctx.flow = None;
                        break;
                    }
                    Some(FlowEvent::Continue(_)) => ctx.flow = None,
                    Some(FlowEvent::Return(..)) => break,
                    None => {}
                }
            }
            if flow.is_some() {
                ctx.flow = flow;
            }
            if parts.is_empty() {
                Ok(Value::None)
            } else {
                Ok(Value::Content(Content::sequence(parts)))
            }
        }
        // `()` em Typst avalia para None via fronteira deliberada (não há parsing
        // de array literal neste passo). Tratar None como iterável vazio.
        Value::None => Ok(Value::None),
        other => Err(vec![SourceDiagnostic::error(
            loop_expr.iterable().span(),
            format!("não é possível iterar sobre {}", other.type_name()),
        )]),
    }
}
