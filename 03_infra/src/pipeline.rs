//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/infra/pipeline.md
//! @prompt-hash 367f8790
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

use comemo::Track;

use typst_core::contracts::world::World;
use typst_core::entities::font_book::{FontBook, FontVariant};
use typst_core::entities::font_list::FontList;
use typst_core::entities::layout_types::{FrameItem, PagedDocument};
use typst_core::entities::module::Module;
use typst_core::entities::source::Source;
use typst_core::entities::source_result::{SourceDiagnostic, SourceResult};
use typst_core::entities::world_types::{Route, Routines, Sink, Traced};
use typst_core::rules::eval::eval;
use typst_core::rules::introspect::introspect;
use typst_core::rules::layout::layout;

use crate::export::{export_pdf, export_pdf_with_font};

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
    let routines = Routines::new();
    let traced   = Traced::default();
    let mut sink = Sink::new();
    let route    = Route::root();
    let result = eval(
        &routines,
        world,
        traced.track(),
        sink.track_mut(),
        route.track(),
        source,
    );
    let warnings = sink.into_diagnostics();
    (result, warnings)
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
    let (eval_result, warnings) = eval_to_module_with_sink(world, source);
    let module = match eval_result {
        Ok(m) => m,
        Err(errors) => return (Err(errors), warnings),
    };
    let content = match module.content() {
        Some(c) => c,
        None => return (Ok(Vec::new()), warnings),
    };
    let state = introspect(content);
    let doc   = layout(content, state);
    let pdf   = match first_font_from_doc(&doc)
        .and_then(|fl| resolve_font(&fl, world.book(), world))
    {
        Some(bytes) => export_pdf_with_font(&doc, &bytes),
        None        => export_pdf(&doc),
    };
    (Ok(pdf), warnings)
}

/// Itera `doc.pages → items` recursivamente (atravessa `Group`)
/// e devolve o primeiro `TextStyle.font` com `Some(FontList)`.
/// MVP single-font: primeira vence (ADR-0055 decisão 3).
fn first_font_from_doc(doc: &PagedDocument) -> Option<FontList> {
    for page in &doc.pages {
        if let Some(fl) = first_font_in_items(&page.items) {
            return Some(fl);
        }
    }
    None
}

fn first_font_in_items(items: &[FrameItem]) -> Option<FontList> {
    for item in items {
        match item {
            FrameItem::Text { style, .. } => {
                if let Some(fl) = &style.font {
                    return Some(fl.clone());
                }
            }
            FrameItem::Group { items, .. } => {
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

/// Resolve a primeira família de `font_list` contra `font_book`
/// e carrega os bytes via `world.font(index)`. Apenas a primeira
/// família é tentada — array fallback chain é Passo 141.
///
/// Selecção usa `FontVariant::default()` (regular). Weight/style
/// continuam a ser renderizados por faux-bold (Passo 139).
fn resolve_font(
    font_list: &FontList,
    font_book: &FontBook,
    world:     &dyn World,
) -> Option<Vec<u8>> {
    let first   = font_list.as_slice().first()?;
    let variant = FontVariant::default();
    let index   = font_book.select(&first.name, &variant)?;
    let font    = world.font(index)?;
    Some(font.as_slice().to_vec())
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
    use typst_core::entities::font_list::FontList;
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
        assert_eq!(fl.as_slice()[0].name, "primeira");
    }

    #[test]
    fn first_font_from_doc_font_em_pagina_segunda_encontrada() {
        // Página 1 sem font, página 2 com font.
        let doc = PagedDocument::new(vec![
            page_with(vec![text_item_with_font(None)]),
            page_with(vec![text_item_with_font(Some(font_list("Inria Serif")))]),
        ]);
        let fl = first_font_from_doc(&doc).expect("deve encontrar na segunda");
        assert_eq!(fl.as_slice()[0].name, "inria serif");
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
}
