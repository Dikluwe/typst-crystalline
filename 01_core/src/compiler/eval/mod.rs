//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/core/compiler/eval/mod.md
//! @prompt-hash db761ae7
//! @layer L1
//! @updated 2026-10-16
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

use std::cell::RefCell;
use std::collections::HashMap;
use std::sync::Arc;

use comemo::{Track, Tracked, TrackedMut};
use ecow::EcoString;
use hayagriva::citationberg::IndependentStyle;

use crate::compiler::eval::operators::error_formatting::vanilla_type_name;
use crate::compiler::layout::FixedMetrics;
use crate::compiler::scopes::Scopes;
use crate::contracts::world::{SysInputs, World};
#[cfg(test)]
use crate::entities::ast::expr::UnOp;
use crate::entities::ast::expr::{ArrayItem, BinOp, Expr};
use crate::entities::ast::markup::Label as AstLabel;
use crate::entities::ast::AstNode;
use crate::entities::content::Content;
#[cfg(test)]
use crate::entities::counter_update::CounterUpdate as CounterAction;
use crate::entities::document_info::DocumentInfo;
use crate::entities::elements::bibliography::BibliographyElem;
use crate::entities::elements::context_block::ContextBlockElem;
use crate::entities::engine::Engine;
use crate::entities::file_id::FileId;
use crate::entities::font_book::FontBook;
use crate::entities::func::{ClosureRepr, Func};
use crate::entities::module::Module;
use crate::entities::package_spec::PackageSpec;
use crate::entities::path::RootedPath;
use crate::entities::scope::Scope;
use crate::entities::show::{RuleId, ShowRule};
use crate::entities::source::Source;
use crate::entities::source_result::{SourceDiagnostic, SourceResult};
use crate::entities::span::Span;
use crate::entities::style_chain::StyleChain;
use crate::entities::syntax_kind::SyntaxKind;
use crate::entities::syntax_node::SyntaxNode;
#[cfg(test)]
use crate::entities::value::Type;
use crate::entities::value::Value;
use crate::entities::world_types::{
    Bytes, Datetime, FileResult, Font, Library, Route, Routines, Sink, Traced,
};

// Submódulos por domínio (Passo 96.1, ADR-0037).
pub(crate) mod cast;
pub(crate) mod font_dict;
mod math;
pub(crate) mod operators;
pub use cast::{cast_length, CastError};
pub(crate) mod flow;
pub use flow::FlowEvent;
pub(crate) mod call_dispatch;
pub(crate) mod closures;
mod control_flow;
pub use call_dispatch::apply_func;
pub(crate) mod bibliography;
pub mod bibtex;
pub(crate) mod bindings;
mod markup;
mod modules;
pub(crate) mod repr;
mod syntax_diagnostics;
pub(crate) mod table;

/// P1406 — entrypoint L1 para a fase pós-introspecção de outlines.
pub fn realize_outlines(
    content: Content,
    introspector: &crate::entities::introspector::TagIntrospector,
    ctx: &mut EvalContext,
    engine: &mut Engine<'_>,
) -> SourceResult<Content> {
    rules::realize_outlines(content, introspector, ctx, engine)
}

/// Representação morfológica pública e estreita de `Stroke` para consumidores
/// que possuem uma branch nominal do tipo. Não é fallback genérico de `Value`.
pub fn repr_stroke_value(stroke: &crate::entities::geometry::Stroke) -> String {
    repr::repr_stroke(stroke)
}

/// Representação morfológica pública usada por serializers estruturados.
///
/// A classificação entre valores estruturais e fallback textual pertence ao
/// formatter consumidor; esta fachada apenas expõe o `repr` total de L1.
pub fn repr_value_for_serialization(value: &crate::entities::value::Value) -> String {
    repr::repr_value(value)
}

/// **P1469** (escopo reduzido — contrato de linguagem) — expõe a mesma
/// formatação de diagnóstico que o braço `format.pdf` de `rules.rs` usa
/// para o conflito documental `standard`, reaproveitada pelo gate CLI-only
/// de `04_wiring/src/main.rs` (`wiring.md` P1469: "mesmo texto que
/// `compiler/eval/rules.rs` P1469 usa para o caso documental — é a mesma
/// função de validação, reaproveitada").
pub fn pdf_standards_conflict_diagnostic(
    conflict: crate::entities::document_format_options::PdfStandardsConflict,
) -> SourceDiagnostic {
    rules::pdf_standards_conflict_diagnostic(Span::detached(), conflict)
}
pub(crate) mod rules;
pub(crate) mod selector_matching;
pub(crate) mod show_rule_termination;

#[cfg(test)]
mod p1339_observer_local_tests {
    use super::*;

    struct MemoryWorld {
        library: Library,
        book: FontBook,
    }
    impl World for MemoryWorld {
        fn library(&self) -> &Library {
            &self.library
        }
        fn book(&self) -> &FontBook {
            &self.book
        }
        fn main(&self) -> FileId {
            FileId::from_raw(std::num::NonZeroU16::new(1).unwrap())
        }
        fn source(&self, _: FileId) -> FileResult<Source> {
            Err(crate::entities::world_types::FileError::NotFound)
        }
        fn file(&self, _: FileId) -> FileResult<Bytes> {
            Err(crate::entities::world_types::FileError::NotFound)
        }
        fn font(&self, _: usize) -> Option<Font> {
            None
        }
        fn today(
            &self,
            _: Option<crate::entities::duration::Duration>,
        ) -> Option<Datetime> {
            None
        }
    }

    #[test]
    fn context_read_replay_uses_captured_context() {
        let world = MemoryWorld { library: Library::new(), book: FontBook::new() };
        let metrics = FixedMetrics;
        let route = Route::root().with_id(world.main());
        let mut styles = StyleChain::default_chain();
        let mut rules: Arc<[ShowRule]> = Arc::from([]);
        let mut guards = Vec::new();
        let mut sink = Sink::new();
        let mut tracked = sink.track_mut();
        let mut engine = Engine {
            world: &world,
            font_metrics: &metrics,
            route: route.track(),
            styles: &mut styles,
            show_rules: &mut rules,
            active_guards: &mut guards,
            current_file: world.main(),
            sink: &mut tracked,
        };
        let mut ctx = EvalContext::new();
        let location = crate::entities::locator::Locator::new().next();
        ctx.in_context = true;
        ctx.current_location = Some(location);
        ctx.replace_context_read_file(Some(world.main()));
        let state = crate::entities::state::State {
            key: "read".into(),
            init: Box::new(Value::Int(1)),
        };
        let body = crate::compiler::stdlib::state::state_get(
            &state,
            &mut ctx,
            &mut engine,
            Span::detached(),
        );
        assert_eq!(
            body.observation_relation(&Ok(Value::Int(1))),
            ObservationRelation::Same
        );
        let mut candidate = crate::entities::introspector::TagIntrospector::empty();
        // A captured request must not be replayed under subsequently changed
        // ambient location/context flags.
        ctx.in_context = false;
        ctx.current_location = None;
        assert!(ctx.context_reads_valid_for(&candidate, &mut engine).unwrap());
        candidate.state.record_action(
            "read",
            &crate::entities::state_update::StateUpdate::Set(Box::new(Value::Int(2))),
            location,
        );
        assert!(!ctx.context_reads_valid_for(&candidate, &mut engine).unwrap());
        assert_eq!(ctx.context_reads.borrow().reads.len(), 1);
        assert_eq!(
            body.observation_relation(&Ok(Value::Int(1))),
            ObservationRelation::Same
        );
        assert!(comemo::internal::to_parts_mut_ref(engine.sink).0.is_empty());
    }

    #[test]
    fn observation_relation_ieee_and_recursive_fields() {
        let nan = Value::Float(f64::from_bits(0x7ff8000000001234));
        assert_eq!(nan.observation_relation(&nan.clone()), ObservationRelation::Same);
        assert_eq!(
            Value::Float(0.0).observation_relation(&Value::Float(-0.0)),
            ObservationRelation::Different
        );
        let a = Value::Content(Content::metadata(nan.clone()));
        let b = Value::Content(Content::metadata(Value::Float(f64::from_bits(
            0x7ff8000000001235,
        ))));
        assert_eq!(a.observation_relation(&a.clone()), ObservationRelation::Same);
        assert_eq!(a.observation_relation(&b), ObservationRelation::Different);
    }

    #[test]
    fn context_read_records_before_selection_and_preserves_error() {
        let ctx = EvalContext::new();
        let result = Err(vec![SourceDiagnostic::error(Span::detached(), "read failed")]);
        let returned = ctx.observe_context_read(
            ContextReadRequest::Here,
            Span::detached(),
            result.clone(),
        );
        assert_eq!(returned.observation_relation(&result), ObservationRelation::Same);
        assert!(!ctx.has_filtered_counter_reads());
        assert_eq!(ctx.context_reads.borrow().reads.len(), 1);
    }
}

mod context_reads;
mod expression_dispatch;
mod markup_stream;
mod observation;
pub(crate) use self::context_reads::ContextReadRequest;
use self::context_reads::ContextReads;
pub(crate) use self::expression_dispatch::eval_expr;
pub(crate) use self::markup_stream::{eval_markup, value_to_display_content};
use self::markup_stream::{eval_markup_body, eval_markup_impl};
#[cfg(test)]
pub(crate) use observation::{
    p1436_math_lr_observation_differs, p1437_math_mid_observation_differs,
    p1439_math_overbrace_observation_differs, p1440_math_overbracket_observation_differs,
    p1441_math_overline_observation_differs, p1442_math_overparen_observation_differs,
    p1443_math_overshell_observation_differs, p1444_math_primes_observation_differs,
    p1448_math_underbrace_observation_differs,
    p1449_math_underbracket_observation_differs,
    p1450_math_underparen_observation_differs, p1451_math_undershell_observation_differs,
};
use observation::{ObservationEq, ObservationRelation};

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
    context_reads: RefCell<ContextReads>,
    pub target: EvalTarget,
    pub features: crate::entities::compiler_features::Features,
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
    /// O entrypoint `eval` executa o programa comum uma vez e captura receitas
    /// para realização posterior. Default `true`.
    pub apply_show_rules: bool,
    pub(crate) defer_show_rules: bool,
    pub(crate) has_deferred_show_rules: bool,
    pub(crate) realizing_show_rules: bool,
    pub(crate) textual_frames: Vec<rules::textual::LexicalFrame>,
    pub(crate) recipe_protected_locations: Vec<crate::entities::location::Location>,
    pub(crate) recipe_math_depth: usize,
    pub(crate) recipe_text_edges: (bool, bool),
    pub(crate) occurrence_show_results: HashMap<
        (crate::entities::location::Location, crate::entities::show::RuleId),
        Option<Content>,
    >,
    pub(crate) occurrence_show_depth: usize,
    pub(crate) occurrence_show_path: Vec<Content>,
    pub(crate) realization_locator: crate::entities::locator::Locator,
    pub(crate) context_occurrence_slots:
        Option<crate::compiler::realize::context_occurrences::OccurrenceSlots>,

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

impl std::default::Default for EvalContext {
    fn default() -> Self {
        Self::new()
    }
}

impl EvalContext {
    pub fn new() -> Self {
        Self {
            context_reads: RefCell::new(ContextReads::new()),
            target: EvalTarget::Paged,
            features: crate::entities::compiler_features::Features::default(),
            loop_iterations: 0,
            max_loop_iterations: 1_000_000,
            next_rule_id: 0,
            introspector: crate::entities::introspector::TagIntrospector::empty(),
            current_location: None,
            full_error: false,
            apply_show_rules: true,
            defer_show_rules: false,
            has_deferred_show_rules: false,
            realizing_show_rules: false,
            textual_frames: Vec::new(),
            recipe_protected_locations: Vec::new(),
            recipe_math_depth: 0,
            recipe_text_edges: (false, false),
            occurrence_show_results: HashMap::new(),
            occurrence_show_depth: 0,
            occurrence_show_path: Vec::new(),
            realization_locator: crate::entities::locator::Locator::new(),
            context_occurrence_slots: None,
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EvalTarget {
    Paged,
    Html,
    Bundle,
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

/// Avalia uma expressão Typst isolada em modo code e scope fresco.
pub fn eval_expression(
    world: &dyn World,
    expression: &str,
) -> (SourceResult<Value>, Vec<SourceDiagnostic>) {
    eval_expression_with_features(
        world,
        expression,
        crate::entities::compiler_features::Features::default(),
    )
}

/// Disponibiliza a Source já analisada, sem modificar o World chamador.
struct ExpressionWorld<'a> {
    inner: &'a dyn World,
    source: &'a Source,
}

impl World for ExpressionWorld<'_> {
    fn library(&self) -> &Library {
        self.inner.library()
    }
    fn book(&self) -> &FontBook {
        self.inner.book()
    }
    fn main(&self) -> FileId {
        self.inner.main()
    }
    fn source(&self, id: FileId) -> FileResult<Source> {
        if id == self.source.id() {
            Ok(self.source.clone())
        } else {
            self.inner.source(id)
        }
    }
    fn file(&self, id: FileId) -> FileResult<Bytes> {
        self.inner.file(id)
    }
    fn resolve_path(&self, file: FileId, path: &str) -> Result<RootedPath, String> {
        self.inner.resolve_path(file, path)
    }
    fn read_path(&self, path: &RootedPath) -> Result<Arc<Vec<u8>>, String> {
        self.inner.read_path(path)
    }
    fn include_path(&self, path: &RootedPath) -> Result<Source, String> {
        self.inner.include_path(path)
    }
    fn read_bytes(&self, file: FileId, path: &str) -> Result<Arc<Vec<u8>>, String> {
        self.inner.read_bytes(file, path)
    }
    fn include_source(&self, file: FileId, path: &str) -> Result<Source, String> {
        self.inner.include_source(file, path)
    }
    fn resolve_package(&self, spec: &PackageSpec) -> Result<Source, String> {
        self.inner.resolve_package(spec)
    }
    fn inputs(&self) -> SysInputs {
        self.inner.inputs()
    }
    fn plugin_host(&self) -> Option<Arc<dyn crate::contracts::plugin_host::PluginHost>> {
        self.inner.plugin_host()
    }
    fn font(&self, index: usize) -> Option<Font> {
        self.inner.font(index)
    }
    fn candidates_for_char(&self, c: char) -> Vec<usize> {
        self.inner.candidates_for_char(c)
    }
    fn today(
        &self,
        offset: Option<crate::entities::duration::Duration>,
    ) -> Option<Datetime> {
        self.inner.today(offset)
    }
}

pub fn eval_expression_with_features(
    world: &dyn World,
    expression: &str,
    features: crate::entities::compiler_features::Features,
) -> (SourceResult<Value>, Vec<SourceDiagnostic>) {
    eval_expression_with_features_and_introspector(
        world,
        expression,
        features,
        &crate::entities::introspector::TagIntrospector::empty(),
    )
}

/// Avalia uma expressão com o snapshot contextual de um documento compilado.
pub fn eval_expression_with_features_and_introspector(
    world: &dyn World,
    expression: &str,
    features: crate::entities::compiler_features::Features,
    introspector: &crate::entities::introspector::TagIntrospector,
) -> (SourceResult<Value>, Vec<SourceDiagnostic>) {
    let source = Source::new_with_parser(
        world.main(),
        expression.to_string(),
        crate::compiler::parse::parse_code,
    );
    eval_source_with_features_and_introspector(world, &source, features, introspector)
}

/// Avalia uma Source code pronta, preservando sua identidade causal.
pub fn eval_source_with_features_and_introspector(
    world: &dyn World,
    source: &Source,
    features: crate::entities::compiler_features::Features,
    introspector: &crate::entities::introspector::TagIntrospector,
) -> (SourceResult<Value>, Vec<SourceDiagnostic>) {
    let inputs = world.inputs();
    let mut global = Scope::new();
    let stdlib = make_stdlib_with_features(&inputs, features);
    global.define("std", Value::Module(Module::stdlib(stdlib.clone())));
    for (name, binding) in stdlib.iter() {
        global.define(name, binding.value().clone());
    }
    for (name, value) in crate::compiler::stdlib::predefined_color_bindings() {
        global.define(name.as_str(), value);
    }
    global.define(
        "text",
        Value::Func(Func::native("text", crate::compiler::stdlib::native_text)),
    );
    let library = Library::with_global(global);
    let mut scopes = Scopes::new(Some(&library));
    let mut ctx = EvalContext::new();
    ctx.features = features;
    ctx.introspector = introspector.clone();
    let mut styles = StyleChain::default_chain();
    let mut show_rules: Arc<[ShowRule]> = Arc::from([]);
    let mut active_guards = Vec::new();
    let current_file = world.main();
    let expression_world = ExpressionWorld { inner: world, source };
    let route = Route::root().with_id(current_file);
    let fixed_metrics = FixedMetrics;
    let mut sink = Sink::new();
    let result = {
        let mut tracked_sink = sink.track_mut();
        let mut engine = Engine {
            world: &expression_world,
            font_metrics: &fixed_metrics,
            route: route.track(),
            styles: &mut styles,
            show_rules: &mut show_rules,
            active_guards: &mut active_guards,
            current_file,
            sink: &mut tracked_sink,
        };
        if let Err(errors) =
            syntax_diagnostics::validate_syntax(source.root(), |diagnostic| {
                engine.sink.warn_diagnostic(diagnostic);
            })
        {
            Err(errors)
        } else {
            (|| -> SourceResult<Value> {
                let mut last = Value::None;
                for child in source.root().children() {
                    if let Some(expr) = Expr::from_untyped(child) {
                        last = eval_expr(expr, &mut scopes, &mut ctx, &mut engine)?;
                    }
                }
                Ok(last)
            })()
        }
    };
    (result, sink.into_diagnostics())
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
#[expect(
    clippy::too_many_arguments,
    reason = "C003: Fachada pública de compatibilidade: registry e full_error são entradas explícitas; _routines/_traced/_route são transportados ao próximo wrapper, mas não consumidos na entrada completa atual"
)]
pub fn eval_with_full_error(
    _routines: &Routines,
    world: &dyn World,
    _traced: Tracked<Traced>,
    sink: TrackedMut<Sink>,
    _route: Tracked<Route>,
    source: &Source,
    // Lote F-3 inc-2: o threading registry→escopo que o F-1 deferiu. Elementos
    // de utilizador registados entram no escopo como funções (`#name(args)`).
    // Em produção é vazio até pacotes registarem elementos; testes injetam.
    registry: &crate::entities::element_registry::ElementRegistry,
    // P350c: flag de erro completo, já resolvida (origem `RunIntent`; L1 não lê env).
    full_error: bool,
) -> SourceResult<Module> {
    eval_with_full_error_and_target(
        _routines,
        world,
        _traced,
        sink,
        _route,
        source,
        registry,
        full_error,
        EvalTarget::Paged,
    )
}

#[expect(
    clippy::too_many_arguments,
    reason = "C004: Fachada pública que acrescenta target à anterior sem ativar features"
)]
pub fn eval_with_full_error_and_target(
    _routines: &Routines,
    world: &dyn World,
    _traced: Tracked<Traced>,
    sink: TrackedMut<Sink>,
    _route: Tracked<Route>,
    source: &Source,
    registry: &crate::entities::element_registry::ElementRegistry,
    full_error: bool,
    target: EvalTarget,
) -> SourceResult<Module> {
    eval_with_full_error_target_and_features(
        _routines,
        world,
        _traced,
        sink,
        _route,
        source,
        registry,
        full_error,
        target,
        crate::entities::compiler_features::Features::default(),
    )
}

#[expect(
    clippy::too_many_arguments,
    reason = "C005: Entrada pública completa de avaliação: World/Source/registry/sink e configuração são utilizados; _routines/_traced/_route não são lidos pelo corpo atual"
)]
pub fn eval_with_full_error_target_and_features(
    _routines: &Routines,
    world: &dyn World,
    _traced: Tracked<Traced>,
    mut sink: TrackedMut<Sink>,
    _route: Tracked<Route>,
    source: &Source,
    registry: &crate::entities::element_registry::ElementRegistry,
    full_error: bool,
    target: EvalTarget,
    features: crate::entities::compiler_features::Features,
) -> SourceResult<Module> {
    let root = source.root();

    // **P786a** — propagação integral de erros de sintaxe (revoga a
    // filtragem selectiva de P648/P649, que só propagava
    // `InvalidHexNumber`/`InvalidUnicodeCodepoint`). O survey empírico
    // deste passo (20 construções válidas, teste
    // `p786a_valid_constructs_without_error_nodes`) mostrou zero falsos
    // positivos no parser actual — `#` em código é erro genuíno (o
    // vanilla 0.15.0 também o rejeita, exit 1 + 2 hints). Warnings de
    // markup (`NoTextWithin*`, produzidos em `parse/markup.rs`) seguem
    // para o sink como warning e **não abortam** (T1 — paridade vanilla:
    // `warning: no text within stars` + hint, exit 0). Hints dos
    // `SyntaxError` são propagados em ambos os caminhos.
    syntax_diagnostics::validate_syntax(root, |diagnostic| {
        sink.warn_diagnostic(diagnostic);
    })?;

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

    // Execute common code once; show recipes retain their lexical environment
    // and are applied by the pure realization phase below.
    let inputs = world.inputs();
    struct EvaluatedPass {
        value: Value,
        scope: Scope,
        bibliography_styles: HashMap<u64, Arc<IndependentStyle>>,
        document_info: DocumentInfo,
        has_recipes: bool,
    }
    let run_pass = |apply_show_rules: bool,
                    pass_sink: &mut TrackedMut<Sink>|
     -> SourceResult<EvaluatedPass> {
        let mut ctx = EvalContext::new();
        ctx.target = target;
        ctx.features = features;
        ctx.full_error = full_error; // P350c: flag resolvida (default false via `eval`)
        ctx.apply_show_rules = apply_show_rules;
        ctx.defer_show_rules = true;

        // Route raiz com o FileId do ficheiro principal — primeira aplicação da
        // ADR-0036 (Passo 92), agora campo do Engine (ADR-0044, Passo 109).
        let route = Route::root().with_id(source.id());
        let mut styles = StyleChain::default_chain();
        let mut show_rules: Arc<[ShowRule]> = Arc::from([]);
        let mut active_guards: Vec<RuleId> = Vec::new();
        let current_file = source.id();

        // P772n — stdlib/cores/`std`/`text`/elementos de utilizador deixam de
        // ser achatados em `scopes.top` (o que os tornava mutáveis sem erro,
        // P772l §2.3). Constrói-se um `Scope` próprio, embrulhado em
        // `Library::with_global`, e passado como `base` — só alcançável por
        // leitura (`Scopes::get`), nunca por `Scopes::get_mut`. Ver
        // `rules/eval.md` §P772n.
        let mut global = Scope::new();
        let stdlib = make_stdlib_with_features(&inputs, features);
        // P709 — `std`: clone independente da stdlib, tirado ANTES de ser
        // espalhada em `global`, dá acesso à versão não-sombreada mesmo que
        // o documento redefina `length`/`calc`/etc. (paridade vanilla,
        // `Library::std = Binding::detached(global.clone())`). Sombreável
        // como qualquer outro nome (medido: `#let std = "oops"` funciona
        // no vanilla) — por isso um binding normal, sem mecanismo especial.
        global.define("std", Value::Module(Module::stdlib(stdlib.clone())));
        for (name, binding) in stdlib.iter() {
            global.define(name, binding.value().clone());
        }
        // P492 — cores predefinidas (red, blue, green, ...) como atalhos globais.
        for (name, value) in crate::compiler::stdlib::predefined_color_bindings() {
            global.define(name.as_str(), value);
        }
        // P492 — constructor `text(...)` no scope global (usado em show-rules, etc.).
        global.define(
            "text",
            Value::Func(crate::entities::func::Func::native(
                "text",
                crate::compiler::stdlib::native_text,
            )),
        );
        // Lote F-3 inc-2: elementos de utilizador registados entram no escopo como
        // funções (`#name(args)` → `Content::Dynamic` via o construtor do registry).
        // Mesmo escopo base que os nativos (document-wide); `#set`/`#show` léxicos
        // por cima seguem o padrão `local_styles` (F-2).
        for name in registry.names() {
            if let Some(ctor) = registry.ctor(name) {
                global.define(
                    name.as_str(),
                    Value::Func(Func::element(name.as_str(), ctor)),
                );
            }
        }
        let library = Library::with_global(global);
        let mut scopes = Scopes::new(Some(&library));

        // ADR-0044 (Passo 109): agregar os 8 campos num `Engine<'_>` e passar
        // `&mut engine` às funções internas em vez de 8 parâmetros individuais.
        // Reborrow do `sink` encurta o lifetime inner do `TrackedMut` ao da
        // stack frame local, permitindo que `Engine<'a>` tenha um único `'a`.
        let fixed_metrics = FixedMetrics;
        let mut local_sink = TrackedMut::reborrow_mut(&mut *pass_sink);
        let mut engine = Engine {
            world,
            font_metrics: &fixed_metrics,
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
        Ok(EvaluatedPass {
            value: content_val,
            scope: module_scope,
            bibliography_styles: ctx.bibliography_styles,
            document_info: ctx.document_info,
            has_recipes: ctx.has_deferred_show_rules,
        })
    };

    let EvaluatedPass {
        value,
        scope: module_scope,
        bibliography_styles,
        document_info,
        has_recipes,
    } = run_pass(true, &mut sink)?;
    let mut realization_introspector = None;
    let content = match value {
        Value::Content(content) => {
            if has_recipes
                || crate::compiler::realize::bare_math::needs_preparation(&content)
                || crate::compiler::realize::figure::needs_preparation(&content)
                || (target == EvalTarget::Paged
                    && crate::compiler::realize::reference_supplements::needs_preparation(
                        &content,
                    ))
            {
                let previous = crate::entities::introspector::TagIntrospector::empty();
                let (result, warnings) = crate::compiler::realize::realize_document(
                    &content, world, source, registry, &previous, full_error, target,
                    features,
                );
                for warning in warnings {
                    sink.warn_diagnostic(warning);
                }
                let realized = result?;
                realization_introspector = Some(realized.introspector);
                Some(realized.visual)
            } else {
                Some(content.wrap_page_columns())
            }
        }
        Value::None
        | Value::Bool(_)
        | Value::Int(_)
        | Value::Float(_)
        | Value::Str(_)
        | Value::Array(_)
        | Value::Dict(_)
        | Value::Module(_)
        | Value::Datetime(_)
        | Value::Func(_)
        | Value::LocatedContent(_, _)
        | Value::Auto
        | Value::Length(_)
        | Value::Relative(_)
        | Value::Ratio(_)
        | Value::Angle(_)
        | Value::Color(_)
        | Value::Stroke(_)
        | Value::Fraction(_)
        | Value::Align(_)
        | Value::Location(_)
        | Value::Gradient(_)
        | Value::Regex(_)
        | Value::Tiling(_)
        | Value::Bytes(_)
        | Value::Decimal(_)
        | Value::Duration(_)
        | Value::Version(_)
        | Value::Selector(_)
        | Value::Symbol(_)
        | Value::Args(_)
        | Value::State(_)
        | Value::Counter(_)
        | Value::Label(_)
        | Value::Dir(_)
        | Value::Path(_)
        | Value::Type(_)
        | Value::SpotColorant(_) => None,
    };
    let mut module = Module::new(source.id().into_raw().get().to_string(), module_scope);
    module.set_introspection_content(content.clone());
    module.set_content(content);
    if let Some(introspector) = realization_introspector {
        module.set_realization_introspector(introspector);
    }
    // P429 (DEBT-63): transportar styles resolvidos do eval para o Module,
    // de onde o pipeline os injectará no BibStore do TagIntrospector.
    module.set_bibliography_styles(bibliography_styles);
    // P536: transportar metadados do documento para o Module.
    module.set_document_info(document_info);
    Ok(module)
}

// eval_args, apply_func, apply_closure extraídos para eval/closures.rs (Passo 96.1).

// Math eval extraído para eval/math.rs no Passo 96.1 (ADR-0037).

// eval_let extraído para eval/bindings.rs (Passo 96.1).

// apply_show_rules e intercept_content extraídos para eval/rules.rs (Passo 96.1).

/// Constrói a stdlib: `type`, `len`, `range`, `rgb`, `luma`, `str`, `int`, `float`, `figure`, `assert`, `upper`, `lower`, `replace`, `calc`.
///
/// Passo 64 (DEBT-16): `native_figure` migrada do interceptador em eval.rs para cá.
/// O avaliador deixa de conhecer o nome "figure" — desacoplamento total.
#[cfg(test)]
fn make_stdlib(inputs: &SysInputs) -> Scope {
    make_stdlib_with_features(
        inputs,
        crate::entities::compiler_features::Features::default(),
    )
}

fn make_stdlib_with_features(
    inputs: &SysInputs,
    features: crate::entities::compiler_features::Features,
) -> Scope {
    crate::compiler::stdlib::make_stdlib_with_features(inputs, features)
}

// ── Auxiliares para intercepção de counter(...).method() ──────────────────

#[path = "math_accent_observation_tests.rs"]
mod math_accent_observation_tests;
#[path = "math_frac_evaluation_tests.rs"]
mod math_frac_evaluation_tests;
#[path = "math_limits_override_observation_tests.rs"]
mod math_limits_override_observation_tests;
#[path = "math_root_evaluation_tests.rs"]
mod math_root_evaluation_tests;

#[path = "math_cases_evaluation_tests.rs"]
mod math_cases_evaluation_tests;

mod math_matrix_evaluation_tests;
// Extrai o nome do contador de uma expressão `counter(key)`.
// extract_counter_key e eval_counter_method extraídos para eval/bindings.rs (Passo 96.1).

#[cfg(test)]
mod tests;
#[cfg(test)]
mod p1292_d_tests {
    use super::*;
    use crate::entities::args::Args;

    #[test]
    fn p1292_d_place_and_with_share_the_flush_namespace() {
        let stdlib = make_stdlib(&SysInputs::default());
        let Value::Func(place) = stdlib.get("place").unwrap() else {
            panic!("place must remain callable");
        };
        assert_eq!(place.name(), Some("place"));

        let namespace = place.namespace().expect("place namespace");
        let Value::Func(flush) = namespace.get("flush").unwrap() else {
            panic!("place.flush must be callable");
        };
        assert_eq!(flush.name(), Some("flush"));

        let with = place.clone().with(Args::positional(Vec::new()));
        assert!(with.namespace().unwrap().get("flush").is_some());
    }
}
#[cfg(test)]
mod p1293_d_tests {
    use super::*;
    use crate::entities::args::Args;

    fn assert_short_namespace_names(namespace_name: &str) {
        let stdlib = make_stdlib(&SysInputs::default());
        let Value::Func(container) = stdlib.get(namespace_name).unwrap() else {
            panic!("{namespace_name} must remain callable");
        };
        let namespace = container.namespace().expect("namespace must exist");
        let with_container = container.clone().with(Args::positional(Vec::new()));
        let with_namespace =
            with_container.namespace().expect("with must preserve namespace");

        for expected in ["cell", "header", "footer", "hline", "vline"] {
            let Value::Func(member) = namespace.get(expected).unwrap() else {
                panic!("{namespace_name}.{expected} must remain callable");
            };
            assert_eq!(member.name(), Some(expected));
            assert_eq!(
                member.clone().with(Args::positional(Vec::new())).name(),
                Some(expected),
            );

            let Value::Func(with_member) = with_namespace.get(expected).unwrap() else {
                panic!("{namespace_name}.with(...).{expected} must remain callable");
            };
            assert_eq!(with_member.name(), Some(expected));
        }
    }

    #[test]
    fn p1293_d_grid_namespace_uses_short_names_direct_and_with() {
        assert_short_namespace_names("grid");
    }

    #[test]
    fn p1293_d_table_namespace_uses_short_names_direct_and_with() {
        assert_short_namespace_names("table");
    }

    #[test]
    fn p1293_d_flat_aliases_keep_historical_names_and_lines_stay_namespaced() {
        let stdlib = make_stdlib(&SysInputs::default());
        for alias in [
            "grid_cell",
            "grid_header",
            "grid_footer",
            "table_cell",
            "table_header",
            "table_footer",
        ] {
            let Value::Func(function) = stdlib.get(alias).unwrap() else {
                panic!("{alias} must remain callable");
            };
            assert_eq!(function.name(), Some(alias));
        }

        for absent in ["grid_hline", "grid_vline", "table_hline", "table_vline"] {
            assert!(stdlib.get(absent).is_none(), "{absent} must stay namespaced");
        }
    }
}
#[cfg(test)]
pub(crate) use crate::compiler::eval::tests::eval_for_test;
// Re-export para o módulo de tests (que usa `use super::*;`).
#[cfg(test)]
pub(crate) use crate::compiler::eval::operators::{eval_binary_op, eval_unary_op};

/// P1471 — RED isolado (Agente B), a partir de `compiler/eval.md` P1471:
/// este owner só regista `pdf`/`html` no scope canónico via
/// `make_stdlib_with_features` (uma única composição,
/// `make_pdf_module`/`make_html_module()` chamadas exatamente uma vez
/// cada); a única mudança observável aqui é o kind do valor registado, de
/// `module` para `function`. Nenhuma implementação foi consultada.
#[cfg(test)]
mod p1471_tests {
    use super::*;
    use crate::entities::compiler_features::{Feature, Features};

    fn profiles() -> Vec<Features> {
        (0..4u8)
            .map(|mask| {
                let mut features = Features::empty();
                if mask & 1 != 0 {
                    features.enable(Feature::Html);
                }
                if mask & 2 != 0 {
                    features.enable(Feature::A11yExtras);
                }
                features
            })
            .collect()
    }

    /// Item 1 (nível de composição) — o scope canónico construído por
    /// `make_stdlib_with_features` regista `pdf` (sempre) e `html` (nos
    /// perfis com `Feature::Html`) como `Value::Func`, não `Value::Module`.
    /// RED nesta baseline: ambos ainda são `Value::Module`.
    #[test]
    fn p1471_scope_registers_pdf_and_html_as_func_across_profiles() {
        for features in profiles() {
            let stdlib = make_stdlib_with_features(&SysInputs::default(), features);

            let pdf = stdlib.get("pdf").expect("pdf deve estar registado no scope");
            let Value::Func(pdf_func) = pdf else {
                panic!(
                    "P1471: scope.pdf deve ser Value::Func em {features:?}, obtido {pdf:?}"
                );
            };
            assert_eq!(pdf_func.name(), Some("pdf"), "nome nativo de scope.pdf");

            if features.contains(Feature::Html) {
                let html =
                    stdlib.get("html").expect("html deve estar registado no scope");
                let Value::Func(html_func) = html else {
                    panic!(
                        "P1471: scope.html deve ser Value::Func em {features:?}, obtido {html:?}"
                    );
                };
                assert_eq!(html_func.name(), Some("html"), "nome nativo de scope.html");
            }
        }
    }
}
