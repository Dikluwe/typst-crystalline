//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/rules/eval.md
//! @prompt-hash 19073424
//! @layer L1
//! @updated 2026-04-22
//!
//! Controlo de fluxo: `if`/`else`, `while`, `for`. Extraído de `eval.rs` no
//! Passo 96.1 conforme ADR-0037 (coesão por domínio).

use std::sync::Arc;

use comemo::Tracked;

use crate::entities::ast::code::{Conditional, ForLoop, WhileLoop};
use crate::entities::ast::AstNode;
use crate::entities::show::{RuleId, ShowRule};
use crate::entities::source_result::{SourceDiagnostic, SourceResult};
use crate::entities::style_chain::StyleChain;
use crate::entities::value::Value;
use crate::entities::world_types::Route;
use crate::rules::scopes::Scopes;

use super::{eval_expr, EvalContext};

pub(super) fn eval_conditional<'r>(
    cond: Conditional<'_>,
    scopes: &mut Scopes<'_>,
    ctx: &mut EvalContext<'_>,
    route: Tracked<'r, Route<'r>>,
    styles: &mut StyleChain,
    show_rules: &mut Arc<[ShowRule]>,
    active_guards: &mut Vec<RuleId>,
) -> SourceResult<Value> {
    let condition = eval_expr(cond.condition(), scopes, ctx, route, styles, show_rules, active_guards)?;
    match condition {
        Value::Bool(true) => eval_expr(cond.if_body(), scopes, ctx, route, styles, show_rules, active_guards),
        Value::Bool(false) => match cond.else_body() {
            Some(else_body) => eval_expr(else_body, scopes, ctx, route, styles, show_rules, active_guards),
            None            => Ok(Value::None),
        },
        other => Err(vec![SourceDiagnostic::error(
            cond.condition().span(),
            format!("condição if deve ser bool, encontrado {}", other.type_name()),
        )]),
    }
}

pub(super) fn eval_while<'r>(
    loop_expr: WhileLoop<'_>,
    scopes: &mut Scopes<'_>,
    ctx: &mut EvalContext<'_>,
    route: Tracked<'r, Route<'r>>,
    styles: &mut StyleChain,
    show_rules: &mut Arc<[ShowRule]>,
    active_guards: &mut Vec<RuleId>,
) -> SourceResult<Value> {
    loop {
        let cond = eval_expr(loop_expr.condition(), scopes, ctx, route, styles, show_rules, active_guards)?;
        match cond {
            Value::Bool(true) => {
                ctx.tick_loop(loop_expr.span())?;
                scopes.enter();
                eval_expr(loop_expr.body(), scopes, ctx, route, styles, show_rules, active_guards)?;
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

pub(super) fn eval_for<'r>(
    loop_expr: ForLoop<'_>,
    scopes: &mut Scopes<'_>,
    ctx: &mut EvalContext<'_>,
    route: Tracked<'r, Route<'r>>,
    styles: &mut StyleChain,
    show_rules: &mut Arc<[ShowRule]>,
    active_guards: &mut Vec<RuleId>,
) -> SourceResult<Value> {
    let iterable = eval_expr(loop_expr.iterable(), scopes, ctx, route, styles, show_rules, active_guards)?;
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
                eval_expr(loop_expr.body(), scopes, ctx, route, styles, show_rules, active_guards)?;
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
