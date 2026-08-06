//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/engine/eval.md
//! @prompt-hash 9d26732a
//! @layer L1
//! @updated 2026-07-09
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
use crate::entities::bytes::Bytes;
use crate::entities::engine::Engine;
use crate::entities::func::{ClosureParam, ClosureRepr, Func, FuncRepr};
use crate::entities::plugin_func::PluginFunc;
use crate::entities::source_result::SourceDiagnostic;
use crate::entities::source_result::SourceResult;
use crate::entities::source_result::Tracepoint;
use crate::entities::span::{Span, Spanned};
use crate::entities::value::{Type, Value};
use comemo::TrackedMut;

use crate::engine::scopes::Scopes;
use crate::engine::stdlib::{
    extract_measure_body,
    // P737 — counter/state chamáveis via despacho de tipos.
    native_counter,
    native_bytes,
    native_datetime,
    native_float,
    native_int,
    native_layout,
    native_measure,
    native_state,
    native_str,
    native_symbol,
    native_type,
    try_dispatch_collection_method,
};
use crate::entities::world_types::{check_call_depth as route_check_call_depth, Route};

use super::{bindings::destructure_let, eval_expr, EvalContext, FlowEvent};

/// Avalia a lista de argumentos de uma chamada de função.
///
/// Posicionais são avaliados em ordem; named args são avaliados e indexados
/// por nome. **P718** — `..spread` mirror de `ast::Args::eval` (vanilla
/// `call.rs:367-416`): `none` ignorado; `array` vira posicionais; `dict`
/// vira nomeados; `Value::Args` (reencaminhamento de um sink `..rest` de
/// outra closure) funde posicionais e nomeados; outro tipo erra `cannot
/// spread {ty}` — **sem** o sufixo "into X" das mensagens de literal
/// (`Expr::Array`/`Expr::Dict`, `mod.rs`), mensagem distinta no vanilla.
pub(super) fn eval_args(
    args_node: crate::entities::ast::expr::Args<'_>,
    scopes: &mut Scopes<'_>,
    ctx: &mut EvalContext,
    engine: &mut Engine<'_>,
) -> SourceResult<Args> {
    // P772s — span da lista de argumentos da chamada real no documento,
    // propagado para todas as mensagens de erro de validação de argumento
    // das funções nativas que usam `args.span` em vez de `Span::detached()`.
    let call_span = args_node.span();
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
            Arg::Spread(spread) => {
                let value = eval_expr(spread.expr(), scopes, ctx, engine)?;
                match value {
                    Value::None => {}
                    Value::Array(arr) => items.extend(arr),
                    Value::Dict(dict) => named.extend(dict),
                    Value::Args(args) => {
                        items.extend(args.items);
                        named.extend(args.named);
                    }
                    other => {
                        return Err(vec![SourceDiagnostic::error(
                            spread.span(),
                            format!(
                                "cannot spread {}",
                                super::bindings::long_type_name(&other)
                            ),
                        )]);
                    }
                }
            }
        }
    }
    Ok(Args { items, named, span: call_span })
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
        // P699 — export de plugin WASM: delega ao host capturado (memoizado
        // em `PluginFunc::call`). Valida args (só bytes) e propaga erros
        // verbatim (`PluginError.message` é observável — ADR-0107).
        FuncRepr::Plugin(p) => call_plugin(p, &args),
        // P702 — aplicação parcial (`f.with(...)`): funde os args pré-ligados
        // com os da chamada final e delega recursivamente — o encadeamento
        // (`f.with(a).with(b)`) resolve-se sozinho, sem lógica extra.
        FuncRepr::With(w) => {
            let (inner, pre) = w.as_ref();
            apply_func(inner.clone(), merge_with_args(pre, args), scopes, ctx, engine)
        }
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

/// **P846 (#57)** — envolve o resultado de uma chamada com um
/// `Tracepoint::Call`, mirror de `call_func` + `Trace::trace` do vanilla
/// (`typst-eval/src/call.rs:166-180`, `typst-library/src/diag.rs:464-479`).
///
/// Regras verbatim do vanilla:
/// - Se o span da chamada não resolve para um byte range (detached ou
///   ficheiro inacessível), os erros propagam inalterados.
/// - Um erro **contido** no span da chamada (mesma fonte, range da chamada ⊇
///   range do erro) não ganha tracepoint — é o caso dos erros de validação
///   de argumentos de nativas e dos erros de `eval` (âncora = literal dentro
///   da chamada). Erros no **corpo** de closures ficam fora do span da
///   chamada → ganham um nível por chamada, innermost primeiro.
/// - `func.name()` → `while calling \`name\``; `None` (closure anónima) →
///   `while calling function` (Display espelhado no renderer L2).
fn trace_call(
    result: SourceResult<Value>,
    func: &Func,
    call_span: Span,
    engine: &Engine<'_>,
) -> SourceResult<Value> {
    let mut errors = match result {
        Ok(value) => return Ok(value),
        Err(errors) => errors,
    };
    // vanilla: `let Some(trace_range) = world.range(span) else { return errors }`.
    let trace_range = call_span
        .id()
        .and_then(|id| engine.world.source(id).ok())
        .and_then(|src| src.span_byte_range(call_span));
    let Some(trace_range) = trace_range else {
        return Err(errors);
    };
    for error in &mut errors {
        // "Skip traces that surround the error" (vanilla `diag.rs:471-478`):
        // mesmo ficheiro e chamada contém o erro → não acrescenta nível.
        if error.span.id() == call_span.id() {
            let contained = call_span
                .id()
                .and_then(|id| engine.world.source(id).ok())
                .and_then(|src| src.span_byte_range(error.span))
                .is_some_and(|error_range| {
                    trace_range.start <= error_range.start
                        && trace_range.end >= error_range.end
                });
            if contained {
                continue;
            }
        }
        error.trace.push(Spanned::new(
            Tracepoint::Call(func.name().map(String::from)),
            call_span,
        ));
    }
    Err(errors)
}

/// **P702** — funde os `Args` pré-ligados por `.with(...)` com os da chamada
/// final. Posicionais: pré-ligados primeiro (paridade vanilla,
/// `foundations/func.rs:360` do vanilla: `pre.items.chain(new.items)`).
/// Nomeados: `new` sobrepõe `pre` em colisão de chave — decisão por defeito,
/// não exercitada pelo vanilla (ver `entities/func.md` §"Variante `With`").
fn merge_with_args(pre: &Args, new: Args) -> Args {
    // P772s — span da chamada final (mais próxima do erro visto pelo
    // utilizador do que o span da chamada de `.with(...)` original).
    let span = new.span;
    let items = pre.items.iter().cloned().chain(new.items).collect();
    let mut named = pre.named.clone();
    named.extend(new.named);
    Args { items, named, span }
}

/// **P699** — Aplica um export de plugin WASM (`FuncRepr::Plugin`).
///
/// - Rejeita named args (a ABI WASM é posicional).
/// - Exige que todos os args posicionais sejam `Value::Bytes`; reúne-os num
///   `Vec<Bytes>` (clone de handle, não de conteúdo).
/// - Delega a `PluginFunc::call` (memoizada): `Ok(bytes) ⇒ Value::Bytes`,
///   `Err(e) ⇒ SourceDiagnostic` com `e.message` verbatim (observável).
/// - **P819** — mensagem verbatim do vanilla (`expected bytes, found <tipo>`,
///   nome longo) e span do callsite (`args.span`, P772s) em vez de detached.
fn call_plugin(p: &PluginFunc, args: &Args) -> SourceResult<Value> {
    if let Some(k) = args.named.keys().next() {
        return Err(vec![SourceDiagnostic::error(
            args.span,
            format!("argumento nomeado inesperado em {}(): '{k}'", p.name),
        )]);
    }

    let mut bufs: Vec<Bytes> = Vec::with_capacity(args.items.len());
    for v in &args.items {
        match v {
            Value::Bytes(b) => bufs.push(b.clone()),
            other => {
                return Err(vec![SourceDiagnostic::error(
                    args.span,
                    format!(
                        "expected bytes, found {}",
                        super::bindings::long_type_name(other),
                    ),
                )]);
            }
        }
    }

    match p.call(bufs) {
        Ok(bytes) => Ok(Value::Bytes(bytes)),
        Err(e) => {
            Err(vec![SourceDiagnostic::error(args.span, e.message.to_string())])
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
                None => Value::None,
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

    // **P717** — Métodos mutantes (`push`/`pop`/`insert`/`remove`): mirror
    // de `maybe_resolve_mutating` (vanilla `call.rs:189-212`), sobre o
    // `access()` de P716. Tem de correr **antes** do bloco P466, que avalia
    // o target como valor (clone) — mutação exige o local. `Ok(None)` =
    // fall-through para a cadeia normal (módulos/funcs com campos com estes
    // nomes continuam a resolver abaixo).
    if let Expr::FieldAccess(access) = call.callee() {
        if bindings::is_mutating_method(access.field().as_str()) {
            if let Some(result) = bindings::try_eval_mutating_method(
                access,
                call.args(),
                call.span(),
                scopes,
                ctx,
                engine,
            )? {
                return Ok(result);
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

    // **P702** — `f.with(...)`: aplicação parcial de argumentos, disponível
    // em qualquer `Value::Func` (nativa com/sem namespace, closure, elemento,
    // plugin, ou já parcialmente aplicada). Mesmo padrão de intercepção das
    // secções acima; se o alvo não for `Value::Func`, não intercepta — cai
    // no field access genérico, sem mudança de comportamento para
    // não-funções (ver `entities/func.md` §"Variante `With`").
    if let Expr::FieldAccess(access) = call.callee() {
        if access.field().as_str() == "with" {
            let target = eval_expr(access.target(), scopes, ctx, engine)?;
            if let Value::Func(f) = target {
                let args = eval_args(call.args(), scopes, ctx, engine)?;
                return Ok(Value::Func(f.with(args)));
            }
        }
    }

    // **P710** — `length.to-absolute()`: resolve a componente `em` usando o
    // tamanho de texto actual (`StyleChain::size()`, já usado pelo layout de
    // texto — nenhum mecanismo novo de resolução de estilo). Divergência
    // documentada (mecânica, não língua — `rules/eval.md` §P710): o vanilla
    // só permite isto dentro de um bloco `context`; o cristalino não
    // distingue "scripting" de "contexto resolvido" (`engine.styles` é
    // sempre acessível), logo não replica esse gate — o valor produzido é
    // idêntico ao vanilla no único caso medido (`cetz`, sempre dentro de
    // `context {...}`). `.pt()`/`.mm()`/`.cm()`/`.inches()`/`.abs`/`.em`
    // ficam scope-out, sem consumidor medido.
    if let Expr::FieldAccess(access) = call.callee() {
        if access.field().as_str() == "to-absolute" {
            let target = eval_expr(access.target(), scopes, ctx, engine)?;
            if let Value::Length(l) = target {
                use crate::entities::layout_types::{Abs, Length};
                let abs_pt = l.abs.to_pt() + l.em * engine.styles.size();
                return Ok(Value::Length(Length { abs: Abs(abs_pt), em: 0.0 }));
            }
        }
    }

    // **P712** — `measure(body)` / `std.measure(body)`: precisa de
    // `engine.styles` (o tamanho medido depende do `#set text(size:)`
    // activo, paridade com o vanilla `context.styles()`) e do gate
    // `ctx.in_context` (mesma convenção já usada por `counter.get()`/
    // `state.get()`, `stdlib/counter.rs:144`/`stdlib/state.rs:63`) —
    // nenhum dos dois acessível pela assinatura genérica
    // `NativeFn(ctx, args, world, file)`. Verifica primeiro a forma
    // sintáctica do callee (barato, sem side-effects) para não avaliar
    // nada quando não é sequer chamado "measure"; só depois avalia e
    // compara identidade de fn-ptr (`native_fn_addr`, não o nome — mesmo
    // padrão de `bindings::eval_element_where`, `bindings.rs:354`) para
    // não capturar um `measure` sombreado pelo utilizador.
    let measure_name_matches = match call.callee() {
        Expr::Ident(ident) => ident.as_str() == "measure",
        Expr::FieldAccess(access) => access.field().as_str() == "measure",
        _ => false,
    };
    if measure_name_matches {
        let target = eval_expr(call.callee(), scopes, ctx, engine)?;
        if let Value::Func(ref f) = target {
            if f.native_fn_addr().is_some_and(|addr| {
                std::ptr::fn_addr_eq(addr, native_measure as fn(_, _, _, _) -> _)
            }) {
                if !ctx.in_context {
                    return Err(vec![SourceDiagnostic::error(
                        call.callee().span(),
                        "measure() can only be used inside context".to_string(),
                    )]);
                }
                let args = eval_args(call.args(), scopes, ctx, engine)?;
                let body = extract_measure_body(&args)?;
                let (width_pt, height_pt) = crate::engine::layout::measure_content_real(
                    &body,
                    engine.styles,
                    engine.font_metrics,
                );
                let mut dict: IndexMap<EcoString, Value, FxBuildHasher> =
                    IndexMap::default();
                dict.insert(
                    "width".into(),
                    Value::Length(crate::entities::layout_types::Length::pt(width_pt)),
                );
                dict.insert(
                    "height".into(),
                    Value::Length(crate::entities::layout_types::Length::pt(height_pt)),
                );
                return Ok(Value::Dict(dict));
            }
        }
    }

    // **P707** — `args.pos()`/`args.named()`: métodos sobre `Value::Args`,
    // distintos dos campos `.positional`/`.named` (P504, sem parênteses).
    // Sem isto, `args.named()` avaliava o campo (Dict) e depois tentava
    // chamá-lo como função ("não é possível chamar dictionary");
    // `args.pos()` nem chegava a existir como campo. `.len()`/`.at()`/
    // `.filter()`/`.map()` do vanilla ficam scope-out — sem consumidor
    // medido em `cetz` (ver `rules/eval.md` §P707).
    if let Expr::FieldAccess(access) = call.callee() {
        let method = access.field().as_str();
        if method == "pos" || method == "named" {
            let target = eval_expr(access.target(), scopes, ctx, engine)?;
            if let Value::Args(a) = target {
                return Ok(match method {
                    "pos" => Value::Array(a.items),
                    _ => Value::Dict(a.named),
                });
            }
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
            // **P742** — Métodos de instância de `Value::Color` (9, padrão
            // P506). Só intercepta os métodos conhecidos; os restantes caem
            // no caminho genérico (erro de field access pré-P742).
            Value::Color(ref color) => {
                if crate::engine::stdlib::color::is_color_instance_method(method) {
                    return super::bindings::eval_color_method(
                        color,
                        method,
                        call.args(),
                        scopes,
                        ctx,
                        engine,
                    );
                }
            }
            // **P796** — `.at(index)` em `Value::Version` (único método de
            // instância; ver `entities/version.md` §8a). Só intercepta
            // "at"; outros métodos caem no caminho genérico existente.
            Value::Version(ref v) => {
                if method == "at" {
                    return super::bindings::eval_version_method_value(
                        v,
                        method,
                        call.args(),
                        scopes,
                        ctx,
                        engine,
                    );
                }
            }
            _ => {}
        }
    }

    // **P792** — `layout(func)`: paridade vanilla `layout/layout.rs:66`.
    // Chama a callback com as dimensões disponíveis (single-pass graded:
    // `available_width`/`available_height` calculadas a partir de
    // `engine.styles`). Mesmo padrão de intercepção que `measure`/P712:
    // verifica nome sintáctico → avalia o callee → compara fn-ptr
    // `native_layout` para não capturar um `layout` sombreado.
    let layout_name_matches = match call.callee() {
        Expr::Ident(ident) => ident.as_str() == "layout",
        Expr::FieldAccess(access) => access.field().as_str() == "layout",
        _ => false,
    };
    if layout_name_matches {
        let target = eval_expr(call.callee(), scopes, ctx, engine)?;
        if let Value::Func(ref f) = target {
            if f.native_fn_addr().is_some_and(|addr| {
                std::ptr::fn_addr_eq(addr, native_layout as fn(_, _, _, _) -> _)
            }) {
                let args = eval_args(call.args(), scopes, ctx, engine)?;
                // Extrair a callback — único argumento posicional obrigatório.
                let func = match args.items.as_slice() {
                    [Value::Func(f)] => f.clone(),
                    [other] => {
                        return Err(vec![SourceDiagnostic::error(
                            call.callee().span(),
                            format!(
                                "layout() requer uma função, recebeu {}",
                                other.type_name()
                            ),
                        )])
                    }
                    _ => {
                        return Err(vec![SourceDiagnostic::error(
                            call.callee().span(),
                            "layout() requer exatamente 1 argumento".to_string(),
                        )])
                    }
                };
                // Dimensões do container (single-pass graded — P792 scope-out two-pass).
                // Lidas dinamicamente da StyleChain caso configuradas por um `#set page`
                // anterior no escopo; caso contrário usa defaults da página A4.
                let page_width_pt = match engine.styles.custom("page.width") {
                    Some(Value::Float(f)) => *f,
                    Some(Value::Int(i)) => *i as f64,
                    _ => 595.28f64,
                };
                let page_height_pt = match engine.styles.custom("page.height") {
                    Some(Value::Float(f)) => *f,
                    Some(Value::Int(i)) => *i as f64,
                    _ => 841.89f64,
                };
                let margin_left = match engine.styles.custom("page.margin-left") {
                    Some(Value::Float(f)) => *f,
                    Some(Value::Int(i)) => *i as f64,
                    _ => 56.69f64,
                };
                let margin_right = match engine.styles.custom("page.margin-right") {
                    Some(Value::Float(f)) => *f,
                    Some(Value::Int(i)) => *i as f64,
                    _ => 56.69f64,
                };
                let margin_top = match engine.styles.custom("page.margin-top") {
                    Some(Value::Float(f)) => *f,
                    Some(Value::Int(i)) => *i as f64,
                    _ => 56.69f64,
                };
                let margin_bottom = match engine.styles.custom("page.margin-bottom") {
                    Some(Value::Float(f)) => *f,
                    Some(Value::Int(i)) => *i as f64,
                    _ => 56.69f64,
                };
                let avail_w = f64::max(0.0, page_width_pt - margin_left - margin_right);
                let avail_h = f64::max(0.0, page_height_pt - margin_top - margin_bottom);
                let mut size_dict: IndexMap<EcoString, Value, FxBuildHasher> =
                    IndexMap::default();
                size_dict.insert(
                    "width".into(),
                    Value::Length(crate::entities::layout_types::Length::pt(avail_w)),
                );
                size_dict.insert(
                    "height".into(),
                    Value::Length(crate::entities::layout_types::Length::pt(avail_h)),
                );
                let size_arg = crate::entities::args::Args {
                    items: vec![Value::Dict(size_dict)],
                    named: indexmap::IndexMap::default(),
                    span: call.span(),
                };
                let result = apply_func(func, size_arg, scopes, ctx, engine)?;
                return if let Value::Content(c) = result {
                    Ok(Value::Content(rules::intercept_content(c, ctx, engine)?))
                } else {
                    Ok(result)
                };
            }
        }
    }

    // **P792** — Métodos de `Location`: `loc.page()`, `loc.position()`,
    // `loc.page-numbering()` — paridade vanilla `location.rs #[scope]`.
    if let Expr::FieldAccess(access) = call.callee() {
        let method = access.field().as_str();
        if matches!(method, "page" | "position" | "page-numbering") {
            let target = eval_expr(access.target(), scopes, ctx, engine)?;
            if let Value::Location(loc) = target {
                let _args = eval_args(call.args(), scopes, ctx, engine)?;
                return eval_location_method(loc, method, ctx);
            }
        }
    }

    // **P829-B** — Métodos de instância de `Value::Content`:
    // `func`/`has`/`at`/`fields`/`location` — paridade vanilla `Content`
    // `#[scope]` (`foundations/content/mod.rs:510-590`). Corre depois de
    // todos os despachos legítimos acima (nenhum intercepta estes nomes para
    // Content) e antes do fallback P815, que de outra forma reportaria
    // `element strong has no method `func`` (medido m14/b1).
    if let Expr::FieldAccess(access) = call.callee() {
        let method = access.field().as_str();
        if matches!(method, "func" | "has" | "at" | "fields" | "location") {
            let target = eval_expr(access.target(), scopes, ctx, engine)?;
            if let Value::Content(c) = target {
                let args = eval_args(call.args(), scopes, ctx, engine)?;
                return bindings::eval_content_method(&c, method, args, call.span());
            }
        }
    }

    // **P815** — `eval_field_callee` do vanilla (`call.rs:239-345`): depois
    // de todos os despachos de método legítimos acima, um callee
    // `target.field` chamado como função cujo alvo não é
    // Symbol/Func/Type/Module produz os erros verbatim do vanilla —
    // método inexistente (`type integer has no method `foo``), dict-key-call
    // com hints, "not a valid method". Sem isto, o caminho genérico avaliava
    // o field access e errava com mensagens divergentes (ou, pior, chamava
    // funções guardadas em dict keys — medido em P815).
    if let Expr::FieldAccess(access) = call.callee() {
        let target = eval_expr(access.target(), scopes, ctx, engine)?;
        if let Some(err) = bindings::field_callee_error(&target, access) {
            return Err(err);
        }
    }

    let callee = eval_expr(call.callee(), scopes, ctx, engine)?;

    // **P846 (#56)** — `eval`: ancorar os erros da re-avaliação ao span do
    // **literal string** (primeiro argumento posicional), paridade com o
    // `SpanMode::Uniform` do vanilla (`foundations/mod.rs:267,318` — medido:
    // `span7.typ` van `4:2` vs cris `3:5`). Solução pontual só para `eval`
    // (override de `args.span` no call site), sem o débito estrutural de
    // span-por-argumento em `Args` (P772s). O cast error (`#eval(5)` →
    // `1:6`) fica igualmente corrigido — o vanilla ancora erros de
    // validação de argumento no span do argumento.
    let eval_anchor = match &callee {
        Value::Func(f)
            if matches!(f.repr(), FuncRepr::NativeWithEngine(n) if n.name == "eval") =>
        {
            call.args().items().find_map(|arg| match arg {
                Arg::Pos(expr) => Some(expr.span()),
                _ => None,
            })
        }
        _ => None,
    };

    let mut args = eval_args(call.args(), scopes, ctx, engine)?;
    if let Some(anchor) = eval_anchor {
        args.span = anchor;
    }

    match callee {
        Value::Func(func) => {
            let result = apply_func(func.clone(), args, scopes, ctx, engine);
            // **P846 (#57)** — call trace: um `Tracepoint::Call` por chamada
            // cujo span não contém o erro (mirror de `call_func` +
            // `Trace::trace` do vanilla — `typst-eval/src/call.rs:166-180`,
            // `typst-library/src/diag.rs:464-479`).
            let result = trace_call(result, &func, call.span(), engine)?;
            // Intercepção eager — show rules aplicadas após apply_func (Passo 68).
            if let Value::Content(c) = result {
                Ok(Value::Content(rules::intercept_content(c, ctx, engine)?))
            } else {
                Ok(result)
            }
        }
        // P685 — nomes de tipo chamáveis: `int("5")`, `str(5)`, `float("3.5")`,
        // `type(1)`. Despacha para o construtor nativo; tipos não chamáveis
        // (`bool`, `length`, `array`, …) → erro "type X does not have a
        // constructor" (paridade vanilla).
        Value::Type(t) => {
            let world = engine.world;
            let current_file = engine.current_file;
            match t {
                Type::Int => native_int(ctx, &args, world, current_file),
                Type::Float => native_float(ctx, &args, world, current_file),
                Type::Str => native_str(ctx, &args, world, current_file),
                Type::Type => native_type(ctx, &args, world, current_file),
                // P737 — `counter`/`state` são tipos chamáveis (paridade
                // vanilla — medido: type(counter)/type(state) → type;
                // counter("x")/state("y", 0) criam instâncias).
                Type::Counter => native_counter(ctx, &args, world, current_file),
                Type::State => native_state(ctx, &args, world, current_file),
                // P765a — `symbol(...)` constructor.
                Type::Symbol => native_symbol(ctx, &args, world, current_file),
                // P843 (F4/F5) — constructors `bytes(...)` e `datetime(...)`.
                Type::Bytes => native_bytes(ctx, &args, world, current_file),
                Type::Datetime => native_datetime(ctx, &args, world, current_file),
                other => Err(vec![SourceDiagnostic::error(
                    call.callee().span(),
                    format!("type {} does not have a constructor", other.name()),
                )]),
            }
        }
        other => Err(vec![SourceDiagnostic::error(
            call.callee().span(),
            format!("não é possível chamar {}", other.type_name()),
        )]),
    }
}

/// **P792** — Despacho de métodos de instância de `Value::Location`.
///
/// Intercepção em `eval_func_call` para `loc.page()`, `loc.position()`,
/// `loc.page-numbering()`. Paridade vanilla `location.rs #[scope]`.
///
/// Acede ao `ctx.introspector` (TagIntrospector implementa `Introspector::position_of`).
/// Se o introspector não tiver a posição da localização (pre-layout ou localização
/// inexistente), usa defaults seguros (page=1, x=0pt, y=0pt).
fn eval_location_method(
    loc: crate::entities::location::Location,
    method: &str,
    ctx: &mut EvalContext,
) -> SourceResult<Value> {
    use crate::entities::introspector::Introspector;
    use crate::entities::layout_types::{Length, Pt};

    match method {
        "page" => {
            // Número da página (1-based) via introspector; default 1 se não disponível.
            let page_num = ctx
                .introspector
                .position_of(loc)
                .map(|p| p.page.get() as i64)
                .unwrap_or(1);
            Ok(Value::Int(page_num))
        }
        "position" => {
            // Dict {page: int, x: length, y: length} em pontos.
            let pos = ctx.introspector.position_of(loc);
            let page_num = pos.as_ref().map(|p| p.page.get() as i64).unwrap_or(1);
            let x_pt = pos.as_ref().map(|p| p.point.x.0).unwrap_or(0.0);
            let y_pt = pos.as_ref().map(|p| p.point.y.0).unwrap_or(0.0);

            let mut dict: IndexMap<EcoString, Value, FxBuildHasher> = IndexMap::default();
            dict.insert("page".into(), Value::Int(page_num));
            dict.insert("x".into(), Value::Length(Length::pt(x_pt)));
            dict.insert("y".into(), Value::Length(Length::pt(y_pt)));
            Ok(Value::Dict(dict))
        }
        "page-numbering" => {
            // Scope-out: infra de numeração por página não implementada.
            // Retorna none por paridade graded (ADR-0054).
            Ok(Value::None)
        }
        _ => unreachable!("eval_location_method chamado com método inesperado: {method}"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicU64, AtomicUsize, Ordering};
    use std::sync::Arc;

    use ecow::EcoString;
    use indexmap::IndexMap;
    use rustc_hash::FxBuildHasher;

    use crate::contracts::plugin_host::{PluginError, PluginHost, PluginModuleId};
    use crate::entities::bytes::Bytes;

    /// Host de teste (sem WASM) para o braço `call_plugin`. Conta chamadas e
    /// devolve `b"OK"`. Nomes de export únicos por teste evitam colisão no
    /// cache global do comemo (`PluginFunc::call` é memoizado).
    struct StubHost {
        next: AtomicU64,
        calls: AtomicUsize,
    }
    impl StubHost {
        fn new() -> Self {
            Self {
                next: AtomicU64::new(1),
                calls: AtomicUsize::new(0),
            }
        }
    }
    impl PluginHost for StubHost {
        fn load(&self, _bytes: &[u8]) -> Result<PluginModuleId, PluginError> {
            Ok(PluginModuleId(self.next.fetch_add(1, Ordering::Relaxed)))
        }
        fn exports(
            &self,
            _module: PluginModuleId,
        ) -> Result<Vec<EcoString>, PluginError> {
            Ok(Vec::new())
        }
        fn call(
            &self,
            _module: PluginModuleId,
            _func_name: &str,
            _args: &[Bytes],
        ) -> Result<Bytes, PluginError> {
            self.calls.fetch_add(1, Ordering::SeqCst);
            Ok(Bytes::new(b"OK".to_vec()))
        }
        fn transition(
            &self,
            _module: PluginModuleId,
            _func_name: &str,
            _args: &[Bytes],
        ) -> Result<PluginModuleId, PluginError> {
            Ok(PluginModuleId(self.next.fetch_add(1, Ordering::Relaxed)))
        }
    }

    fn make_pf(name: &str) -> PluginFunc {
        let host: Arc<StubHost> = Arc::new(StubHost::new());
        let id = host.load(&[]).unwrap();
        PluginFunc {
            host: host as Arc<dyn PluginHost>,
            module: id,
            name: EcoString::from(name),
        }
    }

    #[test]
    fn call_plugin_devolve_bytes() {
        let pf = make_pf("cp_devolve");
        let args = Args::positional(vec![Value::Bytes(Bytes::new(b"x".to_vec()))]);
        let v = call_plugin(&pf, &args).unwrap();
        assert!(matches!(v, Value::Bytes(b) if b.as_slice() == b"OK"));
    }

    #[test]
    fn call_plugin_arg_nao_bytes_erro() {
        let pf = make_pf("cp_argbad");
        let args = Args::positional(vec![Value::Int(1)]);
        let e = call_plugin(&pf, &args).unwrap_err();
        // P819 — verbatim do vanilla (medido t6: `expected bytes, found integer`).
        assert_eq!(e[0].message.as_str(), "expected bytes, found integer", "msg: {}", e[0].message);
    }

    #[test]
    fn call_plugin_named_arg_erro() {
        let pf = make_pf("cp_named");
        let mut named = IndexMap::with_hasher(FxBuildHasher);
        named.insert(EcoString::from("x"), Value::None);
        let args = Args {
            items: vec![Value::Bytes(Bytes::default())],
            named,
            span: Span::detached(),
        };
        let e = call_plugin(&pf, &args).unwrap_err();
        assert!(
            e[0].message.contains("argumento nomeado inesperado"),
            "msg: {}",
            e[0].message,
        );
    }
}
