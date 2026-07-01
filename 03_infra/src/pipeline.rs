//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/infra/pipeline.md
//! @prompt-hash 8695ff8d
//! @layer L3
//! @updated 2026-04-24
//!
//! Pipeline de compilação — orquestra eval → introspect → layout
//! → export_pdf, gere o boilerplate `comemo` (Route, Sink, Traced,
//! Routines) e drena warnings.
//!
//! Materializado no Passo 113 (ADR-0046) a partir de helpers
//! test-only em `integration_tests.rs`. API pública consumível
//! pelo 04_wiring (CLI) e por testes.

#![allow(deprecated)] // P483 — FrameItem::Text fallback path legítimo
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Instant;

use comemo::{Track, TrackedMut};

use typst_core::contracts::world::World;
use typst_core::entities::args::Args;
use typst_core::entities::content::Content;
use typst_core::entities::elements::context_block::ContextBlockElem;
use typst_core::entities::engine::Engine;
use typst_core::entities::font_book::{FontBook, FontVariant};
use typst_core::entities::font_list::FontList;
use typst_core::entities::layout_types::{FrameItem, PagedDocument};
use typst_core::entities::module::Module;
use typst_core::entities::show::ShowRule;
use typst_core::entities::sink::Sink as TypstSink;
use typst_core::entities::source::Source;
use typst_core::entities::source_result::{SourceDiagnostic, SourceResult};
use typst_core::entities::style_chain::StyleChain;
use typst_core::entities::world_types::{Route, Routines, Sink, Traced};
use typst_core::rules::eval::{apply_func, eval_with_full_error, EvalContext};
use typst_core::rules::introspect::introspect_with_introspector;
use typst_core::rules::layout::layout_with_introspector;
use typst_core::rules::scopes::Scopes;
use typst_core::rules::stdlib::value_to_content;

use crate::export::{
    export_pdf,
    export_pdf_multifont_and_timings,
    export_pdf_with_font_and_timings,
};

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
    let routines = Routines::new();
    let traced   = Traced::default();
    let mut sink = Sink::new();
    let route    = Route::root();
    // Lote F-3 inc-2: registry de elementos de utilizador. Vazio até pacotes
    // registarem elementos (não há elemento de utilizador em produção ainda).
    let registry = typst_core::entities::element_registry::ElementRegistry::new();
    let result = eval_with_full_error(
        &routines,
        world,
        traced.track(),
        sink.track_mut(),
        route.track(),
        source,
        &registry,
        full_error,
    );
    let warnings = sink.into_diagnostics();
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
    if intr.context_block_locations.is_empty() {
        return Ok(content);
    }

    // Colecta os ContextBlocks pelo id.
    let blocks = collect_context_blocks(&content);

    // Resolve cada ContextBlock.
    let mut resolved = HashMap::new();
    for (id, loc) in &intr.context_block_locations {
        let Some(elem) = blocks.get(id) else { continue };
        let mut ctx = EvalContext::new();
        ctx.in_context = true;
        ctx.introspector = intr.clone();
        ctx.current_location = Some(*loc);

        let mut scopes = Scopes::new(None);
        let mut styles = StyleChain::default_chain();
        let mut show_rules: Arc<[ShowRule]> = Arc::from([]);
        let mut active_guards: Vec<u64> = Vec::new();
        let mut sink = TypstSink::new();
        let route = Route::root().with_id(source.id());
        let mut tracked_sink = sink.track_mut();
        let mut local_sink = TrackedMut::reborrow_mut(&mut tracked_sink);
        let mut engine = Engine {
            world,
            route: route.track(),
            styles: &mut styles,
            show_rules: &mut show_rules,
            active_guards: &mut active_guards,
            current_file: source.id(),
            sink: &mut local_sink,
        };

        let result = apply_func(
            elem.closure.clone(),
            Args::positional(vec![]),
            &mut scopes,
            &mut ctx,
            &mut engine,
        )?;
        resolved.insert(*id, value_to_content(&result));
    }

    Ok(substitute_context_blocks(content, &resolved))
}

fn collect_context_blocks(content: &Content) -> HashMap<u64, Arc<ContextBlockElem>> {
    let mut map = HashMap::new();
    match content {
        Content::ContextBlock(elem) => {
            map.insert(elem.id, elem.clone());
        }
        Content::Sequence(seq) => {
            for child in seq.iter() {
                map.extend(collect_context_blocks(child));
            }
        }
        Content::Styled(inner, _) => {
            map.extend(collect_context_blocks(inner));
        }
        Content::Strong(e) => map.extend(collect_context_blocks(&e.body)),
        Content::Emph(e) => map.extend(collect_context_blocks(&e.body)),
        Content::Heading(e) => map.extend(collect_context_blocks(&e.body)),
        Content::Raw(_) | Content::Text(_) | Content::Space | Content::Empty => {}
        // Containers não listados: ContextBlock não deve aparecer aninhado
        // dentro de Grid/Table/etc. neste subset. Ignorar defensivamente.
        _ => {}
    }
    map
}

fn substitute_context_blocks(
    content: Content,
    resolved: &HashMap<u64, Content>,
) -> Content {
    match content {
        Content::ContextBlock(elem) => {
            resolved.get(&elem.id).cloned().unwrap_or(Content::Empty)
        }
        Content::Sequence(seq) => Content::Sequence(
            seq.iter()
                .map(|c| substitute_context_blocks(c.clone(), resolved))
                .collect::<Vec<_>>()
                .into(),
        ),
        Content::Styled(inner, styles) => Content::Styled(
            Box::new(substitute_context_blocks(*inner, resolved)),
            styles,
        ),
        Content::Strong(e) => Content::strong(substitute_context_blocks(e.body.clone(), resolved)),
        Content::Emph(e) => Content::emph(substitute_context_blocks(e.body.clone(), resolved)),
        Content::Heading(e) => {
            Content::heading(e.level, substitute_context_blocks(e.body.clone(), resolved))
        }
        other => other,
    }
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
) -> (Result<Vec<u8>, Vec<SourceDiagnostic>>, Vec<SourceDiagnostic>) {
    compile_to_pdf_bytes_full_error(world, source, false)
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
) -> (Result<Vec<u8>, Vec<SourceDiagnostic>>, Vec<SourceDiagnostic>, Timings) {
    compile_to_pdf_bytes_with_timings_full_error(world, source, false)
}

/// Internal variant com instrumentação de tempos e `full_error`.
#[doc(hidden)]
pub fn compile_to_pdf_bytes_with_timings_full_error(
    world: &dyn World,
    source: &Source,
    full_error: bool,
) -> (Result<Vec<u8>, Vec<SourceDiagnostic>>, Vec<SourceDiagnostic>, Timings) {
    let mut timings = Timings::default();
    let result = compile_to_pdf_bytes_impl(world, source, full_error, &mut timings);
    (result.0, result.1, timings)
}

fn compile_to_pdf_bytes_impl(
    world: &dyn World,
    source: &Source,
    full_error: bool,
    timings: &mut Timings,
) -> (Result<Vec<u8>, Vec<SourceDiagnostic>>, Vec<SourceDiagnostic>) {
    let t0 = Instant::now();
    let (eval_result, warnings) = eval_to_module_with_sink_full_error(world, source, full_error);
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
            return (Ok(Vec::new()), warnings);
        }
    };
    // P190I (M6 fechado): popula TagIntrospector a partir do content.
    // P498: usa o conteúdo original (pré-show-rules) para que elementos
    // locatable transformados por show-rules continuem indexados.
    let intr_content = module.introspection_content().unwrap_or(content);
    // P429 (DEBT-63): injecta no BibStore os styles CSL resolvidos em
    // eval time, indexados pela chave do BibliographyElem correspondente.
    let mut intr = introspect_with_introspector(intr_content);
    for (key, style) in module.bibliography_styles() {
        intr.bib_store.add_style(*key, style.clone());
    }
    let t2 = Instant::now();
    timings.introspect_ms = duration_ms(t2.duration_since(t1));

    // P506: expande ContextBlocks pós-introspecção (delayed evaluation).
    let content = match expand_context_blocks(content.clone(), &intr, world, source) {
        Ok(c) => c,
        Err(errors) => {
            timings.expand_context_ms = duration_ms(Instant::now().duration_since(t2));
            timings.total_ms = timings.eval_ms + timings.introspect_ms + timings.expand_context_ms;
            return (Err(errors), warnings);
        }
    };
    let t3 = Instant::now();
    timings.expand_context_ms = duration_ms(t3.duration_since(t2));

    let doc = layout_with_introspector(&content, intr);
    let t4 = Instant::now();
    timings.layout_ms = duration_ms(t4.duration_since(t3));

    // P482 — shaping pass: Text → TextShaped (Trilha 5 Fase 1, ADR-0120 A1).
    let doc = crate::shaper::shape_document(world, doc);
    let t5 = Instant::now();
    timings.shape_ms = duration_ms(t5.duration_since(t4));

    // Passo 146 (ADR-0055 decisão 5): dispatch multi-font.
    // 0 fonts resolvidos → fallback Helvetica.
    // 1 font resolvido → preserva caminho single-font do 140B/141.
    // 2+ fonts resolvidos → multi-font (resource dict com /F1..N).
    let font_lists = collect_fonts_from_doc(&doc);
    let resolved = resolve_fonts(&font_lists, world.book(), world);
    let (pdf, subset_ms) = match resolved.as_slice() {
        []         => (export_pdf(&doc), 0.0),
        [(_, b)]   => export_pdf_with_font_and_timings(&doc, b),
        many       => export_pdf_multifont_and_timings(&doc, many),
    };
    timings.subset_ms = subset_ms;
    let t6 = Instant::now();
    timings.render_ms = duration_ms(t6.duration_since(t5)) - subset_ms;
    timings.total_ms = duration_ms(t6.duration_since(t0));

    (Ok(pdf), warnings)
}

#[doc(hidden)]
pub fn compile_to_pdf_bytes_full_error(
    world: &dyn World,
    source: &Source,
    full_error: bool,
) -> (Result<Vec<u8>, Vec<SourceDiagnostic>>, Vec<SourceDiagnostic>) {
    let mut timings = Timings::default();
    compile_to_pdf_bytes_impl(world, source, full_error, &mut timings)
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
/// e devolve **todas** as `FontList` distintas em ordem de
/// primeira ocorrência (Passo 146, ADR-0055 decisão 5).
///
/// Deduplicação por igualdade estrutural via `Vec::contains`.
/// Complexidade O(N²) em N = fonts distintas; aceite porque N é
/// tipicamente pequeno (<10) em documentos reais.
fn collect_fonts_from_doc(doc: &PagedDocument) -> Vec<FontList> {
    let mut seen: Vec<FontList> = Vec::new();
    for page in &doc.pages {
        collect_fonts_in_items(&page.items, &mut seen);
    }
    seen
}

fn collect_fonts_in_items(items: &[FrameItem], seen: &mut Vec<FontList>) {
    for item in items {
        match item {
            FrameItem::Text { style, .. }
            | FrameItem::TextShaped { style, .. } => {
                if let Some(fl) = &style.font {
                    if !seen.contains(fl) {
                        seen.push(fl.clone());
                    }
                }
            }
            FrameItem::Group { items, .. }
            | FrameItem::Link { items, .. } => {
                collect_fonts_in_items(items, seen);
            }
            FrameItem::Line  { .. }
            | FrameItem::Glyph { .. }
            | FrameItem::Image { .. }
            | FrameItem::Shape { .. } => {}
        }
    }
}

/// Map-filter de `resolve_font` (Passo 141) sobre uma lista de
/// `FontList`. Devolve `(FontList, bytes)` para preservar a
/// associação entre input style e output embed (Passo 146).
///
/// Silent drop quando `resolve_font` devolve `None` — consistente
/// com a política de fallback de fonts (140B/141).
fn resolve_fonts(
    font_lists: &[FontList],
    font_book:  &FontBook,
    world:      &dyn World,
) -> Vec<(FontList, Vec<u8>)> {
    font_lists.iter()
        .filter_map(|fl| {
            resolve_font(fl, font_book, world).map(|bytes| (fl.clone(), bytes))
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
            FrameItem::Text { style, .. }
            | FrameItem::TextShaped { style, .. } => {
                if let Some(fl) = &style.font {
                    return Some(fl.clone());
                }
            }
            FrameItem::Group { items, .. }
            | FrameItem::Link { items, .. } => {
                if let Some(fl) = first_font_in_items(items) {
                    return Some(fl);
                }
            }
            FrameItem::Line  { .. }
            | FrameItem::Glyph { .. }
            | FrameItem::Image { .. }
            | FrameItem::Shape { .. } => {}
        }
    }
    None
}

/// Itera `font_list.as_slice()` em ordem. Para cada família,
/// consulta `font_book.select_pattern(&family.name, &FontVariant::default())`;
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
/// Selecção usa `FontVariant::default()` (regular). Weight/style
/// continuam a ser renderizados por faux-bold (Passo 139).
fn resolve_font(
    font_list: &FontList,
    font_book: &FontBook,
    world:     &dyn World,
) -> Option<Vec<u8>> {
    let variant = FontVariant::default();
    for family in font_list.as_slice() {
        if let Some(index) = font_book.select_pattern(&family.name, &variant) {
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
    use typst_core::entities::world_types::{
        Bytes, Datetime, FileError, FileResult, Font, Library,
    };
    use typst_core::entities::file_id::FileId;
    use typst_core::entities::font_book::FontBook;
    use std::num::NonZeroU16;

    // MockWorld mínimo para smoke test — source única.
    struct MockWorld {
        library: Library,
        book:    FontBook,
        source:  Source,
    }
    impl World for MockWorld {
        fn library(&self) -> &Library { &self.library }
        fn book(&self)    -> &FontBook { &self.book }
        fn main(&self)    -> FileId { self.source.id() }
        fn source(&self, id: FileId) -> FileResult<Source> {
            if id == self.source.id() {
                Ok(self.source.clone())
            } else {
                Err(FileError::NotFound)
            }
        }
        fn file(&self, _: FileId) -> FileResult<Bytes> { Err(FileError::NotFound) }
        fn font(&self, _: usize) -> Option<Font> { None }
        fn today(&self, _: Option<i64>) -> Option<Datetime> { None }
    }

    fn mock_world(src: &str) -> MockWorld {
        let id = FileId::from_raw(NonZeroU16::new(1).unwrap());
        let source = Source::new(id, src.to_string());
        MockWorld {
            library: Library::new(),
            book:    FontBook::new(),
            source,
        }
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
        assert_eq!(warnings.len(), 1,
            "pilot do Passo 106 emite warning para ficheiro vazio");
    }

    #[test]
    fn compile_to_pdf_bytes_produz_pdf_valido() {
        let w = mock_world("Texto de teste");
        let source = w.source.clone();
        let (result, _warnings) = compile_to_pdf_bytes(&w, &source);
        let pdf = result.expect("compilação deve ter sucesso");
        assert!(!pdf.is_empty(), "bytes PDF devem existir");
        assert_eq!(&pdf[..5], b"%PDF-", "header PDF esperado");
    }

    // ── Passo 140B: dispatch font-aware ───────────────────────────────

    use ecow::EcoString;
    use typst_core::entities::font_book::{FontFlags, FontInfo, FontStretch, FontStyle, FontWeight};
    use typst_core::entities::font_list::{FontFamily, FontList};
    use typst_core::entities::layout_types::{FrameItem, Page, PagedDocument, Point, Pt, TextStyle};

    fn text_item_with_font(font: Option<FontList>) -> FrameItem {
        let mut style = TextStyle::regular(Pt(12.0));
        style.font = font;
        FrameItem::Text {
            pos:  Point::ZERO,
            text: "X".into(),
            style,
        }
    }

    fn page_with(items: Vec<FrameItem>) -> Page {
        Page { width: 100.0, height: 100.0, items }
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
        let doc = PagedDocument::new(vec![
            page_with(vec![text_item_with_font(None)]),
        ]);
        assert!(first_font_from_doc(&doc).is_none());
    }

    #[test]
    fn first_font_from_doc_com_font_primeira_vence() {
        // Dois items na mesma página com fonts diferentes.
        let doc = PagedDocument::new(vec![
            page_with(vec![
                text_item_with_font(Some(font_list("Primeira"))),
                text_item_with_font(Some(font_list("Segunda"))),
            ]),
        ]);
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
        book:    FontBook,
        fonts:   Vec<Option<Font>>,
    }
    impl World for FontMockWorld {
        fn library(&self) -> &Library  { &self.library }
        fn book(&self)    -> &FontBook { &self.book }
        fn main(&self)    -> FileId {
            FileId::from_raw(NonZeroU16::new(1).unwrap())
        }
        fn source(&self, _: FileId) -> FileResult<Source> { Err(FileError::NotFound) }
        fn file(&self, _: FileId)   -> FileResult<Bytes>  { Err(FileError::NotFound) }
        fn font(&self, i: usize)    -> Option<Font> { self.fonts.get(i).cloned().flatten() }
        fn today(&self, _: Option<i64>) -> Option<Datetime> { None }
    }

    fn font_info(family: &str) -> FontInfo {
        FontInfo {
            family:  family.into(),
            variant: FontVariant {
                style:   FontStyle::Normal,
                weight:  FontWeight::REGULAR,
                stretch: FontStretch::NORMAL,
            },
            flags: FontFlags::default(),
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
        let got = resolve_font(&fl, world.book(), &world).expect("deve resolver");
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
        assert!(resolve_font(&fl, world.book(), &world).is_none());
    }

    #[test]
    fn resolve_font_font_book_vazio_devolve_none() {
        let world = FontMockWorld {
            library: Library::new(),
            book:    FontBook::new(),
            fonts:   vec![],
        };
        let fl = font_list("Qualquer");
        assert!(resolve_font(&fl, world.book(), &world).is_none());
    }

    // ── Passo 141: array fallback chain ───────────────────────────────

    fn font_list_multi(names: &[&str]) -> FontList {
        let families = names.iter()
            .map(|n| FontFamily::new(EcoString::from(*n)))
            .collect();
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
        let got = resolve_font(&fl, world.book(), &world).expect("deve resolver");
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
        let got = resolve_font(&fl, world.book(), &world).expect("deve resolver via B");
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
        let got = resolve_font(&fl, world.book(), &world).expect("deve resolver via C");
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
        assert!(resolve_font(&fl, world.book(), &world).is_none(),
            "nenhuma família resolve → fallback Helvetica via None");
    }

    // ── Passo 146: multi-font per document ────────────────────────────

    #[test]
    fn collect_fonts_from_doc_documento_vazio_devolve_vazio() {
        let doc = PagedDocument::new(vec![]);
        assert!(collect_fonts_from_doc(&doc).is_empty());
    }

    #[test]
    fn collect_fonts_from_doc_uma_font_devolve_unitario() {
        let doc = PagedDocument::new(vec![
            page_with(vec![text_item_with_font(Some(font_list("Inria")))]),
        ]);
        let collected = collect_fonts_from_doc(&doc);
        assert_eq!(collected.len(), 1);
        assert_eq!(collected[0].as_slice()[0].name.as_str(), Some("inria"));
    }

    #[test]
    fn collect_fonts_from_doc_duas_distintas_devolve_par_em_ordem() {
        let doc = PagedDocument::new(vec![
            page_with(vec![
                text_item_with_font(Some(font_list("Primeira"))),
                text_item_with_font(Some(font_list("Segunda"))),
            ]),
        ]);
        let collected = collect_fonts_from_doc(&doc);
        assert_eq!(collected.len(), 2);
        assert_eq!(collected[0].as_slice()[0].name.as_str(), Some("primeira"));
        assert_eq!(collected[1].as_slice()[0].name.as_str(), Some("segunda"));
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
        let collected = collect_fonts_from_doc(&doc);
        assert_eq!(collected.len(), 2,
            "dedup estrutural: A e B aparecem cada um uma vez no resultado");
        assert_eq!(collected[0].as_slice()[0].name.as_str(), Some("a"));
        assert_eq!(collected[1].as_slice()[0].name.as_str(), Some("b"));
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
        let inputs = vec![font_list("A"), font_list("B")];
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
        let inputs = vec![font_list("A"), font_list("B")];
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
        let inputs = vec![font_list("X"), font_list("Y")];
        assert!(resolve_fonts(&inputs, world.book(), &world).is_empty());
    }

    // ── Passo 407: regex keys em FontList (DEBT-52) ─────────────────────

    use typst_core::entities::font_list::FontNamePattern;
    use typst_core::entities::regex::Regex;

    fn font_list_regex(pattern: &str) -> FontList {
        FontList::new(vec![
            FontFamily::new_regex(Regex::new(pattern).unwrap(), vec![]),
        ]).expect("lista não-vazia")
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
        let got = resolve_font(&fl, world.book(), &world).expect("deve resolver via regex");
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
        assert!(resolve_font(&fl, world.book(), &world).is_none());
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
        let got = resolve_font(&fl, world.book(), &world).expect("literal continua a resolver");
        assert_eq!(got, bytes);
    }
}
