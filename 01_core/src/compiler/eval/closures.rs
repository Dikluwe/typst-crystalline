//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/eval/closures.md
//! @prompt-hash e8835c34
//! @layer L1
//! @updated 2026-08-12
//!
//! Criação e aplicação de closures. Extraído do monólito
//! `compiler/eval/closures.rs` no Passo 1012 conforme ADR-0109 (atomização —
//! forma B, free function no arquivo da unidade). O dispatch geral de chamadas
//! (`eval_func_call`, `apply_func`, avaliação de args, etc.) mudou-se para
//! `compiler/eval/call_dispatch.rs`.

use comemo::TrackedMut;

use crate::compiler::scopes::Scopes;
use crate::entities::args::Args;
use crate::entities::ast::expr::{Closure as ClosureNode, Expr, Param, Pattern};
use crate::entities::ast::AstNode;
use crate::entities::engine::Engine;
use crate::entities::func::{ClosureParam, ClosureRepr, Func};
use crate::entities::source_result::{SourceDiagnostic, SourceResult};
use crate::entities::span::Span;
use crate::entities::value::Value;
use crate::entities::world_types::{check_call_depth as route_check_call_depth, Route};

use super::{bindings::destructure_let, eval_expr, EvalContext, FlowEvent};

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
    mut args: Args,
    ctx: &mut EvalContext,
    engine: &mut Engine<'_>,
) -> SourceResult<Value> {
    // Limite de chamadas vanilla (MAX_CALL_DEPTH = 80) — paridade com
    // `typst-eval/src/call.rs:33` (ADR-0033). Pago parcial do DEBT-45
    // no Passo 93.
    route_check_call_depth(engine.route)?;

    // Criar scope filho do captured — O(1), sem clone dos valores capturados.
    // P772q — propaga por que motivo o scope foi capturado (Function/Context).
    let mut call_scopes =
        Scopes::with_parent(std::sync::Arc::clone(&closure.captured), closure.capturer);

    // Auto-injecção para recursão — definida antes dos params para que um
    // parâmetro com o mesmo nome sombre a função (comportamento do original).
    if let Some(ref name) = closure.name {
        call_scopes.define(name.clone(), Value::Func(func.clone()));
    }

    // Bind parâmetros: named args têm prioridade sobre posicionais.
    // **P708** — `param.default.is_some()` significa que o parâmetro foi
    // declarado como `nome: default` (`Param::Named` no parser) — é
    // keyword-only no vanilla, nunca preenchível por posição (medido:
    // `#let f(close: false) = close; f(true)` → `error: unexpected
    // argument`, não `close = true`). Só parâmetros posicionais
    // (`default.is_none()`, de `Param::Pos`) consomem `args.items`.
    // Invariante documentada em `entities/func.md` §"Invariante P708".
    let mut pos_idx = 0;
    for param in closure.params.iter() {
        // **P733** — o nomeado é consumido (`shift_remove`), não só lido:
        // paridade vanilla `args.named()` em `typst-eval/src/call.rs:679-683`.
        // O que sobrar no mapa após o loop é não consumido — vai para o
        // sink (se houver) ou gera erro "unexpected argument: {name}".
        let val = if let Some(v) = args.named.shift_remove(param.name.as_str()) {
            v
        } else if param.default.is_none() {
            match args.items.get(pos_idx) {
                Some(v) => {
                    pos_idx += 1;
                    v.clone()
                }
                None => {
                    let name = if param.name.is_empty() {
                        "pattern"
                    } else {
                        param.name.as_str()
                    };
                    return Err(vec![SourceDiagnostic::error(
                        args.span,
                        format!("missing argument: {name}"),
                    )]);
                }
            }
        } else {
            param.default.clone().unwrap()
        };
        // **P724** — pattern de desestruturação: bind via `destructure_let`
        // (mesma entrada do `#let` e do `for`, mirror do vanilla
        // `call.rs:659-665`); `Ident` liga directo como antes.
        match &param.pattern {
            Some(node) => {
                let Some(pattern) = Pattern::from_untyped(node) else {
                    return Err(vec![SourceDiagnostic::error(
                        Span::detached(),
                        "padrão de parâmetro inválido".to_string(),
                    )]);
                };
                destructure_let(pattern, val, &mut call_scopes, ctx, engine)?;
            }
            None => call_scopes.define(param.name.as_str(), val),
        }
    }

    // P504 — sink de argumentos: empacota os restantes num `Value::Args`.
    if let Some(ref sink_name) = closure.sink_name {
        let remaining_items = args.items.into_iter().skip(pos_idx).collect();
        call_scopes.define(
            sink_name.as_str(),
            Value::Args(crate::entities::args::Args {
                items: remaining_items,
                // **P733** — só os nomeados NÃO consumidos por parâmetros
                // (paridade vanilla `args.take()`, medido: sink exclui o
                // nomeado já ligado a um parâmetro).
                named: args.named,
                // P772s — span da chamada original (sink `..rest` reflecte
                // os args não consumidos desta mesma chamada).
                span: args.span,
            }),
        );
    } else {
        if pos_idx < args.items.len() {
            // **P708** — sem sink para absorver o excedente, um argumento
            // posicional sem parâmetro correspondente é erro (paridade
            // vanilla verbatim: `"unexpected argument"`), não descarte
            // silencioso. Verificado antes dos nomeados (ordem medida:
            // `f(1, 2, z: 3)` → "unexpected argument").
            return Err(vec![SourceDiagnostic::error(
                Span::detached(),
                "unexpected argument".to_string(),
            )]);
        }
        if let Some(k) = args.named.keys().next() {
            // **P733** — argumento nomeado sem parâmetro correspondente:
            // erro com o nome (paridade vanilla `Args::finish`,
            // `foundations/args.rs:259-268`). O primeiro na ordem de
            // inserção (IndexMap), como o vanilla reporta o primeiro não
            // consumido.
            return Err(vec![SourceDiagnostic::error(
                Span::detached(),
                format!("unexpected argument: {k}"),
            )]);
        }
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
    let output = if let Some(body_expr) = Expr::from_untyped(&closure.body) {
        let mut local_engine = Engine {
            world: engine.world,
            font_metrics: engine.font_metrics,
            route: child_route.track(),
            styles: &mut local_styles,
            show_rules: &mut *engine.show_rules,
            active_guards: &mut *engine.active_guards,
            current_file: engine.current_file,
            sink: &mut local_sink,
        };
        eval_expr(body_expr, &mut call_scopes, ctx, &mut local_engine)?
    } else {
        Value::None
    };

    // P635 — consumir FlowEvent ao sair da closure.
    match ctx.flow.take() {
        Some(FlowEvent::Return(_, Some(explicit), _)) => Ok(explicit),
        Some(FlowEvent::Return(_, None, _)) => Ok(output),
        Some(flow) => Err(vec![flow.forbidden()]),
        None => Ok(output),
    }
}

/// Avalia uma expressão de closure: captura o scope e constrói um `Func`.
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

    // Extrair parâmetros — Param::Pos(Pattern::Normal(Ident)), Param::Pos
    // com pattern de desestruturação/placeholder (**P724** — guarda o
    // pattern completo como SyntaxNode owned; bind via `destructure_let`
    // na chamada, mirror do vanilla `call.rs:655-665`), Param::Named,
    // e sink spread `..args` (P504).
    let mut sink_name: Option<String> = None;
    let params: SourceResult<Vec<ClosureParam>> = closure_expr
        .params()
        .children()
        .filter_map(|param| match param {
            Param::Pos(Pattern::Normal(Expr::Ident(ident))) => Some(Ok(ClosureParam {
                name: ident.as_str().to_string(),
                default: None,
                pattern: None,
            })),
            Param::Pos(pattern) => Some(Ok(ClosureParam {
                name: String::new(),
                default: None,
                pattern: Some(pattern.to_untyped().clone()),
            })),
            Param::Named(named) => {
                let name = named.name().as_str().to_string();
                Some(eval_expr(named.expr(), scopes, ctx, engine).map(|v| ClosureParam {
                    name,
                    default: Some(v),
                    pattern: None,
                }))
            }
            Param::Spread(spread) => {
                if let Some(ident) = spread.sink_ident() {
                    sink_name = Some(ident.as_str().to_string());
                }
                None
            }
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
        // P772q — closure normal (não `context { }`, que constrói o seu
        // próprio ClosureRepr directamente em eval/mod.rs).
        capturer: crate::entities::scope::Capturer::Function,
    })))
}
