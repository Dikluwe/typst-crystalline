//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/infra/pipeline.md
//! @prompt-hash f59d6dd0
//! @layer L3
//! @updated 2026-10-07
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
    axis_variations_for_font_variant, is_variable_font, merge_explicit_variations,
    text_style_to_font_variant, variable_font_instancer_available,
};
use typst_core::compiler::eval::{
    apply_func, eval_expression, eval_expression_with_features,
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
mod runtime;

mod paged;
#[cfg(test)]
use paged::compile_to_paged_document_full_error;
use paged::{
    bind_logical_page_numbers, compile_to_paged_document_full_error_and_features,
    compile_to_paged_document_full_error_and_features_with_introspector,
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
#[cfg(test)]
use bundle::collect_bundle_entries;
use bundle::{bundle_error, introspect_bundle_with_runtime_for_pipeline};
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
#[cfg(test)]
use pdf::{require_ua1_image_alt, resolve_pdf_pages_and_tags};
use pdf::{resolve_pdf_pages_tags_and_standards, resolve_pdf_pretty};
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
use resources::{collect_fonts_from_doc, resolve_export_fonts, resolve_fonts};
#[cfg(test)]
use resources::{first_font_from_doc, resolve_font};

#[cfg(test)]
mod tests;

/// Compiles query context once and returns its final introspector without export.
pub fn compile_query_context(
    world: &dyn World,
    source: &Source,
) -> (
    SourceResult<typst_core::entities::introspector::TagIntrospector>,
    Vec<SourceDiagnostic>,
) {
    let mut timings = Timings::default();
    let (compiled, warnings) =
        compile_to_paged_document_full_error_and_features_with_introspector(
            world,
            source,
            false,
            typst_core::entities::compiler_features::Features::default(),
            &mut timings,
        );
    (compiled.map(|compiled| compiled.introspector), warnings)
}

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

/// Avalia a expressão pronta com identidade independente do main.
pub fn eval_source_with_sink_features(
    world: &dyn World,
    expression: &Source,
    features: typst_core::entities::compiler_features::Features,
) -> (SourceResult<Value>, Vec<SourceDiagnostic>) {
    typst_core::compiler::eval::eval_source_with_features_and_introspector(
        world,
        expression,
        features,
        &typst_core::entities::introspector::TagIntrospector::empty(),
    )
}

/// Compila o contexto uma vez e preserva as origens da Source de expressão.
pub fn eval_source_with_document_features(
    world: &dyn World,
    document: &Source,
    expression: &Source,
    features: typst_core::entities::compiler_features::Features,
) -> (SourceResult<Value>, Vec<SourceDiagnostic>) {
    let mut timings = Timings::default();
    let (compiled, mut warnings) =
        compile_to_paged_document_full_error_and_features_with_introspector(
            world,
            document,
            false,
            features,
            &mut timings,
        );
    let introspector = match compiled {
        Ok(compiled) => compiled.introspector,
        Err(errors) => return (Err(errors), warnings),
    };
    let (result, expression_warnings) =
        typst_core::compiler::eval::eval_source_with_features_and_introspector(
            world,
            expression,
            features,
            &introspector,
        );
    warnings.extend(expression_warnings);
    (result, warnings)
}

/// Compila um documento paginado e avalia uma expressão contra seu contexto.
pub fn eval_expression_with_document_features(
    world: &dyn World,
    source: &Source,
    expression: &str,
    features: typst_core::entities::compiler_features::Features,
) -> (SourceResult<Value>, Vec<SourceDiagnostic>) {
    let expression = Source::new_with_parser(
        world.main(),
        expression.to_string(),
        typst_core::compiler::parse::parse_code,
    );
    eval_source_with_document_features(world, source, &expression, features)
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

fn figure_context_supplement<'a>(
    figure: &'a typst_core::entities::elements::figure::FigureElem,
    chain: &StyleChain,
) -> Option<&'a Content> {
    if figure.caption.is_some()
        && matches!(chain.custom("figure.numbering"), None | Some(Value::Str(_)))
    {
        figure.resolved_supplement.as_ref().and_then(Option::as_ref)
    } else {
        None
    }
}

fn collect_context_blocks_into(
    content: &Content,
    chain: &StyleChain,
    map: &mut HashMap<u64, (Arc<ContextBlockElem>, StyleChain)>,
) {
    if let Some(child) = typst_core::compiler::realize::reference_supplements::prepared_reference_supplement(content) {
        collect_context_blocks_into(child, chain, map);
        return;
    }
    match content {
        Content::Located(e) => collect_context_blocks_into(&e.body, chain, map),
        Content::Realized(e) => collect_context_blocks_into(&e.visual, chain, map),
        Content::Source(e) => collect_context_blocks_into(&e.body, chain, map),
        Content::ShowRecipe(e) => collect_context_blocks_into(&e.body, &e.styles, map),
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
        Content::Sequence(items) => {
            for item in items.iter() {
                collect_context_blocks_into(item, chain, map);
            }
        }
        Content::Figure(figure) => {
            collect_context_blocks_into(&figure.body, chain, map);
            if let Some(supplement) = figure_context_supplement(figure, chain) {
                collect_context_blocks_into(supplement, chain, map);
            }
            if let Some(caption) = &figure.caption {
                collect_context_blocks_into(caption, chain, map);
            }
        }
        Content::Strong(e) => collect_context_blocks_into(&e.body, chain, map),
        Content::Emph(e) => collect_context_blocks_into(&e.body, chain, map),
        Content::Block(e) => collect_context_blocks_into(&e.body, chain, map),
        Content::Boxed(e) => collect_context_blocks_into(&e.body, chain, map),
        Content::Heading(e) => collect_context_blocks_into(&e.body, chain, map),
        Content::Title(e) => collect_context_blocks_into(&e.body, chain, map),
        Content::ListItem(e) => collect_context_blocks_into(&e.body, chain, map),
        Content::EnumItem(e) => collect_context_blocks_into(&e.body, chain, map),
        Content::Link(e) => collect_context_blocks_into(&e.body, chain, map),
        Content::Columns(e) => collect_context_blocks_into(&e.body, chain, map),
        Content::Align(e) => collect_context_blocks_into(&e.body, chain, map),
        Content::Place(e) => collect_context_blocks_into(&e.body, chain, map),
        Content::Repeat(e) => collect_context_blocks_into(&e.body, chain, map),
        Content::Transform(e) => collect_context_blocks_into(&e.body, chain, map),
        Content::Label(e) => collect_context_blocks_into(&e.body, chain, map),
        Content::SmallCaps { body } | Content::Par { body } => {
            collect_context_blocks_into(body, chain, map)
        }
        Content::Table(table) => {
            if let Some(resolved) = &table.resolved {
                for cell in &resolved.cells {
                    collect_context_blocks_into(cell, chain, map);
                }
            } else {
                for child in &table.children {
                    collect_context_blocks_into(child, chain, map);
                }
                if let Some(header) = &table.header {
                    collect_context_blocks_into(header, chain, map);
                }
                if let Some(footer) = &table.footer {
                    collect_context_blocks_into(footer, chain, map);
                }
            }
            if let Some(caption) = &table.caption {
                collect_context_blocks_into(caption, chain, map);
            }
        }
        Content::TableCell(cell) => {
            let visual = cell
                .standalone
                .as_ref()
                .map(|r| &r.visual)
                .or_else(|| cell.resolved.as_ref().map(|r| &r.visual))
                .unwrap_or(&cell.body);
            collect_context_blocks_into(visual, chain, map);
        }
        other => {
            // Fallback exaustivo delegado a L1 para os containers que não têm
            // descida emprestada acima. `map_content` é usado como visitante:
            // o transform devolve sempre `None` e o produto é descartado.
            //
            // `map_content` é bottom-up (filhos antes do nó), por isso um
            // `Styled` mais abaixo é visto **depois** dos seus descendentes:
            // os blocos lá dentro já foram inseridos com a cadeia exterior, e
            // a reentrada em `collect_context_blocks_into` reinsere-os com a
            // cadeia correcta — o `insert` posterior sobrepõe-se, que é o
            // resultado pretendido.
            let _ = other.map_content(&mut |node| {
                if let Some(child) = typst_core::compiler::realize::reference_supplements::prepared_reference_supplement(node) {
                    collect_context_blocks_into(child, chain, map);
                    return Ok(None);
                }
                match node {
                    Content::Located(e) => {
                        collect_context_blocks_into(&e.body, chain, map)
                    }
                    Content::Realized(e) => {
                        collect_context_blocks_into(&e.visual, chain, map)
                    }
                    Content::ShowRecipe(e) => {
                        collect_context_blocks_into(&e.body, &e.styles, map)
                    }
                    Content::ContextBlock(e) => {
                        map.insert(e.id, (e.clone(), chain.clone()));
                    }
                    Content::Styled(styled) => {
                        let inner = &styled.body;
                        let styles = &styled.styles;
                        let inner_chain = chain.push_styles(styles);
                        collect_context_blocks_into(inner, &inner_chain, map);
                    }
                    Content::Source(..)
                    | Content::Empty
                    | Content::Text(..)
                    | Content::Space
                    | Content::SpaceFromNewline
                    | Content::Parbreak
                    | Content::Sequence(..)
                    | Content::HtmlElem(..)
                    | Content::HtmlFrame(..)
                    | Content::Par { .. }
                    | Content::Heading(..)
                    | Content::Title(..)
                    | Content::Strong(..)
                    | Content::Emph(..)
                    | Content::Raw(..)
                    | Content::ListItem(..)
                    | Content::EnumItem(..)
                    | Content::Link(..)
                    | Content::Equation(..)
                    | Content::MathSequence(..)
                    | Content::MathIdent(..)
                    | Content::MathText(..)
                    | Content::MathFrac(..)
                    | Content::MathAttach(..)
                    | Content::MathPrimes(..)
                    | Content::MathRoot(..)
                    | Content::MathStretch(..)
                    | Content::MathDelimited(..)
                    | Content::MathLr(..)
                    | Content::MathMid(..)
                    | Content::MathOverbrace(..)
                    | Content::MathUnderbrace(..)
                    | Content::MathUnderbracket(..)
                    | Content::MathUnderparen(..)
                    | Content::MathUndershell(..)
                    | Content::MathOverbracket(..)
                    | Content::MathOverparen(..)
                    | Content::MathOvershell(..)
                    | Content::MathAlignPoint(..)
                    | Content::Linebreak(..)
                    | Content::MathMatrix(..)
                    | Content::MathCases(..)
                    | Content::MathAccent(..)
                    | Content::MathCancel(..)
                    | Content::MathUnderline(..)
                    | Content::MathOverline(..)
                    | Content::MathVec(..)
                    | Content::MathClassOverride(..)
                    | Content::MathLimitsOverride(..)
                    | Content::MathUnderover(..)
                    | Content::MathOp(..)
                    | Content::MathStyled(..)
                    | Content::Label(..)
                    | Content::Ref(..)
                    | Content::CounterDisplay(..)
                    | Content::CounterUpdate(..)
                    | Content::Outline(..)
                    | Content::OutlineEntry(..)
                    | Content::Figure(..)
                    | Content::FigureCaption(..)
                    | Content::Image(..)
                    | Content::Shape(..)
                    | Content::Curve(..)
                    | Content::Transform(..)
                    | Content::Grid(..)
                    | Content::GridHeader(..)
                    | Content::GridFooter(..)
                    | Content::GridCell(..)
                    | Content::SetPage(..)
                    | Content::PageRun(..)
                    | Content::Align(..)
                    | Content::Place(..)
                    | Content::Flush(..)
                    | Content::Divider(..)
                    | Content::Terms(..)
                    | Content::TermItem(..)
                    | Content::Quote(..)
                    | Content::Document { .. }
                    | Content::Asset { .. }
                    | Content::SmartQuote(..)
                    | Content::PdfAttach(..)
                    | Content::PdfArtifact(..)
                    | Content::Underline(..)
                    | Content::Strike(..)
                    | Content::Overline(..)
                    | Content::SmallCaps { .. }
                    | Content::Pad(..)
                    | Content::Hide(..)
                    | Content::HSpace(..)
                    | Content::VSpace(..)
                    | Content::Pagebreak(..)
                    | Content::Colbreak(..)
                    | Content::Stack(..)
                    | Content::Boxed(..)
                    | Content::Block(..)
                    | Content::TableCell(..)
                    | Content::Bibliography(..)
                    | Content::Cite(..)
                    | Content::Footnote(..)
                    | Content::FootnoteEntry(..)
                    | Content::TableHeader(..)
                    | Content::TableFooter(..)
                    | Content::GridHLine(..)
                    | Content::GridVLine(..)
                    | Content::TableHLine(..)
                    | Content::TableVLine(..)
                    | Content::Table(..)
                    | Content::Repeat(..)
                    | Content::Columns(..)
                    | Content::Metadata(..)
                    | Content::State(..)
                    | Content::StateUpdate(..)
                    | Content::StateDisplay(..)
                    | Content::CounterDisplayCallback(..)
                    | Content::Dynamic(..) => {} // Descendants were visited by the enclosing L1 traversal.
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
    substitute_context_blocks_in_chain(content, resolved, &StyleChain::default_chain())
}

fn substitute_context_blocks_in_chain(
    content: Content,
    resolved: &HashMap<u64, Content>,
    chain: &StyleChain,
) -> Content {
    macro_rules! body {
        ($variant:ident, $elem:expr) => {{
            let mut elem = (**$elem).clone();
            elem.body = substitute_context_blocks_in_chain(elem.body, resolved, chain);
            return Content::$variant(Arc::new(elem));
        }};
    }
    match &content {
        Content::Source(e) => {
            let mut elem = (**e).clone();
            elem.body = substitute_context_blocks_in_chain(elem.body, resolved, chain);
            return Content::Source(Arc::new(elem));
        }
        Content::Located(e) if !matches!(e.body, Content::ContextBlock(_)) => {
            let mut elem = (**e).clone();
            elem.body = substitute_context_blocks_in_chain(elem.body, resolved, chain);
            return Content::Located(Arc::new(elem));
        }
        Content::Realized(e) => {
            let mut elem = (**e).clone();
            elem.visual =
                substitute_context_blocks_in_chain(elem.visual, resolved, chain);
            return Content::Realized(Arc::new(elem));
        }
        Content::ShowRecipe(e) => {
            let mut elem = (**e).clone();
            elem.body =
                substitute_context_blocks_in_chain(elem.body, resolved, &elem.styles);
            return Content::ShowRecipe(Arc::new(elem));
        }
        Content::Styled(e) => {
            if let Ok(Some(rebuilt)) = typst_core::compiler::realize::reference_supplements::map_prepared_reference_supplement(&content, |child| {
                Ok(substitute_context_blocks_in_chain(child.clone(), resolved, chain))
            }) { return rebuilt; }
            let mut elem = (**e).clone();
            elem.body = substitute_context_blocks_in_chain(
                elem.body,
                resolved,
                &chain.push_styles(&elem.styles),
            );
            return Content::Styled(Arc::new(elem));
        }
        Content::Sequence(parts) | Content::MathSequence(parts) => {
            let parts = parts
                .iter()
                .cloned()
                .map(|part| substitute_context_blocks_in_chain(part, resolved, chain))
                .collect::<Vec<_>>()
                .into();
            return if matches!(content, Content::Sequence(_)) {
                Content::Sequence(parts)
            } else {
                Content::MathSequence(parts)
            };
        }
        Content::Figure(e) => {
            let mut elem = (**e).clone();
            elem.body = substitute_context_blocks_in_chain(elem.body, resolved, chain);
            if let Some(supplement) = figure_context_supplement(e, chain) {
                elem.resolved_supplement =
                    Some(Some(substitute_context_blocks_in_chain(
                        supplement.clone(),
                        resolved,
                        chain,
                    )));
            }
            elem.caption = elem.caption.map(|caption| {
                substitute_context_blocks_in_chain(caption, resolved, chain)
            });
            return Content::Figure(Arc::new(elem));
        }
        Content::Strong(e) => body!(Strong, e),
        Content::Emph(e) => body!(Emph, e),
        Content::Block(e) => body!(Block, e),
        Content::Boxed(e) => body!(Boxed, e),
        Content::Heading(e) => body!(Heading, e),
        Content::Title(e) => body!(Title, e),
        Content::ListItem(e) => body!(ListItem, e),
        Content::EnumItem(e) => body!(EnumItem, e),
        Content::Link(e) => body!(Link, e),
        Content::Columns(e) => body!(Columns, e),
        Content::Align(e) => body!(Align, e),
        Content::Place(e) => body!(Place, e),
        Content::Repeat(e) => body!(Repeat, e),
        Content::Transform(e) => body!(Transform, e),
        Content::Label(e) => body!(Label, e),
        Content::Par { body } => {
            return Content::Par {
                body: Box::new(substitute_context_blocks_in_chain(
                    (**body).clone(),
                    resolved,
                    chain,
                )),
            };
        }
        Content::SmallCaps { body } => {
            return Content::SmallCaps {
                body: Box::new(substitute_context_blocks_in_chain(
                    (**body).clone(),
                    resolved,
                    chain,
                )),
            };
        }
        // These nodes delegate descendant substitution to the existing
        // generic mapper. Located(ContextBlock) deliberately retains its marker.
        Content::Located(..)
        | Content::Empty
        | Content::Text(..)
        | Content::Space
        | Content::SpaceFromNewline
        | Content::Parbreak
        | Content::HtmlElem(..)
        | Content::HtmlFrame(..)
        | Content::Raw(..)
        | Content::Equation(..)
        | Content::MathIdent(..)
        | Content::MathText(..)
        | Content::MathFrac(..)
        | Content::MathAttach(..)
        | Content::MathPrimes(..)
        | Content::MathRoot(..)
        | Content::MathStretch(..)
        | Content::MathDelimited(..)
        | Content::MathLr(..)
        | Content::MathMid(..)
        | Content::MathOverbrace(..)
        | Content::MathUnderbrace(..)
        | Content::MathUnderbracket(..)
        | Content::MathUnderparen(..)
        | Content::MathUndershell(..)
        | Content::MathOverbracket(..)
        | Content::MathOverparen(..)
        | Content::MathOvershell(..)
        | Content::MathAlignPoint(..)
        | Content::Linebreak(..)
        | Content::MathMatrix(..)
        | Content::MathCases(..)
        | Content::MathAccent(..)
        | Content::MathCancel(..)
        | Content::MathUnderline(..)
        | Content::MathOverline(..)
        | Content::MathVec(..)
        | Content::MathClassOverride(..)
        | Content::MathLimitsOverride(..)
        | Content::MathUnderover(..)
        | Content::MathOp(..)
        | Content::MathStyled(..)
        | Content::Ref(..)
        | Content::CounterDisplay(..)
        | Content::CounterUpdate(..)
        | Content::Outline(..)
        | Content::OutlineEntry(..)
        | Content::FigureCaption(..)
        | Content::Image(..)
        | Content::Shape(..)
        | Content::Curve(..)
        | Content::Grid(..)
        | Content::GridHeader(..)
        | Content::GridFooter(..)
        | Content::GridCell(..)
        | Content::SetPage(..)
        | Content::PageRun(..)
        | Content::Flush(..)
        | Content::Divider(..)
        | Content::Terms(..)
        | Content::TermItem(..)
        | Content::Quote(..)
        | Content::Document { .. }
        | Content::Asset { .. }
        | Content::SmartQuote(..)
        | Content::PdfAttach(..)
        | Content::PdfArtifact(..)
        | Content::Underline(..)
        | Content::Strike(..)
        | Content::Overline(..)
        | Content::Pad(..)
        | Content::Hide(..)
        | Content::HSpace(..)
        | Content::VSpace(..)
        | Content::Pagebreak(..)
        | Content::Colbreak(..)
        | Content::Stack(..)
        | Content::TableCell(..)
        | Content::Bibliography(..)
        | Content::Cite(..)
        | Content::Footnote(..)
        | Content::FootnoteEntry(..)
        | Content::TableHeader(..)
        | Content::TableFooter(..)
        | Content::GridHLine(..)
        | Content::GridVLine(..)
        | Content::TableHLine(..)
        | Content::TableVLine(..)
        | Content::Table(..)
        | Content::Metadata(..)
        | Content::State(..)
        | Content::StateUpdate(..)
        | Content::StateDisplay(..)
        | Content::CounterDisplayCallback(..)
        | Content::ContextBlock(..)
        | Content::Dynamic(..) => {}
    }
    content
        .map_content(&mut |node| {
            if let Some(rebuilt) = typst_core::compiler::realize::reference_supplements::map_prepared_reference_supplement(node, |child| {
                Ok(substitute_context_blocks_in_chain(child.clone(), resolved, chain))
            })? {
                return Ok(Some(rebuilt));
            }
            Ok(match node {
                Content::Located(e) => {
                    if let Content::ContextBlock(elem) = &e.body {
                        let result =
                            resolved.get(&elem.id).cloned().unwrap_or(Content::Empty);
                        Some(Content::Sequence(vec![node.clone(), result].into()))
                    } else {
                        None
                    }
                }
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
                // Children have already been mapped. No node replacement is
                // needed outside the two contextual-marker forms above.
                Content::Source(..)
                | Content::ShowRecipe(..)
                | Content::Realized(..)
                | Content::Empty
                | Content::Text(..)
                | Content::Space
                | Content::SpaceFromNewline
                | Content::Parbreak
                | Content::Sequence(..)
                | Content::HtmlElem(..)
                | Content::HtmlFrame(..)
                | Content::Par { .. }
                | Content::Heading(..)
                | Content::Title(..)
                | Content::Strong(..)
                | Content::Emph(..)
                | Content::Raw(..)
                | Content::ListItem(..)
                | Content::EnumItem(..)
                | Content::Link(..)
                | Content::Equation(..)
                | Content::MathSequence(..)
                | Content::MathIdent(..)
                | Content::MathText(..)
                | Content::MathFrac(..)
                | Content::MathAttach(..)
                | Content::MathPrimes(..)
                | Content::MathRoot(..)
                | Content::MathStretch(..)
                | Content::MathDelimited(..)
                | Content::MathLr(..)
                | Content::MathMid(..)
                | Content::MathOverbrace(..)
                | Content::MathUnderbrace(..)
                | Content::MathUnderbracket(..)
                | Content::MathUnderparen(..)
                | Content::MathUndershell(..)
                | Content::MathOverbracket(..)
                | Content::MathOverparen(..)
                | Content::MathOvershell(..)
                | Content::MathAlignPoint(..)
                | Content::Linebreak(..)
                | Content::MathMatrix(..)
                | Content::MathCases(..)
                | Content::MathAccent(..)
                | Content::MathCancel(..)
                | Content::MathUnderline(..)
                | Content::MathOverline(..)
                | Content::MathVec(..)
                | Content::MathClassOverride(..)
                | Content::MathLimitsOverride(..)
                | Content::MathUnderover(..)
                | Content::MathOp(..)
                | Content::MathStyled(..)
                | Content::Label(..)
                | Content::Ref(..)
                | Content::CounterDisplay(..)
                | Content::CounterUpdate(..)
                | Content::Outline(..)
                | Content::OutlineEntry(..)
                | Content::Figure(..)
                | Content::FigureCaption(..)
                | Content::Image(..)
                | Content::Shape(..)
                | Content::Curve(..)
                | Content::Transform(..)
                | Content::Grid(..)
                | Content::GridHeader(..)
                | Content::GridFooter(..)
                | Content::GridCell(..)
                | Content::SetPage(..)
                | Content::PageRun(..)
                | Content::Align(..)
                | Content::Place(..)
                | Content::Flush(..)
                | Content::Styled(..)
                | Content::Divider(..)
                | Content::Terms(..)
                | Content::TermItem(..)
                | Content::Quote(..)
                | Content::Document { .. }
                | Content::Asset { .. }
                | Content::SmartQuote(..)
                | Content::PdfAttach(..)
                | Content::PdfArtifact(..)
                | Content::Underline(..)
                | Content::Strike(..)
                | Content::Overline(..)
                | Content::SmallCaps { .. }
                | Content::Pad(..)
                | Content::Hide(..)
                | Content::HSpace(..)
                | Content::VSpace(..)
                | Content::Pagebreak(..)
                | Content::Colbreak(..)
                | Content::Stack(..)
                | Content::Boxed(..)
                | Content::Block(..)
                | Content::TableCell(..)
                | Content::Bibliography(..)
                | Content::Cite(..)
                | Content::Footnote(..)
                | Content::FootnoteEntry(..)
                | Content::TableHeader(..)
                | Content::TableFooter(..)
                | Content::GridHLine(..)
                | Content::GridVLine(..)
                | Content::TableHLine(..)
                | Content::TableVLine(..)
                | Content::Table(..)
                | Content::Repeat(..)
                | Content::Columns(..)
                | Content::Metadata(..)
                | Content::State(..)
                | Content::StateUpdate(..)
                | Content::StateDisplay(..)
                | Content::CounterDisplayCallback(..)
                | Content::Dynamic(..) => None,
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
    /// Avaliação comum única seguida da realização pura de receitas.
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

#[cfg(test)]
mod batch_g_collection_tests {
    use super::*;
    use typst_core::entities::content::{
        LocatedContent, RealizedContent, ShowRecipeContent, SourceContent,
    };
    use typst_core::entities::elements::math_frac::MathFracElem;
    use typst_core::entities::elements::underline::UnderlineElem;
    use typst_core::entities::func::Func;
    use typst_core::entities::layout_types::Length;
    use typst_core::entities::locator::Locator;
    use typst_core::entities::style::{Style, Styles};

    fn producer(id: u64) -> Arc<ContextBlockElem> {
        Arc::new(ContextBlockElem {
            id,
            closure: Func::native("collection-witness", |_ctx, _args, _world, _file| {
                Ok(Value::None)
            }),
        })
    }

    #[test]
    fn batch_g_generic_collection_preserves_located_source_and_visual_projection() {
        let bare = producer(11);
        let located = producer(12);
        let hidden = producer(13);
        let visual = producer(14);
        let tree = Content::MathFrac(Arc::new(MathFracElem {
            num: Content::Underline(Arc::new(UnderlineElem {
                body: Content::sequence(vec![
                    Content::ContextBlock(bare.clone()),
                    Content::Source(Arc::new(SourceContent {
                        span: Span::detached(),
                        body: Content::Located(Arc::new(LocatedContent {
                            location: Locator::new().next(),
                            body: Content::ContextBlock(located.clone()),
                        })),
                    })),
                ]),
                stroke: None,
                offset: None,
                extent: None,
            })),
            den: Content::Realized(Arc::new(RealizedContent {
                original: Content::ContextBlock(hidden),
                visual: Content::ContextBlock(visual.clone()),
            })),
            style: None,
            num_deparenthesized: false,
            denom_deparenthesized: false,
            line: true,
        }));
        let collected = collect_context_blocks(&tree, &StyleChain::default_chain());
        assert_eq!(collected.len(), 3);
        for (id, expected) in [(11, bare), (12, located), (14, visual)] {
            assert!(Arc::ptr_eq(&collected[&id].0, &expected));
        }
        assert!(!collected.contains_key(&13));
        let substituted = substitute_context_blocks(
            tree,
            &HashMap::from([
                (11, Content::text("BARE")),
                (12, Content::text("LOCATED")),
                (14, Content::text("VISUAL")),
            ]),
        );
        assert_eq!(substituted.plain_text(), "(BARELOCATED)/(VISUAL)");
    }

    #[test]
    fn batch_g_generic_collection_uses_captured_chain_then_inner_delta_and_last_payload()
    {
        let world = crate::world::SystemWorld::for_stdin(
            std::path::PathBuf::from("/tmp"),
            String::new(),
        );
        let first = producer(7);
        let last = producer(7);
        let styles = |size| Styles::from_iter([Style::Size(Length::pt(size))]);
        let recipe = Content::ShowRecipe(Arc::new(ShowRecipeContent {
            body: Content::styled(
                Box::new(Content::ContextBlock(last.clone())),
                styles(30.0),
            ),
            textual: false,
            context_id_floor: 8,
            rules: Arc::from([]),
            styles: StyleChain::default_chain().push_styles(&styles(20.0)),
            guards: vec![],
            file: world.main(),
        }));
        let tree = Content::Underline(Arc::new(UnderlineElem {
            body: Content::sequence(vec![Content::ContextBlock(first), recipe]),
            stroke: None,
            offset: None,
            extent: None,
        }));
        let chain = StyleChain::default_chain().push_styles(&styles(40.0));
        let collected = collect_context_blocks(&tree, &chain);
        assert_eq!(collected.len(), 1);
        assert!(Arc::ptr_eq(&collected[&7].0, &last));
        assert_eq!(collected[&7].1.size(), 30.0);
    }
}
