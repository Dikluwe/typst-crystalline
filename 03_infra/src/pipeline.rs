//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/infra/pipeline.md
//! @prompt-hash b7460350
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
    eval_expression_with_features_and_introspector,
    eval_with_full_error_target_and_features, realize_outlines, EvalContext, EvalTarget,
};
use typst_core::compiler::introspect::{
    introspect_bundle_documents_with_starts, introspect_with_introspector,
};
use typst_core::compiler::layout::{
    collect_document_format_options,
    layout_with_introspector_and_metrics_and_callbacks_from, LayoutPassOutcome,
    ParLineCallbackRequest, ParLineCallbackResolution, SealedParLineCallbacks,
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

mod paged;
use paged::{
    bind_logical_page_numbers, compile_to_paged_document_full_error,
    compile_to_paged_document_full_error_and_features,
    compile_to_paged_document_full_error_and_features_with_introspector,
    introspect_with_runtime_for_pipeline, layout_with_realized_math_callbacks,
    layout_with_realized_math_callbacks_from, page_reference_pages,
    realize_page_numberings,
};
mod html;
pub use html::{
    compile_to_html_string, compile_to_html_string_with_features,
    compile_to_html_string_with_features_and_serialization,
    compile_to_html_string_with_features_and_serialization_and_pretty,
    compile_to_html_string_with_features_and_serialization_and_pretty_override,
};
use html::{html_frame_positions, resolve_html_pretty};
mod bundle;
use bundle::{
    bundle_error, collect_bundle_entries, introspect_bundle_with_runtime_for_pipeline,
};
pub use bundle::{compile_to_bundle_files, BundleCompileOptions, BundleFile};
mod pdf;
pub use pdf::{
    compile_to_pdf_bytes, compile_to_pdf_bytes_full_error,
    compile_to_pdf_bytes_full_error_and_document_id,
    compile_to_pdf_bytes_full_error_and_document_id_with_features,
    compile_to_pdf_bytes_full_error_and_document_id_with_features_and_serialization,
    compile_to_pdf_bytes_full_error_and_document_id_with_features_and_tagged_and_pretty_and_pages_and_standard_override,
    compile_to_pdf_bytes_full_error_and_document_id_with_features_and_tagged_and_pretty_and_pages_override,
    compile_to_pdf_bytes_full_error_and_document_id_with_features_and_tagged_and_pretty_override,
    compile_to_pdf_bytes_oracle, compile_to_pdf_bytes_oracle_with_features,
    compile_to_pdf_bytes_oracle_with_features_and_serialization,
    compile_to_pdf_bytes_oracle_with_features_and_tagged_and_pretty_and_pages_and_standard_override,
    compile_to_pdf_bytes_oracle_with_features_and_tagged_and_pretty_and_pages_override,
    compile_to_pdf_bytes_oracle_with_features_and_tagged_and_pretty_override,
    compile_to_pdf_bytes_with_timings, compile_to_pdf_bytes_with_timings_full_error,
    compile_to_pdf_bytes_with_timings_full_error_and_document_id,
    compile_to_pdf_bytes_with_timings_full_error_and_document_id_with_features,
    compile_to_pdf_bytes_with_timings_full_error_and_document_id_with_features_and_serialization,
    compile_to_pdf_bytes_with_timings_full_error_and_document_id_with_features_and_tagged_and_pretty_and_pages_and_standard_override,
    compile_to_pdf_bytes_with_timings_full_error_and_document_id_with_features_and_tagged_and_pretty_and_pages_override,
    compile_to_pdf_bytes_with_timings_full_error_and_document_id_with_features_and_tagged_and_pretty_override,
};
use pdf::{
    require_ua1_image_alt, resolve_pdf_pages_and_tags,
    resolve_pdf_pages_tags_and_standards, resolve_pdf_pretty,
};
mod raster;
pub use raster::{
    compile_to_png_bytes, compile_to_png_bytes_full_error,
    compile_to_png_bytes_with_timings_full_error,
    compile_to_png_bytes_with_timings_full_error_and_features,
    compile_to_png_bytes_with_timings_full_error_and_features_and_ppi_override,
};
mod svg;
pub use svg::{
    compile_to_svg_string, compile_to_svg_string_full_error,
    compile_to_svg_string_with_timings_full_error,
    compile_to_svg_string_with_timings_full_error_and_features,
    compile_to_svg_string_with_timings_full_error_and_features_and_pretty,
    compile_to_svg_string_with_timings_full_error_and_features_and_pretty_override,
};
use svg::{svg_destination_context_for_page, svg_glyph_font_context_for_page};
mod resources;
use resources::{
    collect_fonts_from_doc, first_font_from_doc, resolve_and_instantiate_fonts,
    resolve_font, resolve_fonts,
};

#[cfg(test)]
mod tests;

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
    let result = result.and_then(|module| {
        if target != EvalTarget::Bundle {
            if let Some(content) = module.content() {
                let mut has_asset = false;
                let _ = content.map_content(&mut |node| {
                    has_asset |= matches!(node, Content::Asset { .. });
                    Ok(None)
                });
                if has_asset {
                    return Err(vec![SourceDiagnostic::error(
                        Span::detached(),
                        "assets are only supported in the bundle target",
                    )
                    .with_hint("try enabling the bundle target")]);
                }
            }
        }
        Ok(module)
    });
    (result, warnings)
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
        Content::Styled(styled) => {
            let inner = &styled.body;
            let styles = &styled.styles;
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
                    Content::Styled(styled) => {
                        let inner = &styled.body;
                        let styles = &styled.styles;
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
