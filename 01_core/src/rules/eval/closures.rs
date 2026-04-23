//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/rules/eval.md
//! @prompt-hash 19073424
//! @layer L1
//! @updated 2026-04-22
//!
//! Closures, chamadas de função e avaliação de argumentos. Extraído de
//! `eval.rs` no Passo 96.1 conforme ADR-0037 (coesão por domínio).

use std::sync::Arc;

use comemo::Tracked;
use ecow::EcoString;
use indexmap::IndexMap;
use rustc_hash::FxBuildHasher;

use crate::entities::args::Args;
use crate::entities::file_id::FileId;
use crate::entities::ast::expr::{Arg, Closure as ClosureNode, Expr, FuncCall as FuncCallNode, Param, Pattern};
use crate::entities::ast::AstNode;
use crate::entities::content::Content;
use crate::entities::func::{ClosureParam, ClosureRepr, Func, FuncRepr};
use crate::entities::source_result::SourceDiagnostic;
use crate::entities::show::{RuleId, ShowRule};
use crate::entities::source_result::SourceResult;
use crate::entities::style_chain::StyleChain;
use crate::entities::value::Value;
use crate::entities::world_types::{check_call_depth as route_check_call_depth, Route};
use crate::rules::scopes::Scopes;

use super::{eval_expr, EvalContext};

/// Avalia a lista de argumentos de uma chamada de função.
///
/// Posicionais são avaliados em ordem; named args são avaliados e indexados
/// por nome. Spread ignorado (fronteira deliberada, adiado).
pub(super) fn eval_args<'r>(
    args_node: crate::entities::ast::expr::Args<'_>,
    scopes: &mut Scopes<'_>,
    ctx: &mut EvalContext<'_>,
    route: Tracked<'r, Route<'r>>,
    styles: &mut StyleChain,
    show_rules: &mut Arc<[ShowRule]>,
    active_guards: &mut Vec<RuleId>,
current_file: FileId,
figure_numbering: &mut Option<String>,
) -> SourceResult<Args> {
    let mut items = Vec::new();
    let mut named: IndexMap<EcoString, Value, FxBuildHasher> = IndexMap::default();
    for arg in args_node.items() {
        match arg {
            Arg::Pos(expr) => items.push(eval_expr(expr, scopes, ctx, route, styles, show_rules, active_guards, current_file, figure_numbering)?),
            Arg::Named(name_expr) => {
                named.insert(
                    name_expr.name().as_str().into(),
                    eval_expr(name_expr.expr(), scopes, ctx, route, styles, show_rules, active_guards, current_file, figure_numbering)?,
                );
            }
            Arg::Spread(_) => {}  // fronteira deliberada
        }
    }
    Ok(Args { items, named })
}

/// Aplica uma função (closure ou native) aos args dados.
pub(crate) fn apply_func<'r>(
    func: Func,
    args: Args,
    ctx: &mut EvalContext<'_>,
    route: Tracked<'r, Route<'r>>,
    styles: &mut StyleChain,
    show_rules: &mut Arc<[ShowRule]>,
    active_guards: &mut Vec<RuleId>,
current_file: FileId,
figure_numbering: &mut Option<String>,
) -> SourceResult<Value> {
    match func.repr() {
        FuncRepr::Closure(closure) => apply_closure(closure, &func, args, ctx, route, styles, show_rules, active_guards, current_file, figure_numbering),
        FuncRepr::Native(native)   => (native.call)(ctx, &args, current_file, figure_numbering.as_deref()),
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
pub(super) fn apply_closure<'r>(
    closure: &ClosureRepr,
    func: &Func,
    args: Args,
    ctx: &mut EvalContext<'_>,
    route: Tracked<'r, Route<'r>>,
    styles: &mut StyleChain,
    show_rules: &mut Arc<[ShowRule]>,
    active_guards: &mut Vec<RuleId>,
current_file: FileId,
figure_numbering: &mut Option<String>,
) -> SourceResult<Value> {
    // Limite de chamadas vanilla (MAX_CALL_DEPTH = 80) — paridade com
    // `typst-eval/src/call.rs:33` (ADR-0033). Pago parcial do DEBT-45
    // no Passo 93. Substitui `EvalContext::check_call_depth` antigo
    // (Opção 2 do plano).
    route_check_call_depth(route)?;

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
    // Paridade com `typst-eval/src/call.rs` do vanilla, que reconstrói o
    // Engine com `route: Route::extend(route)` para cada chamada.
    let child_route = Route::extend(route);

    // Avaliar o body com o scope da chamada.
    // Atomização Passo 94: styles local — `#set` dentro da closure muta
    // `local_styles` e não a `styles` do caller (substitui o antigo
    // par save/restore sobre um campo partilhado do contexto).
    let mut local_styles = styles.clone();
    if let Some(body_expr) = Expr::from_untyped(&closure.body) {
        eval_expr(body_expr, &mut call_scopes, ctx, child_route.track(), &mut local_styles, show_rules, active_guards, current_file, figure_numbering)
    } else {
        Ok(Value::None)
    }
}

// ── Dispatcher arms: Closure / FuncCall (Passo 96.2, ADR-0037 Regra 4) ────

pub(super) fn eval_closure_expr<'r>(
    closure_expr: ClosureNode<'_>,
    scopes: &mut Scopes<'_>,
    ctx: &mut EvalContext<'_>,
    route: Tracked<'r, Route<'r>>,
    styles: &mut StyleChain,
    show_rules: &mut Arc<[crate::entities::show::ShowRule]>,
    active_guards: &mut Vec<crate::entities::show::RuleId>,
    current_file: FileId,
    figure_numbering: &mut Option<String>,
) -> crate::entities::source_result::SourceResult<Value> {
    // Captura eager por snapshot — O(N) uma única vez, depois partilhado em O(1).
    // Semântica: snapshot do scope no momento da definição (Opção B — DEBT-2).
    // A closure vê o estado do scope no momento da captura, não da chamada.
    // Integração com comemo para lazy semantics completas: trabalho futuro.
    let captured = std::sync::Arc::new(scopes.snapshot());

    // Nome da closure — preenchido para sintaxe #let fib(n) = ...
    // Para closures anónimas (n) => ..., name é None (preenchido por eval_let).
    let name = closure_expr.name().map(|n| n.as_str().to_string());

    // Extrair parâmetros — Param::Pos(Pattern::Normal(Ident)) e Param::Named
    let params: crate::entities::source_result::SourceResult<Vec<ClosureParam>> = closure_expr.params()
        .children()
        .filter_map(|param| match param {
            Param::Pos(Pattern::Normal(Expr::Ident(ident))) => {
                Some(Ok(ClosureParam { name: ident.as_str().to_string(), default: None }))
            }
            Param::Named(named) => {
                let name = named.name().as_str().to_string();
                Some(eval_expr(named.expr(), scopes, ctx, route, styles, show_rules, active_guards, current_file, figure_numbering)
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

pub(super) fn eval_func_call<'r>(
    call: FuncCallNode<'_>,
    scopes: &mut Scopes<'_>,
    ctx: &mut EvalContext<'_>,
    route: Tracked<'r, Route<'r>>,
    styles: &mut StyleChain,
    show_rules: &mut Arc<[crate::entities::show::ShowRule]>,
    active_guards: &mut Vec<crate::entities::show::RuleId>,
    current_file: FileId,
    figure_numbering: &mut Option<String>,
) -> crate::entities::source_result::SourceResult<Value> {
    use super::{bindings, rules};

    // Intercepção de `counter(key).method(...)` antes de avaliar o callee.
    // Anatomia AST: FuncCall { callee: FieldAccess { target: FuncCall(counter, [key]), field: method } }
    if let Expr::FieldAccess(access) = call.callee() {
        if let Some(counter_key) = bindings::extract_counter_key(access.target()) {
            let method_name = access.field().as_str().to_string();
            return bindings::eval_counter_method(&counter_key, &method_name, call.args(), scopes, ctx, route, styles, show_rules, active_guards, current_file, figure_numbering);
        }
    }

    // Intercepção de `outline()` — produz Content::Outline (Passo 61).
    if let Expr::Ident(ident) = call.callee() {
        if ident.as_str() == "outline" {
            return Ok(Value::Content(Content::Outline));
        }
    }

    let callee = eval_expr(call.callee(), scopes, ctx, route, styles, show_rules, active_guards, current_file, figure_numbering)?;
    let args = eval_args(call.args(), scopes, ctx, route, styles, show_rules, active_guards, current_file, figure_numbering)?;

    match callee {
        Value::Func(func) => {
            let result = apply_func(func, args, ctx, route, styles, show_rules, active_guards, current_file, figure_numbering)?;
            // Intercepção eager — show rules aplicadas após apply_func (Passo 68).
            if let Value::Content(c) = result {
                Ok(Value::Content(rules::intercept_content(c, ctx, route, styles, show_rules, active_guards, current_file, figure_numbering)?))
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
