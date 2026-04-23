//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/rules/eval.md
//! @prompt-hash 19073424
//! @layer L1
//! @updated 2026-04-23
//!
//! Closures, chamadas de função e avaliação de argumentos. Extraído de
//! `eval.rs` no Passo 96.1 conforme ADR-0037 (coesão por domínio).
//! Assinaturas simplificadas no Passo 109 (ADR-0044) via `Engine<'_>`.

use ecow::EcoString;
use indexmap::IndexMap;
use rustc_hash::FxBuildHasher;

use crate::entities::args::Args;
use crate::entities::ast::expr::{Arg, Closure as ClosureNode, Expr, FuncCall as FuncCallNode, Param, Pattern};
use crate::entities::ast::AstNode;
use crate::entities::content::Content;
use crate::entities::engine::Engine;
use crate::entities::func::{ClosureParam, ClosureRepr, Func, FuncRepr};
use crate::entities::source_result::SourceDiagnostic;
use crate::entities::source_result::SourceResult;
use crate::entities::value::Value;
use comemo::TrackedMut;

use crate::entities::world_types::{check_call_depth as route_check_call_depth, Route};
use crate::rules::scopes::Scopes;

use super::{eval_expr, EvalContext};

/// Avalia a lista de argumentos de uma chamada de função.
///
/// Posicionais são avaliados em ordem; named args são avaliados e indexados
/// por nome. Spread ignorado (fronteira deliberada, adiado).
pub(super) fn eval_args(
    args_node: crate::entities::ast::expr::Args<'_>,
    scopes: &mut Scopes<'_>,
    ctx: &mut EvalContext,
    engine: &mut Engine<'_>,
) -> SourceResult<Args> {
    let mut items = Vec::new();
    let mut named: IndexMap<EcoString, Value, FxBuildHasher> = IndexMap::default();
    for arg in args_node.items() {
        match arg {
            Arg::Pos(expr) => items.push(eval_expr(expr, scopes, ctx, engine)?),
            Arg::Named(name_expr) => {
                named.insert(
                    name_expr.name().as_str().into(),
                    eval_expr(name_expr.expr(), scopes, ctx, engine)?,
                );
            }
            Arg::Spread(_) => {}  // fronteira deliberada
        }
    }
    Ok(Args { items, named })
}

/// Aplica uma função (closure ou native) aos args dados.
pub(crate) fn apply_func(
    func: Func,
    args: Args,
    ctx: &mut EvalContext,
    engine: &mut Engine<'_>,
) -> SourceResult<Value> {
    match func.repr() {
        FuncRepr::Closure(closure) => apply_closure(closure, &func, args, ctx, engine),
        FuncRepr::Native(native)   => {
            let world = engine.world;
            let current_file = engine.current_file;
            let figure_numbering = engine.figure_numbering.as_deref();
            (native.call)(ctx, &args, world, current_file, figure_numbering)
        }
    }
}

/// Aplica uma closure: cria scope filho do captured, injeta auto-ref + params.
///
/// **Lookup lazy via Arc**: `Scopes::with_parent(Arc::clone(&closure.captured))`
/// cria um scope filho sem clonar os valores capturados. O lookup percorre
/// `top` (params/auto-ref) → `captured` (scope da definição) sem cópia.
///
/// **Auto-injecção para recursão**: se a closure tem nome, injeta
/// `Value::Func(func.clone())` em `call_scopes.top`. O Arc é destruído
/// quando `call_scopes` sai de scope — sem ciclo permanente.
///
/// **Ordem auto-ref → params**: a auto-referência é definida primeiro para
/// que um parâmetro com o mesmo nome que a função sombre correctamente.
pub(super) fn apply_closure(
    closure: &ClosureRepr,
    func: &Func,
    args: Args,
    ctx: &mut EvalContext,
    engine: &mut Engine<'_>,
) -> SourceResult<Value> {
    // Limite de chamadas vanilla (MAX_CALL_DEPTH = 80) — paridade com
    // `typst-eval/src/call.rs:33` (ADR-0033). Pago parcial do DEBT-45
    // no Passo 93.
    route_check_call_depth(engine.route)?;

    // Criar scope filho do captured — O(1), sem clone dos valores capturados.
    let mut call_scopes = Scopes::with_parent(std::sync::Arc::clone(&closure.captured));

    // Auto-injecção para recursão — definida antes dos params para que um
    // parâmetro com o mesmo nome sombre a função (comportamento do original).
    if let Some(ref name) = closure.name {
        call_scopes.define(name.clone(), Value::Func(func.clone()));
    }

    // Bind parâmetros: named args têm prioridade sobre posicionais;
    // se nenhum, usar default; se não há default, usar None.
    let mut pos_idx = 0;
    for param in closure.params.iter() {
        let val = if let Some(v) = args.named.get(param.name.as_str()) {
            v.clone()
        } else if let Some(v) = args.items.get(pos_idx) {
            pos_idx += 1;
            v.clone()
        } else {
            param.default.clone().unwrap_or(Value::None)
        };
        call_scopes.define(param.name.as_str(), val);
    }

    // Frame de chamada: novo segmento `Route::extend(route)` com `len: 1`.
    // Paridade com `typst-eval/src/call.rs` do vanilla.
    let child_route = Route::extend(engine.route);

    // Engine local com route e styles novos (ADR-0044, Passo 109).
    // `local_styles` substitui o `styles` do engine do caller — `#set`
    // dentro da closure muta `local_styles` e não o styles do caller.
    let mut local_styles = engine.styles.clone();
    // `sink` é `TrackedMut<'caller, Sink>`; precisa de reborrow para
    // shortar o lifetime ao do `local_engine`.
    let mut local_sink = TrackedMut::reborrow_mut(&mut *engine.sink);
    if let Some(body_expr) = Expr::from_untyped(&closure.body) {
        let mut local_engine = Engine {
            world: engine.world,
            route: child_route.track(),
            styles: &mut local_styles,
            show_rules: &mut *engine.show_rules,
            active_guards: &mut *engine.active_guards,
            current_file: engine.current_file,
            figure_numbering: &mut *engine.figure_numbering,
            sink: &mut local_sink,
        };
        eval_expr(body_expr, &mut call_scopes, ctx, &mut local_engine)
    } else {
        Ok(Value::None)
    }
}

// ── Dispatcher arms: Closure / FuncCall (Passo 96.2, ADR-0037 Regra 4) ────

pub(super) fn eval_closure_expr(
    closure_expr: ClosureNode<'_>,
    scopes: &mut Scopes<'_>,
    ctx: &mut EvalContext,
    engine: &mut Engine<'_>,
) -> SourceResult<Value> {
    // Captura eager por snapshot — O(N) uma única vez, depois partilhado em O(1).
    // Semântica: snapshot do scope no momento da definição (Opção B — DEBT-2).
    // A closure vê o estado do scope no momento da captura, não da chamada.
    // Integração com comemo para lazy semantics completas: trabalho futuro.
    let captured = std::sync::Arc::new(scopes.snapshot());

    // Nome da closure — preenchido para sintaxe #let fib(n) = ...
    // Para closures anónimas (n) => ..., name é None (preenchido por eval_let).
    let name = closure_expr.name().map(|n| n.as_str().to_string());

    // Extrair parâmetros — Param::Pos(Pattern::Normal(Ident)) e Param::Named
    let params: SourceResult<Vec<ClosureParam>> = closure_expr.params()
        .children()
        .filter_map(|param| match param {
            Param::Pos(Pattern::Normal(Expr::Ident(ident))) => {
                Some(Ok(ClosureParam { name: ident.as_str().to_string(), default: None }))
            }
            Param::Named(named) => {
                let name = named.name().as_str().to_string();
                Some(eval_expr(named.expr(), scopes, ctx, engine)
                    .map(|v| ClosureParam { name, default: Some(v) }))
            }
            _ => None,  // Spread, Placeholder, Destructuring — adiado
        })
        .collect();
    let params = params?;

    // Body: SyntaxNode clone O(1) via Arc interno
    let body = closure_expr.body().to_untyped().clone();

    Ok(Value::Func(Func::closure(ClosureRepr { name, params, body, captured })))
}

pub(super) fn eval_func_call(
    call: FuncCallNode<'_>,
    scopes: &mut Scopes<'_>,
    ctx: &mut EvalContext,
    engine: &mut Engine<'_>,
) -> SourceResult<Value> {
    use super::{bindings, rules};

    // Intercepção de `counter(key).method(...)` antes de avaliar o callee.
    // Anatomia AST: FuncCall { callee: FieldAccess { target: FuncCall(counter, [key]), field: method } }
    if let Expr::FieldAccess(access) = call.callee() {
        if let Some(counter_key) = bindings::extract_counter_key(access.target()) {
            let method_name = access.field().as_str().to_string();
            return bindings::eval_counter_method(&counter_key, &method_name, call.args(), scopes, ctx, engine);
        }
    }

    // Intercepção de `outline()` — produz Content::Outline (Passo 61).
    if let Expr::Ident(ident) = call.callee() {
        if ident.as_str() == "outline" {
            return Ok(Value::Content(Content::Outline));
        }
    }

    let callee = eval_expr(call.callee(), scopes, ctx, engine)?;
    let args = eval_args(call.args(), scopes, ctx, engine)?;

    match callee {
        Value::Func(func) => {
            let result = apply_func(func, args, ctx, engine)?;
            // Intercepção eager — show rules aplicadas após apply_func (Passo 68).
            if let Value::Content(c) = result {
                Ok(Value::Content(rules::intercept_content(c, ctx, engine)?))
            } else {
                Ok(result)
            }
        },
        other => Err(vec![SourceDiagnostic::error(
            call.callee().span(),
            format!("não é possível chamar {}", other.type_name()),
        )]),
    }
}
