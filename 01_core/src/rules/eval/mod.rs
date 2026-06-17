//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/rules/eval.md
//! @prompt-hash edae68fa
//! @layer L1
//! @updated 2026-04-22
//!
//! Dispatcher central do eval: `EvalContext` struct + impl, `pub fn eval`
//! entry point, `eval_markup` iterator, `eval_expr` dispatcher delegando
//! cada armo para o submódulo do respectivo domínio (ADR-0037 Regra 4,
//! completada no Passo 96.2).
//!
//! Armos triviais (literais `Int`/`Float`/`Bool`/etc., `Ref`/`Label`/
//! `Parenthesized`) permanecem inline; armos que contêm scoping
//! cross-cutting (`CodeBlock`, `ContentBlock`) também, por não
//! pertencerem a nenhum cluster em particular.

use std::sync::Arc;

use comemo::{Tracked, TrackedMut};
use ecow::EcoString;

use crate::contracts::world::World;
use crate::entities::engine::Engine;
use crate::entities::show::{RuleId, ShowRule};
use crate::entities::ast::AstNode;
use crate::entities::content::Content;
#[cfg(test)]
use crate::entities::counter_update::CounterUpdate as CounterAction;
use crate::entities::label::Label;
use crate::entities::ast::expr::{ArrayItem, Expr};
#[cfg(test)]
use crate::entities::ast::expr::{BinOp, UnOp};
use crate::entities::ast::markup::Label as AstLabel;
use crate::entities::layout_types::TextStyle;
use crate::entities::style_chain::StyleChain;
use crate::entities::syntax_kind::SyntaxKind;
use crate::entities::func::Func;
use crate::entities::module::Module;
use crate::entities::scope::Scope;
use crate::entities::source::Source;
use crate::entities::source_result::{SourceDiagnostic, SourceResult};
use crate::entities::span::Span;
use crate::entities::syntax_node::SyntaxNode;
use crate::entities::value::Value;
use crate::entities::world_types::{Route, Routines, Sink, Traced};
use crate::rules::scopes::Scopes;

// Submódulos por domínio (Passo 96.1, ADR-0037).
mod math;
pub(crate) mod operators;
mod control_flow;
pub(crate) mod closures;
mod bindings;
pub(crate) mod rules;
mod markup;
mod modules;

/// Contexto de execução partilhado durante eval().
///
/// Limite de segurança para prevenir loops infinitos:
/// - `max_loop_iterations`: limite total global de iterações. Um contador
///   local por loop permite "loop-bombing" (milhares de loops pequenos que
///   colectivamente travam o motor). Counter global impede isso: 1.000.000
///   iterações falham em segundos independentemente da distribuição.
///
/// A profundidade de chamadas **não** é verificada aqui — é verificada pelo
/// `Route<'a>` através de `route.check_call_depth()` em `apply_closure`
/// (Passo 93, ADR-0033 paridade com vanilla, `MAX_CALL_DEPTH = 80`). O
/// campo antigo `depth`/`max_call_depth`/`enter_call`/`leave_call` foi
/// removido (DEBT-45 parcialmente pago).
///
/// A rota de compilação (`Route<'a>`) **não** é campo do contexto — é
/// passada como parâmetro `route: Tracked<'r, Route<'r>>` às funções
/// `eval_*` que participam na recursão. Paridade estrutural com o vanilla
/// e primeira aplicação concreta da ADR-0036 (atomização progressiva,
/// Passo 92). O campo `route: Vec<FileId>` + API `with_route_id` do Passo
/// 90 foram eliminados no Passo 92 (DEBT-44 fechado).
///
/// A cadeia de estilos (`StyleChain`) **também não** é campo do contexto
/// desde o Passo 94 — propaga-se como `&mut StyleChain` nas funções
/// `eval_*`. Cada bloco de scoping (`CodeBlock`, `ContentBlock`,
/// `Strong`/`Emph`/`Heading`, corpo de closure) cria uma cópia local
/// (`let mut local_styles = styles.clone()` ou `styles.push(delta)`),
/// eliminando o antigo par save/restore sobre um campo partilhado.
/// Segunda aplicação concreta da ADR-0036.
pub struct EvalContext {
    // ADR-0036 Regra 4: contador monotónico global — limite de segurança
    // anti-loop-bombing, independente do fluxo de controlo.
    pub loop_iterations: usize,
    // ADR-0036 Regra 4: limite estático — configuração da execução, sem
    // semântica de fluxo.
    pub max_loop_iterations: usize,
    // ADR-0036 Regra 4: alocador monotónico para IDs de ShowRule (Passo 70)
    // — gera valores únicos durante a sessão de eval, não depende de fluxo.
    //
    // Ficaram fora do contexto em passos anteriores (todos como parâmetros
    // explícitos das funções `eval_*`, agora agregados em `Engine<'a>` no
    // Passo 109, ADR-0044):
    // - `world` (Passo 109) — `&'a dyn World` em `Engine`.
    // - `route` (Passo 92) — `Tracked<'r, Route<'r>>`.
    // - `styles` (Passo 94) — `&mut StyleChain`.
    // - `show_rules` + `active_guards` (Passo 95).
    // - `current_file` + `figure_numbering` (Passo 98).
    // - `sink` (Passo 107).
    pub next_rule_id: crate::entities::show::RuleId,

    /// Snapshot do `TagIntrospector` da iteração de fixpoint anterior
    /// (P174 / M7 sub-passo 1). Default: `TagIntrospector::empty()` —
    /// primeira iteração não vê resultados anteriores. `run_fixpoint`
    /// actualiza este field entre iterações; features stdlib P175+
    /// (`query`, `here`, `counter.at`) leem daqui.
    ///
    /// **Read-only no eval**: stdlib lê, nunca escreve. Mutação
    /// exclusiva por `run_fixpoint`.
    pub introspector: crate::entities::introspector::TagIntrospector,

    /// **P208B (M9c)** — `Location` "actual" disponível durante eval,
    /// para suportar stdlib `here()` (P208B) e `locate()` (P208C).
    ///
    /// **Infra minimal**: `Default = None`. Cristalino single-pass +
    /// fixpoint não avança `current_location` automaticamente no eval
    /// walk (divergência arquitectónica vs vanilla `Tracked<Context>`
    /// per P205A.div-1). Caller que conhece a Location actual (ex.:
    /// futuro show-rule para `Content::Context` block análogo a
    /// vanilla `ContextElem`, ou tests sintéticos) escreve este field
    /// directamente via `with_current_location` antes de invocar
    /// eval/stdlib. `here()` lê este field e devolve
    /// `Value::Location(loc)`; se `None`, erro contextual coerente.
    ///
    /// Mecanismo de captura no eval walk (sub-mecanismo i avançado)
    /// fica deferred — emerge naturalmente quando `Content::Context`
    /// block for materializado (sub-passo dedicado pós-P208).
    pub current_location: Option<crate::entities::location::Location>,
}

impl EvalContext {
    pub fn new() -> Self {
        Self {
            loop_iterations: 0,
            max_loop_iterations: 1_000_000,
            next_rule_id: 0,
            introspector: crate::entities::introspector::TagIntrospector::empty(),
            current_location: None,
        }
    }

    /// **P208B (M9c)** — Setter conveniente para `current_location`.
    /// Usado em tests sintéticos e por consumers futuros que conhecem
    /// a Location actual antes de invocar eval/stdlib `here()`.
    pub fn with_current_location(
        mut self,
        location: crate::entities::location::Location,
    ) -> Self {
        self.current_location = Some(location);
        self
    }

    /// Incrementa o contador de iterações e retorna Err se o limite foi atingido.
    pub fn tick_loop(&mut self, span: Span) -> SourceResult<()> {
        self.loop_iterations += 1;
        if self.loop_iterations > self.max_loop_iterations {
            Err(vec![SourceDiagnostic::error(
                span,
                format!(
                    "limite de iterações de loop atingido ({}) — \
                     possível loop infinito",
                    self.max_loop_iterations
                ),
            )])
        } else {
            Ok(())
        }
    }

}

/// Avalia um ficheiro Typst e retorna o módulo resultante.
///
/// Travessia AST parcial (Passo 17): avalia literais, Ident, Let, CodeBlock,
/// Binary, Unary, Conditional, WhileLoop, ForLoop, Closure, FuncCall.
/// Stdlib mínima injectada: `type`, `len`, `range`.
/// Fronteira deliberada: `_ => Ok(Value::None)` para Content, Styles (ADR-0017).
///
/// **Invariante**: não importa nada de `03_infra`. Acesso ao world
/// sempre via `World` (L1).
pub fn eval(
    _routines: &Routines,
    world: &dyn World,
    _traced: Tracked<Traced>,
    mut sink: TrackedMut<Sink>,
    _route: Tracked<Route>,
    source: &Source,
    // Lote F-3 inc-2: o threading registry→escopo que o F-1 deferiu. Elementos
    // de utilizador registados entram no escopo como funções (`#name(args)`).
    // Em produção é vazio até pacotes registarem elementos; testes injetam.
    registry: &crate::entities::element_registry::ElementRegistry,
) -> SourceResult<Module> {
    let root = source.root();

    // Passo 106 (ADR-0043): canal de warnings activo. Pilot: emitir nota
    // quando o ficheiro fonte está vazio. Prova de vida do canal — o
    // caller lê `sink.into_diagnostics()` após este retorno.
    if source.text().is_empty() {
        sink.warn_note(
            crate::entities::span::Span::detached(),
            "ficheiro vazio: sem conteúdo",
            "",
        );
    }

    let mut ctx = EvalContext::new();

    // Route raiz com o FileId do ficheiro principal — primeira aplicação da
    // ADR-0036 (Passo 92), agora campo do Engine (ADR-0044, Passo 109).
    let route = Route::root().with_id(source.id());
    let mut styles = StyleChain::default_chain();
    let mut show_rules: Arc<[ShowRule]> = Arc::from([]);
    let mut active_guards: Vec<RuleId> = Vec::new();
    let current_file = source.id();

    let mut scopes = Scopes::new(None);
    // Stdlib como scope base — type, len, range visíveis em todo o documento
    let stdlib = make_stdlib();
    for (name, binding) in stdlib.iter() {
        scopes.define(name, binding.value().clone());
    }
    // Lote F-3 inc-2: elementos de utilizador registados entram no escopo como
    // funções (`#name(args)` → `Content::Dynamic` via o construtor do registry).
    // Mesmo escopo base que os nativos (document-wide); `#set`/`#show` léxicos
    // por cima seguem o padrão `local_styles` (F-2).
    for name in registry.names() {
        if let Some(ctor) = registry.ctor(name) {
            scopes.define(
                name.as_str(),
                Value::Func(Func::element(name.as_str(), ctor)),
            );
        }
    }
    scopes.enter();  // âmbito do módulo

    // ADR-0044 (Passo 109): agregar os 8 campos num `Engine<'_>` e passar
    // `&mut engine` às funções internas em vez de 8 parâmetros individuais.
    // Reborrow do `sink` encurta o lifetime inner do `TrackedMut` ao da
    // stack frame local, permitindo que `Engine<'a>` tenha um único `'a`.
    let mut local_sink = TrackedMut::reborrow_mut(&mut sink);
    let mut engine = Engine {
        world,
        route: route.track(),
        styles: &mut styles,
        show_rules: &mut show_rules,
        active_guards: &mut active_guards,
        current_file,
        sink: &mut local_sink,
    };

    let content_val = eval_markup(
        root,
        &mut scopes,
        &mut ctx,
        &mut engine,
    )?;

    let module_scope = scopes.exit();
    let content = match content_val {
        Value::Content(c) => Some(c),
        _ => None,
    };
    let mut module = Module::new(
        source.id().into_raw().get().to_string(),
        module_scope,
    );
    module.set_content(content);
    Ok(module)
}

fn eval_markup(
    node: &SyntaxNode,
    scopes: &mut Scopes<'_>,
    ctx: &mut EvalContext,
    engine: &mut Engine<'_>,
) -> SourceResult<Value> {
    let mut parts: Vec<Content> = Vec::new();
    // Passo 155: estado de alternância para SmartQuote em markup.
    // Cristalino usa o lexer vanilla (1 char = 1 token); o eval emite
    // a aspa localizada open/close conforme alternância par/ímpar dentro
    // da sequência markup. Aspas duplas e simples têm contadores
    // independentes. Aspas simples ainda usam ASCII (aspas secundárias
    // scope-out per spec P155).
    let mut double_open = true; // true = próximo `"` é open
    let mut single_open = true;

    // β1 fatia 1 (P339, L0 §3a.8): snapshot dos 3 customs de numbering à entrada
    // deste corpo, para detectar um `#set …(numbering:)` local e embrulhar o
    // resto do escopo léxico num `Content::Styled` (transporte aditivo
    // `StyledElem`-scoped). Só o #set muta `engine.styles` neste loop
    // (strong/emph/heading usam `local_styles`).
    const NUM_KEYS: [&str; 3] =
        ["heading.numbering", "equation.numbering", "figure.numbering"];
    let snap: [Option<crate::entities::value::Value>; 3] = [
        engine.styles.custom(NUM_KEYS[0]).cloned(),
        engine.styles.custom(NUM_KEYS[1]).cloned(),
        engine.styles.custom(NUM_KEYS[2]).cloned(),
    ];
    let mut wrap_start: Option<usize> = None;

    for child in node.children() {
        match child.kind() {
            SyntaxKind::Text => {
                // Capturar o estilo activo no momento da produção (Passo 30).
                let style = TextStyle::from(&*engine.styles);
                let text_node = Content::Text(child.text().as_str().into(), style);
                // Intercepção eager para Selector::Text (Passo 68).
                parts.push(rules::intercept_content(text_node, ctx, engine)?);
            }
            // Passo 155 — SmartQuote: emite glyph localizado consoante
            // open/close (alternância) e `text.lang` activo.
            SyntaxKind::SmartQuote => {
                let raw = child.text();
                let is_double = raw.as_str() == "\"";
                let style = TextStyle::from(&*engine.styles);
                let lang = engine.styles.lang();
                let (open, close) = match &lang {
                    Some(l) => crate::rules::lang::quotes::localize_quotes(l),
                    None    => crate::rules::lang::quotes::DEFAULT_QUOTES,
                };
                let glyph: &str = if is_double {
                    let g = if double_open { open } else { close };
                    double_open = !double_open;
                    g
                } else {
                    // Aspas simples — scope-out smart-apostrophes neste passo.
                    // Apenas alternância de estado (manter consistente
                    // com o lado duplo); glyph emitido é sempre ASCII `'`.
                    single_open = !single_open;
                    "'"
                };
                let quote_node = Content::Text(glyph.into(), style);
                parts.push(rules::intercept_content(quote_node, ctx, engine)?);
            }
            SyntaxKind::Space | SyntaxKind::Parbreak => parts.push(Content::Space),
            k if k.is_trivia() => continue,
            // Passo 56 — associação retroactiva: <label> envolve o nó precedente.
            // O parser expõe <label> como nó irmão (não filho) do nó anterior.
            // Entre o nó alvo e a label pode haver Space — salta-os para encontrar
            // o elemento real, re-insere-os a seguir ao Labelled.
            SyntaxKind::Label => {
                if let Some(label_ast) = child.cast::<AstLabel<'_>>() {
                    let name = label_ast.get().to_string();
                    // Recolher espaços finais para re-inserir após o Labelled.
                    let mut trailing: Vec<Content> = Vec::new();
                    while matches!(parts.last(), Some(Content::Space) | Some(Content::Empty)) {
                        trailing.push(parts.pop().unwrap());
                    }
                    if let Some(last) = parts.pop() {
                        parts.push(Content::labelled(last, Label(name)));
                        trailing.reverse();
                        parts.extend(trailing);
                    }
                    // Se parts estiver vazio após remover espaços, ignorar.
                }
            }
            _ => {
                if let Some(expr) = Expr::from_untyped(child) {
                    match eval_expr(expr, scopes, ctx, engine)? {
                        Value::Content(c) => parts.push(c),
                        Value::Str(s)     => {
                            let style = TextStyle::from(&*engine.styles);
                            parts.push(Content::Text(s, style));
                        }
                        Value::None       => {}
                        _                 => {}
                    }
                }
            }
        }

        // β1: detectar o #set numbering — quando um custom novo aparece (vs o
        // início deste corpo), o escopo a embrulhar começa nas partes seguintes.
        if wrap_start.is_none()
            && NUM_KEYS
                .iter()
                .enumerate()
                .any(|(i, k)| engine.styles.custom(k) != snap[i].as_ref())
        {
            wrap_start = Some(parts.len());
        }
    }

    // β1: embrulha o resto do escopo num `Content::Styled` carregando o custom
    // (transporte aditivo; o baking permanece autoritativo até o F-5; `f3s3`
    // intocado — é `#show`). Caminho duplo chain↔assado, paridade testada.
    if let Some(start) = wrap_start {
        let tail = parts.split_off(start);
        let mut styles = crate::entities::style::Styles::new();
        for (i, k) in NUM_KEYS.iter().enumerate() {
            if let Some(v) = engine.styles.custom(k) {
                if Some(v) != snap[i].as_ref() {
                    styles = styles.push_custom(*k, v.clone());
                }
            }
        }
        parts.push(Content::Styled(Box::new(Content::sequence(tail)), styles));
    }

    Ok(Value::Content(Content::sequence(parts)))
}

fn eval_expr(
    expr: Expr<'_>,
    scopes: &mut Scopes<'_>,
    ctx: &mut EvalContext,
    engine: &mut Engine<'_>,
) -> SourceResult<Value> {
    match expr {
        Expr::Int(node)   => Ok(Value::Int(node.get())),
        Expr::Float(node) => Ok(Value::Float(node.get())),
        Expr::Str(node)   => Ok(Value::Str(EcoString::from(node.get()))),
        Expr::Bool(node)  => Ok(Value::Bool(node.get())),
        Expr::None(_)     => Ok(Value::None),
        Expr::Auto(_)     => Ok(Value::Auto),

        Expr::Ident(ident) => {
            let name = ident.as_str();
            scopes.get(name)
                .cloned()
                .ok_or_else(|| vec![SourceDiagnostic::error(
                    ident.span(),
                    format!("unknown variable: {name}"),
                )])
        }

        Expr::LetBinding(binding) => bindings::eval_let(binding, scopes, ctx, engine),

        Expr::CodeBlock(code_block) => {
            // Bloco de código — styles e show_rules locais (atomização
            // Passos 94 e 95). `#set`/`#show` dentro do bloco mutam as
            // cópias locais mas não afectam o chamador. Engine
            // reconstruído localmente (ADR-0044, Passo 109).
            let mut local_styles = engine.styles.clone();
            let mut local_show_rules = Arc::clone(engine.show_rules);
            let mut local_sink = TrackedMut::reborrow_mut(&mut *engine.sink);
            let mut last = Value::None;
            {
                let mut local_engine = Engine {
                    world: engine.world,
                    route: engine.route,
                    styles: &mut local_styles,
                    show_rules: &mut local_show_rules,
                    active_guards: &mut *engine.active_guards,
                    current_file: engine.current_file,
                    sink: &mut local_sink,
                };
                for expr in code_block.body().exprs() {
                    last = eval_expr(expr, scopes, ctx, &mut local_engine)?;
                }
            }
            Ok(last)
        }

        Expr::Binary(binary) => {
            let lhs = eval_expr(binary.lhs(), scopes, ctx, engine)?;
            let rhs = eval_expr(binary.rhs(), scopes, ctx, engine)?;
            operators::eval_binary_op(binary.op(), lhs, rhs)
                .map_err(|msg| vec![SourceDiagnostic::error(binary.span(), msg)])
        }

        Expr::Unary(unary) => {
            let operand = eval_expr(unary.expr(), scopes, ctx, engine)?;
            operators::eval_unary_op(unary.op(), operand)
                .map_err(|msg| vec![SourceDiagnostic::error(unary.span(), msg)])
        }

        Expr::Conditional(cond) => control_flow::eval_conditional(cond, scopes, ctx, engine),
        Expr::WhileLoop(loop_expr) => control_flow::eval_while(loop_expr, scopes, ctx, engine),
        Expr::ForLoop(loop_expr) => control_flow::eval_for(loop_expr, scopes, ctx, engine),

        Expr::Closure(c)  => closures::eval_closure_expr(c, scopes, ctx, engine),
        Expr::FuncCall(c) => closures::eval_func_call(c, scopes, ctx, engine),

        Expr::Strong(s)   => markup::eval_strong(s, scopes, ctx, engine),
        Expr::Emph(e)     => markup::eval_emph(e, scopes, ctx, engine),
        Expr::Heading(h)  => markup::eval_heading(h, scopes, ctx, engine),
        Expr::Raw(r)      => markup::eval_raw(r),
        Expr::Link(l)     => markup::eval_link(l, &*engine.styles),
        Expr::ListItem(i) => markup::eval_list_item(i, scopes, ctx, engine),
        Expr::EnumItem(i) => markup::eval_enum_item(i, scopes, ctx, engine),

        Expr::FieldAccess(a) => bindings::eval_field_access(a, scopes, ctx, engine),

        Expr::SetRule(s)  => rules::eval_set_rule(s, scopes, ctx, engine),

        Expr::ContentBlock(content_block) => {
            // Content block [ ] — styles locais ao bloco. Engine
            // reconstruído localmente (ADR-0044, Passo 109).
            //
            // **Caso 4 / f3s3 (P340, F-realização fatia 2):** `show_rules` também
            // local (clone O(1) do Arc), espelhando o `CodeBlock` (`{}`). Antes,
            // o `[]` partilhava `&mut *engine.show_rules` → um `#show` dentro do
            // bloco VAZAVA para fora (mutava a chain do chamador da declaração em
            // diante). Agora confina ao escopo do bloco — paridade com o vanilla,
            // que confina via `StyledElem` (`content/mod.rs:744-752`). É o
            // confinamento estrutural que faltava ao modelo eager no `[]`.
            let mut local_styles = engine.styles.clone();
            let mut local_show_rules = Arc::clone(engine.show_rules);
            let mut local_sink = TrackedMut::reborrow_mut(&mut *engine.sink);
            let mut local_engine = Engine {
                world: engine.world,
                route: engine.route,
                styles: &mut local_styles,
                show_rules: &mut local_show_rules,
                active_guards: &mut *engine.active_guards,
                current_file: engine.current_file,
                sink: &mut local_sink,
            };
            eval_markup(content_block.body().to_untyped(), scopes, ctx, &mut local_engine)
        }

        Expr::Equation(eq) => {
            let block = eq.block();
            let body  = math::eval_math_content(scopes, ctx, eq.body())?;
            // Lote F-2 S2 (P335): assar a numeração ativa da chain léxica
            // (`#set math.equation(numbering:)` empurrou para custom). Só
            // equações de bloco numeram (paridade vanilla).
            let numbering_active = block
                && matches!(
                    engine.styles.custom("equation.numbering"),
                    Some(crate::entities::value::Value::Bool(true))
                );
            let content = if numbering_active {
                Content::equation_numbered(body, block)
            } else {
                Content::equation(body, block)
            };
            Ok(Value::Content(content))
        }

        Expr::Math(math) => {
            // Math node isolado (fora de Equation) — produzir como sequence.
            let content = math::eval_math_content(scopes, ctx, math)?;
            Ok(Value::Content(content))
        }

        Expr::ModuleImport(i)  => modules::eval_module_import(i),
        Expr::ModuleInclude(i) => modules::eval_module_include(i, scopes, ctx, engine),

        // Passo 56 — referência cruzada: @nome → Content::Ref placeholder.
        Expr::Ref(ref_node) => {
            let name = ref_node.target().to_string();
            Ok(Value::Content(Content::reference(Label(name))))
        }

        // Passo 56 — label em contexto de código (raro); a associação retroactiva
        // acontece em eval_markup via SyntaxKind::Label. Aqui apenas ignoramos.
        Expr::Label(_) => Ok(Value::None),

        Expr::ShowRule(s) => rules::eval_show_rule(s, scopes, ctx, engine),

        // Passo 81 — array literal `(1fr, 1fr)` / `(10pt, auto, 1fr)`.
        // Necessário para o argumento `columns` de `grid()`.
        Expr::Array(arr) => {
            let mut items = Vec::new();
            for item in arr.items() {
                if let ArrayItem::Pos(expr) = item {
                    items.push(eval_expr(expr, scopes, ctx, engine)?);
                }
            }
            Ok(Value::Array(items))
        }

        // `(expr)` — parêntese de agrupamento. Expressão única dentro de
        // parênteses avalia para o valor da expressão. Passo 83.
        // (Um tuplo com um elemento requer a vírgula trailing: `(x,)`.)
        Expr::Parenthesized(paren) => eval_expr(paren.expr(), scopes, ctx, engine),

        // Passo 76 — literais numéricos com unidade (ex: 100pt, 1.5em).
        Expr::Numeric(num) => {
            use crate::entities::ast::expr::Unit;
            use crate::entities::layout_types::{Abs, Angle, Length, Ratio};
            let (value, unit) = num.get();
            match unit {
                Unit::Pt      => Ok(Value::Length(Length { abs: Abs(value),            em: 0.0 })),
                Unit::Mm      => Ok(Value::Length(Length { abs: Abs(value * 2.8346),   em: 0.0 })),
                Unit::Cm      => Ok(Value::Length(Length { abs: Abs(value * 28.346),   em: 0.0 })),
                Unit::In      => Ok(Value::Length(Length { abs: Abs(value * 72.0),     em: 0.0 })),
                Unit::Em      => Ok(Value::Length(Length { abs: Abs(0.0),              em: value })),
                Unit::Deg     => Ok(Value::Angle(Angle::deg(value))),
                Unit::Rad     => Ok(Value::Angle(Angle::rad(value))),
                Unit::Percent => Ok(Value::Ratio(Ratio::from_percent(value))),
                Unit::Fr      => Ok(Value::Fraction(value)),
            }
        }

        // Fronteira deliberada — requer tipos não migrados (Content, Styles, etc.)
        _ => Ok(Value::None),
    }
}

/// Avalia o corpo de um nó de markup como Content.
fn eval_markup_body(
    node: &SyntaxNode,
    scopes: &mut Scopes<'_>,
    ctx: &mut EvalContext,
    engine: &mut Engine<'_>,
) -> SourceResult<Content> {
    match eval_markup(node, scopes, ctx, engine)? {
        Value::Content(c) => Ok(c),
        _                 => Ok(Content::Empty),
    }
}

// eval_args, apply_func, apply_closure extraídos para eval/closures.rs (Passo 96.1).

// Math eval extraído para eval/math.rs no Passo 96.1 (ADR-0037).

// eval_let extraído para eval/bindings.rs (Passo 96.1).

// apply_show_rules e intercept_content extraídos para eval/rules.rs (Passo 96.1).

/// Constrói a stdlib: `type`, `len`, `range`, `rgb`, `luma`, `str`, `int`, `float`, `figure`, `assert`, `upper`, `lower`, `replace`, `calc`.
///
/// Passo 64 (DEBT-16): `native_figure` migrada do interceptador em eval.rs para cá.
/// O avaliador deixa de conhecer o nome "figure" — desacoplamento total.
fn make_stdlib() -> Scope {
    use crate::rules::stdlib::{
        make_calc_module, make_gradient_module, make_math_module, native_accent, native_align, native_assert, native_bibliography, native_block, native_box, native_cancel, native_circle, native_cite, native_divider,
        native_ellipse, native_emph, native_figure, native_float, native_footnote, native_grid, native_h, native_heading,
        native_hide, native_image, native_int, native_len, native_line,
        native_counter_at, native_counter_display, native_counter_final, native_counter_step, native_curve, native_here, native_locate, native_lower, native_luma, native_measure, native_metadata, native_move, native_pad, native_pagebreak, native_place, native_polygon, native_query, native_state, native_state_at, native_state_display, native_state_final, native_state_update, native_state_update_with,
        native_cmyk, native_colbreak, native_columns, native_hsl, native_hsv, native_linear_rgb, native_oklab, native_oklch, native_op, native_quote, native_range, native_rect, native_repeat, native_replace, native_raw, native_rgb, native_rotate,
        native_scale, native_skew, native_smartquote, native_stack, native_str, native_strike, native_stroke, native_strong, native_table, native_table_cell, native_table_footer, native_table_header, native_grid_cell, native_grid_footer, native_grid_header, native_terms, native_type, native_underline, native_underover, native_overline, native_upper, native_v,
        // P311b.3 — math style funcs.
        native_bb, native_bold, native_cal, native_frak, native_math_italic,
        native_mono, native_sans, native_scr, native_script, native_serif,
        native_sscript, native_upright,
    };
    let mut scope = Scope::new();
    scope.define("type",    Value::Func(Func::native("type",    native_type)));
    scope.define("len",     Value::Func(Func::native("len",     native_len)));
    scope.define("range",   Value::Func(Func::native("range",   native_range)));
    scope.define("rgb",        Value::Func(Func::native("rgb",        native_rgb)));
    scope.define("luma",       Value::Func(Func::native("luma",       native_luma)));
    // P257 (ADR-0083 PROPOSTO) — 6 stdlib funcs novas para espaços
    // de cor materializados (paridade vanilla `oklab`/`oklch`/
    // `linear-rgb`/`cmyk`/`color.hsl`/`color.hsv`).
    scope.define("oklab",      Value::Func(Func::native("oklab",      native_oklab)));
    scope.define("oklch",      Value::Func(Func::native("oklch",      native_oklch)));
    scope.define("linear_rgb", Value::Func(Func::native("linear_rgb", native_linear_rgb)));
    scope.define("cmyk",       Value::Func(Func::native("cmyk",       native_cmyk)));
    scope.define("hsl",        Value::Func(Func::native("hsl",        native_hsl)));
    scope.define("hsv",        Value::Func(Func::native("hsv",        native_hsv)));
    scope.define("str",     Value::Func(Func::native("str",     native_str)));
    scope.define("int",     Value::Func(Func::native("int",     native_int)));
    scope.define("float",   Value::Func(Func::native("float",   native_float)));
    scope.define("heading",   Value::Func(Func::native("heading",   native_heading)));
    scope.define("strong",    Value::Func(Func::native("strong",    native_strong)));
    scope.define("emph",      Value::Func(Func::native("emph",      native_emph)));
    scope.define("raw",       Value::Func(Func::native("raw",       native_raw)));
    // P284 (ADR-0054 graded): text decoration — underline / strike /
    // overline. Cosméticos `stroke`/`offset`/`extent` opcionais; `evade`
    // e `background` scope-out per diagnóstico §A.1.
    scope.define("underline", Value::Func(Func::native("underline", native_underline)));
    scope.define("strike",    Value::Func(Func::native("strike",    native_strike)));
    scope.define("overline",  Value::Func(Func::native("overline",  native_overline)));
    // P287 (frente `P-smartquote`): função stdlib paralela ao markup `"..."`
    // P155. `alternative`/`quotes` scope-out per diagnóstico §A.2; `enabled:
    // false` emite glyph ASCII literal (paridade vanilla).
    scope.define("smartquote", Value::Func(Func::native("smartquote", native_smartquote)));
    scope.define("figure",  Value::Func(Func::native("figure",  native_figure)));
    scope.define("image",   Value::Func(Func::native("image",   native_image)));
    scope.define("rect",    Value::Func(Func::native("rect",    native_rect)));
    scope.define("ellipse", Value::Func(Func::native("ellipse", native_ellipse)));
    scope.define("circle",  Value::Func(Func::native("circle",  native_circle)));
    scope.define("line",    Value::Func(Func::native("line",    native_line)));
    scope.define("polygon", Value::Func(Func::native("polygon", native_polygon)));
    // P293 (frente `P-curve-geometry`): activação posterior de
    // `PathItem::CubicTo` via stdlib novo. Reaplicação ADR-0099 para
    // `PathItem` (paralelo P285-P292 para `Style`). Hash `export.rs`
    // preservado pelo 10º passo consecutivo — emit já existe.
    scope.define("curve",   Value::Func(Func::native("curve",   native_curve)));
    scope.define("grid",    Value::Func(Func::native("grid",    native_grid)));
    // Lote F-2 S4/D4 (P335): `page(...)` função-forma legacy removida.
    scope.define("move",    Value::Func(Func::native("move",    native_move)));
    scope.define("rotate",  Value::Func(Func::native("rotate",  native_rotate)));
    scope.define("scale",   Value::Func(Func::native("scale",   native_scale)));
    // Passo 156F (ADR-0061 Fase 1, sub-passo 4): skew via matriz unificada.
    scope.define("skew",    Value::Func(Func::native("skew",    native_skew)));
    scope.define("align",   Value::Func(Func::native("align",   native_align)));
    scope.define("place",   Value::Func(Func::native("place",   native_place)));
    scope.define("assert",  Value::Func(Func::native("assert",  native_assert)));
    // P169 (M9 sub-passo 1): metadata(value) — feature Introspection vanilla.
    scope.define("metadata", Value::Func(Func::native("metadata", native_metadata)));
    // P171 (M9 sub-passo 3): state(key, init) + state_update(key, value).
    scope.define("state", Value::Func(Func::native("state", native_state)));
    scope.define("state_update", Value::Func(Func::native("state_update", native_state_update)));
    // P236 (Fase 5 Layout candidata Categoria D 1/?, refino aditivo
    // pós-P236.div-1): state_final(key) — valor final do state pós-walk.
    // Paralelo a counter_final P176. Reusa Introspector::state_final_value
    // P171. Retorna Value (init se ausente; última update caso contrário).
    scope.define("state_final", Value::Func(Func::native("state_final", native_state_final)));
    // P237 (Fase 5 Layout candidata Categoria D 1/?, refino estendido):
    // state_at(key, label) — valor do state na Location associada ao label.
    // Paralelo absoluto a counter_at P177. Reusa query_by_label P139+P140
    // + state_value P171. Retorna Value::None se key/label inexistentes
    // ou state nunca actualizado antes da Location.
    scope.define("state_at", Value::Func(Func::native("state_at", native_state_at)));
    // P172 (M9 sub-passo 4): state_update_with(key, fn) — callback variant.
    // **Stub**: from_tags ignora Func variant até pipeline restructuring.
    scope.define("state_update_with", Value::Func(Func::native("state_update_with", native_state_update_with)));
    // P240 (M9d/M7+1; ADR-0081 PROPOSTO P239 Opção γ):
    // state_display(key, [callback]) — render-mediated state display real
    // walk-time. Walk emite `Content::StateDisplay` tag; `apply_state_displays`
    // pós-fixpoint pre-renderiza Content via apply_func(callback, [value],
    // ctx, engine). Layouter consome via Introspector::state_display_value
    // (Layouter permanece puro — Opção γ vs α/β/δ P239 audit).
    scope.define("state_display", Value::Func(Func::native("state_display", native_state_display)));
    // P241 (M9d/M7+2; ADR-0081 IMPLEMENTADO parcial M7+2 paralelo P240):
    // counter_display(key, [callback]) — render-mediated counter display
    // real walk-time. Walk emite `Content::CounterDisplayCallback` tag;
    // `apply_counter_displays` pós-fixpoint converte counter slice para
    // Value::Array e aplica callback. Distinto de Content::CounterDisplay
    // { kind } legacy single-pass.
    scope.define("counter_display", Value::Func(Func::native("counter_display", native_counter_display)));
    // P175 (M9 sub-passo 5): query(kind_str) — consulta ctx.introspector
    // da iter de fixpoint anterior. Retorna Value::Int(count) — forma
    // minimal sem Value::Location.
    scope.define("query", Value::Func(Func::native("query", native_query)));
    // P208B (M9c Bloco IV): here() — retorna Value::Location(loc) onde
    // loc = ctx.current_location. Erro contextual se current_location
    // é None (P208B infra minimal; captura automática deferred).
    scope.define("here", Value::Func(Func::native("here", native_here)));
    // P208C (M9c Bloco IV): locate(kind) — retorna primeira Location
    // do kind indicado, ou Value::None se sem matches. Reusa pattern
    // de native_query + Selector::Kind (P175 minimal); locate(<label>)
    // requer P209 (Selector::Label).
    scope.define("locate", Value::Func(Func::native("locate", native_locate)));
    // P210B (M9c Bloco V): counter_step(key) — emite
    // Content::CounterUpdate { key, action: Step } que aplica em
    // layout time. Q1=β subset minimal (counter.display + state.get
    // deferred até walk advance per P210A C3).
    scope.define("counter_step", Value::Func(Func::native("counter_step", native_counter_step)));
    // P176 (M9 sub-passo 6): counter_final(key) — formato hierárquico
    // do counter na iter de fixpoint anterior. Reusa
    // Introspector::formatted_counter (P170). Retorna Value::Str.
    scope.define("counter_final", Value::Func(Func::native("counter_final", native_counter_final)));
    // P177 (M9 sub-passo 7): counter_at(key, label) — valor do counter
    // na Location associada ao label. Reusa query_by_label +
    // formatted_counter_at. Retorna Value::Str.
    scope.define("counter_at", Value::Func(Func::native("counter_at", native_counter_at)));
    scope.define("upper",   Value::Func(Func::native("upper",   native_upper)));
    scope.define("lower",   Value::Func(Func::native("lower",   native_lower)));
    scope.define("replace", Value::Func(Func::native("replace", native_replace)));
    // Passo 154B (ADR-0060 Fase 1): terms + divider.
    scope.define("terms",   Value::Func(Func::native("terms",   native_terms)));
    scope.define("divider", Value::Func(Func::native("divider", native_divider)));
    // Passo 155 (ADR-0060 Fase 1, sub-passo 2): quote.
    scope.define("quote",   Value::Func(Func::native("quote",   native_quote)));
    // Passo 295 — footnote Fase 1 (marker only).
    scope.define("footnote", Value::Func(Func::native("footnote", native_footnote)));
    // Passo 296 — math accent + cancel (HIV + (a) minimal).
    scope.define("accent",  Value::Func(Func::native("accent",  native_accent)));
    scope.define("cancel",  Value::Func(Func::native("cancel",  native_cancel)));
    // Passo 297 — math underover (HV'.a + (b) Option fields).
    scope.define("underover", Value::Func(Func::native("underover", native_underover)));
    // Passo 298 — math op (HV'' adaptado; cross-variant interaction).
    scope.define("op",        Value::Func(Func::native("op",        native_op)));
    // P311b.3 — 12 funções math style (paridade categoria 12/12 = 100%).
    scope.define("bb",        Value::Func(Func::native("bb",        native_bb)));
    scope.define("bold",      Value::Func(Func::native("bold",      native_bold)));
    scope.define("cal",       Value::Func(Func::native("cal",       native_cal)));
    scope.define("frak",      Value::Func(Func::native("frak",      native_frak)));
    scope.define("italic",    Value::Func(Func::native("italic",    native_math_italic)));
    scope.define("mono",      Value::Func(Func::native("mono",      native_mono)));
    scope.define("sans",      Value::Func(Func::native("sans",      native_sans)));
    scope.define("scr",       Value::Func(Func::native("scr",       native_scr)));
    scope.define("script",    Value::Func(Func::native("script",    native_script)));
    scope.define("serif",     Value::Func(Func::native("serif",     native_serif)));
    scope.define("sscript",   Value::Func(Func::native("sscript",   native_sscript)));
    scope.define("upright",   Value::Func(Func::native("upright",   native_upright)));
    // Passo 156C (ADR-0061 Fase 1, sub-passo 1): pad + hide.
    scope.define("pad",     Value::Func(Func::native("pad",     native_pad)));
    scope.define("hide",    Value::Func(Func::native("hide",    native_hide)));
    // Passo 156D (ADR-0061 Fase 1, sub-passo 2): h + v spacing.
    scope.define("h",       Value::Func(Func::native("h",       native_h)));
    scope.define("v",       Value::Func(Func::native("v",       native_v)));
    // Passo 156E (ADR-0061 Fase 1, sub-passo 3): pagebreak manual.
    scope.define("pagebreak", Value::Func(Func::native("pagebreak", native_pagebreak)));
    // Passo 156G (ADR-0061 Fase 2 sub-passo 1): block container.
    scope.define("block",   Value::Func(Func::native("block",   native_block)));
    // Passo 156H (ADR-0061 Fase 2 sub-passo 2): box inline container.
    scope.define("box",     Value::Func(Func::native("box",     native_box)));
    // Passo 156I (ADR-0061 Fase 2 sub-passo 3): stack compositivo.
    // **Último sub-passo Fase 2; atinge target 72% Layout.**
    scope.define("stack",   Value::Func(Func::native("stack",   native_stack)));
    // Passo 156J (ADR-0061 Fase 3 sub-passo 1): repeat (paridade
    // estrutural; algoritmo dinâmico diferido per ADR-0054 graded).
    // **Primeira aplicação Fase 3.**
    scope.define("repeat",  Value::Func(Func::native("repeat",  native_repeat)));
    // P218 (DEBT-56 sub-fase b — Layout Fase 3): columns(count, body,
    // gutter: ?). Variant Content::Columns materializado em P217;
    // arm Layouter é stub transparente (consumer real P219).
    scope.define("columns", Value::Func(Func::native("columns", native_columns)));
    // P220 (DEBT-56 sub-fase b 4/4 — Layout Fase 3): colbreak(weak: ?).
    // Variant Content::Colbreak agregado (variant + arm + stdlib);
    // arm Layouter Opção β graded — downgrade a pagebreak literal.
    // **Fecha sub-fase (b) DEBT-56 estructuralmente.**
    scope.define("colbreak", Value::Func(Func::native("colbreak", native_colbreak)));
    // P222 (Fase 4 Layout candidata sub-passo 1; ADR-0066 §"Plano
    // promoção" Bloco C primeira materialização parcial): measure(body)
    // → Dict { width: Length, height: Length }. Helper privado
    // `measure_content` promovido a `pub(crate)`; semantic graded
    // (single-pass; runtime queries genuínas diferidas; width override
    // scope-out Opção β).
    scope.define("measure", Value::Func(Func::native("measure", native_measure)));
    // P227 (ADR-0079 PROPOSTO Fase 5 Categoria A.1 sub-passo 1):
    // stroke(paint: ?, thickness: ?) constructor para Value::Stroke;
    // parametriza borders Grid/Table via Stroke shorthand parsing.
    // Valida ADR-0080 PROPOSTO N=7 → 8 (L0 não tocado em P227).
    scope.define("stroke", Value::Func(Func::native("stroke", native_stroke)));
    // Passo 157A (ADR-0060 Fase 2 sub-passo 1): table minimal
    // (subset 3 fields; reusa layout_grid; TableCell/Header/Footer
    // diferidos para P157B/C). **Primeiro sub-passo Model Fase 2.**
    scope.define("table",   Value::Func(Func::native("table",   native_table)));
    // Passo 157B (ADR-0060 Fase 2 sub-passo 2): table cell
    // (subset 5 fields; ADR-0064 Caso A para x/y, Caso C para
    // colspan/rowspan; placement diferido em DEBT-34e).
    // Naming `table_cell` flat (não vanilla `table.cell`) per
    // diagnóstico P157B §8 — FieldAccess actual não suporta
    // namespacing de funcs.
    scope.define("table_cell", Value::Func(Func::native("table_cell", native_table_cell)));
    // Passo 157C (ADR-0060 Fase 2 sub-passo 3 — fecha "table foundations"):
    // par simétrico TableHeader/TableFooter. ADR-0064 Caso D para
    // `repeat: bool` default true (primeira aplicação Caso D em
    // Model). Algoritmo de repetição em page breaks diferido em
    // DEBT-56 (refactor multi-region). Naming flat per padrão P157B.
    scope.define("table_header", Value::Func(Func::native("table_header", native_table_header)));
    scope.define("table_footer", Value::Func(Func::native("table_footer", native_table_footer)));
    // P224 (ADR-0061 Fase 4 Layout candidata sub-passo 3 — fecha série α
    // "terminar Layout"): grid_cell + grid_header + grid_footer paridade
    // P157B/C literal; grid_cell resolve placement real via P224.C
    // grid_placement.rs (fecha DEBT-34e).
    scope.define("grid_cell",   Value::Func(Func::native("grid_cell",   native_grid_cell)));
    scope.define("grid_header", Value::Func(Func::native("grid_header", native_grid_header)));
    scope.define("grid_footer", Value::Func(Func::native("grid_footer", native_grid_footer)));
    // Passo 159A (ADR-0060 Fase 2 — Bibliography + Cite par acoplado):
    // subset minimal sem hayagriva (input cristalino literal
    // Vec<BibEntry>). Naming flat per padrão P157B; placeholder
    // render per ADR-0033 + ADR-0054 graded; sem validação
    // cross-reference (ADR-0017 adiada). Refinos futuros (CSL,
    // form, hayagriva) NÃO reservados per política P158.
    scope.define("bibliography", Value::Func(Func::native("bibliography", native_bibliography)));
    scope.define("cite",         Value::Func(Func::native("cite",         native_cite)));
    scope.define("calc",    make_calc_module());
    // P262 — `gradient.linear(...)` via module dict (ADR-0087).
    scope.define("gradient", make_gradient_module());
    // P299 — `math.sin`/`math.lim`/etc. (P298.X; 42 operadores
    // pré-definidos paridade vanilla via SSoT MathOp).
    scope.define("math",     make_math_module());

    // Constantes de alinhamento (Passo 84.5, encerra DEBT-36).
    // Sintaxe preferida: `align(center, ...)`, `align(center + bottom, ...)`.
    use crate::entities::layout_types::{Align2D, HAlign, VAlign};
    scope.define("left",    Value::Align(Align2D { h: Some(HAlign::Left),    v: None }));
    scope.define("center",  Value::Align(Align2D { h: Some(HAlign::Center),  v: None }));
    scope.define("right",   Value::Align(Align2D { h: Some(HAlign::Right),   v: None }));
    scope.define("top",     Value::Align(Align2D { h: None, v: Some(VAlign::Top) }));
    scope.define("horizon", Value::Align(Align2D { h: None, v: Some(VAlign::Horizon) }));
    scope.define("bottom",  Value::Align(Align2D { h: None, v: Some(VAlign::Bottom) }));

    scope
}

// ── Auxiliares para intercepção de counter(...).method() ──────────────────

/// Extrai o nome do contador de uma expressão `counter(key)`.
// extract_counter_key e eval_counter_method extraídos para eval/bindings.rs (Passo 96.1).

#[cfg(test)]
mod tests;
#[cfg(test)]
pub(crate) use crate::rules::eval::tests::eval_for_test;
// Re-export para o módulo de tests (que usa `use super::*;`).
#[cfg(test)]
pub(crate) use crate::rules::eval::operators::{eval_binary_op, eval_unary_op};
