//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/infra/pipeline.md
//! @prompt-hash 9a0675ca
//! @layer L3
//! @updated 2026-09-21
//!
//! Pipeline de compilação — orquestra eval → introspect → layout
//! → export_pdf, gere o boilerplate `comemo` (Route, Sink, Traced,
//! Routines) e drena warnings.
//!
//! Materializado no Passo 113 (ADR-0046) a partir de helpers
//! test-only em `integration_tests.rs`. API pública consumível
//! pelo 04_wiring (CLI) e por testes.

#![allow(deprecated)] // P483 — FrameItem::Text fallback path legítimo
use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use std::time::Instant;

use comemo::{Track, TrackedMut};

use typst_core::contracts::world::World;
use typst_core::entities::args::Args;
use typst_core::entities::content::Content;
use typst_core::entities::document_format_options::{
    DocumentFormatOptions, PageRanges, PdfAccessibility, PdfArchival, PdfStandards,
    PdfTagged,
};
use typst_core::entities::element_kind::ElementKind;
use typst_core::entities::elements::context_block::ContextBlockElem;
use typst_core::entities::engine::Engine;
use typst_core::entities::font_book::{FontBook, FontVariant};
use typst_core::entities::font_list::FontList;
use typst_core::entities::font_variations::FontVariations;
use typst_core::entities::introspector::Introspector;
use typst_core::entities::layout_types::{FrameItem, Page, PagedDocument};

use crate::font_variant::{
    axis_variations_for_font_variant, instantiate_variable_font, is_variable_font,
    merge_explicit_variations, text_style_to_font_variant,
    variable_font_instancer_available,
};
use typst_core::compiler::eval::{
    apply_func, eval_expression, eval_expression_with_features,
    eval_expression_with_features_and_introspector, eval_with_full_error,
    eval_with_full_error_and_target, eval_with_full_error_target_and_features,
    realize_outlines, EvalContext, EvalTarget,
};
use typst_core::compiler::introspect::introspect_with_introspector;
use typst_core::compiler::layout::{
    collect_document_format_options, layout_with_introspector_and_metrics_and_callbacks,
    LayoutPassOutcome, ParLineCallbackRequest, ParLineCallbackResolution,
    SealedParLineCallbacks,
};
use typst_core::compiler::math::layout::callbacks::{
    MathCancelResolution, SealedMathCallbacks,
};
use typst_core::compiler::scopes::Scopes;
use typst_core::compiler::stdlib::{realize_numbering, value_to_content};
use typst_core::entities::module::Module;
use typst_core::entities::show::ShowRule;
use typst_core::entities::sink::Sink as TypstSink;
use typst_core::entities::source::Source;
use typst_core::entities::source_result::{SourceDiagnostic, SourceResult};
use typst_core::entities::span::Span;
use typst_core::entities::style_chain::StyleChain;
use typst_core::entities::value::Value;
use typst_core::entities::world_types::{Route, Routines, Sink, Traced};

use crate::export::{
    export_pdf_multifont_and_timings_and_document_id_and_tags_and_serialization_and_pages_and_standard,
    export_pdf_with_document_id_and_tags_and_serialization_and_pages_and_standard,
    export_pdf_with_font_and_timings_and_document_id_and_tags_and_serialization_and_pages_and_standard,
    export_png, export_png_with_fonts, export_svg_with_context,
    export_svg_with_fonts_and_contexts, FontKey, GlyphFontRequest, PdfSerializationMode,
    PdfTags, StreamMode, SvgDestinationContext, SvgGlyphFontContext,
};
use crate::font_metrics::FallbackFontMetrics;
use crate::image_sizer::ImageSizeImageSizer;

mod context_stabilization;

/// Avalia uma expressão code isolada e devolve o valor + warnings crus.
pub fn eval_expression_with_sink(
    world: &dyn World,
    expression: &str,
) -> (SourceResult<Value>, Vec<SourceDiagnostic>) {
    eval_expression(world, expression)
}

pub fn eval_expression_with_sink_features(
    world: &dyn World,
    expression: &str,
    features: typst_core::entities::compiler_features::Features,
) -> (SourceResult<Value>, Vec<SourceDiagnostic>) {
    eval_expression_with_features(world, expression, features)
}

/// Compila um documento paginado e avalia uma expressão contra seu contexto.
pub fn eval_expression_with_document_features(
    world: &dyn World,
    source: &Source,
    expression: &str,
    features: typst_core::entities::compiler_features::Features,
) -> (SourceResult<Value>, Vec<SourceDiagnostic>) {
    let mut timings = Timings::default();
    let (compiled, mut warnings) =
        compile_to_paged_document_full_error_and_features_with_introspector(
            world,
            source,
            false,
            features,
            &mut timings,
        );
    let (_, introspector) = match compiled {
        Ok(pair) => pair,
        Err(errors) => return (Err(errors), warnings),
    };
    let (result, expression_warnings) = eval_expression_with_features_and_introspector(
        world,
        expression,
        features,
        &introspector,
    );
    warnings.extend(expression_warnings);
    (result, warnings)
}

/// Avalia `source` contra `world` e devolve `(Module, warnings)`.
///
/// Boilerplate `comemo` (Routines, Traced, Sink, Route) gerido
/// internamente. Warnings drenados do `Sink` após retorno.
///
/// Para compilar directamente a PDF, usar `compile_to_pdf_bytes`.
pub fn eval_to_module_with_sink(
    world: &dyn World,
    source: &Source,
) -> (SourceResult<Module>, Vec<SourceDiagnostic>) {
    eval_to_module_with_sink_full_error(world, source, false)
}

/// Internal variant that wires the `full_error` flag down to L1.
///
/// The public API stays on [`eval_to_module_with_sink`] (default `false`) so
/// callers are not exposed to the flag. This helper exists only to support
/// the internal path from `RunIntent.full_error` (DEBT-59 / P428).
fn eval_to_module_with_sink_full_error(
    world: &dyn World,
    source: &Source,
    full_error: bool,
) -> (SourceResult<Module>, Vec<SourceDiagnostic>) {
    eval_to_module_with_sink_target(world, source, full_error, EvalTarget::Paged)
}

fn eval_to_module_with_sink_target(
    world: &dyn World,
    source: &Source,
    full_error: bool,
    target: EvalTarget,
) -> (SourceResult<Module>, Vec<SourceDiagnostic>) {
    eval_to_module_with_sink_target_features(
        world,
        source,
        full_error,
        target,
        typst_core::entities::compiler_features::Features::default(),
    )
}

fn eval_to_module_with_sink_target_features(
    world: &dyn World,
    source: &Source,
    full_error: bool,
    target: EvalTarget,
    features: typst_core::entities::compiler_features::Features,
) -> (SourceResult<Module>, Vec<SourceDiagnostic>) {
    let routines = Routines::new();
    let traced = Traced::default();
    let mut sink = Sink::new();
    let route = Route::root();
    // Lote F-3 inc-2: registry de elementos de utilizador. Vazio até pacotes
    // registarem elementos (não há elemento de utilizador em produção ainda).
    let registry = typst_core::entities::element_registry::ElementRegistry::new();
    let result = eval_with_full_error_target_and_features(
        &routines,
        world,
        traced.track(),
        sink.track_mut(),
        route.track(),
        source,
        &registry,
        full_error,
        target,
        features,
    );
    let warnings = sink.into_diagnostics();
    (result, warnings)
}

/// Pipeline semântico HTML, sem layout paginado.
pub fn compile_to_html_string(
    world: &dyn World,
    source: &Source,
) -> (Result<String, Vec<SourceDiagnostic>>, Vec<SourceDiagnostic>) {
    compile_to_html_string_with_features_and_serialization(
        world,
        source,
        typst_core::entities::compiler_features::Features::default(),
        crate::export::HtmlSerializationMode::Crystalline,
    )
}

pub fn compile_to_html_string_with_features(
    world: &dyn World,
    source: &Source,
    features: typst_core::entities::compiler_features::Features,
) -> (Result<String, Vec<SourceDiagnostic>>, Vec<SourceDiagnostic>) {
    compile_to_html_string_with_features_and_serialization(
        world,
        source,
        features,
        crate::export::HtmlSerializationMode::Crystalline,
    )
}

pub fn compile_to_html_string_with_features_and_serialization(
    world: &dyn World,
    source: &Source,
    features: typst_core::entities::compiler_features::Features,
    mode: crate::export::HtmlSerializationMode,
) -> (Result<String, Vec<SourceDiagnostic>>, Vec<SourceDiagnostic>) {
    compile_to_html_string_with_features_and_serialization_and_pretty_override(
        world, source, features, mode, None,
    )
}

pub fn compile_to_html_string_with_features_and_serialization_and_pretty(
    world: &dyn World,
    source: &Source,
    features: typst_core::entities::compiler_features::Features,
    mode: crate::export::HtmlSerializationMode,
    pretty: bool,
) -> (Result<String, Vec<SourceDiagnostic>>, Vec<SourceDiagnostic>) {
    compile_to_html_string_with_features_and_serialization_and_pretty_override(
        world,
        source,
        features,
        mode,
        Some(pretty),
    )
}

fn resolve_html_pretty(
    pretty_override: Option<bool>,
    options: &DocumentFormatOptions,
) -> bool {
    pretty_override.unwrap_or(options.html.pretty)
}

pub fn compile_to_html_string_with_features_and_serialization_and_pretty_override(
    world: &dyn World,
    source: &Source,
    features: typst_core::entities::compiler_features::Features,
    mode: crate::export::HtmlSerializationMode,
    pretty_override: Option<bool>,
) -> (Result<String, Vec<SourceDiagnostic>>, Vec<SourceDiagnostic>) {
    if !features.contains(typst_core::entities::compiler_features::Feature::Html) {
        return (
            Err(vec![SourceDiagnostic::error(
                Span::detached(),
                "html export is only available when `--features html` is passed",
            )
            .with_hint("html export is under active development and incomplete")
            .with_hint(
                "see https://github.com/typst/typst/issues/5512 for more information",
            )]),
            vec![],
        );
    }
    let (result, mut warnings) = eval_to_module_with_sink_target_features(
        world,
        source,
        false,
        EvalTarget::Html,
        features,
    );
    let html = result.and_then(|module| {
        let content = module.content().cloned().unwrap_or(Content::Empty);
        let format_options = collect_document_format_options(&content)?;
        let pretty = resolve_html_pretty(pretty_override, &format_options);
        let intr = introspect_with_introspector(&content);
        let (prepared, contextual_warnings) = context_stabilization::prepare_html(
            content, &intr, world, source, false, features,
        );
        warnings.extend(contextual_warnings);
        let prepared = prepared?;
        let metrics = FallbackFontMetrics::new(world);
        let resolve_frame = |elem: &typst_core::entities::html::HtmlFrameElem,
                             chain: &StyleChain|
         -> Result<String, SourceDiagnostic> {
            let mut contains_html = false;
            elem.body
                .map_content(&mut |node| {
                    contains_html |= matches!(node, Content::HtmlElem(_));
                    Ok(None)
                })
                .expect("html.frame support scan is total");
            if contains_html {
                return Err(SourceDiagnostic::error(
                    Span::detached(),
                    "html.frame body contains content unsupported by paged layout",
                ));
            }
            let page = typst_core::compiler::layout::layout_html_frame(
                elem,
                chain,
                metrics.clone(),
                ImageSizeImageSizer,
            );
            let doc = PagedDocument::new(vec![page.clone()]);
            let fonts = resolve_and_instantiate_fonts(&doc, world)?;
            let glyph_fonts = svg_glyph_font_context_for_page(&page, world);
            let options = crate::export::SvgOptions { pretty, ..Default::default() };
            Ok(options.export_html_frame_with_fonts(&page, &fonts, &glyph_fonts))
        };
        mode.export_with_resolved_frames(&prepared, pretty, &resolve_frame)
            .map_err(|error| vec![error])
    });
    (html, warnings)
}

#[cfg(test)]
mod p1465_tests {
    use super::*;

    #[test]
    fn html_pretty_precedence_is_override_then_document_then_false() {
        let mut options = DocumentFormatOptions::default();
        assert!(!resolve_html_pretty(None, &options));

        options.html.pretty = true;
        assert!(resolve_html_pretty(None, &options));
        assert!(!resolve_html_pretty(Some(false), &options));

        options.html.pretty = false;
        assert!(resolve_html_pretty(Some(true), &options));
    }
}

/// **P506** — Expande todos os `Content::ContextBlock` do documento
/// avaliando as closures num contexto com `in_context = true` e a
/// localização capturada pelo walk de introspecção.
pub fn expand_context_blocks(
    content: Content,
    intr: &typst_core::entities::introspector::TagIntrospector,
    world: &dyn World,
    source: &Source,
) -> SourceResult<Content> {
    context_stabilization::expand(content, intr, world, source)
}

/// **P844** (achado #54 de P831) — Expansão de ContextBlocks **+
/// re-introspecção** do conteúdo expandido.
///
/// Sonda (medição em `temp/p844/`): o `ContextBlock` é locatable no
/// walk de introspecção, mas `substitute_context_blocks` troca-o por
/// conteúdo não-locatable. O walk de layout tem Locator próprio
/// (invariante P185C "sincronizado-por-construção" com o walk de
/// introspecção); com um locatable removido, as Locations atribuídas
/// no layout desfasam do introspector pré-expansão e
/// `CounterRegistry::value_at` devolve o snapshot anterior — daí
/// `= Um / #context 1 / = Dois / = Três` numerar `1.|1.|2.` em vez de
/// `1.|2.|3.`. Probes: `#metadata(1)` / `#counter(heading).step()`
/// (locatable que PERMANECE) não dessincronizam.
///
/// A re-introspecção reconstrói o introspector a partir do conteúdo
/// que o layout vai efectivamente percorrer, restaurando a sincronia
/// por construção. Nota: styles CSL injectados no `BibStore`
/// (pipeline.rs, P429) não sobrevivem à reconstrução — o caller
/// re-injecta.
pub fn expand_context_blocks_and_reintrospect(
    content: Content,
    intr: &typst_core::entities::introspector::TagIntrospector,
    world: &dyn World,
    source: &Source,
) -> SourceResult<(Content, typst_core::entities::introspector::TagIntrospector)> {
    let expanded = expand_context_blocks(content, intr, world, source)?;
    let intr2 = typst_core::compiler::introspect::introspect_with_introspector(&expanded);
    Ok((expanded, intr2))
}

/// Executa os pós-processadores de introspecção que precisam de Engine sobre
/// o conteúdo final, já com ContextBlocks expandidos e Locations estáveis.
fn introspect_with_runtime_for_pipeline(
    content: &Content,
    world: &dyn World,
    source: &Source,
) -> (
    SourceResult<typst_core::entities::introspector::TagIntrospector>,
    Vec<SourceDiagnostic>,
) {
    let font_metrics = FallbackFontMetrics::new(world);
    let mut ctx = EvalContext::new();
    let mut styles = StyleChain::default_chain();
    let mut show_rules: Arc<[ShowRule]> = Arc::from([]);
    let mut active_guards = Vec::new();
    let mut sink = TypstSink::new();
    let route = Route::root().with_id(source.id());
    let result = {
        let mut tracked_sink = sink.track_mut();
        let mut local_sink = TrackedMut::reborrow_mut(&mut tracked_sink);
        let mut engine = Engine {
            world,
            font_metrics: &font_metrics,
            route: route.track(),
            styles: &mut styles,
            show_rules: &mut show_rules,
            active_guards: &mut active_guards,
            current_file: source.id(),
            sink: &mut local_sink,
        };
        typst_core::compiler::introspect::introspect_with_runtime(
            content,
            &mut engine,
            &mut ctx,
        )
    };
    (result, sink.into_diagnostics())
}

/// P1406 — realiza outlines com Engine em L3, mantendo toda a decisão
/// semântica em L1. Um segundo passe focal só ocorre quando os carriers de
/// prefixo revelam máximos novos.
fn realize_outlines_for_pipeline(
    content: &Content,
    introspector: &typst_core::entities::introspector::TagIntrospector,
    world: &dyn World,
    source: &Source,
) -> (
    SourceResult<(Content, typst_core::entities::introspector::TagIntrospector)>,
    Vec<SourceDiagnostic>,
) {
    let run_once = |input: &typst_core::entities::introspector::TagIntrospector| {
        let font_metrics = FallbackFontMetrics::new(world);
        let mut ctx = EvalContext::new();
        ctx.in_context = true;
        ctx.introspector = input.clone();
        let mut styles = StyleChain::default_chain();
        let mut show_rules: Arc<[ShowRule]> = Arc::from([]);
        let mut active_guards = Vec::new();
        let mut sink = TypstSink::new();
        let route = Route::root().with_id(source.id());
        let result = {
            let mut tracked_sink = sink.track_mut();
            let mut local_sink = TrackedMut::reborrow_mut(&mut tracked_sink);
            let mut engine = Engine {
                world,
                font_metrics: &font_metrics,
                route: route.track(),
                styles: &mut styles,
                show_rules: &mut show_rules,
                active_guards: &mut active_guards,
                current_file: source.id(),
                sink: &mut local_sink,
            };
            realize_outlines(content.clone(), input, &mut ctx, &mut engine)
        };
        (result, sink.into_diagnostics())
    };

    let (first, mut warnings) = run_once(introspector);
    let first = match first {
        Ok(content) => content,
        Err(errors) => return (Err(errors), warnings),
    };
    let observed = introspect_with_introspector(&first).outline_prefix_widths;
    let mut stabilized = introspector.clone();
    if observed != stabilized.outline_prefix_widths {
        stabilized.outline_prefix_widths = observed;
        let (second, second_warnings) = run_once(&stabilized);
        warnings.extend(second_warnings);
        match second {
            Ok(content) => (Ok((content, stabilized)), warnings),
            Err(errors) => (Err(errors), warnings),
        }
    } else {
        (Ok((first, stabilized)), warnings)
    }
}

/// P1411 — expande blocos contextuais que só passam a existir durante a
/// realização pós-introspecção de outlines. A topologia realizada fornece
/// apenas as Locations desses produtores; páginas, posições e demais stores
/// continuam vindo do snapshot paginado causal recebido.
fn expand_realized_outline_contexts(
    content: Content,
    input: &Content,
    introspector: &typst_core::entities::introspector::TagIntrospector,
    world: &dyn World,
    source: &Source,
) -> SourceResult<Content> {
    let mut existing = Vec::new();
    let _ = input.map_content(&mut |node| {
        if let Content::ContextBlock(elem) = node {
            existing.push(elem.clone());
        }
        Ok(None)
    });
    let mut has_new_context = false;
    let _ = content.map_content(&mut |node| {
        if let Content::ContextBlock(elem) = node {
            has_new_context |= !existing.iter().any(|old| Arc::ptr_eq(old, elem));
        }
        Ok(None)
    });
    // Sem produtores introduzidos pela realização de outlines, a passagem
    // seria um no-op semântico. Preservar os marcadores existentes é porém
    // essencial: o layout atribui-lhes a Position usada por `here()` na
    // iteração seguinte de `location.page-numbering()`.
    if !has_new_context {
        return Ok(content);
    }
    let content = content.map_content(&mut |node| {
        let Content::ContextBlock(elem) = node else { return Ok(None) };
        Ok(existing
            .iter()
            .any(|old| Arc::ptr_eq(old, elem))
            .then_some(Content::Empty))
    })?;
    let realized_topology = introspect_with_introspector(&content);
    let mut contextual_intr = introspector.clone();
    contextual_intr.context_block_locations = realized_topology.context_block_locations;
    context_stabilization::expand_legacy(content, &contextual_intr, world, source)
}

/// **P1037** — recolhe **todos** os `ContextBlock` da árvore, cada um com a
/// `StyleChain` da sua posição.
///
/// A descida é exaustiva: os containers que não afectam a cadeia são
/// percorridos por `Content::map_content` (L1), o `match` exaustivo que é a
/// fonte única da forma da árvore. Antes de P1037 este walk reenumerava
/// containers aqui em L3 (Sequence/Styled/Strong/Emph/Heading) e um
/// `ContextBlock` fora dessa lista — `#box[…]`, item de lista, célula de
/// grid — nunca era recolhido; `substitute_context_blocks` trocava-o então
/// por `Content::Empty`, produzindo resultado silenciosamente vazio. Ver
/// `00_nucleo/prompts/infra/pipeline.md` §P1037.
fn collect_context_blocks(
    content: &Content,
    chain: &StyleChain,
) -> HashMap<u64, (Arc<ContextBlockElem>, StyleChain)> {
    let mut map = HashMap::new();
    collect_context_blocks_into(content, chain, &mut map);
    map
}

fn collect_context_blocks_into(
    content: &Content,
    chain: &StyleChain,
    map: &mut HashMap<u64, (Arc<ContextBlockElem>, StyleChain)>,
) {
    match content {
        Content::ContextBlock(elem) => {
            map.insert(elem.id, (elem.clone(), chain.clone()));
        }
        // P711 — acumula o delta na cadeia ao descer, em vez de o descartar;
        // é esta cadeia que `expand_context_blocks` usa como `engine.styles`.
        Content::Styled(inner, styles) => {
            let inner_chain = chain.push_styles(styles);
            collect_context_blocks_into(inner, &inner_chain, map);
        }
        other => {
            // Descida exaustiva delegada a L1. `map_content` é usado como
            // **visitante**: o transform devolve sempre `None`, logo a árvore
            // devolvida é descartada sem alteração.
            //
            // `map_content` é bottom-up (filhos antes do nó), por isso um
            // `Styled` mais abaixo é visto **depois** dos seus descendentes:
            // os blocos lá dentro já foram inseridos com a cadeia exterior, e
            // a reentrada em `collect_context_blocks_into` reinsere-os com a
            // cadeia correcta — o `insert` posterior sobrepõe-se, que é o
            // resultado pretendido.
            let _ = other.map_content(&mut |node| {
                match node {
                    Content::ContextBlock(e) => {
                        map.insert(e.id, (e.clone(), chain.clone()));
                    }
                    Content::Styled(inner, styles) => {
                        let inner_chain = chain.push_styles(styles);
                        collect_context_blocks_into(inner, &inner_chain, map);
                    }
                    _ => {} // neutro: N16[β] — Content não-ContextBlock/Styled é ignorado na colecta de blocos de contexto
                }
                Ok(None)
            });
        }
    }
}

/// **P1037** — substitui cada `ContextBlock` pelo conteúdo resolvido, em
/// **toda** a árvore. Simétrico de `collect_context_blocks`: a descida é a de
/// `Content::map_content` (L1), não uma reenumeração de containers em L3.
///
/// Um `id` ausente de `resolved` continua a dar `Content::Empty` — mas isso
/// deixa de acontecer por o walk não ter chegado ao bloco, que era a causa do
/// defeito de P1037.
fn substitute_context_blocks(
    content: Content,
    resolved: &HashMap<u64, Content>,
) -> Content {
    content
        .map_content(&mut |node| {
            Ok(match node {
                Content::ContextBlock(elem) => {
                    // Preserva um marcador locatable e zero-size antes do
                    // resultado. Assim o mesmo Location recebe Position no
                    // layout e pode consultar PageStore na passagem seguinte.
                    let result =
                        resolved.get(&elem.id).cloned().unwrap_or(Content::Empty);
                    Some(Content::Sequence(
                        vec![Content::ContextBlock(elem.clone()), result].into(),
                    ))
                }
                _ => None,
            })
        })
        .unwrap_or(content)
}

/// Tempos das fases do pipeline cristalino, em milissegundos.
///
/// P507 — instrumentação de benchmark; não altera a semântica da compilação.
/// P518 — adiciona `shape_ms` e `subset_ms` para decompor o estágio de
/// produção real (shaping + subsetting + render PDF).
#[derive(Debug, Clone, Default)]
pub struct Timings {
    /// Parse do `.typ` → `Source` (medido pelo caller, tipicamente L2/L4).
    pub parse_ms: f64,
    /// Eval com passagem dupla (P498): introspection + layout.
    pub eval_ms: f64,
    /// Construção do `TagIntrospector` a partir do conteúdo original.
    pub introspect_ms: f64,
    /// Expansão pós-introspecção de `Content::ContextBlock` (P506).
    pub expand_context_ms: f64,
    /// Layout engine → `PagedDocument`.
    pub layout_ms: f64,
    /// Shaping: Text → TextShaped (rustybuzz + font fallback).
    pub shape_ms: f64,
    /// Subsetting TrueType/OpenType para embed no PDF.
    pub subset_ms: f64,
    /// Export PDF após subsetting.
    pub render_ms: f64,
    /// Tempo total do pipeline (sem o parse).
    pub total_ms: f64,
}

/// Pipeline completo `Source` → bytes PDF.
///
/// Retorna `(Ok(pdf_bytes), warnings)` em sucesso ou
/// `(Err(errors), warnings)` em falha. Warnings são sempre
/// devolvidos, mesmo em erro — caller decide se os imprime.
///
/// Dispatch font-aware (Passo 140B, ADR-0055): se o documento
/// contém `#set text(font: ...)` e a primeira família resolve em
/// `world.book()`, embute a font via `export_pdf_with_font`.
/// Caso contrário, fallback `export_pdf` (Helvetica Type1). MVP
/// single-font per document — spans com font diferente após a
/// primeira são silenciosamente ignorados.
pub fn compile_to_pdf_bytes(
    world: &dyn World,
    source: &Source,
    stream_mode: StreamMode,
    pdf_tags: PdfTags,
) -> (Result<Vec<u8>, Vec<SourceDiagnostic>>, Vec<SourceDiagnostic>) {
    compile_to_pdf_bytes_full_error(world, source, false, stream_mode, pdf_tags)
}

/// Internal variant that wires the `full_error` flag down to L1.
///
/// The stable public API stays on [`compile_to_pdf_bytes`] (default `false`).
/// This function exists only to support the internal path from
/// `RunIntent.full_error` (DEBT-59 / P428). It is `pub` only because L4 lives
/// in a different crate; treat it as an implementation detail.
/// Compila `source` contra `world` para bytes PDF e devolve tempos por fase.
///
/// P507 — instrumentação de benchmark; API pública para testes e ferramentas.
/// O campo `parse_ms` não é preenchido por esta função (o parse ocorre no
/// caller); o caller deve preenchê-lo se desejar o tempo total completo.
pub fn compile_to_pdf_bytes_with_timings(
    world: &dyn World,
    source: &Source,
    stream_mode: StreamMode,
    pdf_tags: PdfTags,
) -> (Result<Vec<u8>, Vec<SourceDiagnostic>>, Vec<SourceDiagnostic>, Timings) {
    compile_to_pdf_bytes_with_timings_full_error(
        world,
        source,
        false,
        stream_mode,
        pdf_tags,
    )
}

/// Internal variant com instrumentação de tempos e `full_error`.
#[doc(hidden)]
pub fn compile_to_pdf_bytes_with_timings_full_error(
    world: &dyn World,
    source: &Source,
    full_error: bool,
    stream_mode: StreamMode,
    pdf_tags: PdfTags,
) -> (Result<Vec<u8>, Vec<SourceDiagnostic>>, Vec<SourceDiagnostic>, Timings) {
    compile_to_pdf_bytes_with_timings_full_error_and_document_id(
        world,
        source,
        full_error,
        None,
        stream_mode,
        pdf_tags,
    )
}

/// **P617** — variant com `DocumentID` externo e instrumentação de tempos.
#[doc(hidden)]
pub fn compile_to_pdf_bytes_with_timings_full_error_and_document_id(
    world: &dyn World,
    source: &Source,
    full_error: bool,
    document_id: Option<[u8; 16]>,
    stream_mode: StreamMode,
    pdf_tags: PdfTags,
) -> (Result<Vec<u8>, Vec<SourceDiagnostic>>, Vec<SourceDiagnostic>, Timings) {
    compile_to_pdf_bytes_with_timings_full_error_and_document_id_with_features(
        world,
        source,
        full_error,
        document_id,
        stream_mode,
        pdf_tags,
        typst_core::entities::compiler_features::Features::default(),
    )
}

#[doc(hidden)]
pub fn compile_to_pdf_bytes_with_timings_full_error_and_document_id_with_features(
    world: &dyn World,
    source: &Source,
    full_error: bool,
    document_id: Option<[u8; 16]>,
    stream_mode: StreamMode,
    pdf_tags: PdfTags,
    features: typst_core::entities::compiler_features::Features,
) -> (Result<Vec<u8>, Vec<SourceDiagnostic>>, Vec<SourceDiagnostic>, Timings) {
    compile_to_pdf_bytes_with_timings_full_error_and_document_id_with_features_and_serialization(
        world,
        source,
        full_error,
        document_id,
        stream_mode,
        pdf_tags,
        features,
        PdfSerializationMode::Compact,
    )
}

#[doc(hidden)]
pub fn compile_to_pdf_bytes_with_timings_full_error_and_document_id_with_features_and_serialization(
    world: &dyn World,
    source: &Source,
    full_error: bool,
    document_id: Option<[u8; 16]>,
    stream_mode: StreamMode,
    pdf_tags: PdfTags,
    features: typst_core::entities::compiler_features::Features,
    serialization: PdfSerializationMode,
) -> (Result<Vec<u8>, Vec<SourceDiagnostic>>, Vec<SourceDiagnostic>, Timings) {
    compile_to_pdf_bytes_with_timings_full_error_and_document_id_with_features_and_tagged_and_pretty_override(
        world,
        source,
        full_error,
        document_id,
        stream_mode,
        Some(matches!(pdf_tags, PdfTags::Enabled)),
        features,
        Some(matches!(serialization, PdfSerializationMode::Pretty)),
    )
}

/// Compila `source` contra `world` até produzir um `PagedDocument` pronto
/// para exportação (eval → introspect → expand → layout → bidi → shape → validação de imagens).
///
/// Esta função é partilhada pelos backends PDF, PNG e SVG (P870).
fn compile_to_paged_document_full_error(
    world: &dyn World,
    source: &Source,
    full_error: bool,
    timings: &mut Timings,
) -> (Result<PagedDocument, Vec<SourceDiagnostic>>, Vec<SourceDiagnostic>) {
    compile_to_paged_document_full_error_and_features(
        world,
        source,
        full_error,
        typst_core::entities::compiler_features::Features::default(),
        timings,
    )
}

fn compile_to_paged_document_full_error_and_features(
    world: &dyn World,
    source: &Source,
    full_error: bool,
    features: typst_core::entities::compiler_features::Features,
    timings: &mut Timings,
) -> (Result<PagedDocument, Vec<SourceDiagnostic>>, Vec<SourceDiagnostic>) {
    let (result, warnings) =
        compile_to_paged_document_full_error_and_features_with_introspector(
            world, source, full_error, features, timings,
        );
    (result.map(|(document, _)| document), warnings)
}

fn compile_to_paged_document_full_error_and_features_with_introspector(
    world: &dyn World,
    source: &Source,
    full_error: bool,
    features: typst_core::entities::compiler_features::Features,
    timings: &mut Timings,
) -> (
    Result<
        (PagedDocument, typst_core::entities::introspector::TagIntrospector),
        Vec<SourceDiagnostic>,
    >,
    Vec<SourceDiagnostic>,
) {
    let t0 = Instant::now();
    let (eval_result, mut warnings) = eval_to_module_with_sink_target_features(
        world,
        source,
        full_error,
        EvalTarget::Paged,
        features,
    );
    let t1 = Instant::now();
    timings.eval_ms = duration_ms(t1.duration_since(t0));

    let module = match eval_result {
        Ok(m) => m,
        Err(errors) => {
            timings.total_ms = timings.eval_ms;
            return (Err(errors), warnings);
        }
    };
    let content = match module.content() {
        Some(c) => c,
        None => {
            timings.total_ms = timings.eval_ms;
            return (
                Ok((
                    PagedDocument::new(vec![]),
                    typst_core::entities::introspector::TagIntrospector::empty(),
                )),
                warnings,
            );
        }
    };
    let intr_content = module.introspection_content().unwrap_or(content).clone();
    let intr_content =
        typst_core::compiler::introspect::convert_bib_refs_to_cites(intr_content);
    let content =
        typst_core::compiler::introspect::convert_bib_refs_to_cites(content.clone());
    // Preservado para reexpandir `context` depois de o PageStore existir.
    let contextual_content = content.clone();

    let mut intr = introspect_with_introspector(&intr_content);
    for (key, style) in module.bibliography_styles() {
        intr.bib_store.add_style(*key, style.clone());
    }
    let t2 = Instant::now();
    timings.introspect_ms = duration_ms(t2.duration_since(t1));

    let (prepared, contextual_warnings) = context_stabilization::prepare(
        content.clone(),
        &intr,
        world,
        source,
        full_error,
        features,
        Some(&module),
    );
    warnings.extend(contextual_warnings);
    let prepared = match prepared {
        Ok(prepared) => prepared,
        Err(errors) => {
            timings.expand_context_ms = duration_ms(Instant::now().duration_since(t2));
            timings.total_ms =
                timings.eval_ms + timings.introspect_ms + timings.expand_context_ms;
            return (Err(errors), warnings);
        }
    };
    let mut content = prepared.content;
    let t3 = Instant::now();
    timings.expand_context_ms = duration_ms(t3.duration_since(t2));
    let mut format_content = content.clone();
    let (mut doc, mut intr_for_positions, extracted_headings) = if let Some((intr, doc)) =
        prepared.settled
    {
        let headings = intr.headings_for_bookmarks().to_vec();
        (doc, intr, headings)
    } else {
        let (runtime_intr, runtime_warnings) =
            introspect_with_runtime_for_pipeline(&content, world, source);
        warnings.extend(runtime_warnings);
        let mut intr = match runtime_intr {
            Ok(intr) => intr,
            Err(errors) => {
                timings.expand_context_ms =
                    duration_ms(Instant::now().duration_since(t2));
                timings.total_ms =
                    timings.eval_ms + timings.introspect_ms + timings.expand_context_ms;
                return (Err(errors), warnings);
            }
        };
        for (key, style) in module.bibliography_styles() {
            intr.bib_store.add_style(*key, style.clone());
        }
        let extracted_headings = intr.headings_for_bookmarks().to_vec();
        let logical_page_start = intr
            .counter_final_values(&typst_core::entities::counter::CounterKey::Page)
            .and_then(|values| values.first().copied())
            .unwrap_or(1);
        let mut intr_for_positions = intr.clone();
        let mut layout_intr = intr;
        let (realized, realization_warnings) =
            realize_outlines_for_pipeline(&content, &layout_intr, world, source);
        warnings.extend(realization_warnings);
        let (realized_content, realized_intr) = match realized {
            Ok(pair) => pair,
            Err(errors) => return (Err(errors), warnings),
        };
        layout_intr = realized_intr;
        let realized_content = match expand_realized_outline_contexts(
            realized_content,
            &content,
            &layout_intr,
            world,
            source,
        ) {
            Ok(content) => content,
            Err(errors) => return (Err(errors), warnings),
        };
        let mut final_realized_content = realized_content.clone();
        let (mut doc, callback_warnings) = match layout_with_realized_math_callbacks(
            &realized_content,
            layout_intr.clone(),
            world,
            source,
        ) {
            Ok(pair) => pair,
            Err(errors) => return (Err(errors), warnings),
        };
        warnings.extend(callback_warnings);
        // P1159: callbacks são executados somente aqui, onde existe Engine. O
        // layouter recebe apenas vistas Content seladas da passagem anterior.
        for _ in 0..5 {
            let previous_pages = doc.pages.len();
            let reference_pages = page_reference_pages(&content, &doc);
            let (store, numbering_warnings) = match realize_page_numberings(
                world,
                source,
                &doc,
                logical_page_start,
                &reference_pages,
            ) {
                Ok(pair) => pair,
                Err(errors) => return (Err(errors), warnings),
            };
            warnings.extend(numbering_warnings);
            layout_intr.inject_positions(doc.extracted_positions.clone());
            layout_intr.inject_pages(store.clone());
            // `location.page-numbering()` vive em `context`; a primeira expansão
            // ocorreu legitimamente sem páginas conhecidas. Reexecutá-la com o
            // snapshot anterior é parte do fixpoint, não um default falso.
            let expanded = match context_stabilization::expand_legacy(
                contextual_content.clone(),
                &layout_intr,
                world,
                source,
            ) {
                Ok(expanded) => expanded,
                Err(errors) => return (Err(errors), warnings),
            };
            let (runtime_expanded, runtime_expanded_warnings) =
                introspect_with_runtime_for_pipeline(&expanded, world, source);
            warnings.extend(runtime_expanded_warnings);
            let mut expanded_intr = match runtime_expanded {
                Ok(intr) => intr,
                Err(errors) => return (Err(errors), warnings),
            };
            for (key, style) in module.bibliography_styles() {
                expanded_intr.bib_store.add_style(*key, style.clone());
            }
            expanded_intr.inject_positions(doc.extracted_positions.clone());
            expanded_intr.inject_pages(store);
            content = expanded;
            layout_intr = expanded_intr;
            let (realized, realization_warnings) =
                realize_outlines_for_pipeline(&content, &layout_intr, world, source);
            warnings.extend(realization_warnings);
            let (realized_content, realized_intr) = match realized {
                Ok(pair) => pair,
                Err(errors) => return (Err(errors), warnings),
            };
            layout_intr = realized_intr;
            let realized_content = match expand_realized_outline_contexts(
                realized_content,
                &content,
                &layout_intr,
                world,
                source,
            ) {
                Ok(content) => content,
                Err(errors) => return (Err(errors), warnings),
            };
            let (next, callback_warnings) = match layout_with_realized_math_callbacks(
                &realized_content,
                layout_intr.clone(),
                world,
                source,
            ) {
                Ok(pair) => pair,
                Err(errors) => return (Err(errors), warnings),
            };
            warnings.extend(callback_warnings);
            let converged = next.pages.len() == previous_pages;
            final_realized_content = realized_content;
            doc = next;
            if converged {
                break;
            }
        }
        intr_for_positions.inject_pages(layout_intr.page_store.clone());
        format_content = final_realized_content;
        (doc, intr_for_positions, extracted_headings)
    };

    doc.format_options = match collect_document_format_options(&format_content) {
        Ok(options) => options,
        Err(errors) => {
            timings.layout_ms = duration_ms(Instant::now().duration_since(t3));
            timings.total_ms = timings.eval_ms
                + timings.introspect_ms
                + timings.expand_context_ms
                + timings.layout_ms;
            return (Err(errors), warnings);
        }
    };
    for warning in doc.layout_warnings.drain(..) {
        warnings.push(SourceDiagnostic::warning(Span::detached(), warning));
    }
    if !doc.layout_errors.is_empty() {
        let errors: Vec<SourceDiagnostic> = doc.layout_errors.drain(..).collect();
        timings.layout_ms = duration_ms(Instant::now().duration_since(t3));
        timings.total_ms = timings.eval_ms
            + timings.introspect_ms
            + timings.expand_context_ms
            + timings.layout_ms;
        return (Err(errors), warnings);
    }
    doc.extracted_headings = extracted_headings;

    let heading_locations = intr_for_positions.query_by_kind(ElementKind::Heading);
    for (idx, (label, _, _, _)) in doc.extracted_headings.iter().enumerate() {
        if let Some(loc) = heading_locations.get(idx) {
            if let Some(pos) = doc.extracted_positions.position_of(*loc) {
                doc.extracted_label_pages.insert(label.clone(), pos.page.get());
                doc.extracted_label_positions.insert(label.clone(), pos.point);
            }
        }
    }

    doc.document_info = module.document_info().clone();

    let t4 = Instant::now();
    timings.layout_ms = duration_ms(t4.duration_since(t3));

    let doc =
        crate::layout_bidi::reorder_bidi_document(doc, &FallbackFontMetrics::new(world));

    let doc = crate::shaper::shape_document(world, doc);
    let doc = crate::shaper::fix_line_positions(world, doc);
    let t5 = Instant::now();
    timings.shape_ms = duration_ms(t5.duration_since(t4));

    if let Err(msg) = crate::export::validate_document_images(&doc) {
        timings.total_ms = timings.eval_ms
            + timings.introspect_ms
            + timings.expand_context_ms
            + timings.layout_ms
            + timings.shape_ms;
        return (Err(vec![SourceDiagnostic::error(Span::detached(), msg)]), warnings);
    }

    intr_for_positions.inject_positions(doc.extracted_positions.clone());
    (Ok((doc, intr_for_positions)), warnings)
}

/// Runs the pure L1 transcript until a complete document is produced. Every
/// provisional document is consumed by `Pending` and therefore cannot reach
/// numbering, shaping, or export.
fn layout_with_realized_math_callbacks(
    content: &Content,
    introspector: typst_core::entities::introspector::TagIntrospector,
    world: &dyn World,
    source: &Source,
) -> SourceResult<(PagedDocument, Vec<SourceDiagnostic>)> {
    let mut math_store: Option<SealedMathCallbacks> = None;
    let mut par_line_store: Option<SealedParLineCallbacks> = None;
    let mut warnings = Vec::new();
    for _ in 0..8 {
        match layout_with_introspector_and_metrics_and_callbacks(
            content,
            introspector.clone(),
            FallbackFontMetrics::new(world),
            ImageSizeImageSizer,
            11.0,
            math_store.as_ref(),
            par_line_store.as_ref(),
        ) {
            LayoutPassOutcome::Complete(document) => return Ok((document, warnings)),
            LayoutPassOutcome::Pending(requests) => {
                if requests.math.is_empty() && requests.par_line.is_empty() {
                    return Err(vec![SourceDiagnostic::error(
                        Span::detached(),
                        "layout did not converge before callback realization",
                    )]);
                }
                if !requests.math.is_empty() {
                    let (realized, pass_warnings) =
                        realize_math_cancel_requests(world, source, requests.math)?;
                    warnings.extend(pass_warnings);
                    math_store = Some(realized);
                }
                if !requests.par_line.is_empty() {
                    let (realized, pass_warnings) =
                        realize_par_line_requests(world, source, requests.par_line)?;
                    warnings.extend(pass_warnings);
                    par_line_store = Some(realized);
                }
            }
        }
    }
    Err(vec![SourceDiagnostic::error(
        Span::detached(),
        "layout callbacks did not converge",
    )])
}

fn realize_par_line_requests(
    world: &dyn World,
    source: &Source,
    requests: Vec<ParLineCallbackRequest>,
) -> SourceResult<(SealedParLineCallbacks, Vec<SourceDiagnostic>)> {
    let font_metrics = FallbackFontMetrics::new(world);
    let mut ctx = EvalContext::new();
    let mut scopes = Scopes::new(None);
    let mut styles = StyleChain::default_chain();
    let mut show_rules: Arc<[ShowRule]> = Arc::from([]);
    let mut active_guards = Vec::new();
    let mut sink = TypstSink::new();
    let route = Route::root().with_id(source.id());
    let mut resolutions = Vec::with_capacity(requests.len());
    {
        let mut tracked_sink = sink.track_mut();
        let mut local_sink = TrackedMut::reborrow_mut(&mut tracked_sink);
        let mut engine = Engine {
            world,
            font_metrics: &font_metrics,
            route: route.track(),
            styles: &mut styles,
            show_rules: &mut show_rules,
            active_guards: &mut active_guards,
            current_file: source.id(),
            sink: &mut local_sink,
        };
        for request in requests {
            *engine.styles = request.styles.clone();
            let numbering =
                typst_core::entities::numbering::Numbering::Func(request.func.clone());
            let content = realize_numbering(
                &numbering,
                &[request.number],
                request.span,
                &mut scopes,
                &mut ctx,
                &mut engine,
            )
            .map_err(|mut diagnostics| {
                for diagnostic in &mut diagnostics {
                    diagnostic.span = request.span;
                }
                diagnostics
            })?;
            resolutions.push(ParLineCallbackResolution { request, content });
        }
    }
    Ok((SealedParLineCallbacks::new(resolutions), sink.into_diagnostics()))
}

fn realize_math_cancel_requests(
    world: &dyn World,
    source: &Source,
    requests: Vec<typst_core::compiler::math::layout::callbacks::MathCancelRequest>,
) -> SourceResult<(SealedMathCallbacks, Vec<SourceDiagnostic>)> {
    let font_metrics = FallbackFontMetrics::new(world);
    let mut ctx = EvalContext::new();
    let mut scopes = Scopes::new(None);
    let mut styles = StyleChain::default_chain();
    let mut show_rules: Arc<[ShowRule]> = Arc::from([]);
    let mut active_guards = Vec::new();
    let mut sink = TypstSink::new();
    let route = Route::root().with_id(source.id());
    let mut resolutions = Vec::with_capacity(requests.len());
    {
        let mut tracked_sink = sink.track_mut();
        let mut local_sink = TrackedMut::reborrow_mut(&mut tracked_sink);
        let mut engine = Engine {
            world,
            font_metrics: &font_metrics,
            route: route.track(),
            styles: &mut styles,
            show_rules: &mut show_rules,
            active_guards: &mut active_guards,
            current_file: source.id(),
            sink: &mut local_sink,
        };
        for request in requests {
            *engine.styles = request.styles.clone();
            let mut value = apply_func(
                request.func.clone(),
                Args::positional(vec![Value::Angle(request.default)]),
                &mut scopes,
                &mut ctx,
                &mut engine,
            )
            .map_err(|mut diagnostics| {
                for diagnostic in &mut diagnostics {
                    diagnostic.span = request.span;
                }
                diagnostics
            })?;
            // A contextual callback is represented by the existing delayed
            // ContextBlock carrier. At this L3 boundary its captured closure
            // can be evaluated against the request's sealed style snapshot.
            if let Value::Content(Content::ContextBlock(contextual)) = value {
                value = apply_func(
                    contextual.closure.clone(),
                    Args::positional(Vec::new()),
                    &mut scopes,
                    &mut ctx,
                    &mut engine,
                )
                .map_err(|mut diagnostics| {
                    for diagnostic in &mut diagnostics {
                        diagnostic.span = request.span;
                    }
                    diagnostics
                })?;
            }
            let Value::Angle(angle) = value else {
                let found = match value.type_name() {
                    "int" => "integer",
                    "str" => "string",
                    "bool" => "boolean",
                    other => other,
                };
                return Err(vec![SourceDiagnostic::error(
                    request.span,
                    format!("expected angle, found {found}"),
                )]);
            };
            resolutions.push(MathCancelResolution { request, angle });
        }
    }
    Ok((SealedMathCallbacks::new(resolutions), sink.into_diagnostics()))
}

/// Realiza as vistas binária (margem) e unária (referência) de cada página.
fn realize_page_numberings(
    world: &dyn World,
    source: &Source,
    doc: &PagedDocument,
    logical_start: usize,
    reference_pages: &HashSet<usize>,
) -> SourceResult<(typst_core::entities::page_store::PageStore, Vec<SourceDiagnostic>)> {
    let Some(total) = std::num::NonZeroUsize::new(doc.pages.len()) else {
        return Ok((typst_core::entities::page_store::PageStore::empty(), Vec::new()));
    };
    let font_metrics = FallbackFontMetrics::new(world);
    let mut ctx = EvalContext::new();
    let mut scopes = Scopes::new(None);
    let mut styles = StyleChain::default_chain();
    let mut show_rules: Arc<[ShowRule]> = Arc::from([]);
    let mut active_guards = Vec::new();
    let mut sink = TypstSink::new();
    let route = Route::root().with_id(source.id());
    let mut visible = Vec::with_capacity(total.get());
    let mut reference = Vec::with_capacity(total.get());
    {
        let mut tracked_sink = sink.track_mut();
        let mut local_sink = TrackedMut::reborrow_mut(&mut tracked_sink);
        let mut engine = Engine {
            world,
            font_metrics: &font_metrics,
            route: route.track(),
            styles: &mut styles,
            show_rules: &mut show_rules,
            active_guards: &mut active_guards,
            current_file: source.id(),
            sink: &mut local_sink,
        };
        for (index, page) in doc.pages.iter().enumerate() {
            let current = logical_start + index;
            let logical_total = logical_start + total.get() - 1;
            match &page.numbering {
                Some(numbering) => {
                    visible.push(Some(typst_core::compiler::stdlib::realize_numbering(
                        numbering,
                        &[current, logical_total],
                        Span::detached(),
                        &mut scopes,
                        &mut ctx,
                        &mut engine,
                    )?));
                    if reference_pages.contains(&(index + 1)) {
                        reference.push(Some(
                            typst_core::compiler::stdlib::realize_numbering(
                                numbering,
                                &[current],
                                Span::detached(),
                                &mut scopes,
                                &mut ctx,
                                &mut engine,
                            )?,
                        ));
                    } else {
                        reference.push(None);
                    }
                }
                None => {
                    visible.push(None);
                    reference.push(None);
                }
            }
        }
    }
    let warnings = sink.into_diagnostics();
    Ok((
        typst_core::entities::page_store::PageStore::from_realized(
            total,
            doc.pages.iter().map(|page| page.numbering.clone()).collect(),
            (logical_start..logical_start + total.get()).collect(),
            vec![Span::detached(); total.get()],
            visible,
            reference,
            doc.pages.iter().map(|page| page.supplement.clone()).collect(),
        ),
        warnings,
    ))
}

fn page_reference_pages(content: &Content, doc: &PagedDocument) -> HashSet<usize> {
    use typst_core::entities::label::Label;

    let mut pages = HashSet::new();
    let _ = content.clone().map_content(&mut |node| {
        if let Content::Ref(elem) = node {
            if elem.form == typst_core::entities::elements::r#ref::RefForm::Page {
                if let Some(page) =
                    doc.extracted_label_pages.get(&Label(elem.name.to_string()))
                {
                    pages.insert(*page);
                }
            }
        }
        Ok(None)
    });
    pages
}

/// **P1467** — resolve a precedência `--pdf-tagged` (override explícito) >
/// `#set format.pdf(tagged: ...)` (documento) > `auto` (habilitado, sem
/// seleção de páginas — P1468 revisita). Mesmo padrão de `resolve_svg_pretty`/
/// `resolve_html_pretty`.
fn resolve_pdf_tags(
    tagged_override: Option<bool>,
    options: &DocumentFormatOptions,
) -> PdfTags {
    let enabled = match tagged_override {
        Some(explicit) => explicit,
        None => match options.pdf.tagged {
            PdfTagged::Explicit(value) => value,
            // Sem seleção de páginas (P1468 revisita): auto resolve habilitado.
            PdfTagged::Auto => true,
        },
    };
    if enabled {
        PdfTags::Enabled
    } else {
        PdfTags::Disabled
    }
}

/// **P1468** — estende `resolve_pdf_tags` com a precedência `pages_override`
/// (CLI, `--pages`) > `options.pdf.pages` (documento, `#set format.pdf
/// (pages: ...)`) > `None` (todas as páginas), e decide a interação com
/// `tagged` medida em `typst-pdf/src/lib.rs:95-118` do vanilla ratificado:
/// documento pedindo `tagged: true` explícito + `pages` ativo é erro
/// (`"cannot enable tagged PDF and export a page range"`); `tagged` que só
/// resolveria `Enabled` por `Auto` produz o warning documental (2 hints) e
/// força `Disabled`; `tagged` já `Disabled` (CLI ou documento explícitos)
/// não emite diagnóstico novo. A checagem CLI-only (`--pdf-tagged=true` +
/// `--pages`) é barrada antes desta função, em `wiring.md` — aqui só é
/// alcançável o `tagged_override == Some(true)` caso o gate de wiring não
/// tenha corrido (mantém `Enabled` sem diagnóstico, mesmo comportamento
/// anterior). Sem `pages` resolvido, delega integralmente a
/// `resolve_pdf_tags` (comportamento P1467 byte-idêntico).
fn resolve_pdf_pages_and_tags(
    pages_override: Option<PageRanges>,
    tagged_override: Option<bool>,
    options: &DocumentFormatOptions,
) -> SourceResult<(Option<PageRanges>, PdfTags, Vec<SourceDiagnostic>)> {
    let pages = pages_override.or_else(|| options.pdf.pages.clone());
    let Some(pages) = pages else {
        return Ok((None, resolve_pdf_tags(tagged_override, options), Vec::new()));
    };

    let (tags, diagnostics) = match (tagged_override, &options.pdf.tagged) {
        (Some(false), _) => (PdfTags::Disabled, Vec::new()),
        // Combinação CLI-only barrada em `wiring.md` antes de chegar aqui;
        // preservada como não-erro se alcançada por outra via (ex.: teste
        // directo desta função).
        (Some(true), _) => (PdfTags::Enabled, Vec::new()),
        (None, PdfTagged::Explicit(false)) => (PdfTags::Disabled, Vec::new()),
        (None, PdfTagged::Explicit(true)) => {
            return Err(vec![SourceDiagnostic::error(
                Span::detached(),
                "cannot enable tagged PDF and export a page range",
            )]);
        }
        (None, PdfTagged::Auto) => (
            PdfTags::Disabled,
            vec![SourceDiagnostic::warning(
                Span::detached(),
                "exporting a page range disables PDF tagging",
            )
            .with_hint("the resulting PDF will be inaccessible")
            .with_hint("set `pdf(tagged: false)` to silence this warning")],
        ),
    };

    Ok((Some(pages), tags, diagnostics))
}

/// **P1469** (escopo reduzido — contrato de linguagem) — nome de exibição
/// do perfil acessível ativo em `standards` (`a-1a`/`a-2a`/`a-3a`/`ua-1`),
/// ou `None` se nenhum dos quatro perfis acessíveis está presente.
/// Espelha `typst-pdf/src/lib.rs:120-143` do vanilla ratificado (a lista
/// dos quatro perfis "acessíveis" — não confundir com `a-1b`/`a-2b`/etc.,
/// que não exigem tagging).
fn pdf_standards_accessible_name(standards: &PdfStandards) -> Option<&'static str> {
    if let Some(archival @ (PdfArchival::A1a | PdfArchival::A2a | PdfArchival::A3a)) =
        standards.archival
    {
        return Some(archival.label());
    }
    if let Some(accessibility @ PdfAccessibility::Ua1) = standards.accessibility {
        return Some(accessibility.label());
    }
    None
}

/// **P1469** (escopo reduzido — contrato de linguagem) — estende
/// `resolve_pdf_pages_and_tags` (P1468) com a resolução de
/// `format.pdf.standard` (precedência CLI > documento > `PdfStandards::
/// default()`, Núcleo `format-option-precedence`) e a checagem documental
/// completa `standard`+`tagged` (depois de CLI+documento combinados),
/// espelhando `typst-pdf/src/lib.rs:120-143` do vanilla ratificado. A
/// checagem CLI-only equivalente (só a lista crua da CLI) pertence a
/// `wiring.md` P1469.
fn resolve_pdf_pages_tags_and_standards(
    pages_override: Option<PageRanges>,
    tagged_override: Option<bool>,
    standard_override: Option<PdfStandards>,
    options: &DocumentFormatOptions,
) -> SourceResult<(Option<PageRanges>, PdfTags, PdfStandards, Vec<SourceDiagnostic>)> {
    let (pages, tags, diagnostics) =
        resolve_pdf_pages_and_tags(pages_override, tagged_override, options)?;

    let standards =
        standard_override.unwrap_or_else(|| options.pdf.standard.unwrap_or_default());

    if let Some(name) = pdf_standards_accessible_name(&standards) {
        if tags == PdfTags::Disabled {
            // A causa raiz de `Disabled` é distinguível pelo warning que
            // `resolve_pdf_pages_and_tags` já produziu no ramo `Auto` +
            // `pages` — reaproveitada aqui sem recalcular (contrato do L0).
            let caused_by_pages = diagnostics.iter().any(|d| {
                d.message.contains("exporting a page range disables PDF tagging")
            });
            let mut diag = SourceDiagnostic::error(
                Span::detached(),
                format!(
                    "cannot set `pdf(tagged: false)` when exporting a {name} document"
                ),
            );
            if caused_by_pages {
                diag = diag
                    .with_hint("using `pdf(pages: ...)` implies `pdf(tagged: false)`");
            }
            return Err(vec![diag]);
        }
    }

    Ok((pages, tags, standards, diagnostics))
}

/// **P1480** — PDF/UA-1 exige `document(title:)` (krilla `ValidationError::
/// NoDocumentTitle`, medido em typst-passo-1480.md §0: vanilla recusa
/// compilar sob `--pdf-standard ua-1` sem título; cristalino compilava
/// silenciosamente antes desta checagem). Mensagem/hint byte-idênticos ao
/// vanilla (`typst-pdf/src/convert.rs:748-752`); `prefix` fixo em
/// `"PDF/UA-1"` — combinação com archival não medida, ver `infra/
/// pipeline.md` §P1480. Função pura, testável isoladamente do mesmo modo
/// que `resolve_pdf_pages_tags_and_standards` (P1469).
fn require_ua1_document_title(
    doc: &PagedDocument,
    standards: &PdfStandards,
) -> Option<SourceDiagnostic> {
    if matches!(standards.accessibility, Some(PdfAccessibility::Ua1))
        && doc.document_info.title.is_none()
    {
        Some(
            SourceDiagnostic::error(
                Span::detached(),
                "PDF/UA-1 error: missing document title",
            )
            .with_hint("set the title with `set document(title: [...])`"),
        )
    } else {
        None
    }
}

/// P1481: imagens reais precisam de texto alternativo sob PDF/UA-1.
fn require_ua1_image_alt(
    doc: &PagedDocument,
    standards: &PdfStandards,
) -> Option<SourceDiagnostic> {
    if !matches!(standards.accessibility, Some(PdfAccessibility::Ua1)) {
        return None;
    }
    fn missing(items: &[FrameItem]) -> bool {
        items.iter().any(|item| match item {
            FrameItem::Image { alt, .. } => alt.is_none(),
            FrameItem::Semantic {
                kind: typst_core::entities::layout_types::SemanticKind::Artifact(_),
                ..
            } => false,
            FrameItem::Semantic { items, .. }
            | FrameItem::Group { items, .. }
            | FrameItem::Link { items, .. } => missing(items),
            _ => false,
        })
    }
    if doc.pages.iter().any(|page| missing(&page.items)) {
        Some(
            SourceDiagnostic::error(Span::detached(), "PDF/UA-1 error: missing alt text")
                .with_hint("make sure your images and equations have alt text"),
        )
    } else {
        None
    }
}

/// **P1467** — resolve a precedência `--pretty` (override explícito) >
/// `#set format.pdf(pretty: ...)` (documento) > `false`.
fn resolve_pdf_pretty(
    pretty_override: Option<bool>,
    options: &DocumentFormatOptions,
) -> PdfSerializationMode {
    if pretty_override.unwrap_or(options.pdf.pretty) {
        PdfSerializationMode::Pretty
    } else {
        PdfSerializationMode::Compact
    }
}

fn compile_to_pdf_bytes_impl(
    world: &dyn World,
    source: &Source,
    full_error: bool,
    document_id: Option<[u8; 16]>,
    stream_mode: StreamMode,
    pdf_tagged_override: Option<bool>,
    features: typst_core::entities::compiler_features::Features,
    pretty_override: Option<bool>,
    pages_override: Option<PageRanges>,
    standard_override: Option<PdfStandards>,
    oracle: bool,
    timings: &mut Timings,
) -> (Result<Vec<u8>, Vec<SourceDiagnostic>>, Vec<SourceDiagnostic>) {
    let (doc_result, mut warnings) = compile_to_paged_document_full_error_and_features(
        world, source, full_error, features, timings,
    );
    let doc = match doc_result {
        Ok(d) => d,
        Err(errors) => {
            timings.total_ms = timings.eval_ms
                + timings.introspect_ms
                + timings.expand_context_ms
                + timings.layout_ms
                + timings.shape_ms;
            return (Err(errors), warnings);
        }
    };
    // **P1468/P1469** — resolve `pages` (precedência CLI > documento >
    // `None`), `standard` (precedência CLI > documento > default) e as
    // interações com `tagged` antes de qualquer resolução de fontes; erro
    // documental aborta aqui, mesmo padrão do erro de anexo duplicado abaixo.
    let (pages, pdf_tags, pdf_standards, pages_diagnostics) =
        match resolve_pdf_pages_tags_and_standards(
            pages_override,
            pdf_tagged_override,
            standard_override,
            &doc.format_options,
        ) {
            Ok(resolved) => resolved,
            Err(errors) => {
                timings.total_ms = timings.eval_ms
                    + timings.introspect_ms
                    + timings.expand_context_ms
                    + timings.layout_ms
                    + timings.shape_ms;
                return (Err(errors), warnings);
            }
        };
    warnings.extend(pages_diagnostics);

    // **P1480** — `pdf_standards`/`doc` já resolvidos; checagem de título
    // obrigatório sob PDF/UA-1 (ver `require_ua1_document_title`).
    if let Some(diag) = require_ua1_document_title(&doc, &pdf_standards) {
        return (Err(vec![diag]), warnings);
    }
    if let Some(diag) = require_ua1_image_alt(&doc, &pdf_standards) {
        return (Err(vec![diag]), warnings);
    }

    let serialization = resolve_pdf_pretty(pretty_override, &doc.format_options);

    // P1286 — nomes duplicados só são erro no target PDF, onde ambos
    // disputariam a mesma chave da name tree `/EmbeddedFiles`.
    let mut attachment_paths = HashSet::new();
    for attachment in &doc.attachments {
        if !attachment_paths.insert(attachment.path.clone()) {
            return (
                Err(vec![SourceDiagnostic::error(
                    Span::detached(),
                    format!("attempted to attach file {} twice", attachment.path),
                )]),
                warnings,
            );
        }
    }

    // Passo 146 (ADR-0055 decisão 5): dispatch multi-font.
    // 0 fonts resolvidos → fallback Helvetica.
    // 1 font resolvido → preserva caminho single-font do 140B/141.
    // 2+ fonts resolvidos → multi-font (resource dict com /F1..N).
    let font_combos = collect_fonts_from_doc(&doc, world);
    let resolved = resolve_fonts(&font_combos, world.book(), world);

    // P667/P671/P836 — gate de instanciação de fontes variáveis.
    let needs_variable_font_instancer =
        resolved.iter().any(|((_, font_variant, variations), bytes)| {
            is_variable_font(bytes)
                && !merge_explicit_variations(
                    axis_variations_for_font_variant(font_variant),
                    variations,
                )
                .is_empty()
        });
    if needs_variable_font_instancer && !variable_font_instancer_available() {
        for ((font_list, font_variant, variations), bytes) in &resolved {
            if is_variable_font(bytes)
                && !merge_explicit_variations(
                    axis_variations_for_font_variant(font_variant),
                    variations,
                )
                .is_empty()
            {
                let name = font_list
                    .as_slice()
                    .first()
                    .and_then(|f| f.name.as_str())
                    .unwrap_or("fonte variável");
                return (
                    Err(vec![SourceDiagnostic::error(
                        Span::detached(),
                        format!(
                            "fonte variável '{}' requer instanciação, mas Python/fontTools não está disponível",
                            name
                        ),
                    )]),
                    warnings,
                );
            }
        }
    }

    let t_render = Instant::now();
    // **P980** — caminho oráculo: mesma resolução de fontes, emissão com
    // as transformações de paridade de operador (`export/oracle.rs`).
    // **P1468** — `pages` não é fiado até esta família (scope-out
    // deliberado de `infra/export/mod.md` P1468 — só `export_pdf`/
    // `export_pdf_with_font`/`export_pdf_multifont` ganham variantes
    // `_and_pages`); o `PdfBuilder` do oráculo continua sem filtro de
    // páginas, comportamento anterior preservado.
    let (pdf, subset_ms) = if oracle {
        let (pdf, subset) = crate::export::export_pdf_oracle_and_serialization(
            &doc,
            &resolved,
            document_id,
            stream_mode,
            pdf_tags,
            serialization,
        );
        (pdf, subset)
    } else {
        match resolved.as_slice() {
            [] => (
                export_pdf_with_document_id_and_tags_and_serialization_and_pages_and_standard(
                    &doc,
                    document_id,
                    stream_mode,
                    pdf_tags,
                    serialization,
                    pages.clone(),
                    pdf_standards,
                ),
                0.0,
            ),
            [single @ ((_, font_variant, variations), bytes)] => {
                // P668 — se a única fonte resolvida for uma VF com eixos
                // não-default, usar o caminho multi-font, que já instancia
                // correctamente (P530/P666). O caminho single-font
                // (`build_cidfont`) não faz instanciação.
                // P836 — eixos fundidos (derivados + explícitos).
                if is_variable_font(bytes)
                    && !merge_explicit_variations(
                        axis_variations_for_font_variant(font_variant),
                        variations,
                    )
                    .is_empty()
                {
                    export_pdf_multifont_and_timings_and_document_id_and_tags_and_serialization_and_pages_and_standard(
                        &doc,
                        std::slice::from_ref(single),
                        document_id,
                        stream_mode,
                        pdf_tags,
                        serialization,
                        pages.clone(),
                        pdf_standards,
                    )
                } else {
                    export_pdf_with_font_and_timings_and_document_id_and_tags_and_serialization_and_pages_and_standard(
                        &doc,
                        bytes,
                        document_id,
                        stream_mode,
                        pdf_tags,
                        serialization,
                        pages.clone(),
                        pdf_standards,
                    )
                }
            }
            many => export_pdf_multifont_and_timings_and_document_id_and_tags_and_serialization_and_pages_and_standard(
                &doc,
                many,
                document_id,
                stream_mode,
                pdf_tags,
                serialization,
                pages.clone(),
                pdf_standards,
            ),
        }
    };
    timings.subset_ms = subset_ms;
    timings.render_ms = duration_ms(Instant::now().duration_since(t_render)) - subset_ms;
    timings.total_ms = timings.eval_ms
        + timings.introspect_ms
        + timings.expand_context_ms
        + timings.layout_ms
        + timings.shape_ms
        + timings.subset_ms
        + timings.render_ms;

    (Ok(pdf), warnings)
}

/// **P1468** — variante que também recebe a seleção de páginas resolvida
/// (`pages_override`, precedência CLI > documento > `None`, `infra/
/// pipeline.md` P1468). Com instrumentação de tempos.
/// **P1469** — delega `None` para `standard`; comportamento byte-idêntico
/// preservado. Usar a variante `..._and_standard_override` para
/// `--pdf-standard`.
#[doc(hidden)]
pub fn compile_to_pdf_bytes_with_timings_full_error_and_document_id_with_features_and_tagged_and_pretty_and_pages_override(
    world: &dyn World,
    source: &Source,
    full_error: bool,
    document_id: Option<[u8; 16]>,
    stream_mode: StreamMode,
    pdf_tagged_override: Option<bool>,
    features: typst_core::entities::compiler_features::Features,
    pretty_override: Option<bool>,
    pages_override: Option<PageRanges>,
) -> (Result<Vec<u8>, Vec<SourceDiagnostic>>, Vec<SourceDiagnostic>, Timings) {
    compile_to_pdf_bytes_with_timings_full_error_and_document_id_with_features_and_tagged_and_pretty_and_pages_and_standard_override(
        world,
        source,
        full_error,
        document_id,
        stream_mode,
        pdf_tagged_override,
        features,
        pretty_override,
        pages_override,
        None,
    )
}

/// **P1469** (escopo reduzido — contrato de linguagem) — variante que
/// também recebe o conjunto de standards PDF resolvido (`standard_override`,
/// precedência CLI > documento > `PdfStandards::default()`, `infra/
/// pipeline.md` P1469). Com instrumentação de tempos.
#[doc(hidden)]
pub fn compile_to_pdf_bytes_with_timings_full_error_and_document_id_with_features_and_tagged_and_pretty_and_pages_and_standard_override(
    world: &dyn World,
    source: &Source,
    full_error: bool,
    document_id: Option<[u8; 16]>,
    stream_mode: StreamMode,
    pdf_tagged_override: Option<bool>,
    features: typst_core::entities::compiler_features::Features,
    pretty_override: Option<bool>,
    pages_override: Option<PageRanges>,
    standard_override: Option<PdfStandards>,
) -> (Result<Vec<u8>, Vec<SourceDiagnostic>>, Vec<SourceDiagnostic>, Timings) {
    let mut timings = Timings::default();
    let result = compile_to_pdf_bytes_impl(
        world,
        source,
        full_error,
        document_id,
        stream_mode,
        pdf_tagged_override,
        features,
        pretty_override,
        pages_override,
        standard_override,
        false,
        &mut timings,
    );
    (result.0, result.1, timings)
}

/// **P1467** — variante que preserva a ausência de override de `tagged`/
/// `pretty` até consultar `doc.format_options` (Núcleo
/// `format-option-precedence`). Com instrumentação de tempos.
/// **P1468** — delega `None` para `pages`; comportamento byte-idêntico
/// preservado. Usar a variante `..._and_pages_override` para `--pages`.
#[doc(hidden)]
pub fn compile_to_pdf_bytes_with_timings_full_error_and_document_id_with_features_and_tagged_and_pretty_override(
    world: &dyn World,
    source: &Source,
    full_error: bool,
    document_id: Option<[u8; 16]>,
    stream_mode: StreamMode,
    pdf_tagged_override: Option<bool>,
    features: typst_core::entities::compiler_features::Features,
    pretty_override: Option<bool>,
) -> (Result<Vec<u8>, Vec<SourceDiagnostic>>, Vec<SourceDiagnostic>, Timings) {
    compile_to_pdf_bytes_with_timings_full_error_and_document_id_with_features_and_tagged_and_pretty_and_pages_override(
        world,
        source,
        full_error,
        document_id,
        stream_mode,
        pdf_tagged_override,
        features,
        pretty_override,
        None,
    )
}

/// **P1468** — mesma variante sem instrumentação de tempos, com `pages`.
/// **P1469** — delega `None` para `standard`; comportamento byte-idêntico
/// preservado.
#[doc(hidden)]
pub fn compile_to_pdf_bytes_full_error_and_document_id_with_features_and_tagged_and_pretty_and_pages_override(
    world: &dyn World,
    source: &Source,
    full_error: bool,
    document_id: Option<[u8; 16]>,
    stream_mode: StreamMode,
    pdf_tagged_override: Option<bool>,
    features: typst_core::entities::compiler_features::Features,
    pretty_override: Option<bool>,
    pages_override: Option<PageRanges>,
) -> (Result<Vec<u8>, Vec<SourceDiagnostic>>, Vec<SourceDiagnostic>) {
    compile_to_pdf_bytes_full_error_and_document_id_with_features_and_tagged_and_pretty_and_pages_and_standard_override(
        world,
        source,
        full_error,
        document_id,
        stream_mode,
        pdf_tagged_override,
        features,
        pretty_override,
        pages_override,
        None,
    )
}

/// **P1469** (escopo reduzido — contrato de linguagem) — mesma variante sem
/// instrumentação de tempos, com `standard`.
#[doc(hidden)]
pub fn compile_to_pdf_bytes_full_error_and_document_id_with_features_and_tagged_and_pretty_and_pages_and_standard_override(
    world: &dyn World,
    source: &Source,
    full_error: bool,
    document_id: Option<[u8; 16]>,
    stream_mode: StreamMode,
    pdf_tagged_override: Option<bool>,
    features: typst_core::entities::compiler_features::Features,
    pretty_override: Option<bool>,
    pages_override: Option<PageRanges>,
    standard_override: Option<PdfStandards>,
) -> (Result<Vec<u8>, Vec<SourceDiagnostic>>, Vec<SourceDiagnostic>) {
    let mut timings = Timings::default();
    compile_to_pdf_bytes_impl(
        world,
        source,
        full_error,
        document_id,
        stream_mode,
        pdf_tagged_override,
        features,
        pretty_override,
        pages_override,
        standard_override,
        false,
        &mut timings,
    )
}

/// **P1467** — mesma variante sem instrumentação de tempos.
/// **P1468** — delega `None` para `pages`; comportamento byte-idêntico
/// preservado.
#[doc(hidden)]
pub fn compile_to_pdf_bytes_full_error_and_document_id_with_features_and_tagged_and_pretty_override(
    world: &dyn World,
    source: &Source,
    full_error: bool,
    document_id: Option<[u8; 16]>,
    stream_mode: StreamMode,
    pdf_tagged_override: Option<bool>,
    features: typst_core::entities::compiler_features::Features,
    pretty_override: Option<bool>,
) -> (Result<Vec<u8>, Vec<SourceDiagnostic>>, Vec<SourceDiagnostic>) {
    compile_to_pdf_bytes_full_error_and_document_id_with_features_and_tagged_and_pretty_and_pages_override(
        world,
        source,
        full_error,
        document_id,
        stream_mode,
        pdf_tagged_override,
        features,
        pretty_override,
        None,
    )
}

/// **P1468** — variante oráculo com `pages_override`. O `PageRanges`
/// resolvido ainda decide o diagnóstico `tagged`+`pages` (documental), mas
/// não filtra o PDF do oráculo — `infra/export/mod.md` P1468 não estende a
/// família `export_pdf_oracle*` (ferramenta de diagnóstico de paridade de
/// operador, `prompts/infra/export/oracle.md`).
/// **P1469** — delega `None` para `standard`; comportamento byte-idêntico
/// preservado (o oráculo também não fia `standard` ao `PdfBuilder` — mesmo
/// scope-out de `pages`).
pub fn compile_to_pdf_bytes_oracle_with_features_and_tagged_and_pretty_and_pages_override(
    world: &dyn World,
    source: &Source,
    full_error: bool,
    document_id: Option<[u8; 16]>,
    stream_mode: StreamMode,
    pdf_tagged_override: Option<bool>,
    features: typst_core::entities::compiler_features::Features,
    pretty_override: Option<bool>,
    pages_override: Option<PageRanges>,
) -> (Result<Vec<u8>, Vec<SourceDiagnostic>>, Vec<SourceDiagnostic>) {
    compile_to_pdf_bytes_oracle_with_features_and_tagged_and_pretty_and_pages_and_standard_override(
        world,
        source,
        full_error,
        document_id,
        stream_mode,
        pdf_tagged_override,
        features,
        pretty_override,
        pages_override,
        None,
    )
}

/// **P1469** (escopo reduzido — contrato de linguagem) — variante oráculo
/// com `standard_override`.
pub fn compile_to_pdf_bytes_oracle_with_features_and_tagged_and_pretty_and_pages_and_standard_override(
    world: &dyn World,
    source: &Source,
    full_error: bool,
    document_id: Option<[u8; 16]>,
    stream_mode: StreamMode,
    pdf_tagged_override: Option<bool>,
    features: typst_core::entities::compiler_features::Features,
    pretty_override: Option<bool>,
    pages_override: Option<PageRanges>,
    standard_override: Option<PdfStandards>,
) -> (Result<Vec<u8>, Vec<SourceDiagnostic>>, Vec<SourceDiagnostic>) {
    let mut timings = Timings::default();
    compile_to_pdf_bytes_impl(
        world,
        source,
        full_error,
        document_id,
        stream_mode,
        pdf_tagged_override,
        features,
        pretty_override,
        pages_override,
        standard_override,
        true,
        &mut timings,
    )
}

/// **P1467** — variante oráculo (`export/oracle.rs`) que preserva a ausência
/// de override de `tagged`/`pretty` até consultar `doc.format_options`.
/// **P1468** — delega `None` para `pages`; comportamento byte-idêntico
/// preservado.
pub fn compile_to_pdf_bytes_oracle_with_features_and_tagged_and_pretty_override(
    world: &dyn World,
    source: &Source,
    full_error: bool,
    document_id: Option<[u8; 16]>,
    stream_mode: StreamMode,
    pdf_tagged_override: Option<bool>,
    features: typst_core::entities::compiler_features::Features,
    pretty_override: Option<bool>,
) -> (Result<Vec<u8>, Vec<SourceDiagnostic>>, Vec<SourceDiagnostic>) {
    compile_to_pdf_bytes_oracle_with_features_and_tagged_and_pretty_and_pages_override(
        world,
        source,
        full_error,
        document_id,
        stream_mode,
        pdf_tagged_override,
        features,
        pretty_override,
        None,
    )
}

#[doc(hidden)]
pub fn compile_to_pdf_bytes_full_error(
    world: &dyn World,
    source: &Source,
    full_error: bool,
    stream_mode: StreamMode,
    pdf_tags: PdfTags,
) -> (Result<Vec<u8>, Vec<SourceDiagnostic>>, Vec<SourceDiagnostic>) {
    compile_to_pdf_bytes_full_error_and_document_id(
        world,
        source,
        full_error,
        None,
        stream_mode,
        pdf_tags,
    )
}

/// **P617** — variant com `DocumentID` externo.
#[doc(hidden)]
pub fn compile_to_pdf_bytes_full_error_and_document_id(
    world: &dyn World,
    source: &Source,
    full_error: bool,
    document_id: Option<[u8; 16]>,
    stream_mode: StreamMode,
    pdf_tags: PdfTags,
) -> (Result<Vec<u8>, Vec<SourceDiagnostic>>, Vec<SourceDiagnostic>) {
    compile_to_pdf_bytes_full_error_and_document_id_with_features(
        world,
        source,
        full_error,
        document_id,
        stream_mode,
        pdf_tags,
        typst_core::entities::compiler_features::Features::default(),
    )
}

#[doc(hidden)]
pub fn compile_to_pdf_bytes_full_error_and_document_id_with_features(
    world: &dyn World,
    source: &Source,
    full_error: bool,
    document_id: Option<[u8; 16]>,
    stream_mode: StreamMode,
    pdf_tags: PdfTags,
    features: typst_core::entities::compiler_features::Features,
) -> (Result<Vec<u8>, Vec<SourceDiagnostic>>, Vec<SourceDiagnostic>) {
    compile_to_pdf_bytes_full_error_and_document_id_with_features_and_serialization(
        world,
        source,
        full_error,
        document_id,
        stream_mode,
        pdf_tags,
        features,
        PdfSerializationMode::Compact,
    )
}

#[doc(hidden)]
pub fn compile_to_pdf_bytes_full_error_and_document_id_with_features_and_serialization(
    world: &dyn World,
    source: &Source,
    full_error: bool,
    document_id: Option<[u8; 16]>,
    stream_mode: StreamMode,
    pdf_tags: PdfTags,
    features: typst_core::entities::compiler_features::Features,
    serialization: PdfSerializationMode,
) -> (Result<Vec<u8>, Vec<SourceDiagnostic>>, Vec<SourceDiagnostic>) {
    compile_to_pdf_bytes_full_error_and_document_id_with_features_and_tagged_and_pretty_override(
        world,
        source,
        full_error,
        document_id,
        stream_mode,
        Some(matches!(pdf_tags, PdfTags::Enabled)),
        features,
        Some(matches!(serialization, PdfSerializationMode::Pretty)),
    )
}

/// **P980** — entrada do oráculo de paridade de operador (diagnóstico;
/// flag CLI `--oracle-pdf`). Idêntica a
/// `compile_to_pdf_bytes_full_error_and_document_id` mas com as
/// transformações de `export/oracle.rs` nos content streams.
/// `prompts/infra/export/oracle.md`.
pub fn compile_to_pdf_bytes_oracle(
    world: &dyn World,
    source: &Source,
    full_error: bool,
    document_id: Option<[u8; 16]>,
    stream_mode: StreamMode,
    pdf_tags: PdfTags,
) -> (Result<Vec<u8>, Vec<SourceDiagnostic>>, Vec<SourceDiagnostic>) {
    compile_to_pdf_bytes_oracle_with_features(
        world,
        source,
        full_error,
        document_id,
        stream_mode,
        pdf_tags,
        typst_core::entities::compiler_features::Features::default(),
    )
}

pub fn compile_to_pdf_bytes_oracle_with_features(
    world: &dyn World,
    source: &Source,
    full_error: bool,
    document_id: Option<[u8; 16]>,
    stream_mode: StreamMode,
    pdf_tags: PdfTags,
    features: typst_core::entities::compiler_features::Features,
) -> (Result<Vec<u8>, Vec<SourceDiagnostic>>, Vec<SourceDiagnostic>) {
    compile_to_pdf_bytes_oracle_with_features_and_serialization(
        world,
        source,
        full_error,
        document_id,
        stream_mode,
        pdf_tags,
        features,
        PdfSerializationMode::Compact,
    )
}

pub fn compile_to_pdf_bytes_oracle_with_features_and_serialization(
    world: &dyn World,
    source: &Source,
    full_error: bool,
    document_id: Option<[u8; 16]>,
    stream_mode: StreamMode,
    pdf_tags: PdfTags,
    features: typst_core::entities::compiler_features::Features,
    serialization: PdfSerializationMode,
) -> (Result<Vec<u8>, Vec<SourceDiagnostic>>, Vec<SourceDiagnostic>) {
    compile_to_pdf_bytes_oracle_with_features_and_tagged_and_pretty_override(
        world,
        source,
        full_error,
        document_id,
        stream_mode,
        Some(matches!(pdf_tags, PdfTags::Enabled)),
        features,
        Some(matches!(serialization, PdfSerializationMode::Pretty)),
    )
}

/// Resolve e instancia estaticamente as fontes necessárias para
/// rasterização PNG / exportação SVG (P870).
///
/// Aplica o mesmo gate de fontes variáveis do caminho PDF: se uma VF
/// precisa de instanciação e Python/fontTools não está disponível,
/// devolve um diagnóstico de erro. Fontes estáticas são passadas
/// inalteradas; VFs são instanciadas para as variações fundidas
/// (derivadas + explícitas).
fn resolve_and_instantiate_fonts(
    doc: &PagedDocument,
    world: &dyn World,
) -> Result<Vec<FontKey>, SourceDiagnostic> {
    let combos = collect_fonts_from_doc(doc, world);
    let resolved = resolve_fonts(&combos, world.book(), world);

    let needs_instancer =
        resolved.iter().any(|((_, font_variant, variations), bytes)| {
            is_variable_font(bytes)
                && !merge_explicit_variations(
                    axis_variations_for_font_variant(font_variant),
                    variations,
                )
                .is_empty()
        });
    if needs_instancer && !variable_font_instancer_available() {
        for ((font_list, font_variant, variations), bytes) in &resolved {
            if is_variable_font(bytes)
                && !merge_explicit_variations(
                    axis_variations_for_font_variant(font_variant),
                    variations,
                )
                .is_empty()
            {
                let name = font_list
                    .as_slice()
                    .first()
                    .and_then(|f| f.name.as_str())
                    .unwrap_or("fonte variável");
                return Err(SourceDiagnostic::error(
                    Span::detached(),
                    format!(
                        "fonte variável '{}' requer instanciação, mas Python/fontTools não está disponível",
                        name
                    ),
                ));
            }
        }
    }

    Ok(resolved
        .into_iter()
        .map(|(key, bytes)| {
            let (_, font_variant, variations) = &key;
            let merged = merge_explicit_variations(
                axis_variations_for_font_variant(font_variant),
                variations,
            );
            let tuple_vars: Vec<(ttf_parser::Tag, f32)> =
                merged.iter().map(|v| (v.tag, v.value)).collect();
            let instanced =
                instantiate_variable_font(&bytes, &tuple_vars).unwrap_or(bytes);
            (key, instanced)
        })
        .collect())
}

/// Pipeline completo `Source` → bytes PNG (primeira página).
pub fn compile_to_png_bytes(
    world: &dyn World,
    source: &Source,
) -> (Result<Vec<u8>, Vec<SourceDiagnostic>>, Vec<SourceDiagnostic>) {
    compile_to_png_bytes_full_error(world, source, false)
}

/// Variante internal com `full_error`.
#[doc(hidden)]
pub fn compile_to_png_bytes_full_error(
    world: &dyn World,
    source: &Source,
    full_error: bool,
) -> (Result<Vec<u8>, Vec<SourceDiagnostic>>, Vec<SourceDiagnostic>) {
    let (result, warnings, _) =
        compile_to_png_bytes_with_timings_full_error(world, source, full_error);
    (result, warnings)
}

/// Variante instrumentada de `compile_to_png_bytes`.
#[doc(hidden)]
pub fn compile_to_png_bytes_with_timings_full_error(
    world: &dyn World,
    source: &Source,
    full_error: bool,
) -> (Result<Vec<u8>, Vec<SourceDiagnostic>>, Vec<SourceDiagnostic>, Timings) {
    compile_to_png_bytes_with_timings_full_error_and_features(
        world,
        source,
        full_error,
        typst_core::entities::compiler_features::Features::default(),
    )
}

#[doc(hidden)]
pub fn compile_to_png_bytes_with_timings_full_error_and_features(
    world: &dyn World,
    source: &Source,
    full_error: bool,
    features: typst_core::entities::compiler_features::Features,
) -> (Result<Vec<u8>, Vec<SourceDiagnostic>>, Vec<SourceDiagnostic>, Timings) {
    compile_to_png_bytes_with_timings_full_error_and_features_and_ppi_override(
        world, source, full_error, features, None,
    )
}

/// Variante PNG que preserva a ausência de override até consultar as opções
/// resolvidas do documento.
#[doc(hidden)]
pub fn compile_to_png_bytes_with_timings_full_error_and_features_and_ppi_override(
    world: &dyn World,
    source: &Source,
    full_error: bool,
    features: typst_core::entities::compiler_features::Features,
    ppi_override: Option<f64>,
) -> (Result<Vec<u8>, Vec<SourceDiagnostic>>, Vec<SourceDiagnostic>, Timings) {
    let mut timings = Timings::default();
    let (doc_result, warnings) = compile_to_paged_document_full_error_and_features(
        world,
        source,
        full_error,
        features,
        &mut timings,
    );
    let result = match doc_result {
        Ok(doc) => {
            let effective_ppi = ppi_override.unwrap_or(doc.format_options.png.ppi);
            let pixel_per_pt = (effective_ppi / 72.0) as f32;
            let render_options =
                crate::export::RenderOptions { pixel_per_pt, render_bleed: false };
            let fonts = match resolve_and_instantiate_fonts(&doc, world) {
                Ok(f) => f,
                Err(diag) => {
                    timings.total_ms = timings.eval_ms
                        + timings.introspect_ms
                        + timings.expand_context_ms
                        + timings.layout_ms
                        + timings.shape_ms;
                    return (Err(vec![diag]), warnings, timings);
                }
            };
            let t_render = Instant::now();
            let page = doc.pages.first().cloned().unwrap_or_else(|| Page {
                width: 0.0,
                height: 0.0,
                numbering: None,
                supplement: typst_core::entities::content::Content::Empty,
                bleed: Default::default(),
                fill: Default::default(),
                background: vec![],
                foreground: vec![],
                items: vec![],
            });
            let png = if fonts.is_empty() {
                export_png(&page, &render_options)
            } else {
                export_png_with_fonts(&page, &render_options, &fonts)
            };
            timings.render_ms = duration_ms(Instant::now().duration_since(t_render));
            timings.total_ms = timings.eval_ms
                + timings.introspect_ms
                + timings.expand_context_ms
                + timings.layout_ms
                + timings.shape_ms
                + timings.render_ms;
            Ok(png)
        }
        Err(errors) => {
            timings.total_ms = timings.eval_ms
                + timings.introspect_ms
                + timings.expand_context_ms
                + timings.layout_ms
                + timings.shape_ms;
            Err(errors)
        }
    };
    (result, warnings, timings)
}

/// Pipeline completo `Source` → string SVG (primeira página).
pub fn compile_to_svg_string(
    world: &dyn World,
    source: &Source,
) -> (Result<String, Vec<SourceDiagnostic>>, Vec<SourceDiagnostic>) {
    compile_to_svg_string_full_error(world, source, false)
}

/// Variante internal com `full_error`.
#[doc(hidden)]
pub fn compile_to_svg_string_full_error(
    world: &dyn World,
    source: &Source,
    full_error: bool,
) -> (Result<String, Vec<SourceDiagnostic>>, Vec<SourceDiagnostic>) {
    let (result, warnings, _) =
        compile_to_svg_string_with_timings_full_error(world, source, full_error);
    (result, warnings)
}

/// Variante instrumentada de `compile_to_svg_string`.
#[doc(hidden)]
pub fn compile_to_svg_string_with_timings_full_error(
    world: &dyn World,
    source: &Source,
    full_error: bool,
) -> (Result<String, Vec<SourceDiagnostic>>, Vec<SourceDiagnostic>, Timings) {
    compile_to_svg_string_with_timings_full_error_and_features(
        world,
        source,
        full_error,
        typst_core::entities::compiler_features::Features::default(),
    )
}

#[doc(hidden)]
pub fn compile_to_svg_string_with_timings_full_error_and_features(
    world: &dyn World,
    source: &Source,
    full_error: bool,
    features: typst_core::entities::compiler_features::Features,
) -> (Result<String, Vec<SourceDiagnostic>>, Vec<SourceDiagnostic>, Timings) {
    compile_to_svg_string_with_timings_full_error_and_features_and_pretty_override(
        world, source, full_error, features, None,
    )
}

#[doc(hidden)]
pub fn compile_to_svg_string_with_timings_full_error_and_features_and_pretty(
    world: &dyn World,
    source: &Source,
    full_error: bool,
    features: typst_core::entities::compiler_features::Features,
    pretty: bool,
) -> (Result<String, Vec<SourceDiagnostic>>, Vec<SourceDiagnostic>, Timings) {
    compile_to_svg_string_with_timings_full_error_and_features_and_pretty_override(
        world,
        source,
        full_error,
        features,
        Some(pretty),
    )
}

fn resolve_svg_pretty(pretty_override: Option<bool>, document: &PagedDocument) -> bool {
    pretty_override.unwrap_or(document.format_options.svg.pretty)
}

/// Variante SVG que preserva a ausência de override até consultar as
/// opções resolvidas do documento.
#[doc(hidden)]
pub fn compile_to_svg_string_with_timings_full_error_and_features_and_pretty_override(
    world: &dyn World,
    source: &Source,
    full_error: bool,
    features: typst_core::entities::compiler_features::Features,
    pretty_override: Option<bool>,
) -> (Result<String, Vec<SourceDiagnostic>>, Vec<SourceDiagnostic>, Timings) {
    let mut timings = Timings::default();
    let (doc_result, warnings) = compile_to_paged_document_full_error_and_features(
        world,
        source,
        full_error,
        features,
        &mut timings,
    );
    let result = match doc_result {
        Ok(doc) => {
            let pretty = resolve_svg_pretty(pretty_override, &doc);
            let fonts = match resolve_and_instantiate_fonts(&doc, world) {
                Ok(f) => f,
                Err(diag) => {
                    timings.total_ms = timings.eval_ms
                        + timings.introspect_ms
                        + timings.expand_context_ms
                        + timings.layout_ms
                        + timings.shape_ms;
                    return (Err(vec![diag]), warnings, timings);
                }
            };
            let t_render = Instant::now();
            let page = doc.pages.first().cloned().unwrap_or_else(|| Page {
                width: 0.0,
                height: 0.0,
                numbering: None,
                supplement: typst_core::entities::content::Content::Empty,
                bleed: Default::default(),
                fill: Default::default(),
                background: vec![],
                foreground: vec![],
                items: vec![],
            });
            let destinations = svg_destination_context_for_page(&doc, 1);
            let glyph_fonts = svg_glyph_font_context_for_page(&page, world);
            let options = crate::export::SvgOptions { pretty, ..Default::default() };
            let svg = if fonts.is_empty() {
                export_svg_with_context(&page, &options, &destinations)
            } else {
                export_svg_with_fonts_and_contexts(
                    &page,
                    &options,
                    &fonts,
                    &destinations,
                    &glyph_fonts,
                )
            };
            timings.render_ms = duration_ms(Instant::now().duration_since(t_render));
            timings.total_ms = timings.eval_ms
                + timings.introspect_ms
                + timings.expand_context_ms
                + timings.layout_ms
                + timings.shape_ms
                + timings.render_ms;
            Ok(svg)
        }
        Err(errors) => {
            timings.total_ms = timings.eval_ms
                + timings.introspect_ms
                + timings.expand_context_ms
                + timings.layout_ms
                + timings.shape_ms;
            Err(errors)
        }
    };
    (result, warnings, timings)
}

#[cfg(test)]
mod p1462_tests {
    use super::*;

    #[test]
    fn svg_pretty_precedence_is_override_then_document_then_false() {
        let mut document = PagedDocument::new(vec![]);
        assert!(!resolve_svg_pretty(None, &document));

        document.format_options.svg.pretty = true;
        assert!(resolve_svg_pretty(None, &document));
        assert!(!resolve_svg_pretty(Some(false), &document));

        document.format_options.svg.pretty = false;
        assert!(resolve_svg_pretty(Some(true), &document));
    }
}

fn svg_destination_context_for_page(
    doc: &PagedDocument,
    selected_page_number: usize,
) -> SvgDestinationContext {
    SvgDestinationContext::from_destinations(
        doc.extracted_label_pages
            .iter()
            .filter(|(_, page)| **page == selected_page_number)
            .filter_map(|(label, _)| {
                doc.extracted_label_positions
                    .get(label)
                    .copied()
                    .map(|point| (label.clone(), point))
            }),
    )
}

fn svg_glyph_font_context_for_page(
    page: &Page,
    world: &dyn World,
) -> SvgGlyphFontContext {
    fn collect(
        items: &[FrameItem],
        metrics: &FallbackFontMetrics<'_>,
        out: &mut Vec<(GlyphFontRequest, (FontList, FontVariant, FontVariations))>,
    ) {
        for item in items {
            match item {
                FrameItem::Glyph { style, base_char, .. } => {
                    if let Some(identity) = metrics.resolve_font_combo(*base_char, style)
                    {
                        out.push((GlyphFontRequest::new(*base_char, style), identity));
                    }
                }
                FrameItem::Group { items, .. }
                | FrameItem::Link { items, .. }
                | FrameItem::Semantic { items, .. } => collect(items, metrics, out),
                _ => {}
            }
        }
    }
    let metrics = FallbackFontMetrics::new(world);
    let mut resolved = Vec::new();
    collect(&page.background, &metrics, &mut resolved);
    collect(&page.items, &metrics, &mut resolved);
    collect(&page.foreground, &metrics, &mut resolved);
    SvgGlyphFontContext::from_resolutions(resolved)
}

fn duration_ms(d: std::time::Duration) -> f64 {
    d.as_secs_f64() * 1000.0
}

impl Timings {
    /// Serializa os tempos como JSON compacto (sem dependência externa).
    pub fn to_json(&self) -> String {
        format!(
            "{{\"parse_ms\":{:.6},\"eval_ms\":{:.6},\"introspect_ms\":{:.6},\"expand_context_ms\":{:.6},\"layout_ms\":{:.6},\"shape_ms\":{:.6},\"subset_ms\":{:.6},\"render_ms\":{:.6},\"total_ms\":{:.6}}}",
            self.parse_ms,
            self.eval_ms,
            self.introspect_ms,
            self.expand_context_ms,
            self.layout_ms,
            self.shape_ms,
            self.subset_ms,
            self.render_ms,
            self.total_ms,
        )
    }
}

/// Itera `doc.pages → items` recursivamente (atravessa `Group`)
/// e devolve **todas** as combinações `(FontList, FontVariant)` distintas
/// em ordem de primeira ocorrência (Passo 146, ADR-0055 decisão 5;
/// P530 — chave expandida para incluir weight/style).
///
/// Deduplicação por igualdade estrutural via `Vec::contains`.
/// Complexidade O(N²) em N = combinações distintas; aceite porque N é
/// tipicamente pequeno (<10) em documentos reais.
/// **P906** — `world` adicionado: `FrameItem::Glyph` precisa de resolver a
/// fonte que efectivamente o cobre (`FallbackFontMetrics::resolve_font_
/// combo`, via `base_char`), à semelhança do que `style.font` já faz
/// directamente para `Text`/`TextShaped`. Ver nota em `collect_fonts_in_
/// items` abaixo.
fn collect_fonts_from_doc(
    doc: &PagedDocument,
    world: &dyn World,
) -> Vec<(FontList, FontVariant, FontVariations)> {
    let metrics = FallbackFontMetrics::new(world);
    let mut seen: Vec<(FontList, FontVariant, FontVariations)> = Vec::new();
    for page in &doc.pages {
        collect_fonts_in_items(&page.background, &metrics, &mut seen);
        collect_fonts_in_items(&page.items, &metrics, &mut seen);
        collect_fonts_in_items(&page.foreground, &metrics, &mut seen);
    }
    seen
}

/// **P906** — braço `FrameItem::Glyph` adicionado. Antes deste passo, era
/// ignorado (`FrameItem::Glyph { .. } => {}`) — todo glifo de esticamento
/// matemático (`layout_stretchy_delimiter`/`layout_assembly`, eixo
/// vertical e horizontal) era invisível para a selecção Cidfont-vs-
/// Multifont, que decide cedo no pipeline QUAIS fontes embutir a partir
/// só dos caracteres vistos em `Text`/`TextShaped`. Achado: uma equação
/// cujo ÚNICO conteúdo a precisar da fonte companion MATH fosse um glifo
/// de esticamento (ex: `underbracket(a+b+c)` sem mais nenhum texto
/// itálico matemático à volta) escolhia só a fonte de corpo como
/// candidata Cidfont única — o glyph_id do esticamento, correcto na fonte
/// MATH, caía fora do subset embutido (glifo `.notdef` no PDF real).
/// Corrigido resolvendo `base_char`+`style` via `resolve_font_combo`
/// (mesmo mecanismo `covering`/`resolve_primary_with_math_fallback` já
/// usado internamente por `FallbackFontMetrics` para o LAYOUT) — Glyph
/// entra na mesma lista `seen`, com a mesma deduplicação, como se fosse
/// mais um span de texto. Ver `pipeline.md` §P906.
fn collect_fonts_in_items(
    items: &[FrameItem],
    metrics: &FallbackFontMetrics,
    seen: &mut Vec<(FontList, FontVariant, FontVariations)>,
) {
    for item in items {
        match item {
            FrameItem::Text { style, .. } | FrameItem::TextShaped { style, .. } => {
                if let Some(fl) = &style.font {
                    let variant = text_style_to_font_variant(style);
                    // P836 — variações explícitas (`#text(variations:)`)
                    // entram na chave: runs com o mesmo FontVariant mas
                    // eixos explícitos distintos embutem fontes distintas.
                    let key = (
                        fl.clone(),
                        variant,
                        style.variations.clone().unwrap_or_default(),
                    );
                    if !seen.contains(&key) {
                        seen.push(key);
                    }
                }
            }
            FrameItem::Glyph { style, base_char, .. } => {
                if let Some(key) = metrics.resolve_font_combo(*base_char, style) {
                    if !seen.contains(&key) {
                        seen.push(key);
                    }
                }
            }
            FrameItem::Group { items, .. }
            | FrameItem::Link { items, .. }
            | FrameItem::Semantic { items, .. } => {
                collect_fonts_in_items(items, metrics, seen);
            }
            FrameItem::Line { .. }
            | FrameItem::Image { .. }
            | FrameItem::Shape { .. } => {}
        }
    }
}

/// Map-filter de `resolve_font` (Passo 141) sobre uma lista de
/// combinações `(FontList, FontVariant)`. Devolve
/// `((FontList, FontVariant), bytes)` para preservar a associação entre
/// input style e output embed (Passo 146; P530 — chave expandida).
///
/// Silent drop quando `resolve_font` devolve `None` — consistente
/// com a política de fallback de fonts (140B/141).
fn resolve_fonts(
    font_combos: &[(FontList, FontVariant, FontVariations)],
    font_book: &FontBook,
    world: &dyn World,
) -> Vec<((FontList, FontVariant, FontVariations), Vec<u8>)> {
    font_combos
        .iter()
        .filter_map(|(fl, variant, variations)| {
            resolve_font(fl, variant, font_book, world)
                .map(|bytes| ((fl.clone(), variant.clone(), variations.clone()), bytes))
        })
        .collect()
}

/// Itera `doc.pages → items` recursivamente (atravessa `Group`)
/// e devolve o primeiro `TextStyle.font` com `Some(FontList)`.
/// Preservada do Passo 140B (MVP single-font: primeira vence —
/// ADR-0055 decisão 3) para tests directos do helper. O dispatch
/// principal usa `collect_fonts_from_doc` (Passo 146).
#[allow(dead_code)]
fn first_font_from_doc(doc: &PagedDocument) -> Option<FontList> {
    for page in &doc.pages {
        if let Some(fl) = first_font_in_items(&page.items) {
            return Some(fl);
        }
    }
    None
}

#[allow(dead_code)]
fn first_font_in_items(items: &[FrameItem]) -> Option<FontList> {
    for item in items {
        match item {
            FrameItem::Text { style, .. } | FrameItem::TextShaped { style, .. } => {
                if let Some(fl) = &style.font {
                    return Some(fl.clone());
                }
            }
            FrameItem::Group { items, .. }
            | FrameItem::Link { items, .. }
            | FrameItem::Semantic { items, .. } => {
                if let Some(fl) = first_font_in_items(items) {
                    return Some(fl);
                }
            }
            FrameItem::Line { .. }
            | FrameItem::Glyph { .. }
            | FrameItem::Image { .. }
            | FrameItem::Shape { .. } => {}
        }
    }
    None
}

/// Itera `font_list.as_slice()` em ordem. Para cada família,
/// consulta `font_book.select_pattern(&family.name, variant)`;
/// se devolve `Some(index)`, chama `world.font(index)`; primeira
/// família que completa ambos os passos vence. Se nenhuma
/// completa, devolve `None` (pipeline cai em fallback Helvetica).
///
/// Paridade com vanilla: semântica "primeira-que-resolve" do
/// `#set text(font: (...))` (Passo 141).
///
/// Cenário patológico (índice stale: `select` devolve `Some` mas
/// `world.font` devolve `None`) **continua** a tentar as famílias
/// seguintes — não curto-circuita.
///
/// P530 — usa a `FontVariant` real do `TextStyle` na selecção. Se o
/// sistema tiver instâncias estáticas (ex.: UbuntuSans-Bold.ttf), são
/// preferidas. Caso contrário, a fonte VF é resolvida e instanciada
/// estaticamente mais tarde no export.
fn resolve_font(
    font_list: &FontList,
    variant: &FontVariant,
    font_book: &FontBook,
    world: &dyn World,
) -> Option<Vec<u8>> {
    for family in font_list.as_slice() {
        if let Some(index) = font_book.select_pattern(&family.name, variant) {
            if let Some(font) = world.font(index) {
                return Some(font.as_slice().to_vec());
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::num::NonZeroU16;
    use typst_core::entities::file_id::FileId;
    use typst_core::entities::font_book::FontBook;
    use typst_core::entities::world_types::{
        Bytes, Datetime, FileError, FileResult, Font, Library,
    };

    #[test]
    fn p1247_contexto_svg_filtra_pagina_e_posicao_homologa() {
        use typst_core::entities::label::Label;
        use typst_core::entities::layout_types::{Point, Pt};

        let mut doc = PagedDocument::new(vec![]);
        let p0 = Label("p0".to_string());
        let p1 = Label("p1".to_string());
        let sem_posicao = Label("sem-posicao".to_string());
        doc.extracted_label_pages.insert(p0.clone(), 1);
        doc.extracted_label_pages.insert(p1.clone(), 2);
        doc.extracted_label_pages.insert(sem_posicao.clone(), 1);
        doc.extracted_label_positions
            .insert(p0.clone(), Point { x: Pt(8.0), y: Pt(9.0) });
        doc.extracted_label_positions
            .insert(p1.clone(), Point { x: Pt(80.0), y: Pt(90.0) });

        let context = svg_destination_context_for_page(&doc, 1);
        assert!(context.id_for(&p0).is_some());
        assert!(context.id_for(&p1).is_none());
        assert!(context.id_for(&sem_posicao).is_none());
    }

    // MockWorld mínimo para smoke test — source única.
    struct MockWorld {
        library: Library,
        book: FontBook,
        source: Source,
    }
    impl World for MockWorld {
        fn library(&self) -> &Library {
            &self.library
        }
        fn book(&self) -> &FontBook {
            &self.book
        }
        fn main(&self) -> FileId {
            self.source.id()
        }
        fn source(&self, id: FileId) -> FileResult<Source> {
            if id == self.source.id() {
                Ok(self.source.clone())
            } else {
                Err(FileError::NotFound)
            }
        }
        fn file(&self, _: FileId) -> FileResult<Bytes> {
            Err(FileError::NotFound)
        }
        fn font(&self, _: usize) -> Option<Font> {
            None
        }
        fn today(
            &self,
            _: Option<typst_core::entities::duration::Duration>,
        ) -> Option<Datetime> {
            None
        }
    }

    fn mock_world(src: &str) -> MockWorld {
        let id = FileId::from_raw(NonZeroU16::new(1).unwrap());
        let source = Source::new(id, src.to_string());
        MockWorld {
            library: Library::new(),
            book: FontBook::new(),
            source,
        }
    }

    #[test]
    fn p1462_pipeline_collects_svg_pretty_and_empty_set_is_neutral() {
        for (source, expected) in [
            ("#set format.svg(pretty: true)\nHello", true),
            ("#set format.svg()\nHello", false),
            ("#set format.svg(pretty: true)\n#set format.svg()\nHello", true),
            (
                "#set format.svg(pretty: true)\n#set format.svg(pretty: false)\nHello",
                false,
            ),
            (
                "#set format.svg(pretty: false)\n#set format.svg(pretty: true)\nHello",
                true,
            ),
        ] {
            let world = mock_world(source);
            let mut timings = Timings::default();
            let (result, _) = compile_to_paged_document_full_error(
                &world,
                &world.source,
                false,
                &mut timings,
            );

            assert_eq!(result.unwrap().format_options.svg.pretty, expected);
        }
    }

    #[test]
    fn p1462_pipeline_rejects_nested_format_set_rule() {
        let world = mock_world("#block[\n#set format.svg(pretty: true)\nNested\n]");
        let mut timings = Timings::default();
        let (result, _) = compile_to_paged_document_full_error(
            &world,
            &world.source,
            false,
            &mut timings,
        );

        assert!(result.unwrap_err().iter().any(|error| {
            error
                .message
                .contains("format set rules are not allowed inside of containers")
        }));
    }

    #[test]
    fn p1462_set_rule_diagnostics_are_explicit() {
        for (source, expected) in [
            ("#set format.svg(true)", "unexpected argument"),
            ("#set format.svg(unknown: true)", "unexpected argument: unknown"),
            (
                "#set format.svg(pretty: true, pretty: false)",
                "duplicate argument: pretty",
            ),
            ("#set format.svg(pretty: 1)", "expected boolean, found integer"),
        ] {
            let world = mock_world(source);
            let mut timings = Timings::default();
            let (result, _) = compile_to_paged_document_full_error(
                &world,
                &world.source,
                false,
                &mut timings,
            );

            assert!(result
                .unwrap_err()
                .iter()
                .any(|error| error.message.contains(expected)));
        }
    }

    #[test]
    fn p1462_format_svg_cannot_be_constructed_manually() {
        let world = mock_world("#format.svg()");
        let mut timings = Timings::default();
        let (result, _) = compile_to_paged_document_full_error(
            &world,
            &world.source,
            false,
            &mut timings,
        );

        assert!(result
            .unwrap_err()
            .iter()
            .any(|error| error.message.contains("cannot be constructed manually")));
    }

    #[test]
    fn p1462_external_svg_observes_document_and_caller_precedence() {
        fn render(source: &str, pretty_override: Option<bool>) -> String {
            let world = mock_world(source);
            let (result, _, _) =
                compile_to_svg_string_with_timings_full_error_and_features_and_pretty_override(
                    &world,
                    &world.source,
                    false,
                    typst_core::entities::compiler_features::Features::default(),
                    pretty_override,
                );
            result.unwrap()
        }

        let body = "#rect(width: 10pt, height: 10pt)";
        let document_pretty = render(
            "#set format.svg(pretty: true)\n#rect(width: 10pt, height: 10pt)",
            None,
        );
        let caller_compact = render(
            "#set format.svg(pretty: true)\n#rect(width: 10pt, height: 10pt)",
            Some(false),
        );
        let default_compact = render(body, None);
        let caller_pretty = render(body, Some(true));

        assert!(document_pretty.contains("\n  <"));
        assert!(caller_pretty.contains("\n  <"));
        assert!(!caller_compact.contains("\n  <"));
        assert!(!default_compact.contains("\n  <"));
    }

    #[test]
    fn p1339_filtered_context_waits_for_later_contextual_update() {
        let world = mock_world("#let c = counter(heading.where(level: 1))\n#context assert.eq(c.final(), (3,))\n#context c.update(3)");
        let mut timings = Timings::default();
        let (result, _) = compile_to_paged_document_full_error(
            &world,
            &world.source,
            false,
            &mut timings,
        );
        assert!(result.is_ok(), "{result:?}");
    }

    #[test]
    fn p1339_filtered_context_does_not_discard_independent_error() {
        let world = mock_world("#let c = counter(heading.where(level: 1))\n#context panic(\"independent\")\n#context c.final()");
        let mut timings = Timings::default();
        let (result, _) = compile_to_paged_document_full_error(
            &world,
            &world.source,
            false,
            &mut timings,
        );
        assert!(result
            .unwrap_err()
            .iter()
            .any(|error| error.message.contains("independent")));
    }

    #[test]
    fn p1406_show_outline_entry_direto_realiza_entries() {
        let world = mock_world(concat!(
            "#set heading(numbering: \"1.\")\n",
            "#show outline.entry: it => [CUSTOM #(it.level): #it.body()]\n",
            "#outline(title: none)\n\n= Alpha\n== Beta\n",
        ));
        let mut timings = Timings::default();
        let (result, _) = compile_to_paged_document_full_error(
            &world,
            &world.source,
            false,
            &mut timings,
        );
        let text = result.expect("P1406 deve aceitar o seletor direto").plain_text();
        assert!(text.contains("CUSTOM 1") && text.contains("Alpha"), "{text:?}");
        assert!(text.contains("CUSTOM 2") && text.contains("Beta"), "{text:?}");
        assert!(!text.contains("Contents"), "title: none deve omitir título: {text:?}");
    }

    #[test]
    fn p1406_where_filtra_level_da_entry_gerada() {
        let world = mock_world(concat!(
            "#set heading(numbering: \"1.\")\n",
            "#show outline.entry.where(level: 2): it => [SECOND: #it.body()]\n",
            "#outline(title: none)\n\n= Alpha\n== Beta\n",
        ));
        let mut timings = Timings::default();
        let (result, _) = compile_to_paged_document_full_error(
            &world,
            &world.source,
            false,
            &mut timings,
        );
        let text = result.expect("P1406 where deve compilar").plain_text();
        assert_eq!(text.matches("SECOND").count(), 1, "{text:?}");
        assert!(text.contains("SECOND") && text.contains("Beta"), "{text:?}");
    }

    #[test]
    fn p1406_snapshot_de_show_e_lexical_por_outline() {
        let world = mock_world(concat!(
            "#set heading(numbering: \"1.\")\n",
            "#outline(title: none)\n",
            "#show outline.entry: it => [CUSTOM: #it.body()]\n",
            "#outline(title: none)\n\n= Alpha\n",
        ));
        let mut timings = Timings::default();
        let (result, _) = compile_to_paged_document_full_error(
            &world,
            &world.source,
            false,
            &mut timings,
        );
        let text = result.expect("P1406 lexical deve compilar").plain_text();
        assert_eq!(text.matches("CUSTOM").count(), 1, "{text:?}");
    }

    #[test]
    fn p1406_parent_existe_somente_durante_realizacao() {
        let world = mock_world(concat!(
            "#set heading(numbering: \"1.\")\n",
            "#show outline.entry: it => it.indented(it.prefix(), it.body())\n",
            "#outline(title: none, indent: 2em)\n\n= Alpha\n== Beta\n",
        ));
        let mut timings = Timings::default();
        let (result, _) = compile_to_paged_document_full_error(
            &world,
            &world.source,
            false,
            &mut timings,
        );
        let text = result.expect("parent P1406 deve existir durante show").plain_text();
        assert!(text.contains("Alpha") && text.contains("Beta"), "{text:?}");
    }

    #[test]
    fn p1406_alias_preserva_identidade_do_seletor() {
        let world = mock_world(concat!(
            "#let entry-kind = outline.entry\n",
            "#show entry-kind: it => [ALIAS #it.body()]\n",
            "#outline(title: none)\n\n= Alpha\n",
        ));
        let mut timings = Timings::default();
        let (result, _) = compile_to_paged_document_full_error(
            &world,
            &world.source,
            false,
            &mut timings,
        );
        let text = result.expect("alias deve conservar identidade nativa").plain_text();
        assert_eq!(text.matches("ALIAS").count(), 1, "{text:?}");
    }

    #[test]
    fn p1406_query_nao_duplica_e_contexto_nao_pula_realizacao() {
        let world = mock_world(concat!(
            "#show outline.entry: it => [ENTRY #it.body()]\n",
            "#outline(title: none)\n\n= Alpha\n",
            "#context assert.eq(query(outline.entry).len(), 1)\n",
        ));
        let mut timings = Timings::default();
        let (result, _) = compile_to_paged_document_full_error(
            &world,
            &world.source,
            false,
            &mut timings,
        );
        let text = result.expect("query deve manter uma entry").plain_text();
        assert_eq!(text.matches("ENTRY").count(), 1, "{text:?}");
    }

    #[test]
    fn p1406_default_emite_link_interno() {
        let world = mock_world("#outline(title: none)\n\n= Alpha\n");
        let mut timings = Timings::default();
        let (result, _) = compile_to_paged_document_full_error(
            &world,
            &world.source,
            false,
            &mut timings,
        );
        let doc = result.expect("outline default deve compilar");
        assert!(format!("{:?}", doc.pages).contains("Destination"));
    }

    #[test]
    fn p1406_depth_e_outlined_filtram_entries_realizadas() {
        let world = mock_world(concat!(
            "#show outline.entry: it => [ENTRY #(it.level): #it.body()]\n",
            "#outline(title: none, depth: 1)\n",
            "#heading(outlined: false)[Hidden]\n",
            "= Visible\n",
            "== Deep\n",
        ));
        let mut timings = Timings::default();
        let (result, _) = compile_to_paged_document_full_error(
            &world,
            &world.source,
            false,
            &mut timings,
        );
        let text = result.expect("filtros P1406 devem compilar").plain_text();
        assert_eq!(text.matches("ENTRY").count(), 1, "{text:?}");
        assert!(text.contains("ENTRY 1") && text.contains("Visible"), "{text:?}");
    }

    #[test]
    fn p1407_repeat_em_box_fr_multiplica_sem_duplicar_siblings() {
        let world = mock_world(concat!(
            "#set page(width: 180pt, height: auto, margin: 20pt)\n",
            "A#box(width: 100pt, repeat(\".\", gap: 2pt, justify: false))B\n",
        ));
        let mut timings = Timings::default();
        let (result, _) = compile_to_paged_document_full_error(
            &world,
            &world.source,
            false,
            &mut timings,
        );
        let text = result.expect("repeat P1407 deve compilar").plain_text();
        assert_eq!(text.matches('A').count(), 1, "{text:?}");
        assert_eq!(text.matches('B').count(), 1, "{text:?}");
        assert!(text.matches('.').count() > 10, "{text:?}");
    }

    #[test]
    fn p1407_repeat_responde_a_largura_finita() {
        let compile = |width: usize| {
            let world = mock_world(&format!(
                "#set page(width: 300pt, height: auto, margin: 20pt)\nA#box(width: {width}pt, repeat(\".\", gap: 2pt, justify: false))B\n"
            ));
            let mut timings = Timings::default();
            compile_to_paged_document_full_error(
                &world,
                &world.source,
                false,
                &mut timings,
            )
            .0
            .expect("largura P1407 deve compilar")
            .plain_text()
            .matches('.')
            .count()
        };
        let narrow = compile(120);
        let wide = compile(240);
        assert!(narrow > 1 && wide > narrow, "narrow={narrow}, wide={wide}");
    }

    fn p1407_destination_dot_count(items: &[FrameItem]) -> usize {
        items
            .iter()
            .map(|item| match item {
                FrameItem::Link { target, items, .. }
                    if matches!(
                        target,
                        typst_core::entities::layout_types::LinkTarget::Destination(_)
                    ) =>
                {
                    frame_items_text(items).matches('.').count()
                }
                FrameItem::Group { items, .. }
                | FrameItem::Link { items, .. }
                | FrameItem::Semantic { items, .. } => p1407_destination_dot_count(items),
                FrameItem::Text { .. }
                | FrameItem::TextShaped { .. }
                | FrameItem::Line { .. }
                | FrameItem::Glyph { .. }
                | FrameItem::Image { .. }
                | FrameItem::Shape { .. } => 0,
            })
            .sum()
    }

    #[test]
    fn p1407_outline_expande_fill_dentro_do_link() {
        let world = mock_world(concat!(
            "#set page(width: 240pt, height: auto, margin: 20pt)\n",
            "#set heading(numbering: \"1.\")\n",
            "#outline(title: none)\n\n= Alpha\n",
        ));
        let mut timings = Timings::default();
        let (result, _) = compile_to_paged_document_full_error(
            &world,
            &world.source,
            false,
            &mut timings,
        );
        let doc = result.expect("outline P1407 deve compilar");
        let dots: usize = doc
            .pages
            .iter()
            .map(|page| p1407_destination_dot_count(&page.items))
            .sum();
        assert!(dots > 10, "fill dentro do link tem somente {dots} pontos");
    }

    #[test]
    fn p1407_repeat_degenerado_e_extremo_permanecem_limitados() {
        let compile = |source: &str| {
            let world = mock_world(source);
            let mut timings = Timings::default();
            compile_to_paged_document_full_error(
                &world,
                &world.source,
                false,
                &mut timings,
            )
            .0
            .expect("caso limitado P1407 deve compilar")
            .plain_text()
        };
        let oversized = compile(concat!(
            "#set page(width: 100pt, height: auto, margin: 10pt)\n",
            "A#box(width: 1pt, repeat(\"LONG\", justify: true))B\n",
        ));
        assert_eq!(oversized.matches("LONG").count(), 1, "{oversized:?}");

        let capped = compile(concat!(
            "#set page(width: 100100pt, height: auto, margin: 10pt)\n",
            "A#box(width: 100000pt, repeat(\".\", gap: 0pt, justify: false))B\n",
        ));
        assert_eq!(capped.matches('.').count(), 1000);
    }

    fn p1407_dot_positions(items: &[FrameItem], positions: &mut Vec<f64>) {
        for item in items {
            match item {
                FrameItem::Text { pos, text, .. }
                | FrameItem::TextShaped { pos, text, .. } => {
                    if text.as_str() == "." {
                        positions.push(pos.x.0);
                    }
                }
                FrameItem::Group { items, .. }
                | FrameItem::Link { items, .. }
                | FrameItem::Semantic { items, .. } => {
                    p1407_dot_positions(items, positions)
                }
                FrameItem::Line { .. }
                | FrameItem::Glyph { .. }
                | FrameItem::Image { .. }
                | FrameItem::Shape { .. } => {}
            }
        }
    }

    #[test]
    fn p1407_justify_distribui_residuo_sem_criar_copia() {
        let compile = |justify: bool| {
            let world = mock_world(&format!(
                "#set page(width: 180pt, height: auto, margin: 20pt)\nA#box(width: 101pt, repeat(\".\", gap: 7pt, justify: {justify}))B\n"
            ));
            let mut timings = Timings::default();
            let doc = compile_to_paged_document_full_error(
                &world,
                &world.source,
                false,
                &mut timings,
            )
            .0
            .expect("justify P1407 deve compilar");
            let mut positions = Vec::new();
            for page in &doc.pages {
                p1407_dot_positions(&page.items, &mut positions);
            }
            positions
        };
        let fixed = compile(false);
        let justified = compile(true);
        assert_eq!(fixed.len(), justified.len());
        assert!(fixed.len() > 1);
        assert!(
            justified.last().unwrap() > fixed.last().unwrap(),
            "fixed={fixed:?}, justified={justified:?}"
        );
    }

    #[allow(deprecated)]
    fn p1408_text_x(items: &[FrameItem], needle: &str) -> Option<f64> {
        for item in items {
            match item {
                FrameItem::Text { pos, text, .. } if text.as_str().contains(needle) => {
                    return Some(pos.x.0);
                }
                FrameItem::TextShaped { pos, text, .. }
                    if text.as_str().contains(needle) =>
                {
                    return Some(pos.x.0);
                }
                FrameItem::Group { items, .. } => {
                    if let Some(x) = p1408_text_x(items, needle) {
                        return Some(x);
                    }
                }
                FrameItem::Link { items, .. } => {
                    if let Some(x) = p1408_text_x(items, needle) {
                        return Some(x);
                    }
                }
                FrameItem::Semantic { items, .. } => {
                    if let Some(x) = p1408_text_x(items, needle) {
                        return Some(x);
                    }
                }
                FrameItem::Text { .. } => {}
                FrameItem::TextShaped { .. } => {}
                FrameItem::Line { .. } => {}
                FrameItem::Glyph { .. } => {}
                FrameItem::Image { .. } => {}
                FrameItem::Shape { .. } => {}
            }
        }
        None
    }

    fn p1408_compile_positions(source: &str) -> (f64, Option<f64>) {
        let world = mock_world(source);
        let mut timings = Timings::default();
        let doc = compile_to_paged_document_full_error(
            &world,
            &world.source,
            false,
            &mut timings,
        )
        .0
        .expect("box fracionária P1408 deve compilar");
        let items = &doc.pages[0].items;
        (
            p1408_text_x(items, "B").expect("suffix B deve existir"),
            p1408_text_x(items, "Y"),
        )
    }

    #[test]
    fn p1408_um_fr_e_fr_negativo_isolado_ocupam_restante() {
        let prefix = "#set page(width: 180pt, height: auto, margin: 10pt)\n";
        let (one_b, _) =
            p1408_compile_positions(&format!("{prefix}A#box(width: 1fr)[X]B\n"));
        let (negative_b, _) =
            p1408_compile_positions(&format!("{prefix}A#box(width: -1fr)[X]B\n"));
        assert!(one_b > 150.0, "one_b={one_b}");
        assert!((one_b - negative_b).abs() < 0.01, "one={one_b}, negative={negative_b}");
    }

    #[test]
    fn p1408_zero_fr_nao_recebe_restante() {
        let (suffix, _) = p1408_compile_positions(concat!(
            "#set page(width: 180pt, height: auto, margin: 10pt)\n",
            "A#box(width: 0fr)[X]B\n",
        ));
        assert!(suffix < 40.0, "suffix={suffix}");
    }

    #[test]
    fn p1408_duas_caixas_preservam_razao_um_para_dois() {
        let (suffix, middle) = p1408_compile_positions(concat!(
            "#set page(width: 180pt, height: auto, margin: 10pt)\n",
            "A#box(width: 1fr)[X]#box(width: 2fr)[Y]B\n",
        ));
        let middle = middle.expect("body Y deve existir");
        assert!(
            (middle - 65.533).abs() < 0.05,
            "middle={middle}, esperado interno=65.533 (PDF vanila=66.274)"
        );
        assert!(suffix > 150.0, "suffix={suffix}");
    }

    #[test]
    fn p1408_soma_zero_termina_sem_distribuicao() {
        let (suffix, _) = p1408_compile_positions(concat!(
            "#set page(width: 180pt, height: auto, margin: 10pt)\n",
            "A#box(width: 1fr)[X]#box(width: -1fr)[Y]B\n",
        ));
        assert!(suffix < 50.0, "suffix={suffix}");
    }

    #[test]
    fn p1408_height_fracionaria_permanece_rejeitada() {
        let world = mock_world("#box(height: 1fr)[X]\n");
        let mut timings = Timings::default();
        let result = compile_to_paged_document_full_error(
            &world,
            &world.source,
            false,
            &mut timings,
        )
        .0;
        let errors =
            result.expect_err("height fracionária deve continuar fora do contrato");
        assert!(errors[0].message.contains("box(height:) espera length"));
    }

    #[test]
    fn p1340_selected_context_preserves_page_numbering_relayout() {
        let world = mock_world("#set page(numbering: (current, total) => [V#current/#total])\n#let c = counter(heading.where())\n#context c.final()\nTexto");
        let (result, _) = compile_to_paged_document_full_error(
            &world,
            &world.source,
            false,
            &mut Timings::default(),
        );
        let document = result.unwrap();
        assert_eq!(frame_items_text(&document.pages[0].foreground), "V1/1");
    }

    #[test]
    fn p1340_reused_unselected_context_keeps_legacy_result() {
        let world = mock_world("#let x = context [x]\n#x #x");
        let (result, _) = compile_to_paged_document_full_error(
            &world,
            &world.source,
            false,
            &mut Timings::default(),
        );
        assert!(result.is_ok(), "{result:?}");
    }

    #[test]
    fn p1340_discovery_keeps_legacy_context_options() {
        let world = mock_world("#context [legacy]");
        let options = context_stabilization::discovered_options_for_test(
            &world,
            &world.source,
            typst_core::entities::compiler_features::Features::html(),
        );
        assert!(!options.is_empty());
        assert!(options.into_iter().all(|(full_error, features)| !full_error
            && features == typst_core::entities::compiler_features::Features::default()));
    }

    fn frame_items_text(items: &[FrameItem]) -> String {
        let mut out = String::new();
        for item in items {
            match item {
                FrameItem::Text { text, .. } | FrameItem::TextShaped { text, .. } => {
                    out.push_str(text)
                }
                FrameItem::Group { items, .. }
                | FrameItem::Link { items, .. }
                | FrameItem::Semantic { items, .. } => {
                    out.push_str(&frame_items_text(items))
                }
                _ => {}
            }
        }
        out
    }

    fn contains_bold_text(items: &[FrameItem], expected: &str) -> bool {
        items.iter().any(|item| match item {
            FrameItem::Text { text, style, .. }
            | FrameItem::TextShaped { text, style, .. } => text == expected && style.bold,
            FrameItem::Group { items, .. }
            | FrameItem::Link { items, .. }
            | FrameItem::Semantic { items, .. } => contains_bold_text(items, expected),
            _ => false,
        })
    }

    #[test]
    fn eval_to_module_retorna_modulo_e_sem_warnings() {
        let w = mock_world("Olá");
        let source = w.source.clone();
        let (result, warnings) = eval_to_module_with_sink(&w, &source);
        assert!(result.is_ok());
        assert!(warnings.is_empty());
    }

    #[test]
    fn eval_to_module_ficheiro_vazio_emite_warning() {
        let w = mock_world("");
        let source = w.source.clone();
        let (result, warnings) = eval_to_module_with_sink(&w, &source);
        assert!(result.is_ok());
        assert_eq!(
            warnings.len(),
            1,
            "pilot do Passo 106 emite warning para ficheiro vazio"
        );
    }

    #[test]
    fn compile_to_pdf_bytes_produz_pdf_valido() {
        let w = mock_world("Texto de teste");
        let source = w.source.clone();
        let (result, _warnings) = compile_to_pdf_bytes(
            &w,
            &source,
            StreamMode::Verbose,
            crate::export::PdfTags::Enabled,
        );
        let pdf = result.expect("compilação deve ter sucesso");
        assert!(!pdf.is_empty(), "bytes PDF devem existir");
        assert_eq!(&pdf[..5], b"%PDF-", "header PDF esperado");
    }

    #[test]
    fn p1159_callback_de_page_numbering_chega_a_margem() {
        let w = mock_world(
            "#set page(numbering: (current, total) => [V#current/#total])\nTexto",
        );
        let source = w.source.clone();
        let mut timings = Timings::default();
        let (result, _) =
            compile_to_paged_document_full_error(&w, &source, false, &mut timings);
        let doc = result.expect("callback de numbering deve compilar");
        let margin = frame_items_text(&doc.pages[0].foreground);
        assert_eq!(margin, "V1/1");
    }

    #[test]
    fn p1159_location_page_numbering_ve_func_no_relayout() {
        let w = mock_world(
            "#set page(numbering: (current, total) => [V#current/#total])\n#context repr(here().page-numbering())",
        );
        let source = w.source.clone();
        let mut timings = Timings::default();
        let (result, _) =
            compile_to_paged_document_full_error(&w, &source, false, &mut timings);
        let doc = result.expect("page-numbering contextual deve compilar");
        assert!(
            matches!(
                &doc.pages[0].numbering,
                Some(typst_core::entities::numbering::Numbering::Func(_))
            ),
            "a página deve preservar o callback de numbering: {:?}",
            doc.pages[0].numbering
        );
        let text = doc.pages[0].plain_text();
        assert!(text.contains("=>"), "repr contextual inesperada: {text:?}");
    }

    #[test]
    fn p1159_callback_usa_counter_logico_de_pagina() {
        let w = mock_world(
            "#counter(page).update(7)\n#set page(numbering: (current, total) => [N#current/#total])\nA#pagebreak()B#pagebreak()C",
        );
        let source = w.source.clone();
        let mut timings = Timings::default();
        let (result, _) =
            compile_to_paged_document_full_error(&w, &source, false, &mut timings);
        let doc = result.expect("counter(page) lógico deve compilar");
        let margins = doc
            .pages
            .iter()
            .map(|page| frame_items_text(&page.foreground))
            .collect::<Vec<_>>();
        assert_eq!(margins, ["N7/9", "N8/9", "N9/9"]);
    }

    #[test]
    fn p1159_referencia_usa_vista_unaria_com_footer_explicito() {
        let w = mock_world(
            "#counter(page).update(7)\n#set page(numbering: (..nums) => [R#nums.pos().first()], footer: [EXPL])\n= Alvo <alvo>\n#pagebreak()\n#ref(<alvo>, form: \"page\")",
        );
        let source = w.source.clone();
        let mut timings = Timings::default();
        let (result, _) =
            compile_to_paged_document_full_error(&w, &source, false, &mut timings);
        let doc = result.expect("referência funcional deve compilar");
        assert!(doc.pages.iter().any(|page| page.plain_text().contains("R7")));
        assert!(doc.pages.iter().all(|page| {
            !page.foreground.iter().any(|item| match item {
                FrameItem::Text { text, .. } | FrameItem::TextShaped { text, .. } => {
                    text.starts_with('R')
                }
                _ => false,
            })
        }));
    }

    #[test]
    fn p1160_aridade_visivel_preserva_erros_do_callback() {
        for (source_text, expected) in [
            ("#set page(numbering: x => [#x])\nX", "unexpected argument"),
            ("#set page(numbering: (a, b, c) => [#a/#b/#c])\nX", "missing argument: c"),
        ] {
            let w = mock_world(source_text);
            let source = w.source.clone();
            let mut timings = Timings::default();
            let (result, _) =
                compile_to_paged_document_full_error(&w, &source, false, &mut timings);
            let errors = result.expect_err("aridade inválida deve falhar");
            assert!(errors.iter().any(|error| error.message.contains(expected)));
        }
    }

    #[test]
    fn p1160_referencia_binaria_falha_sem_total() {
        let w = mock_world(
            "#set page(numbering: (current, total) => [#current/#total])\n= A <a>\n#pagebreak()\n#ref(<a>, form: \"page\")",
        );
        let source = w.source.clone();
        let mut timings = Timings::default();
        let (result, _) =
            compile_to_paged_document_full_error(&w, &source, false, &mut timings);
        let errors = result.expect_err("referência passa somente current");
        assert!(errors
            .iter()
            .any(|error| error.message.contains("missing argument: total")));
    }

    #[test]
    fn p1160_retorno_int_e_none_preservam_morfologia() {
        let w = mock_world(
            "#set page(numbering: (current, total) => 42)\nA\n#pagebreak()\n#set page(numbering: (current, total) => none)\nB",
        );
        let source = w.source.clone();
        let mut timings = Timings::default();
        let (result, _) =
            compile_to_paged_document_full_error(&w, &source, false, &mut timings);
        let doc = result.expect("retornos heterogéneos devem compilar");
        let margins = doc
            .pages
            .iter()
            .map(|page| frame_items_text(&page.foreground))
            .collect::<Vec<_>>();
        assert_eq!(margins, ["42", ""]);
    }

    #[test]
    fn p1160_page_run_restaura_numbering_externo() {
        let w = mock_world(
            "#set page(numbering: \"I\")\n#page(numbering: (current, total) => [X#current/#total])[A#pagebreak()B]\n#pagebreak()\nC",
        );
        let source = w.source.clone();
        let mut timings = Timings::default();
        let (result, _) =
            compile_to_paged_document_full_error(&w, &source, false, &mut timings);
        let doc = result.expect("page-run funcional deve compilar");
        let margins = doc
            .pages
            .iter()
            .map(|page| frame_items_text(&page.foreground))
            .collect::<Vec<_>>();
        assert_eq!(margins, ["X1/3", "X2/3", "III"]);
    }

    #[test]
    fn p1160_callback_preserva_markup_forte() {
        let w = mock_world("#set page(numbering: (a, b) => [*B*])\nX");
        let source = w.source.clone();
        let mut timings = Timings::default();
        let (result, _) =
            compile_to_paged_document_full_error(&w, &source, false, &mut timings);
        let doc = result.expect("markup forte deve compilar");
        assert!(contains_bold_text(&doc.pages[0].foreground, "B"));
    }

    // ── Passo 140B: dispatch font-aware ───────────────────────────────

    use ecow::EcoString;
    use typst_core::entities::font_book::{
        Coverage, FontFlags, FontInfo, FontStretch, FontStyle, FontWeight,
    };
    use typst_core::entities::font_list::{FontFamily, FontList};
    use typst_core::entities::layout_types::{
        FrameItem, PagedDocument, Point, Pt, TextStyle,
    };

    fn text_item_with_font(font: Option<FontList>) -> FrameItem {
        let mut style = TextStyle::regular(Pt(12.0));
        style.font = font;
        FrameItem::Text { pos: Point::ZERO, text: "X".into(), style }
    }

    fn page_with(items: Vec<FrameItem>) -> Page {
        Page {
            width: 100.0,
            height: 100.0,
            numbering: None,
            supplement: typst_core::entities::content::Content::Empty,
            bleed: Default::default(),
            fill: Default::default(),
            background: vec![],
            foreground: vec![],
            items,
        }
    }

    fn font_list(name: &str) -> FontList {
        FontList::single(EcoString::from(name))
    }

    #[test]
    fn first_font_from_doc_documento_vazio_devolve_none() {
        let doc = PagedDocument::new(vec![]);
        assert!(first_font_from_doc(&doc).is_none());
    }

    #[test]
    fn first_font_from_doc_sem_font_devolve_none() {
        let doc = PagedDocument::new(vec![page_with(vec![text_item_with_font(None)])]);
        assert!(first_font_from_doc(&doc).is_none());
    }

    #[test]
    fn first_font_from_doc_com_font_primeira_vence() {
        // Dois items na mesma página com fonts diferentes.
        let doc = PagedDocument::new(vec![page_with(vec![
            text_item_with_font(Some(font_list("Primeira"))),
            text_item_with_font(Some(font_list("Segunda"))),
        ])]);
        let fl = first_font_from_doc(&doc).expect("deve resolver");
        assert_eq!(fl.as_slice()[0].name.as_str(), Some("primeira"));
    }

    #[test]
    fn first_font_from_doc_font_em_pagina_segunda_encontrada() {
        // Página 1 sem font, página 2 com font.
        let doc = PagedDocument::new(vec![
            page_with(vec![text_item_with_font(None)]),
            page_with(vec![text_item_with_font(Some(font_list("Inria Serif")))]),
        ]);
        let fl = first_font_from_doc(&doc).expect("deve encontrar na segunda");
        assert_eq!(fl.as_slice()[0].name.as_str(), Some("inria serif"));
    }

    // ── resolve_font ──────────────────────────────────────────────────

    /// MockWorld com `FontBook` e bytes de font injectados por índice.
    struct FontMockWorld {
        library: Library,
        book: FontBook,
        fonts: Vec<Option<Font>>,
    }
    impl World for FontMockWorld {
        fn library(&self) -> &Library {
            &self.library
        }
        fn book(&self) -> &FontBook {
            &self.book
        }
        fn main(&self) -> FileId {
            FileId::from_raw(NonZeroU16::new(1).unwrap())
        }
        fn source(&self, _: FileId) -> FileResult<Source> {
            Err(FileError::NotFound)
        }
        fn file(&self, _: FileId) -> FileResult<Bytes> {
            Err(FileError::NotFound)
        }
        fn font(&self, i: usize) -> Option<Font> {
            self.fonts.get(i).cloned().flatten()
        }
        fn today(
            &self,
            _: Option<typst_core::entities::duration::Duration>,
        ) -> Option<Datetime> {
            None
        }
    }

    fn font_info(family: &str) -> FontInfo {
        FontInfo {
            family: family.into(),
            variant: FontVariant {
                style: FontStyle::Normal,
                weight: FontWeight::REGULAR,
                stretch: FontStretch::NORMAL,
            },
            flags: FontFlags::default(),
            coverage: Coverage::default(),
        }
    }

    #[test]
    fn resolve_font_match_primeiro_devolve_bytes() {
        let mut book = FontBook::new();
        book.push(font_info("Inria Serif"));
        let bytes_esperados = vec![0x11, 0x22, 0x33];
        let world = FontMockWorld {
            library: Library::new(),
            book,
            fonts: vec![Some(Font::from_data(bytes_esperados.clone()))],
        };
        let fl = font_list("Inria Serif");
        let got = resolve_font(&fl, &FontVariant::default(), world.book(), &world)
            .expect("deve resolver");
        assert_eq!(got, bytes_esperados);
    }

    #[test]
    fn resolve_font_nao_match_devolve_none() {
        let mut book = FontBook::new();
        book.push(font_info("Outra"));
        let world = FontMockWorld {
            library: Library::new(),
            book,
            fonts: vec![Some(Font::from_data(vec![0]))],
        };
        let fl = font_list("Não Existe");
        assert!(
            resolve_font(&fl, &FontVariant::default(), world.book(), &world).is_none()
        );
    }

    #[test]
    fn resolve_font_font_book_vazio_devolve_none() {
        let world = FontMockWorld {
            library: Library::new(),
            book: FontBook::new(),
            fonts: vec![],
        };
        let fl = font_list("Qualquer");
        assert!(
            resolve_font(&fl, &FontVariant::default(), world.book(), &world).is_none()
        );
    }

    // ── Passo 141: array fallback chain ───────────────────────────────

    fn font_list_multi(names: &[&str]) -> FontList {
        let families =
            names.iter().map(|n| FontFamily::new(EcoString::from(*n))).collect();
        FontList::new(families).expect("lista não-vazia")
    }

    #[test]
    fn resolve_font_lista_match_indice_0() {
        let mut book = FontBook::new();
        book.push(font_info("A"));
        let bytes_a = vec![0xAA, 0xAA];
        let world = FontMockWorld {
            library: Library::new(),
            book,
            fonts: vec![Some(Font::from_data(bytes_a.clone()))],
        };
        let fl = font_list_multi(&["A", "B", "C"]);
        let got = resolve_font(&fl, &FontVariant::default(), world.book(), &world)
            .expect("deve resolver");
        assert_eq!(got, bytes_a, "primeira família vence quando existe");
    }

    #[test]
    fn resolve_font_lista_match_indice_1() {
        // FontBook só tem "B"; "X" não resolve, "B" sim.
        let mut book = FontBook::new();
        book.push(font_info("B"));
        let bytes_b = vec![0xBB, 0xBB];
        let world = FontMockWorld {
            library: Library::new(),
            book,
            fonts: vec![Some(Font::from_data(bytes_b.clone()))],
        };
        let fl = font_list_multi(&["X", "B", "C"]);
        let got = resolve_font(&fl, &FontVariant::default(), world.book(), &world)
            .expect("deve resolver via B");
        assert_eq!(got, bytes_b, "segunda família vence quando primeira falha");
    }

    #[test]
    fn resolve_font_lista_match_indice_2() {
        // FontBook só tem "C"; "X" e "Y" não resolvem.
        let mut book = FontBook::new();
        book.push(font_info("C"));
        let bytes_c = vec![0xCC, 0xCC];
        let world = FontMockWorld {
            library: Library::new(),
            book,
            fonts: vec![Some(Font::from_data(bytes_c.clone()))],
        };
        let fl = font_list_multi(&["X", "Y", "C"]);
        let got = resolve_font(&fl, &FontVariant::default(), world.book(), &world)
            .expect("deve resolver via C");
        assert_eq!(got, bytes_c, "terceira família vence quando duas primeiras falham");
    }

    #[test]
    fn resolve_font_lista_sem_match_devolve_none() {
        // FontBook só tem "Outra"; nenhuma família da lista resolve.
        let mut book = FontBook::new();
        book.push(font_info("Outra"));
        let world = FontMockWorld {
            library: Library::new(),
            book,
            fonts: vec![Some(Font::from_data(vec![0xFF]))],
        };
        let fl = font_list_multi(&["X", "Y", "Z"]);
        assert!(
            resolve_font(&fl, &FontVariant::default(), world.book(), &world).is_none(),
            "nenhuma família resolve → fallback Helvetica via None"
        );
    }

    // ── Passo 146: multi-font per document ────────────────────────────

    #[test]
    fn collect_fonts_from_doc_documento_vazio_devolve_vazio() {
        let doc = PagedDocument::new(vec![]);
        let world = mock_world("");
        assert!(collect_fonts_from_doc(&doc, &world).is_empty());
    }

    #[test]
    fn collect_fonts_from_doc_uma_font_devolve_unitario() {
        let doc = PagedDocument::new(vec![page_with(vec![text_item_with_font(Some(
            font_list("Inria"),
        ))])]);
        let world = mock_world("");
        let collected = collect_fonts_from_doc(&doc, &world);
        assert_eq!(collected.len(), 1);
        assert_eq!(collected[0].0.as_slice()[0].name.as_str(), Some("inria"));
    }

    #[test]
    fn collect_fonts_from_doc_duas_distintas_devolve_par_em_ordem() {
        let doc = PagedDocument::new(vec![page_with(vec![
            text_item_with_font(Some(font_list("Primeira"))),
            text_item_with_font(Some(font_list("Segunda"))),
        ])]);
        let world = mock_world("");
        let collected = collect_fonts_from_doc(&doc, &world);
        assert_eq!(collected.len(), 2);
        assert_eq!(collected[0].0.as_slice()[0].name.as_str(), Some("primeira"));
        assert_eq!(collected[1].0.as_slice()[0].name.as_str(), Some("segunda"));
    }

    #[test]
    fn collect_fonts_from_doc_duas_iguais_dispersas_dedup() {
        // "A" e "B" intercalados em duas páginas: dedup deve produzir
        // [A, B] em ordem de primeira ocorrência.
        let doc = PagedDocument::new(vec![
            page_with(vec![
                text_item_with_font(Some(font_list("A"))),
                text_item_with_font(Some(font_list("B"))),
            ]),
            page_with(vec![
                text_item_with_font(Some(font_list("A"))),
                text_item_with_font(Some(font_list("B"))),
            ]),
        ]);
        let world = mock_world("");
        let collected = collect_fonts_from_doc(&doc, &world);
        assert_eq!(
            collected.len(),
            2,
            "dedup estrutural: A e B aparecem cada um uma vez no resultado"
        );
        assert_eq!(collected[0].0.as_slice()[0].name.as_str(), Some("a"));
        assert_eq!(collected[1].0.as_slice()[0].name.as_str(), Some("b"));
    }

    // resolve_fonts (plural)

    #[test]
    fn resolve_fonts_todos_resolvem() {
        let mut book = FontBook::new();
        book.push(font_info("A"));
        book.push(font_info("B"));
        let world = FontMockWorld {
            library: Library::new(),
            book,
            fonts: vec![
                Some(Font::from_data(vec![0xAA])),
                Some(Font::from_data(vec![0xBB])),
            ],
        };
        let inputs = vec![
            (font_list("A"), FontVariant::default(), FontVariations::default()),
            (font_list("B"), FontVariant::default(), FontVariations::default()),
        ];
        let out = resolve_fonts(&inputs, world.book(), &world);
        assert_eq!(out.len(), 2);
        assert_eq!(out[0].1, vec![0xAA]);
        assert_eq!(out[1].1, vec![0xBB]);
    }

    #[test]
    fn resolve_fonts_alguns_nao_resolvem_filtrados() {
        // FontBook só tem "A"; "B" não resolve → filtrado silenciosamente.
        let mut book = FontBook::new();
        book.push(font_info("A"));
        let world = FontMockWorld {
            library: Library::new(),
            book,
            fonts: vec![Some(Font::from_data(vec![0xAA]))],
        };
        let inputs = vec![
            (font_list("A"), FontVariant::default(), FontVariations::default()),
            (font_list("B"), FontVariant::default(), FontVariations::default()),
        ];
        let out = resolve_fonts(&inputs, world.book(), &world);
        assert_eq!(out.len(), 1, "B silenciosamente filtrado");
        assert_eq!(out[0].1, vec![0xAA]);
    }

    #[test]
    fn resolve_fonts_nenhum_resolve_devolve_vazio() {
        let mut book = FontBook::new();
        book.push(font_info("Outra"));
        let world = FontMockWorld {
            library: Library::new(),
            book,
            fonts: vec![Some(Font::from_data(vec![0]))],
        };
        let inputs = vec![
            (font_list("X"), FontVariant::default(), FontVariations::default()),
            (font_list("Y"), FontVariant::default(), FontVariations::default()),
        ];
        assert!(resolve_fonts(&inputs, world.book(), &world).is_empty());
    }

    // ── Passo 407: regex keys em FontList (DEBT-52) ─────────────────────

    use typst_core::entities::font_list::FontNamePattern;
    use typst_core::entities::regex::Regex;

    fn font_list_regex(pattern: &str) -> FontList {
        FontList::new(vec![FontFamily::new_regex(Regex::new(pattern).unwrap(), vec![])])
            .expect("lista não-vazia")
    }

    #[test]
    fn resolve_font_regex_key_match() {
        let mut book = FontBook::new();
        book.push(font_info("Name Bold"));
        let bytes = vec![0xDD, 0xDD];
        let world = FontMockWorld {
            library: Library::new(),
            book,
            fonts: vec![Some(Font::from_data(bytes.clone()))],
        };
        let fl = font_list_regex("Name.*");
        let got = resolve_font(&fl, &FontVariant::default(), world.book(), &world)
            .expect("deve resolver via regex");
        assert_eq!(got, bytes);
    }

    #[test]
    fn resolve_font_regex_key_no_match() {
        let mut book = FontBook::new();
        book.push(font_info("Other"));
        let world = FontMockWorld {
            library: Library::new(),
            book,
            fonts: vec![Some(Font::from_data(vec![0xEE]))],
        };
        let fl = font_list_regex("Name.*");
        assert!(
            resolve_font(&fl, &FontVariant::default(), world.book(), &world).is_none()
        );
    }

    #[test]
    fn resolve_font_literal_still_works_after_p407() {
        let mut book = FontBook::new();
        book.push(font_info("Inria Serif"));
        let bytes = vec![0x11, 0x22];
        let world = FontMockWorld {
            library: Library::new(),
            book,
            fonts: vec![Some(Font::from_data(bytes.clone()))],
        };
        let fl = font_list("Inria Serif");
        let got = resolve_font(&fl, &FontVariant::default(), world.book(), &world)
            .expect("literal continua a resolver");
        assert_eq!(got, bytes);
    }

    // ── P711: collect_context_blocks acumula StyleChain da posição ────

    use typst_core::entities::func::Func;
    use typst_core::entities::style::{Style, Styles};
    use typst_core::entities::value::Value;

    fn dummy_context_block(id: u64) -> Content {
        let closure = Func::native("dummy", |_ctx, _args, _world, _file| Ok(Value::None));
        Content::ContextBlock(Arc::new(ContextBlockElem { id, closure }))
    }

    #[test]
    fn collect_context_blocks_acumula_size_de_set_ancestral() {
        let styles = Styles::from_iter([Style::Size(Pt(20.0))]);
        let content = Content::Styled(Box::new(dummy_context_block(1)), styles);
        let blocks = collect_context_blocks(&content, &StyleChain::default_chain());
        let (_, chain) = blocks.get(&1).expect("bloco 1 deve ter sido colectado");
        assert_eq!(
            chain.size(),
            20.0,
            "P711: a cadeia colectada tem de reflectir o #set ancestral, não o default"
        );
    }

    #[test]
    fn collect_context_blocks_sem_set_mantem_default() {
        let content = dummy_context_block(2);
        let blocks = collect_context_blocks(&content, &StyleChain::default_chain());
        let (_, chain) = blocks.get(&2).expect("bloco 2 deve ter sido colectado");
        assert_eq!(
            chain.size(),
            11.0,
            "sem #set ancestral, o tamanho por defeito (11pt) fica inalterado"
        );
    }

    #[test]
    fn collect_context_blocks_sets_aninhados_o_mais_interno_vence() {
        // #set text(size: 20pt) [ #set text(size: 30pt) [ context ] ]
        let inner_styles = Styles::from_iter([Style::Size(Pt(30.0))]);
        let inner = Content::Styled(Box::new(dummy_context_block(3)), inner_styles);
        let outer_styles = Styles::from_iter([Style::Size(Pt(20.0))]);
        let content = Content::Styled(Box::new(inner), outer_styles);
        let blocks = collect_context_blocks(&content, &StyleChain::default_chain());
        let (_, chain) = blocks.get(&3).expect("bloco 3 deve ter sido colectado");
        assert_eq!(
            chain.size(),
            30.0,
            "o #set mais interno (mais próximo do bloco) tem de vencer"
        );
    }

    #[test]
    fn collect_context_blocks_multiplos_blocos_em_sequence_cadeias_independentes() {
        let a = Content::Styled(
            Box::new(dummy_context_block(4)),
            Styles::from_iter([Style::Size(Pt(14.0))]),
        );
        let b = dummy_context_block(5);
        let content = Content::sequence(vec![a, b]);
        let blocks = collect_context_blocks(&content, &StyleChain::default_chain());
        assert_eq!(blocks.get(&4).unwrap().1.size(), 14.0);
        assert_eq!(
            blocks.get(&5).unwrap().1.size(),
            11.0,
            "bloco irmão fora do Styled não deve herdar o #set do outro ramo"
        );
    }

    // ── P1467 (RED — Agente B) — resolução documental/CLI de
    // `format.pdf.tagged`/`format.pdf.pretty` ao nível de integração de
    // pipeline. A entry point pública sugerida pelo L0
    // (`infra/pipeline.md` §P1467 "API e resolução") ainda não existe
    // neste baseline — a suite inteira falha por erro de compilação até
    // `compile_to_pdf_bytes_full_error_and_document_id_with_features_and_tagged_and_pretty_override`
    // ser materializada (Agente A, em paralelo).

    fn has_tagged_pdf_morphology(pdf: &[u8]) -> bool {
        let raw = String::from_utf8_lossy(pdf);
        raw.contains("/StructTreeRoot") && raw.contains("/MarkInfo")
    }

    fn newline_count(pdf: &[u8]) -> usize {
        pdf.iter().filter(|&&byte| byte == b'\n').count()
    }

    fn compile_pdf_with_overrides(
        source_text: &str,
        pdf_tagged_override: Option<bool>,
        pretty_override: Option<bool>,
    ) -> Vec<u8> {
        let world = mock_world(source_text);
        let (result, _warnings) =
            compile_to_pdf_bytes_full_error_and_document_id_with_features_and_tagged_and_pretty_override(
                &world,
                &world.source,
                false,
                None,
                StreamMode::Verbose,
                pdf_tagged_override,
                typst_core::entities::compiler_features::Features::default(),
                pretty_override,
            );
        result.expect("compilação PDF P1467 deve ter sucesso")
    }

    #[test]
    fn p1467_pipeline_absence_of_both_resolves_document_auto_to_tagged() {
        let pdf = compile_pdf_with_overrides("Hello", None, None);
        assert!(
            has_tagged_pdf_morphology(&pdf),
            "auto sem seleção de páginas deve resolver para tagueado"
        );
    }

    #[test]
    fn p1467_pipeline_document_tagged_false_untags_without_override() {
        let pdf = compile_pdf_with_overrides(
            "#set format.pdf(tagged: false)\nHello",
            None,
            None,
        );
        assert!(!has_tagged_pdf_morphology(&pdf));
    }

    #[test]
    fn p1467_pipeline_document_tagged_true_is_explicit_and_tags() {
        let pdf = compile_pdf_with_overrides(
            "#set format.pdf(tagged: true)\nHello",
            None,
            None,
        );
        assert!(has_tagged_pdf_morphology(&pdf));
    }

    #[test]
    fn p1467_pipeline_override_false_wins_over_document_tagged_true() {
        let pdf = compile_pdf_with_overrides(
            "#set format.pdf(tagged: true)\nHello",
            Some(false),
            None,
        );
        assert!(!has_tagged_pdf_morphology(&pdf));
    }

    #[test]
    fn p1467_pipeline_override_true_wins_over_document_tagged_false() {
        let pdf = compile_pdf_with_overrides(
            "#set format.pdf(tagged: false)\nHello",
            Some(true),
            None,
        );
        assert!(has_tagged_pdf_morphology(&pdf));
    }

    #[test]
    fn p1467_pipeline_pretty_override_wins_over_document_pretty_false() {
        let compact = compile_pdf_with_overrides(
            "#set format.pdf(pretty: false)\nHello",
            None,
            None,
        );
        let pretty = compile_pdf_with_overrides(
            "#set format.pdf(pretty: false)\nHello",
            None,
            Some(true),
        );
        assert!(
            newline_count(&pretty) > newline_count(&compact),
            "--pretty deve vencer o documento e produzir mais newlines estruturais"
        );
    }

    #[test]
    fn p1467_pipeline_pretty_override_false_wins_over_document_pretty_true() {
        let baseline = compile_pdf_with_overrides("Hello", None, None);
        let overridden = compile_pdf_with_overrides(
            "#set format.pdf(pretty: true)\nHello",
            None,
            Some(false),
        );
        assert_eq!(
            newline_count(&overridden),
            newline_count(&baseline),
            "override explícito false deve produzir a mesma forma compacta do baseline"
        );
    }

    #[test]
    fn p1467_pipeline_document_pretty_true_without_override() {
        let compact = compile_pdf_with_overrides("Hello", None, None);
        let pretty = compile_pdf_with_overrides(
            "#set format.pdf(pretty: true)\nHello",
            None,
            None,
        );
        assert!(newline_count(&pretty) > newline_count(&compact));
    }

    #[test]
    fn p1467_pipeline_absence_of_both_flags_is_compact_by_default() {
        let baseline = compile_pdf_with_overrides("Hello", None, None);
        let explicit_false = compile_pdf_with_overrides(
            "#set format.pdf(pretty: false)\nHello",
            None,
            None,
        );
        assert_eq!(newline_count(&baseline), newline_count(&explicit_false));
    }

    #[test]
    fn p1467_pipeline_tagged_and_pretty_combine_independently() {
        let untagged_pretty = compile_pdf_with_overrides(
            "#set format.pdf(tagged: false)\n#set format.pdf(pretty: true)\nHello",
            None,
            None,
        );
        let untagged_compact = compile_pdf_with_overrides(
            "#set format.pdf(tagged: false)\nHello",
            None,
            None,
        );

        assert!(!has_tagged_pdf_morphology(&untagged_pretty));
        assert!(!has_tagged_pdf_morphology(&untagged_compact));
        assert!(newline_count(&untagged_pretty) > newline_count(&untagged_compact));
    }

    #[test]
    fn p1467_svg_output_is_neutral_to_pdf_format_options() {
        let world_without = mock_world("#rect(width: 10pt, height: 10pt)");
        let (without, _, _) =
            compile_to_svg_string_with_timings_full_error_and_features_and_pretty_override(
                &world_without,
                &world_without.source,
                false,
                typst_core::entities::compiler_features::Features::default(),
                None,
            );

        let world_with_pdf = mock_world(
            "#set format.pdf(tagged: false, pretty: true)\n#rect(width: 10pt, height: 10pt)",
        );
        let (with_pdf_options, _, _) =
            compile_to_svg_string_with_timings_full_error_and_features_and_pretty_override(
                &world_with_pdf,
                &world_with_pdf.source,
                false,
                typst_core::entities::compiler_features::Features::default(),
                None,
            );

        assert_eq!(
            without.expect("SVG sem opções PDF deve compilar"),
            with_pdf_options
                .expect("SVG com #set format.pdf deve compilar e ser idêntico"),
            "P1467: format.pdf não pode influenciar o export SVG"
        );
    }

    // ── P1468 (RED — Agente B) — resolução de `format.pdf.pages` e
    // interação com `tagged`. `resolve_pdf_pages_and_tags` (contrato de
    // `infra/pipeline.md` §P1468) ainda não existe neste baseline (só
    // `resolve_pdf_tags`, P1467); a nova entry point pública
    // `..._tagged_and_pretty_and_pages_override` também não existe. A
    // suite inteira falha por erro de compilação até ambas serem
    // materializadas (Agente A, em paralelo).
    use typst_core::entities::document_format_options::{PageRange, PageRanges};

    fn single_page(n: usize) -> PageRanges {
        let p = std::num::NonZeroUsize::new(n).unwrap();
        PageRanges::new(vec![PageRange { start: Some(p), end: Some(p) }])
    }

    #[test]
    fn p1468_resolve_pages_absent_matches_p1467_tagged_resolution() {
        let auto = DocumentFormatOptions::default();
        let (pages, tags, diagnostics) = resolve_pdf_pages_and_tags(None, None, &auto)
            .expect("sem pages, sem erro (comportamento P1467)");
        assert_eq!(pages, None);
        assert_eq!(tags, PdfTags::Enabled, "auto sem pages continua tagueado (P1467)");
        assert!(diagnostics.is_empty());

        let mut tagged_false = DocumentFormatOptions::default();
        tagged_false.pdf.tagged = PdfTagged::Explicit(false);
        let (pages, tags, diagnostics) =
            resolve_pdf_pages_and_tags(None, None, &tagged_false)
                .expect("sem pages, sem erro (comportamento P1467)");
        assert_eq!(pages, None);
        assert_eq!(tags, PdfTags::Disabled);
        assert!(diagnostics.is_empty());
    }

    #[test]
    fn p1468_resolve_pages_present_with_document_tagged_explicit_true_errors() {
        let mut options = DocumentFormatOptions::default();
        options.pdf.tagged = PdfTagged::Explicit(true);
        let errors = resolve_pdf_pages_and_tags(Some(single_page(1)), None, &options)
            .expect_err("documento pede tagged:true + pages deve falhar");
        assert!(
            errors.iter().any(|e| e
                .message
                .contains("cannot enable tagged PDF and export a page range")),
            "{errors:?}"
        );
    }

    #[test]
    fn p1468_resolve_pages_present_with_auto_tagged_warns_and_disables() {
        let options = DocumentFormatOptions::default(); // tagged == Auto
        let (pages, tags, diagnostics) =
            resolve_pdf_pages_and_tags(Some(single_page(1)), None, &options)
                .expect("auto + pages não deve ser erro, só warning");
        assert_eq!(pages, Some(single_page(1)));
        assert_eq!(tags, PdfTags::Disabled);
        assert!(
            diagnostics.iter().any(|d| d
                .message
                .contains("exporting a page range disables PDF tagging")),
            "{diagnostics:?}"
        );
        let hints: Vec<_> = diagnostics.iter().flat_map(|d| d.hints.iter()).collect();
        assert!(
            hints
                .iter()
                .any(|h| h.contains("the resulting PDF will be inaccessible")),
            "{hints:?}"
        );
        assert!(
            hints
                .iter()
                .any(|h| h.contains("set `pdf(tagged: false)` to silence this warning")),
            "{hints:?}"
        );
    }

    #[test]
    fn p1468_resolve_pages_present_with_document_tagged_explicit_false_is_silent() {
        let mut options = DocumentFormatOptions::default();
        options.pdf.tagged = PdfTagged::Explicit(false);
        let (pages, tags, diagnostics) =
            resolve_pdf_pages_and_tags(Some(single_page(1)), None, &options)
                .expect("tagged:false + pages não deve falhar");
        assert_eq!(pages, Some(single_page(1)));
        assert_eq!(tags, PdfTags::Disabled);
        assert!(diagnostics.is_empty(), "{diagnostics:?}");
    }

    #[test]
    fn p1468_resolve_pages_cli_override_wins_over_document_pages() {
        let mut options = DocumentFormatOptions::default();
        options.pdf.tagged = PdfTagged::Explicit(false);
        options.pdf.pages = Some(single_page(5));
        let (pages, _tags, _diagnostics) =
            resolve_pdf_pages_and_tags(Some(single_page(1)), None, &options)
                .expect("override deve vencer, sem erro");
        assert_eq!(
            pages,
            Some(single_page(1)),
            "override CLI de pages deve vencer o documento"
        );
    }

    #[test]
    fn p1468_resolve_pages_falls_back_to_document_then_none() {
        let mut with_document_pages = DocumentFormatOptions::default();
        with_document_pages.pdf.tagged = PdfTagged::Explicit(false);
        with_document_pages.pdf.pages = Some(single_page(3));
        let (pages, _, _) = resolve_pdf_pages_and_tags(None, None, &with_document_pages)
            .expect("documento define pages, sem override");
        assert_eq!(pages, Some(single_page(3)));

        let neither = DocumentFormatOptions::default();
        let (pages, _, _) = resolve_pdf_pages_and_tags(None, None, &neither)
            .expect("nenhum define pages");
        assert_eq!(pages, None);
    }

    #[test]
    fn p1468_resolve_pages_tagged_override_still_applies_when_pages_absent() {
        // Regressão P1467: `tagged_override` continua a funcionar quando
        // `pages` não está presente (comportamento idêntico a
        // `resolve_pdf_tags`).
        let options = DocumentFormatOptions::default();
        let (pages, tags, diagnostics) =
            resolve_pdf_pages_and_tags(None, Some(false), &options)
                .expect("override tagged sem pages não deve falhar");
        assert_eq!(pages, None);
        assert_eq!(tags, PdfTags::Disabled);
        assert!(diagnostics.is_empty());
    }

    #[test]
    fn p1468_pipeline_new_entry_point_with_none_pages_matches_old_entry_point() {
        let world = mock_world("#set format.pdf(pretty: true)\nHello");
        let (old_result, _old_warnings) =
            compile_to_pdf_bytes_full_error_and_document_id_with_features_and_tagged_and_pretty_override(
                &world,
                &world.source,
                false,
                None,
                StreamMode::Verbose,
                None,
                typst_core::entities::compiler_features::Features::default(),
                None,
            );
        let (new_result, _new_warnings) =
            compile_to_pdf_bytes_full_error_and_document_id_with_features_and_tagged_and_pretty_and_pages_override(
                &world,
                &world.source,
                false,
                None,
                StreamMode::Verbose,
                None,
                typst_core::entities::compiler_features::Features::default(),
                None,
                None,
            );
        assert_eq!(
            old_result.expect("compilação antiga deve suceder"),
            new_result.expect("compilação nova deve suceder"),
            "P1468: entry point antiga (P1467) deve continuar byte-idêntica quando pages=None"
        );
    }

    #[test]
    fn p1468_pipeline_pages_override_without_document_tagged_decision_disables_tags_with_warning(
    ) {
        let world = mock_world("Hello\n#pagebreak()\nWorld");
        let (result, warnings) =
            compile_to_pdf_bytes_full_error_and_document_id_with_features_and_tagged_and_pretty_and_pages_override(
                &world,
                &world.source,
                false,
                None,
                StreamMode::Verbose,
                None,
                typst_core::entities::compiler_features::Features::default(),
                None,
                Some(single_page(1)),
            );
        let pdf =
            result.expect("pages sem tagged explícito não deve falhar a compilação");
        assert!(
            !has_tagged_pdf_morphology(&pdf),
            "auto + pages deve resolver destagueado (warning, não erro)"
        );
        assert!(
            warnings.iter().any(|w| w
                .message
                .contains("exporting a page range disables PDF tagging")),
            "{warnings:?}"
        );
    }

    #[test]
    fn p1468_pipeline_pages_override_with_document_tagged_true_errors_end_to_end() {
        let world = mock_world("#set format.pdf(tagged: true)\nHello");
        let (result, _warnings) =
            compile_to_pdf_bytes_full_error_and_document_id_with_features_and_tagged_and_pretty_and_pages_override(
                &world,
                &world.source,
                false,
                None,
                StreamMode::Verbose,
                None,
                typst_core::entities::compiler_features::Features::default(),
                None,
                Some(single_page(1)),
            );
        let errors =
            result.expect_err("documento pede tagged:true + pages CLI deve falhar");
        assert!(
            errors.iter().any(|e| e
                .message
                .contains("cannot enable tagged PDF and export a page range")),
            "{errors:?}"
        );
    }

    // ── P1469 (RED — Agente B) — resolução de `format.pdf.standard` e
    // interação com `tagged`/`pages` (contrato de linguagem, escopo
    // reduzido — ver `infra/pipeline.md` P1469 e
    // `entities/document_format_options.md` P1469). `PdfStandard`/
    // `PdfStandards`/`PdfVersion`/`PdfArchival` (entities) e
    // `resolve_pdf_pages_tags_and_standards` ainda não existem neste
    // baseline (`65e68ad45dcbfd8db00aeda78d0d2fbaaadefb27`); este módulo
    // falha por erro de compilação até o carrier P1469 (mesmo gate
    // ADR-0127) ser materializado por outro agente em paralelo.
    use typst_core::entities::document_format_options::{
        PdfArchival, PdfStandard, PdfStandards, PdfVersion,
    };

    fn standards(names: &[PdfStandard]) -> PdfStandards {
        PdfStandards::new(names.iter().copied())
            .expect("combinação de teste deve ser válida")
    }

    #[test]
    fn p1469_resolve_standard_absent_matches_p1468_pages_and_tags_behavior() {
        // Sem `standard` (nem override, nem documento): comportamento
        // (pages, tags) idêntico a `resolve_pdf_pages_and_tags` (P1468), e
        // `standards_final == PdfStandards::default()` (versão efetiva
        // 1.7, sem archival/accessibility).
        let options = DocumentFormatOptions::default();
        let (pages, tags, standards_final, diagnostics) =
            resolve_pdf_pages_tags_and_standards(None, None, None, &options)
                .expect("sem standard, sem erro");
        assert_eq!(pages, None);
        assert_eq!(tags, PdfTags::Enabled);
        assert_eq!(standards_final, PdfStandards::default());
        assert_eq!(standards_final.version, PdfVersion::V1_7);
        assert!(diagnostics.is_empty());
    }

    #[test]
    fn p1469_resolve_standard_non_accessible_profile_with_tagged_false_has_no_error() {
        let mut options = DocumentFormatOptions::default();
        options.pdf.tagged = PdfTagged::Explicit(false);
        let (_, tags, standards_final, diagnostics) =
            resolve_pdf_pages_tags_and_standards(
                None,
                None,
                Some(standards(&[PdfStandard::A2b])),
                &options,
            )
            .expect("a-2b (não acessível) + tagged:false não deve falhar");
        assert_eq!(tags, PdfTags::Disabled);
        assert_eq!(standards_final.archival, Some(PdfArchival::A2b));
        assert!(diagnostics.is_empty());
    }

    #[test]
    fn p1469_resolve_standard_accessible_profile_with_document_tagged_explicit_false_errors_without_pages_hint(
    ) {
        let mut options = DocumentFormatOptions::default();
        options.pdf.tagged = PdfTagged::Explicit(false);
        let errors = resolve_pdf_pages_tags_and_standards(
            None,
            None,
            Some(standards(&[PdfStandard::A1a])),
            &options,
        )
        .expect_err("perfil acessível + tagged:false documental deve falhar");
        assert!(
            errors.iter().any(|e| e
                .message
                .contains("cannot set `pdf(tagged: false)` when exporting a")),
            "{errors:?}"
        );
        let hints: Vec<_> = errors.iter().flat_map(|e| e.hints.iter()).collect();
        assert!(
            hints.is_empty(),
            "sem `pages` ativo, não deve haver hint de pages: {hints:?}"
        );
    }

    #[test]
    fn p1469_resolve_standard_accessible_profile_with_pages_active_errors_with_pages_hint(
    ) {
        let options = DocumentFormatOptions::default(); // tagged == Auto
        let errors = resolve_pdf_pages_tags_and_standards(
            Some(single_page(1)),
            None,
            Some(standards(&[PdfStandard::Ua1])),
            &options,
        )
        .expect_err("perfil acessível + pages ativo deve falhar");
        assert!(
            errors.iter().any(|e| e
                .message
                .contains("cannot set `pdf(tagged: false)` when exporting a")),
            "{errors:?}"
        );
        let hints: Vec<_> = errors.iter().flat_map(|e| e.hints.iter()).collect();
        assert!(
            hints
                .iter()
                .any(|h| h
                    .contains("using `pdf(pages: ...)` implies `pdf(tagged: false)`")),
            "causa raiz foi `pages`, hint deve aparecer: {hints:?}"
        );
    }

    #[test]
    fn p1469_resolve_standard_accessible_profile_with_tagged_true_or_no_decision_has_no_error(
    ) {
        // `tagged: true` explícito (documento) + perfil acessível: sem erro.
        let mut explicit_true = DocumentFormatOptions::default();
        explicit_true.pdf.tagged = PdfTagged::Explicit(true);
        let (_, tags, _, diagnostics) = resolve_pdf_pages_tags_and_standards(
            None,
            None,
            Some(standards(&[PdfStandard::A2a])),
            &explicit_true,
        )
        .expect("tagged:true explícito + perfil acessível não deve falhar");
        assert_eq!(tags, PdfTags::Enabled);
        assert!(diagnostics.is_empty());

        // Documento sem decisão (Auto) + perfil acessível + sem `pages`:
        // `tagged` resolve `Enabled` por P1467, sem conflito.
        let no_decision = DocumentFormatOptions::default();
        let (_, tags, _, diagnostics) = resolve_pdf_pages_tags_and_standards(
            None,
            None,
            Some(standards(&[PdfStandard::A2a])),
            &no_decision,
        )
        .expect("auto sem pages + perfil acessível não deve falhar");
        assert_eq!(tags, PdfTags::Enabled);
        assert!(diagnostics.is_empty());
    }

    #[test]
    fn p1469_resolve_standard_precedence_cli_override_wins_over_document() {
        let mut options = DocumentFormatOptions::default();
        options.pdf.standard = Some(standards(&[PdfStandard::A1b]));

        let (_, _, standards_final, _) = resolve_pdf_pages_tags_and_standards(
            None,
            None,
            Some(standards(&[PdfStandard::A4])),
            &options,
        )
        .expect("a-4 sozinho não deve falhar (não acessível)");
        assert_eq!(
            standards_final.archival,
            Some(PdfArchival::A4),
            "override CLI de standard deve vencer o documento"
        );

        // Sem override: cai para o documento.
        let (_, _, standards_final, _) =
            resolve_pdf_pages_tags_and_standards(None, None, None, &options)
                .expect("documento a-1b sozinho não deve falhar");
        assert_eq!(standards_final.archival, Some(PdfArchival::A1b));

        // Nenhum dos dois: cai para o default.
        let neither = DocumentFormatOptions::default();
        let (_, _, standards_final, _) =
            resolve_pdf_pages_tags_and_standards(None, None, None, &neither)
                .expect("nenhum define standard");
        assert_eq!(standards_final, PdfStandards::default());
    }

    #[test]
    fn p1469_resolve_standard_new_function_with_none_standard_matches_old_resolve_pdf_pages_and_tags(
    ) {
        // Regressão: com `standard_override = None` e documento sem
        // `pdf.standard`, o resultado (pages, tags, diagnostics) deve ser
        // idêntico ao de `resolve_pdf_pages_and_tags` (P1468), e o
        // `PdfStandards` efetivo deve ser o default.
        let options = DocumentFormatOptions::default();
        let (old_pages, old_tags, old_diagnostics) =
            resolve_pdf_pages_and_tags(Some(single_page(2)), None, &options)
                .expect("P1468 não deve falhar");
        let (new_pages, new_tags, standards_final, new_diagnostics) =
            resolve_pdf_pages_tags_and_standards(
                Some(single_page(2)),
                None,
                None,
                &options,
            )
            .expect("P1469 com standard=None não deve falhar");

        assert_eq!(old_pages, new_pages);
        assert_eq!(old_tags, new_tags);
        assert_eq!(
            old_diagnostics.iter().map(|d| d.message.clone()).collect::<Vec<_>>(),
            new_diagnostics.iter().map(|d| d.message.clone()).collect::<Vec<_>>(),
        );
        assert_eq!(standards_final, PdfStandards::default());
    }

    #[test]
    fn p1469_pipeline_new_entry_point_with_none_standard_matches_old_entry_point() {
        // Mesmo padrão de `p1468_pipeline_new_entry_point_with_none_pages_matches_old_entry_point`:
        // a entry point antiga (`..._and_pages_override`, P1468) deve
        // continuar byte-idêntica quando a nova entry point
        // (`..._and_pages_and_standard_override`, P1469) recebe
        // `standard_override = None`.
        let world = mock_world("#set format.pdf(pretty: true)\nHello");
        let (old_result, _old_warnings) =
            compile_to_pdf_bytes_full_error_and_document_id_with_features_and_tagged_and_pretty_and_pages_override(
                &world,
                &world.source,
                false,
                None,
                StreamMode::Verbose,
                None,
                typst_core::entities::compiler_features::Features::default(),
                None,
                None,
            );
        let (new_result, _new_warnings) =
            compile_to_pdf_bytes_full_error_and_document_id_with_features_and_tagged_and_pretty_and_pages_and_standard_override(
                &world,
                &world.source,
                false,
                None,
                StreamMode::Verbose,
                None,
                typst_core::entities::compiler_features::Features::default(),
                None,
                None,
                None,
            );
        assert_eq!(
            old_result.expect("compilação antiga deve suceder"),
            new_result.expect("compilação nova deve suceder"),
            "P1469: entry point antiga (P1468) deve continuar byte-idêntica quando standard=None"
        );
    }

    fn p1480_compile_with_standard(
        source: &str,
        standard: Option<PdfStandards>,
    ) -> Result<Vec<u8>, Vec<SourceDiagnostic>> {
        let world = mock_world(source);
        let (result, _warnings) =
            compile_to_pdf_bytes_full_error_and_document_id_with_features_and_tagged_and_pretty_and_pages_and_standard_override(
                &world,
                &world.source,
                false,
                None,
                StreamMode::Verbose,
                None,
                typst_core::entities::compiler_features::Features::default(),
                None,
                None,
                standard,
            );
        result
    }

    #[test]
    fn p1480_ua1_sem_titulo_retorna_diagnostico_exato() {
        let errors =
            p1480_compile_with_standard("Hello", Some(standards(&[PdfStandard::Ua1])))
                .expect_err("ua-1 sem document(title:) deve falhar");

        assert_eq!(errors.len(), 1, "esperado um único diagnóstico: {errors:?}");
        assert_eq!(errors[0].message, "PDF/UA-1 error: missing document title");
        assert_eq!(
            errors[0].hints,
            vec!["set the title with `set document(title: [...])`"]
        );
    }

    #[test]
    fn p1481_ua1_imagem_real_exige_alt_artifact_nao() {
        use typst_core::entities::elements::pdf_artifact::ArtifactKind;
        use typst_core::entities::layout_types::{SemanticKind, SemanticPlacement};
        let image = FrameItem::Image {
            pos: Point::ZERO,
            data: Arc::new(vec![0xff, 0xd8, 0xff]),
            alt: None,
            width: Pt(10.0),
            height: Pt(10.0),
            intrinsic_width: 1,
            intrinsic_height: 1,
            clip_rect: None,
            orientation: 1,
        };
        let ua1 = standards(&[PdfStandard::Ua1]);
        let mut doc = PagedDocument::new(vec![page_with(vec![image.clone()])]);
        let diag = require_ua1_image_alt(&doc, &ua1).expect("imagem real sem alt");
        assert_eq!(diag.message, "PDF/UA-1 error: missing alt text");
        assert_eq!(diag.hints, vec!["make sure your images and equations have alt text"]);
        assert!(require_ua1_image_alt(&doc, &PdfStandards::default()).is_none());
        if let FrameItem::Image { alt, .. } = &mut doc.pages[0].items[0] {
            *alt = Some("".into());
        }
        assert!(require_ua1_image_alt(&doc, &ua1).is_none());
        doc.pages[0].items = vec![FrameItem::Semantic {
            kind: SemanticKind::Artifact(ArtifactKind::Other),
            placement: SemanticPlacement::Inline,
            alt: None,
            items: vec![image],
        }];
        assert!(require_ua1_image_alt(&doc, &ua1).is_none());
    }

    #[test]
    fn p1480_ua1_com_titulo_compila_e_emite_identificacao() {
        let pdf = p1480_compile_with_standard(
            "#set document(title: \"PDF/UA parity audit\")\nHello",
            Some(standards(&[PdfStandard::Ua1])),
        )
        .expect("ua-1 com título deve continuar a compilar");
        let rendered = String::from_utf8_lossy(&pdf);
        assert!(
            rendered.contains("<pdfuaid:part>1</pdfuaid:part>"),
            "pipeline ua-1 deve chegar ao builder com o standard resolvido"
        );
        assert!(
            rendered.contains("/ViewerPreferences << /DisplayDocTitle true >>"),
            "pipeline ua-1 deve emitir a preferência de título"
        );
    }

    #[test]
    fn p1480_sem_ua1_nao_exige_titulo() {
        let default_pdf = p1480_compile_with_standard("Hello", None)
            .expect("sem standard, título deve continuar opcional");
        assert!(
            !String::from_utf8_lossy(&default_pdf).contains("pdfuaid"),
            "sem standard não deve emitir identificação PDF/UA"
        );

        let pdfa_pdf =
            p1480_compile_with_standard("Hello", Some(standards(&[PdfStandard::A2u])))
                .expect("perfil archival isolado não deve exigir título");
        assert!(
            !String::from_utf8_lossy(&pdfa_pdf).contains("pdfuaid"),
            "a-2u isolado não deve emitir identificação PDF/UA"
        );
    }
}
