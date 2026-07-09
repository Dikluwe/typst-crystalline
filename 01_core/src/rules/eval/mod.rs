//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/rules/eval.md
//! @prompt-hash a8523b4b
//! @layer L1
//! @updated 2026-07-09
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

use std::collections::HashMap;
use std::sync::Arc;

use comemo::{Track, Tracked, TrackedMut};
use ecow::EcoString;
use hayagriva::citationberg::IndependentStyle;

use crate::contracts::world::World;
use crate::entities::document_info::DocumentInfo;
use crate::entities::engine::Engine;
use crate::entities::show::{RuleId, ShowRule};
use crate::entities::ast::AstNode;
use crate::entities::content::Content;
use crate::entities::elements::bibliography::BibliographyElem;
use crate::entities::elements::context_block::ContextBlockElem;
#[cfg(test)]
use crate::entities::counter_update::CounterUpdate as CounterAction;
use crate::entities::ast::expr::{ArrayItem, Expr};
#[cfg(test)]
use crate::entities::ast::expr::{BinOp, UnOp};
use crate::entities::ast::markup::Label as AstLabel;
use crate::entities::style_chain::StyleChain;
use crate::entities::syntax_kind::SyntaxKind;
use crate::entities::func::{ClosureRepr, Func};
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
pub(crate) mod cast;
pub use cast::{cast_length, CastError};
pub(crate) mod flow;
pub use flow::FlowEvent;
mod control_flow;
pub(crate) mod closures;
pub use closures::apply_func;
mod bindings;
pub(crate) mod rules;
mod markup;
mod modules;
pub(crate) mod bibliography;
pub mod bibtex;
pub(crate) mod repr;

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

    /// **P350c — flag de "erro completo" (capacidade interna)**. Quando ligada,
    /// o erro de recursão de `#show` (teto, `apply_show_rules`) ganha um **3º hint**
    /// classificando **cíclico** (uma morfologia do caminho repetiu — fato medido
    /// pelo `==` do P345) ou **não-convergente** (teto sem repetição). A **mensagem
    /// base** e os 2 hints do vanilla **não mudam**; o 3º hint só aparece com a flag.
    /// **Caminho quente intacto**: o histórico de morfologias só é alocado quando
    /// esta flag está ligada (atrás do `if`). **L1 não lê env** — recebe o booleano
    /// já resolvido (via `eval_with_full_error`; origem em `RunIntent`, fio L4→L1
    /// interno + parsing CLI = débito P350c). Default `false` (= comportamento
    /// byte-idêntico ao vanilla).
    pub full_error: bool,

    /// **P498 — separação entre conteúdo original (para introspecção) e output
    /// de show-rules (para layout/render)**. Quando `false`, `intercept_content`
    /// não aplica show-rules, produzindo a árvore original de elementos locatable.
    /// O entrypoint `eval` corre duas passagens: uma com `false` (captura original)
    /// e outra com `true` (render real). Default `true`.
    pub apply_show_rules: bool,

    /// **P506 — indica que o eval está a correr dentro da expansão de um
    /// `context { ... }`. Quando `true`, métodos `.get()` e `.display()` de
    /// `state`/`counter` podem consultar o introspector e a localização actual.
    pub in_context: bool,

    /// **P506 — contador monotónico para IDs de `ContextBlock`. Garante
    /// identificadores estáveis entre a criação em eval e a expansão
    /// pós-introspecção.
    pub next_context_id: u64,

    /// **P429 (DEBT-63)** — styles CSL resolvidos em eval time, indexados pela
    /// chave determinística do `BibliographyElem` correspondente. Transporta-se
    /// para o `Module` no fim do eval e depois para o `BibStore` do
    /// `TagIntrospector` no pipeline (L3), evitando que o elemento guarde cache
    /// de estado computado.
    pub bibliography_styles: HashMap<u64, Arc<IndependentStyle>>,

    /// **P536** — metadados do documento definidos por `#set document(...)`.
    /// Transporta-se para o `Module` no fim do eval e depois para o
    /// exportador PDF (`/Info`).
    pub document_info: DocumentInfo,

    /// **P635 — evento de controlo de fluxo activo**. Equivalente a `vm.flow`
    /// do vanilla (`typst-eval/src/vm.rs:20`). Propagado de `eval_expr` para
    /// ciclos, funções e o entrypoint.
    pub flow: Option<FlowEvent>,
}

impl EvalContext {
    pub fn new() -> Self {
        Self {
            loop_iterations: 0,
            max_loop_iterations: 1_000_000,
            next_rule_id: 0,
            introspector: crate::entities::introspector::TagIntrospector::empty(),
            current_location: None,
            full_error: false,
            apply_show_rules: true,
            in_context: false,
            next_context_id: 0,
            bibliography_styles: HashMap::new(),
            document_info: DocumentInfo::empty(),
            flow: None,
        }
    }

    /// **P506** — gera um ID estável para `ContextBlockElem`.
    pub fn next_context_id(&mut self) -> u64 {
        let id = self.next_context_id;
        self.next_context_id += 1;
        id
    }

    /// **P429 (DEBT-63)** — regista o style CSL resolvido para o
    /// `BibliographyElem` indicado, usando a sua chave determinística.
    pub fn register_bibliography_style(
        &mut self,
        elem: &BibliographyElem,
        style: Arc<IndependentStyle>,
    ) {
        self.bibliography_styles.insert(elem.style_key(), style);
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

/// Avalia um ficheiro Typst e retorna o módulo resultante (flag de erro completo
/// **desligada** — comportamento byte-idêntico ao vanilla). **Delegado** de
/// `eval_with_full_error` (P350c): mantém a assinatura estável para os ~165 callers
/// (produção L3 + testes) — a capacidade da flag entra pela sibling, não por aqui.
pub fn eval(
    routines: &Routines,
    world: &dyn World,
    traced: Tracked<Traced>,
    sink: TrackedMut<Sink>,
    route: Tracked<Route>,
    source: &Source,
    registry: &crate::entities::element_registry::ElementRegistry,
) -> SourceResult<Module> {
    eval_with_full_error(routines, world, traced, sink, route, source, registry, false)
}

/// Como [`eval`], mas com a flag de **erro completo** (P350c) explícita. Quando
/// `full_error` está ligada, o erro de recursão de `#show` ganha um 3º hint
/// classificando cíclico/não-convergente (ver [`EvalContext::full_error`]). A
/// origem do booleano é `RunIntent` (L2), via o caminho **interno** de L3 — a
/// assinatura **pública** de L3 (`compile_to_pdf_bytes`) **não** muda; o parsing
/// da CLI + o fio `RunIntent`→L3-interno são **débito** (P350c). L1 recebe o
/// booleano já resolvido (não lê env).
///
/// Travessia AST parcial: literais, Ident, Let, CodeBlock, Binary, Unary,
/// Conditional, WhileLoop, ForLoop, Closure, FuncCall. **Invariante**: não importa
/// nada de `03_infra`; acesso ao world sempre via `World` (L1).
pub fn eval_with_full_error(
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
    // P350c: flag de erro completo, já resolvida (origem `RunIntent`; L1 não lê env).
    full_error: bool,
) -> SourceResult<Module> {
    let root = source.root();

    // **P648** — propagação selectiva de erros de sintaxe que o eval
    // actualmente descarta. A tentativa de propagar *todos* os erros de
    // parser (P634) quebrou dezassete testes porque o parser assinala
    // construções válidas como erro (smart quotes, `#set` dentro de blocos,
    // etc.). Só propagamos classes de erro confirmadas como genuínas:
    // literais numéricos malformados e escapes Unicode inválidos em markup.
    let syntax_errors: Vec<SourceDiagnostic> = root
        .errors()
        .into_iter()
        .filter(|e| {
            let msg = e.message.as_str();
            msg.starts_with("invalid hexadecimal number:")
                || msg.starts_with("invalid Unicode codepoint:")
        })
        .map(|e| SourceDiagnostic::error(e.span, e.message.to_string()))
        .collect();
    if !syntax_errors.is_empty() {
        return Err(syntax_errors);
    }

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

    // P498 — passagem dupla do eval:
    // 1. `apply_show_rules = false`: produz o conteúdo original (pré-show-rules)
    //    para alimentar a introspecção. Espelha o modelo vanilla, onde o
    //    Introspector vê os elementos antes da realização das show-rules.
    // 2. `apply_show_rules = true`: produz o output renderizado para layout/PDF.
    //
    // O scope, as definições de stdlib e as show-rules registadas são idênticos
    // nas duas passagens; só a aplicação das show-rules difere.
    let mut run_pass = |apply_show_rules: bool,
                        pass_sink: &mut TrackedMut<Sink>|
     -> SourceResult<(Value, Scope, HashMap<u64, Arc<IndependentStyle>>, DocumentInfo)> {
        let mut ctx = EvalContext::new();
        ctx.full_error = full_error; // P350c: flag resolvida (default false via `eval`)
        ctx.apply_show_rules = apply_show_rules; // P498

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
        // P492 — cores predefinidas (red, blue, green, ...) como atalhos globais.
        for (name, value) in crate::rules::stdlib::predefined_color_bindings() {
            scopes.define(name.as_str(), value);
        }
        // P492 — constructor `text(...)` no scope global (usado em show-rules, etc.).
        scopes.define(
            "text",
            Value::Func(crate::entities::func::Func::native("text", crate::rules::stdlib::native_text)),
        );
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
        let mut local_sink = TrackedMut::reborrow_mut(&mut *pass_sink);
        let mut engine = Engine {
            world,
            route: route.track(),
            styles: &mut styles,
            show_rules: &mut show_rules,
            active_guards: &mut active_guards,
            current_file,
            sink: &mut local_sink,
        };

        let content_val = eval_markup(root, &mut scopes, &mut ctx, &mut engine)?;
        if let Some(flow) = ctx.flow {
            return Err(vec![flow.forbidden()]);
        }
        let module_scope = scopes.exit();
        Ok((content_val, module_scope, ctx.bibliography_styles, ctx.document_info))
    };

    // Passo 1: captura do conteúdo original (pré-show-rules).
    // Usa o mesmo sink principal para que warnings emitidos antes de um erro
    // cheguem ao caller (dedup por (span, message) previne duplicação com passo 2).
    let (original_val, _, _, _) = run_pass(false, &mut sink)?;
    let original_content = match original_val {
        Value::Content(c) => Some(c),
        _ => None,
    };

    // Passo 2: eval normal (com show-rules) — este é o resultado oficial.
    let (rendered_val, module_scope, bibliography_styles, document_info) =
        run_pass(true, &mut sink)?;
    let rendered_content = match rendered_val {
        // **P537b** — ligar `#set page(columns: N)` ao consumer `Content::Columns`.
        Value::Content(c) => Some(c.wrap_page_columns()),
        _ => None,
    };

    let mut module = Module::new(
        source.id().into_raw().get().to_string(),
        module_scope,
    );
    module.set_content(rendered_content);
    module.set_introspection_content(original_content);
    // P429 (DEBT-63): transportar styles resolvidos do eval para o Module,
    // de onde o pipeline os injectará no BibStore do TagIntrospector.
    module.set_bibliography_styles(bibliography_styles);
    // P536: transportar metadados do documento para o Module.
    module.set_document_info(document_info);
    Ok(module)
}

pub(crate) fn eval_markup(
    node: &SyntaxNode,
    scopes: &mut Scopes<'_>,
    ctx: &mut EvalContext,
    engine: &mut Engine<'_>,
) -> SourceResult<Value> {
    let mut parts: Vec<Content> = Vec::new();
    // Passo 445: smart quotes context-aware em markup.
    // O lexer marca `"` e `'` como `SyntaxKind::SmartQuote`; o eval decide
    // open/close/apostrophe com base nos caracteres adjacentes e no `text.lang`.
    // Aspas duplas e simples são resolvidas independentemente.
    let src = node.clone().into_text();
    let src_str = src.as_str();
    let mut byte_offset = 0_usize;

    fn is_opening_context(c: Option<char>) -> bool {
        c.is_none() || c.unwrap().is_whitespace() || matches!(c.unwrap(), '(' | '[' | '{' | '<')
    }
    fn is_word_char(c: Option<char>) -> bool {
        c.is_some_and(|c| c.is_alphanumeric())
    }

    // β1 fatia 1 (P339, §3a.8) + F-item3 (P368, §3a.11): snapshot do **delta
    // completo** à entrada deste corpo, para detectar um `#set` local (numbering,
    // prop de elemento de usuário, ou estilo tipado como `text.bold`) e embrulhar
    // o resto do escopo léxico num `Content::Styled` (transporte aditivo
    // `StyledElem`-scoped). Generaliza o antigo snapshot só-de-custom.
    // P373 (§3a.14): correção do transporte — **wraps aninhados por escopo léxico**
    // (espelho do `styled_with_map` por-`#set` do vanilla, recursivo). Rastreia cada
    // **fronteira de `#set`** (mudança do delta vs o estado corrente) com o delta
    // QUE ESTE `#set` introduziu; no fim, fold de dentro para fora produz o
    // aninhamento. Conserta o bug single-wrap-final-collapse (P372): `#set`
    // sequenciais da mesma chave deixam de colapsar para o valor final.
    // P431 (DEBT-50): o delta agora inclui origem (`from_strong`/`from_emph`),
    // propagada pelo `StyleDelta` e embrulhada via `diff_styles`.
    let snap_delta = engine.styles.collapse();
    let mut running_delta = snap_delta.clone();
    let mut boundaries: Vec<(usize, crate::entities::style::Styles)> = Vec::new();

    for child in node.children() {
        match child.kind() {
            SyntaxKind::Text => {
                // F-5b fatia 2 (P373): o render não é mais assado no node — viaja
                // na chain (custom), resolvido no layout.
                let text_node = Content::Text(child.text().as_str().into());
                // Intercepção eager para Selector::Text (Passo 68).
                parts.push(rules::intercept_content(text_node, ctx, engine)?);
            }
            // Passo 445 — SmartQuote: emite glyph localizado consoante
            // open/close (contexto adjacente) e `text.lang` activo.
            SyntaxKind::SmartQuote => {
                let raw = child.text();
                let is_double = raw.as_str() == "\"";
                let lang = engine.styles.lang();

                let off = byte_offset;
                let prev = src_str[..off].chars().last();
                let next = src_str[off + child.len()..].chars().next();

                let glyph: &str = if is_double {
                    let (open, close) = match &lang {
                        Some(l) => crate::rules::lang::quotes::localize_quotes(l),
                        None    => crate::rules::lang::quotes::DEFAULT_QUOTES,
                    };
                    if is_opening_context(prev) {
                        open
                    } else {
                        close
                    }
                } else {
                    let (open, close) = match &lang {
                        Some(l) => crate::rules::lang::quotes::localize_single_quotes(l),
                        None    => crate::rules::lang::quotes::DEFAULT_SINGLE_QUOTES,
                    };
                    // Contracções / possessivos: `don't`, `Alice's` → apostrophe (U+2019).
                    if is_word_char(prev) && is_word_char(next) {
                        close
                    } else if is_opening_context(prev) {
                        open
                    } else {
                        close
                    }
                };
                let quote_node = Content::Text(glyph.into());
                parts.push(rules::intercept_content(quote_node, ctx, engine)?);
            }
            SyntaxKind::Space => parts.push(Content::Space),
            // P622: quebra de parágrafo semântica — distinta de Space.
            SyntaxKind::Parbreak => parts.push(Content::Parbreak),
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
                        parts.push(Content::label_auto(name, last));
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
                            parts.push(Content::Text(s));
                        }
                        // P471 — símbolo Unicode em markup → char como Content::Text.
                        Value::Symbol(s)  => {
                            parts.push(Content::Text(EcoString::from(s.ch)));
                        }
                        // P506 — state(key, init) em markup → Content::State locatável.
                        Value::State(s) => parts.push(Content::state(
                            s.key.to_string(),
                            s.init.as_ref().clone(),
                        )),
                        // P506 — counter(selector) em markup → terminal Empty (só é
                        // visível quando emitido via .update()/.step()/.display()).
                        Value::Counter(_) => {}
                        Value::None       => {}
                        // **P545** — interpolação #{expr} em markup: valores
                        // primitivos convertem-se para texto. Int, Float, Bool,
                        // Array, Dict, Length, Datetime, etc. usam repr_value.
                        other => {
                            let text = crate::rules::eval::repr::repr_value(&other);
                            if !text.is_empty() {
                                parts.push(Content::Text(text.into()));
                            }
                        }
                    }
                }
            }
        }

        byte_offset += child.len();

        // P373/P431: fronteira de `#set` — o delta completo mudou vs o estado
        // corrente. Regista `(parts.len(), Styles do delta introduzido)` e actualiza
        // o corrente. (Só o `#set` muta `engine.styles` neste loop; o `#set` não
        // produz `part`, logo `parts.len()` é o início da cauda que este `#set`
        // escopa.)
        let cur = engine.styles.collapse();
        if cur != running_delta {
            let styles = cur.diff_styles(&running_delta);
            boundaries.push((parts.len(), styles));
            running_delta = cur;
        }
    }

    // P373: fold de dentro para fora — cada fronteira embrulha a sua cauda
    // (`parts[idx..]`) no delta do seu `#set`, produzindo o aninhamento
    // `Styled(A, [X, Styled(B, [Y])])` (escopo léxico, fiel ao vanilla). `#set`
    // único → 1 fronteira → 1 wrap = comportamento de antes (content-preserving).
    for (idx, styles) in boundaries.into_iter().rev() {
        let tail = parts.split_off(idx);
        parts.push(Content::Styled(Box::new(Content::sequence(tail)), styles));
    }

    Ok(Value::Content(Content::sequence(parts)))
}

pub(crate) fn eval_expr(
    expr: Expr<'_>,
    scopes: &mut Scopes<'_>,
    ctx: &mut EvalContext,
    engine: &mut Engine<'_>,
) -> SourceResult<Value> {
    match expr {
        Expr::Int(node)   => Ok(Value::Int(node.get())),
        Expr::Float(node) => Ok(Value::Float(node.get())),
        Expr::Str(node)   => Ok(Value::Str(EcoString::from(node.get()?))),
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
                    if ctx.flow.is_some() {
                        break;
                    }
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
            let body  = math::eval_math_content(scopes, ctx, engine, eq.body())?;
            // F-5a de-bake (P364, `f_fronteira_e1.md` §3a.9): a equação **não
            // baka** mais o gate. O `#set math.equation(numbering:)` vive **só na
            // chain** (`custom("equation.numbering")` no `Content::Styled` da
            // fatia-1). O consumidor lê o gate da chain e mantém `block &&
            // numbering` (só equações de bloco numeram, paridade vanilla). Fonte
            // única.
            let content = Content::equation(body, block);
            Ok(Value::Content(content))
        }

        Expr::Math(math) => {
            // Math node isolado (fora de Equation) — produzir como sequence.
            let content = math::eval_math_content(scopes, ctx, engine, math)?;
            Ok(Value::Content(content))
        }

        Expr::ModuleImport(i)  => modules::eval_module_import(i),
        Expr::ModuleInclude(i) => modules::eval_module_include(i, scopes, ctx, engine),

        // Passo 56 — referência cruzada: @nome → Content::Ref placeholder.
        Expr::Ref(ref_node) => {
            let name = ref_node.target().to_string();
            Ok(Value::Content(Content::reference(name)))
        }

        // Passo 56 — label em contexto de código; associação retroactiva em markup
        // acontece via SyntaxKind::Label. Em código, <label> é um valor de primeira
        // classe (P509) para query/locate.
        Expr::Label(label_node) => {
            let name = label_node.get().to_string();
            Ok(Value::Label(crate::entities::label::Label(name)))
        }

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

        // **P466** — dict literal `(a: 1, b: 2)` e `("a": 1)`.
        // Dicts com chaves keyed não-string (ex.: regex) são deixados como
        // `Value::None` para que callers especializados (ex.: `#set text(font:)`)
        // possam inspeccionar o AST directamente.
        Expr::Dict(dict) => {
            let mut map = indexmap::IndexMap::default();
            for item in dict.items() {
                match item {
                    crate::entities::ast::expr::DictItem::Named(named) => {
                        let key = named.name().as_str();
                        let value = eval_expr(named.expr(), scopes, ctx, engine)?;
                        map.insert(key.into(), value);
                    }
                    crate::entities::ast::expr::DictItem::Keyed(keyed) => {
                        let key_expr = keyed.key();
                        let key = match key_expr {
                            crate::entities::ast::expr::Expr::Str(node) => {
                                EcoString::from(node.get()?)
                            }
                            _ => return Ok(Value::None),
                        };
                        let value = eval_expr(keyed.expr(), scopes, ctx, engine)?;
                        map.insert(key, value);
                    }
                    crate::entities::ast::expr::DictItem::Spread(_) => {}
                }
            }
            Ok(Value::Dict(map))
        }

        // `(expr)` — parêntese de agrupamento. Expressão única dentro de
        // parênteses avalia para o valor da expressão. Passo 83.
        // (Um tuplo com um elemento requer a vírgula trailing: `(x,)`.)
        Expr::Parenthesized(paren) => eval_expr(paren.expr(), scopes, ctx, engine),

        // Passo 76 — literais numéricos com unidade (ex: 100pt, 1.5em).
        Expr::Numeric(num) => {
            use crate::entities::ast::expr::Unit;
            use crate::entities::layout_types::{Abs, Angle, Length};
            use crate::entities::rel::Rel;
            let (value, unit) = num.get();
            match unit {
                Unit::Pt      => Ok(Value::Length(Length { abs: Abs(value),            em: 0.0 })),
                Unit::Mm      => Ok(Value::Length(Length { abs: Abs(value * 2.8346),   em: 0.0 })),
                Unit::Cm      => Ok(Value::Length(Length { abs: Abs(value * 28.346),   em: 0.0 })),
                Unit::In      => Ok(Value::Length(Length { abs: Abs(value * 72.0),     em: 0.0 })),
                Unit::Em      => Ok(Value::Length(Length { abs: Abs(0.0),              em: value })),
                Unit::Deg     => Ok(Value::Angle(Angle::deg(value))),
                Unit::Rad     => Ok(Value::Angle(Angle::rad(value))),
                // P469 — percentual puro materializa comprimento relativo.
                Unit::Percent => Ok(Value::Relative(Rel::from_percent(value))),
                Unit::Fr      => Ok(Value::Fraction(value)),
            }
        }

        // **P506** — `context { body }` cria um bloco de delayed evaluation.
        // O parser expõe `Contextual` como sugar; construímos uma closure
        // sem argumentos que captura o scope actual e devolvemos
        // `Content::ContextBlock`.
        Expr::Contextual(node) => {
            let body = node.body().to_untyped().clone();
            let captured = std::sync::Arc::new(scopes.snapshot());
            let closure = Func::closure(ClosureRepr {
                name: None,
                params: Vec::new(),
                sink_name: None,
                body,
                captured,
            });
            let id = ctx.next_context_id();
            Ok(Value::Content(Content::ContextBlock(Arc::new(
                ContextBlockElem { id, closure },
            ))))
        }

        Expr::Escape(v) => Ok(Value::Str(ecow::EcoString::from(v.get()))),
        Expr::Shorthand(v) => Ok(Value::Str(ecow::EcoString::from(v.get()))),
        Expr::Linebreak(_) => Ok(Value::Content(Content::linebreak())),

        // P635 — controlo de fluxo: definir `FlowEvent` em `ctx.flow` e
        // devolver `Value::None`. O consumo (e a detecção de "fora de
        // contexto") é feito pelos ciclos, por `apply_closure` e pelo
        // entrypoint `eval_with_full_error`.
        Expr::LoopBreak(node) => {
            if ctx.flow.is_none() {
                ctx.flow = Some(FlowEvent::Break(node.span()));
            }
            Ok(Value::None)
        }
        Expr::LoopContinue(node) => {
            if ctx.flow.is_none() {
                ctx.flow = Some(FlowEvent::Continue(node.span()));
            }
            Ok(Value::None)
        }
        Expr::FuncReturn(node) => {
            let value = node.body().map(|body| eval_expr(body, scopes, ctx, engine)).transpose()?;
            if ctx.flow.is_none() {
                ctx.flow = Some(FlowEvent::Return(node.span(), value, false));
            }
            Ok(Value::None)
        }

        // Fronteira deliberada — variantes estritamente estruturais que não
        // entram no dispatcher normal (markup/math) ou ainda não migradas.
        Expr::Text(_)
        | Expr::Space(_)
        | Expr::Parbreak(_)
        | Expr::SmartQuote(_)
        | Expr::TermItem(_)
        | Expr::MathText(_)
        | Expr::MathIdent(_)
        | Expr::MathShorthand(_)
        | Expr::MathAlignPoint(_)
        | Expr::MathDelimited(_)
        | Expr::MathAttach(_)
        | Expr::MathPrimes(_)
        | Expr::MathFrac(_)
        | Expr::MathRoot(_) => Ok(Value::None),

        Expr::DestructAssignment(node) => Err(vec![SourceDiagnostic::error(
            node.span(),
            "destructuring assignment is not yet implemented",
        )]),
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
        native_ellipse, native_emph, native_figure, native_float, native_footnote, native_grid, native_grid_cell, native_grid_footer, native_grid_header, native_grid_hline, native_grid_vline, native_h, native_heading,
        native_hide, native_image, native_int, native_len, native_line, native_outline,
        native_counter, native_counter_at, native_counter_display, native_counter_final, native_counter_step, native_context, native_curve, native_curve_close, native_curve_cubic, native_curve_line, native_curve_move, native_curve_quad, native_eval, native_here, native_locate, native_lower, native_lorem, native_luma, native_measure, native_metadata, native_move, native_pad, native_pagebreak, native_place, native_polygon, native_query, native_regex, native_selector, native_state, native_state_at, native_state_display, native_state_final, native_state_update, native_state_update_with,
        native_asset, native_cmyk, native_colbreak, native_columns, native_document, native_hsl, native_hsv, native_label, native_linear_rgb, native_link, native_oklab, native_oklch, native_op, native_panic, native_quote, native_range, native_rect, native_repeat, native_replace, native_raw, native_repr, native_rgb, native_rotate,
        native_square, native_tiling,
        native_highlight, native_scale, native_skew, native_smallcaps, native_smartquote, native_stack, native_str, native_str_from_unicode, native_strike, native_stroke, native_strong, native_subscript, native_superscript, native_table, native_table_cell, native_table_footer, native_table_header, native_table_hline, native_table_vline, native_terms, native_type, native_underline, native_underover, native_overline, native_upper, native_v,
        native_ref,
        // P311b.3 — math style funcs.
        native_bb, native_bold, native_cal, native_frak, native_math_italic,
        native_mono, native_sans, native_scr, native_script, native_serif,
        native_sscript, native_upright,
        // P387 (ADR-0111) — data import.
        native_cbor, native_csv, native_json, native_read, native_toml, native_xml, native_yaml,
        // P403 — constructors stdlib para tipos primitivos L1.
        native_decimal, native_duration, native_version,
        // P470 — list/enum com marcadores configuráveis.
        native_list, native_enum,
        // P471 — módulo sym.
        build_sym_dict,
        // P472 — lof/lot.
        native_lof, native_lot,
        // P476 — módulo color.
        make_color_module,
    };
    let mut scope = Scope::new();
    scope.define("type",    Value::Func(Func::native("type",    native_type)));
    scope.define("repr",    Value::Func(Func::native("repr",    native_repr)));
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
    // P501 — `str` com namespace para métodos estáticos (`str.from-unicode`).
    {
        let mut str_ns = Scope::new();
        str_ns.define("from-unicode", Value::Func(Func::native("str.from-unicode", native_str_from_unicode)));
        scope.define("str", Value::Func(Func::native_with_namespace("str", native_str, std::sync::Arc::new(str_ns))));
    }
    // P504 — `int` com namespace para `int.min` / `int.max` (Typst 0.15.0).
    {
        let mut int_ns = Scope::new();
        int_ns.define("min", Value::Int(i64::MIN));
        int_ns.define("max", Value::Int(i64::MAX));
        scope.define(
            "int",
            Value::Func(Func::native_with_namespace("int", native_int, Arc::new(int_ns))),
        );
    }
    scope.define("float",   Value::Func(Func::native("float",   native_float)));
    // P403 — constructors stdlib para tipos primitivos L1 modelados em P399–P401.
    scope.define("decimal",  Value::Func(Func::native("decimal",  native_decimal)));
    scope.define("duration", Value::Func(Func::native("duration", native_duration)));
    scope.define("version",  Value::Func(Func::native("version",  native_version)));
    scope.define("heading",   Value::Func(Func::native("heading",   native_heading)));
    scope.define("outline",   Value::Func(Func::native("outline",   native_outline)));
    // **P472** — lof() e lot() como aliases de outline(target: "figures"/"tables").
    scope.define("lof",       Value::Func(Func::native("lof",       native_lof)));
    scope.define("lot",       Value::Func(Func::native("lot",       native_lot)));
    scope.define("strong",    Value::Func(Func::native("strong",    native_strong)));
    scope.define("emph",      Value::Func(Func::native("emph",      native_emph)));
    scope.define("raw",       Value::Func(Func::native("raw",       native_raw)));
    // P284 (ADR-0054 graded): text decoration — underline / strike /
    // overline. Cosméticos `stroke`/`offset`/`extent` opcionais; `evade`
    // e `background` scope-out per diagnóstico §A.1.
    scope.define("underline", Value::Func(Func::native("underline", native_underline)));
    scope.define("strike",    Value::Func(Func::native("strike",    native_strike)));
    scope.define("overline",  Value::Func(Func::native("overline",  native_overline)));
    // P408: smallcaps — variant + stdlib materializados; consumer em layout é
    // stub transparente (small caps real requer shaping OpenType, DEBT-53).
    scope.define("smallcaps", Value::Func(Func::native("smallcaps", native_smallcaps)));
    // P448: subscript e superscript via Content::Styled + Style.
    scope.define("sub",   Value::Func(Func::native("sub",   native_subscript)));
    scope.define("super", Value::Func(Func::native("super", native_superscript)));
    // P449: highlight via Content::Styled + Style::Highlight(fill).
    scope.define("highlight", Value::Func(Func::native("highlight", native_highlight)));
    // P287 (frente `P-smartquote`): função stdlib paralela ao markup `"..."`
    // P155. `alternative`/`quotes` scope-out per diagnóstico §A.2; `enabled:
    // false` emite glyph ASCII literal (paridade vanilla).
    scope.define("smartquote", Value::Func(Func::native("smartquote", native_smartquote)));
    scope.define("lorem",   Value::Func(Func::native("lorem",   native_lorem)));
    scope.define("regex",   Value::Func(Func::native("regex",   native_regex)));
    scope.define("figure",  Value::Func(Func::native("figure",  native_figure)));
    scope.define("image",   Value::Func(Func::native("image",   native_image)));
    // P396 — constructor `tiling(...)` (pattern fill).
    scope.define("tiling",  Value::Func(Func::native("tiling",  native_tiling)));
    // P387 (ADR-0111) — data import: read + 6 parsers. Decode L1 puro compõe
    // com L3 World::read_bytes. Paridade do Value de saída (ADR-0107).
    scope.define("read",    Value::Func(Func::native("read",    native_read)));
    scope.define("csv",     Value::Func(Func::native("csv",     native_csv)));
    scope.define("json",    Value::Func(Func::native("json",    native_json)));
    scope.define("yaml",    Value::Func(Func::native("yaml",    native_yaml)));
    scope.define("toml",    Value::Func(Func::native("toml",    native_toml)));
    scope.define("cbor",    Value::Func(Func::native("cbor",    native_cbor)));
    scope.define("xml",     Value::Func(Func::native("xml",     native_xml)));
    scope.define("rect",    Value::Func(Func::native("rect",    native_rect)));
    scope.define("square",  Value::Func(Func::native("square",  native_square)));
    scope.define("ellipse", Value::Func(Func::native("ellipse", native_ellipse)));
    scope.define("circle",  Value::Func(Func::native("circle",  native_circle)));
    scope.define("line",    Value::Func(Func::native("line",    native_line)));
    scope.define("polygon", Value::Func(Func::native("polygon", native_polygon)));
    // P293 (frente `P-curve-geometry`): activação posterior de
    // `PathItem::CubicTo` via stdlib novo. Reaplicação ADR-0099 para
    // `PathItem` (paralelo P285-P292 para `Style`). Hash `export.rs`
    // preservado pelo 10º passo consecutivo — emit já existe.
    // P513 — `curve` ganha namespace com move/line/cubic/quad/close.
    {
        let mut curve_namespace = Scope::new();
        curve_namespace.define("move",  Value::Func(Func::native("curve.move",  native_curve_move)));
        curve_namespace.define("line",  Value::Func(Func::native("curve.line",  native_curve_line)));
        curve_namespace.define("cubic", Value::Func(Func::native("curve.cubic", native_curve_cubic)));
        curve_namespace.define("quad",  Value::Func(Func::native("curve.quad",  native_curve_quad)));
        curve_namespace.define("close", Value::Func(Func::native("curve.close", native_curve_close)));
        scope.define(
            "curve",
            Value::Func(Func::native_with_namespace("curve", native_curve, Arc::new(curve_namespace))),
        );
    }
    // P512 — `grid` com namespace para cell/header/footer/hline/vline.
    {
        let mut grid_namespace = Scope::new();
        grid_namespace.define("cell",   Value::Func(Func::native("grid_cell",   native_grid_cell)));
        grid_namespace.define("header", Value::Func(Func::native("grid_header", native_grid_header)));
        grid_namespace.define("footer", Value::Func(Func::native("grid_footer", native_grid_footer)));
        grid_namespace.define("hline",  Value::Func(Func::native("grid_hline",  native_grid_hline)));
        grid_namespace.define("vline",  Value::Func(Func::native("grid_vline",  native_grid_vline)));
        scope.define(
            "grid",
            Value::Func(Func::native_with_namespace("grid", native_grid, Arc::new(grid_namespace))),
        );
    }
    // Lote F-2 S4/D4 (P335): `page(...)` função-forma legacy removida.
    scope.define("move",    Value::Func(Func::native("move",    native_move)));
    scope.define("rotate",  Value::Func(Func::native("rotate",  native_rotate)));
    scope.define("scale",   Value::Func(Func::native("scale",   native_scale)));
    // Passo 156F (ADR-0061 Fase 1, sub-passo 4): skew via matriz unificada.
    scope.define("skew",    Value::Func(Func::native("skew",    native_skew)));
    scope.define("align",   Value::Func(Func::native("align",   native_align)));
    scope.define("place",   Value::Func(Func::native("place",   native_place)));
    scope.define("assert",  Value::Func(Func::native("assert",  native_assert)));
    scope.define("panic",   Value::Func(Func::native("panic",   native_panic)));
    // P394: eval(source) — re-avalia string como markup Typst no contexto actual.
    scope.define("eval",    Value::Func(Func::native_with_engine("eval", native_eval)));
    // P169 (M9 sub-passo 1): metadata(value) — feature Introspection vanilla.
    scope.define("metadata", Value::Func(Func::native("metadata", native_metadata)));
    // P506: state(key, init) como valor de primeira classe.
    scope.define("state", Value::Func(Func::native("state", native_state)));
    // P506: counter(selector) como valor de primeira classe.
    scope.define("counter", Value::Func(Func::native("counter", native_counter)));
    // P506: context { expr } — delayed evaluation block.
    scope.define("context", Value::Func(Func::native("context", native_context)));
    // P171 (M9 sub-passo 3): state_update(key, value) — mantido como compatibilidade.
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
    // P504: selector(kind|func) — constrói selector como valor de primeira classe.
    scope.define("selector", Value::Func(Func::native("selector", native_selector)));
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
    // P397 — document metadata wrapper + asset placeholder.
    scope.define("document", Value::Func(Func::native("document", native_document)));
    scope.define("asset",    Value::Func(Func::native("asset",    native_asset)));
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
    // P493b — table com namespace anexado para table.header/footer/cell.
    // P512 — adiciona table.hline / table.vline.
    let mut table_namespace = Scope::new();
    table_namespace.define("header", Value::Func(Func::native("table_header", native_table_header)));
    table_namespace.define("footer", Value::Func(Func::native("table_footer", native_table_footer)));
    table_namespace.define("cell",   Value::Func(Func::native("table_cell",   native_table_cell)));
    table_namespace.define("hline",  Value::Func(Func::native("table_hline",  native_table_hline)));
    table_namespace.define("vline",  Value::Func(Func::native("table_vline",  native_table_vline)));
    scope.define(
        "table",
        Value::Func(Func::native_with_namespace("table", native_table, Arc::new(table_namespace))),
    );
    // Passo 157B (ADR-0060 Fase 2 sub-passo 2): table cell
    // (subset 5 fields; ADR-0064 Caso A para x/y, Caso C para
    // colspan/rowspan; placement diferido em DEBT-34e).
    // Mantém bindings flat como fallback não-regressão.
    scope.define("table_cell", Value::Func(Func::native("table_cell", native_table_cell)));
    // Passo 157C (ADR-0060 Fase 2 sub-passo 3 — fecha "table foundations"):
    // par simétrico TableHeader/TableFooter. ADR-0064 Caso D para
    // `repeat: bool` default true (primeira aplicação Caso D em
    // Model). Algoritmo de repetição em page breaks diferido em
    // DEBT-56 (refactor multi-region). Mantém bindings flat como fallback.
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
    scope.define("link",         Value::Func(Func::native("link",         native_link)));
    scope.define("label",        Value::Func(Func::native("label",        native_label)));
    scope.define("ref",          Value::Func(Func::native("ref",          native_ref)));
    // P470 — list/enum com marcadores configuráveis.
    scope.define("list", Value::Func(Func::native("list", native_list)));
    scope.define("enum", Value::Func(Func::native("enum", native_enum)));
    // P471 — módulo sym como Value::Dict de símbolos Unicode.
    scope.define("sym", build_sym_dict());
    scope.define("calc",    make_calc_module());
    // P476 — módulo `color` com operadores lighten/darken/mix/negate.
    scope.define("color", make_color_module());
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
    scope.define("start",   Value::Align(Align2D { h: Some(HAlign::Start),   v: None }));
    scope.define("end",     Value::Align(Align2D { h: Some(HAlign::End),     v: None }));
    scope.define("top",     Value::Align(Align2D { h: None, v: Some(VAlign::Top) }));
    scope.define("horizon", Value::Align(Align2D { h: None, v: Some(VAlign::Horizon) }));
    scope.define("bottom",  Value::Align(Align2D { h: None, v: Some(VAlign::Bottom) }));

    // Constantes de direcção (Passo 576).
    use crate::entities::dir::Dir;
    scope.define("ltr", Value::Dir(Dir::LTR));
    scope.define("rtl", Value::Dir(Dir::RTL));
    scope.define("ttb", Value::Dir(Dir::TTB));
    scope.define("btt", Value::Dir(Dir::BTT));

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
