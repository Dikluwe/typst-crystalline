//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/rules/eval.md
//! @prompt-hash a9cc504d
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
use crate::entities::ast::expr::{
    Arg, Closure as ClosureNode, Expr, FuncCall as FuncCallNode, Param, Pattern,
};
use crate::entities::ast::AstNode;
use crate::entities::engine::Engine;
use crate::entities::func::{ClosureParam, ClosureRepr, Func, FuncRepr};
use crate::entities::source_result::SourceDiagnostic;
use crate::entities::source_result::SourceResult;
use crate::entities::value::Value;
use comemo::TrackedMut;

use crate::entities::world_types::{check_call_depth as route_check_call_depth, Route};
use crate::rules::scopes::Scopes;
use crate::rules::stdlib::try_dispatch_collection_method;

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
            Arg::Spread(_) => {} // fronteira deliberada
        }
    }
    Ok(Args { items, named })
}

/// Aplica uma função (closure, native ou native-with-engine) aos args dados.
pub fn apply_func(
    func: Func,
    args: Args,
    scopes: &mut Scopes<'_>,
    ctx: &mut EvalContext,
    engine: &mut Engine<'_>,
) -> SourceResult<Value> {
    match func.repr() {
        FuncRepr::Closure(closure) => apply_closure(closure, &func, args, ctx, engine),
        // Lote F-3 inc-2: elemento de utilizador (fronteira E1) — `#name(args)`
        // invoca o construtor do registry e devolve `Content::Dynamic`. Mesmo
        // ponto de despacho dos nativos (sem caminho paralelo). Erro do catálogo
        // existente se o ctor falhar (não panic).
        FuncRepr::Element(ef) => Ok(Value::Content((ef.ctor)(&args.items)?)),
        FuncRepr::Native(native) => {
            let world = engine.world;
            let current_file = engine.current_file;
            // F-5a de-bake (P365, `f_fronteira_e1.md` §3a.9): o dispatch não lê mais
            // `custom("figure.numbering")` para alimentar a native — o padrão vive
            // só na chain e é lido pelo **consumidor** (layout/introspect). O
            // parâmetro `figure_numbering` foi colapsado do ABI (`func.rs`). Fonte
            // única.
            (native.call)(ctx, &args, world, current_file)
        }
        // P394: native function com acesso ao scope/engine actuais (eval).
        FuncRepr::NativeWithEngine(native) => {
            let world = engine.world;
            let current_file = engine.current_file;
            (native.call)(ctx, &args, world, current_file, scopes, engine)
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

    // P504 — sink de argumentos: empacota os restantes num `Value::Args`.
    if let Some(ref sink_name) = closure.sink_name {
        let remaining_items = args.items.into_iter().skip(pos_idx).collect();
        call_scopes.define(
            sink_name.as_str(),
            Value::Args(crate::entities::args::Args {
                items: remaining_items,
                named: args.named,
            }),
        );
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

    // Extrair parâmetros — Param::Pos(Pattern::Normal(Ident)), Param::Named,
    // e sink spread `..args` (P504).
    let mut sink_name: Option<String> = None;
    let params: SourceResult<Vec<ClosureParam>> = closure_expr
        .params()
        .children()
        .filter_map(|param| match param {
            Param::Pos(Pattern::Normal(Expr::Ident(ident))) => {
                Some(Ok(ClosureParam { name: ident.as_str().to_string(), default: None }))
            }
            Param::Named(named) => {
                let name = named.name().as_str().to_string();
                Some(
                    eval_expr(named.expr(), scopes, ctx, engine)
                        .map(|v| ClosureParam { name, default: Some(v) }),
                )
            }
            Param::Spread(spread) => {
                if let Some(ident) = spread.sink_ident() {
                    sink_name = Some(ident.as_str().to_string());
                }
                None
            }
            _ => None, // Placeholder, Destructuring — adiado
        })
        .collect();
    let params = params?;

    // Body: SyntaxNode clone O(1) via Arc interno
    let body = closure_expr.body().to_untyped().clone();

    Ok(Value::Func(Func::closure(ClosureRepr {
        name,
        params,
        sink_name,
        body,
        captured,
    })))
}

pub(super) fn eval_func_call(
    call: FuncCallNode<'_>,
    scopes: &mut Scopes<'_>,
    ctx: &mut EvalContext,
    engine: &mut Engine<'_>,
) -> SourceResult<Value> {
    use super::{bindings, rules};

    // **P417 (M)** — Intercepção de `heading.where(field: value)` (method call
    // syntax) antes de avaliar o callee genérico. O target deve avaliar para
    // uma `Value::Func` nativa de elemento (heading, figure, strong, emph, raw).
    // O resultado é `Value::Selector(Selector::Where { base: Kind(...), ... })`.
    if let Expr::FieldAccess(access) = call.callee() {
        if access.field().as_str() == "where" {
            if let Some(selector) = bindings::eval_element_where(
                access.target(),
                call.args(),
                scopes,
                ctx,
                engine,
            )? {
                return Ok(Value::Selector(selector));
            }
        }
    }

    // **P423 (S-M)** — Intercepção de `selector.or(other)` e
    // `selector.and(other)` antes de avaliar o callee genérico. O target e o
    // argumento devem avaliar para `Value::Selector`.
    if let Expr::FieldAccess(access) = call.callee() {
        let method = access.field().as_str();
        if method == "or" || method == "and" {
            if let Some(selector) = bindings::eval_selector_or_and(
                access.target(),
                method,
                call.args(),
                scopes,
                ctx,
                engine,
            )? {
                return Ok(Value::Selector(selector));
            }
        }
    }

    // **P504** — Intercepção de `selector.within(ancestor)`.
    if let Expr::FieldAccess(access) = call.callee() {
        if access.field().as_str() == "within" {
            if let Some(selector) = bindings::eval_selector_within(
                access.target(),
                call.args(),
                scopes,
                ctx,
                engine,
            )? {
                return Ok(Value::Selector(selector));
            }
        }
    }

    // **P466** — Métodos de instância para `array`, `dict` e `str`.
    if let Expr::FieldAccess(access) = call.callee() {
        let target = eval_expr(access.target(), scopes, ctx, engine)?;
        let method = access.field().as_str();
        let args = eval_args(call.args(), scopes, ctx, engine)?;
        if let Some(result) =
            try_dispatch_collection_method(target, method, args, scopes, ctx, engine)
        {
            return result;
        }
    }

    // **P506** — Métodos de instância para `state` e `counter`.
    if let Expr::FieldAccess(access) = call.callee() {
        let target = eval_expr(access.target(), scopes, ctx, engine)?;
        let method = access.field().as_str();
        match target {
            Value::State(ref state) => {
                return super::bindings::eval_state_method(
                    state,
                    method,
                    call.args(),
                    scopes,
                    ctx,
                    engine,
                )
            }
            Value::Counter(ref counter) => {
                return super::bindings::eval_counter_method_value(
                    counter,
                    method,
                    call.args(),
                    scopes,
                    ctx,
                    engine,
                )
            }
            _ => {}
        }
    }

    let callee = eval_expr(call.callee(), scopes, ctx, engine)?;
    let args = eval_args(call.args(), scopes, ctx, engine)?;

    match callee {
        Value::Func(func) => {
            let result = apply_func(func, args, scopes, ctx, engine)?;
            // Intercepção eager — show rules aplicadas após apply_func (Passo 68).
            if let Value::Content(c) = result {
                Ok(Value::Content(rules::intercept_content(c, ctx, engine)?))
            } else {
                Ok(result)
            }
        }
        other => Err(vec![SourceDiagnostic::error(
            call.callee().span(),
            format!("não é possível chamar {}", other.type_name()),
        )]),
    }
}
