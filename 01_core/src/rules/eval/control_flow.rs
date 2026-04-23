//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/rules/eval.md
//! @prompt-hash 19073424
//! @layer L1
//! @updated 2026-04-23
//!
//! Controlo de fluxo: `if`/`else`, `while`, `for`. Extraído de `eval.rs` no
//! Passo 96.1 conforme ADR-0037 (coesão por domínio). Assinaturas
//! simplificadas no Passo 109 (ADR-0044) via `Engine<'_>`.

use crate::entities::ast::code::{Conditional, ForLoop, WhileLoop};
use crate::entities::ast::AstNode;
use crate::entities::engine::Engine;
use crate::entities::source_result::{SourceDiagnostic, SourceResult};
use crate::entities::value::Value;
use crate::rules::scopes::Scopes;

use super::{eval_expr, EvalContext};

pub(super) fn eval_conditional(
    cond: Conditional<'_>,
    scopes: &mut Scopes<'_>,
    ctx: &mut EvalContext,
    engine: &mut Engine<'_>,
) -> SourceResult<Value> {
    let condition = eval_expr(cond.condition(), scopes, ctx, engine)?;
    match condition {
        Value::Bool(true) => eval_expr(cond.if_body(), scopes, ctx, engine),
        Value::Bool(false) => match cond.else_body() {
            Some(else_body) => eval_expr(else_body, scopes, ctx, engine),
            None            => Ok(Value::None),
        },
        other => Err(vec![SourceDiagnostic::error(
            cond.condition().span(),
            format!("condição if deve ser bool, encontrado {}", other.type_name()),
        )]),
    }
}

pub(super) fn eval_while(
    loop_expr: WhileLoop<'_>,
    scopes: &mut Scopes<'_>,
    ctx: &mut EvalContext,
    engine: &mut Engine<'_>,
) -> SourceResult<Value> {
    loop {
        let cond = eval_expr(loop_expr.condition(), scopes, ctx, engine)?;
        match cond {
            Value::Bool(true) => {
                ctx.tick_loop(loop_expr.span())?;
                scopes.enter();
                eval_expr(loop_expr.body(), scopes, ctx, engine)?;
                scopes.exit();
            }
            Value::Bool(false) => break,
            other => return Err(vec![SourceDiagnostic::error(
                loop_expr.condition().span(),
                format!("condição while deve ser bool, encontrado {}", other.type_name()),
            )]),
        }
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
            let name = bindings.first()
                .map(|ident| ident.as_str().to_string())
                .unwrap_or_default();
            for item in items {
                ctx.tick_loop(loop_expr.span())?;
                scopes.enter();
                scopes.define(name.as_str(), item);
                eval_expr(loop_expr.body(), scopes, ctx, engine)?;
                scopes.exit();
            }
            Ok(Value::None)
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
