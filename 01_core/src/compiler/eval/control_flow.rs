//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/eval.md
//! @prompt-hash 93aba14a
//! @layer L1
//! @updated 2026-07-09
//!
//! Controlo de fluxo: `if`/`else`, `while`, `for`. Extraído de `eval.rs` no
//! Passo 96.1 conforme ADR-0037 (coesão por domínio). Assinaturas
//! simplificadas no Passo 109 (ADR-0044) via `Engine<'_>`.

use crate::compiler::scopes::Scopes;
use crate::entities::ast::code::{Conditional, ForLoop, WhileLoop};
use crate::entities::ast::AstNode;
use crate::entities::engine::Engine;
use crate::entities::source_result::{SourceDiagnostic, SourceResult};
use crate::entities::value::Value;

use super::{bindings::destructure_let, eval_expr, operators, EvalContext, FlowEvent};

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
            None => Ok(Value::None),
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
    // **P729** — o valor do `while` é o `join` dos valores dos corpos de
    // todas as iterações (paridade vanilla `typst-eval/src/flow.rs:69,86`),
    // não `None`. Reaproveita `operators::join` de P728; o valor de cada
    // corpo já traz o join intra-bloco (`Expr::CodeBlock`).
    let mut output = Value::None;
    loop {
        let cond = eval_expr(loop_expr.condition(), scopes, ctx, engine)?;
        match cond {
            Value::Bool(true) => {
                ctx.tick_loop(loop_expr.span())?;
                scopes.enter();
                let value = eval_expr(loop_expr.body(), scopes, ctx, engine)?;
                let joined = operators::join(output, value);
                scopes.exit();
                output = joined.map_err(|msg| {
                    vec![SourceDiagnostic::error(loop_expr.body().span(), msg)]
                })?;

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
            other => {
                return Err(vec![SourceDiagnostic::error(
                    loop_expr.condition().span(),
                    format!(
                        "condição while deve ser bool, encontrado {}",
                        other.type_name()
                    ),
                )])
            }
        }
    }
    if flow.is_some() {
        ctx.flow = flow;
    }
    Ok(output)
}

pub(super) fn eval_for(
    loop_expr: ForLoop<'_>,
    scopes: &mut Scopes<'_>,
    ctx: &mut EvalContext,
    engine: &mut Engine<'_>,
) -> SourceResult<Value> {
    let iterable = eval_expr(loop_expr.iterable(), scopes, ctx, engine)?;
    match iterable {
        Value::Array(items) => run_for_loop(items, loop_expr, scopes, ctx, engine),
        // **P719** — mirror do vanilla (`typst-eval/flow.rs:159-162`,
        // `dict.iter()`): cada par (chave, valor) vira o mesmo formato usado
        // pela iteração de array — `Value::Array([Str(key), value])`,
        // espelho de `IntoValue for (&Str, &Value)` (`foundations/cast.rs:
        // 186-189`). Reaproveita directamente `run_for_loop` — um só nome
        // liga o par inteiro; dois nomes destroem posicionalmente (mesma
        // lógica já usada para array, sem código novo de bind).
        Value::Dict(dict) => {
            let items: Vec<Value> = dict
                .into_iter()
                .map(|(k, v)| Value::Array(vec![Value::Str(k), v]))
                .collect();
            run_for_loop(items, loop_expr, scopes, ctx, engine)
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

/// Corpo do `for`: percorre `items` (já achatado para o formato de
/// iteração — array directo, ou pares de dict como `Value::Array([k, v])`
/// via P719), ligando o padrão a cada item.
fn run_for_loop(
    items: Vec<Value>,
    loop_expr: ForLoop<'_>,
    scopes: &mut Scopes<'_>,
    ctx: &mut EvalContext,
    engine: &mut Engine<'_>,
) -> SourceResult<Value> {
    let flow = ctx.flow.take();
    // **P729** — o valor do `for` é o `join` dos valores dos corpos de
    // todas as iterações (paridade vanilla `typst-eval/src/flow.rs:120,
    // 132`), não só `Content`/`Str` acumulados — qualquer tipo junta-se
    // pela tabela de `operators::join` (P728), o resto é erro
    // "cannot join X with Y". Medido: `#for i in (1,) { (1,); (2,) }` →
    // `(1, 2)` no vanilla (cristalino pré-P729 errava "corpo do for deve
    // ser content").
    let mut output = Value::None;
    for item in items {
        ctx.tick_loop(loop_expr.span())?;
        scopes.enter();

        // **P723** — delegar o binding de cada item ao destructuring
        // genérico (mesma entrada do `#let`, bindings.rs): suporta spread
        // `..sink` (cetz path-util.typ:106), destrói tuplos de 1 elemento
        // e produz as mensagens de aridade exactas do vanilla
        // (wrong_number_of_elements, P715). Substitui o bind manual de
        // P540 (sem spread, mensagem própria).
        destructure_let(loop_expr.pattern(), item, scopes, ctx, engine)?;

        let value = eval_expr(loop_expr.body(), scopes, ctx, engine)?;
        let joined = operators::join(output, value);
        scopes.exit();
        output = joined
            .map_err(|msg| vec![SourceDiagnostic::error(loop_expr.body().span(), msg)])?;

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
    Ok(output)
}
