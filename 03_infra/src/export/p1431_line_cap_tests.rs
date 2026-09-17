//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/infra/export/p1431-line-cap-tests.md
//! @prompt-hash db758b4e
//! @layer L3
#![cfg(test)]

use super::*;
use std::num::NonZeroU16;
use typst_core::compiler::eval::EvalContext;
use typst_core::compiler::layout::layout;
use typst_core::compiler::stdlib::make_math_module;
use typst_core::contracts::world::World;
use typst_core::entities::args::Args;
use typst_core::entities::content::Content;
use typst_core::entities::file_id::FileId;
use typst_core::entities::font_book::FontBook;
use typst_core::entities::geometry::{LineCap, Stroke};
use typst_core::entities::layout_types::{
    FrameItem, PagedDocument, Point, Pt, TransformMatrix,
};
use typst_core::entities::page_canvas::PageFill;
use typst_core::entities::source::Source;
use typst_core::entities::value::Value;
use typst_core::entities::world_types::{
    Bytes, Datetime, FileError, FileResult, Font, Library,
};

#[derive(Default)]
struct NullWorld {
    library: Library,
    book: FontBook,
}
impl World for NullWorld {
    fn library(&self) -> &Library {
        &self.library
    }
    fn book(&self) -> &FontBook {
        &self.book
    }
    fn main(&self) -> FileId {
        id()
    }
    fn source(&self, _: FileId) -> FileResult<Source> {
        Err(FileError::NotFound)
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
fn id() -> FileId {
    FileId::from_raw(NonZeroU16::new(1431).unwrap())
}
fn cv(text: &str) -> Value {
    Value::Content(Content::text(text))
}
fn matrix_document(cap: Option<LineCap>) -> PagedDocument {
    let Value::Module(module) = make_math_module() else { panic!("math module") };
    let Some(Value::Func(function)) = module.scope().get("mat") else {
        panic!("math.mat")
    };
    let Value::Dict(mut augment) = Value::Dict(Default::default()) else {
        unreachable!()
    };
    augment.insert("vline".into(), Value::Int(1));
    if let Some(cap) = cap {
        let mut stroke = Stroke::default();
        stroke.thickness = 0.6;
        stroke.cap = cap;
        stroke.specified.thickness = true;
        stroke.specified.cap = true;
        augment.insert("stroke".into(), Value::Stroke(stroke));
    }
    let mut args = Args::positional(vec![
        Value::Array(vec![cv("a"), cv("b")]),
        Value::Array(vec![cv("c"), cv("d")]),
    ]);
    args.named.insert("delim".into(), Value::None);
    args.named.insert("augment".into(), Value::Dict(augment));
    let Value::Content(matrix) = function.native_fn_addr().unwrap()(
        &mut EvalContext::new(),
        &args,
        &NullWorld::default(),
        id(),
    )
    .unwrap() else {
        panic!("mat content")
    };
    layout(&Content::equation(matrix, false))
}
fn first_line(items: &[FrameItem]) -> Option<FrameItem> {
    for item in items {
        match item {
            FrameItem::Line { .. } => return Some(item.clone()),
            FrameItem::Group { items, .. }
            | FrameItem::Link { items, .. }
            | FrameItem::Semantic { items, .. } => {
                if let Some(line) = first_line(items) {
                    return Some(line);
                }
            }
            _ => {}
        }
    }
    None
}
fn isolated_page(
    document: &PagedDocument,
    local: bool,
) -> typst_core::entities::layout_types::Page {
    let source = document.pages.first().expect("matrix page");
    let line = first_line(&source.items).expect("one attributable matrix line");
    let mut page = source.clone();
    page.fill = PageFill::None;
    page.background.clear();
    page.foreground.clear();
    page.items = if local {
        vec![FrameItem::Group {
            pos: Point { x: Pt(0.0), y: Pt(0.0) },
            matrix: TransformMatrix::identity(),
            clip_mask: None,
            inner_width: page.width,
            inner_height: page.height,
            items: vec![line],
        }]
    } else {
        vec![line]
    };
    page
}
fn alpha_pixels(png: &[u8]) -> usize {
    image::load_from_memory(png)
        .expect("deterministic PNG")
        .to_rgba8()
        .pixels()
        .filter(|pixel| pixel.0[3] != 0)
        .count()
}
fn page_stream(page: typst_core::entities::layout_types::Page) -> String {
    let pdf = export_pdf(&PagedDocument::new(vec![page]), StreamMode::Verbose);
    extract_page_content_streams_text(&pdf)
}
fn assert_scoped(stream: &str, operator: &str) {
    let at = stream
        .find(operator)
        .unwrap_or_else(|| panic!("missing {operator}: {stream}"));
    assert!(stream[..at].rfind('q').is_some(), "{operator} outside q: {stream}");
    assert!(stream[at..].find('Q').is_some(), "{operator} outside Q: {stream}");
}

#[test]
fn p1431_raster_mapeia_butt_round_square_e_auto_square() {
    let opts = RenderOptions { pixel_per_pt: 16.0, render_bleed: false };
    let render = |cap| {
        let doc = matrix_document(cap);
        render_page_to_png(&isolated_page(&doc, false), &opts)
    };
    let butt = render(Some(LineCap::Butt));
    let round = render(Some(LineCap::Round));
    let square = render(Some(LineCap::Square));
    let auto = render(None);
    let counts = [alpha_pixels(&butt), alpha_pixels(&round), alpha_pixels(&square)];
    assert!(counts[0] < counts[1] && counts[1] < counts[2], "P35 endpoints: {counts:?}");
    assert_eq!(auto, square, "P35 matrix auto must rasterize as Square");
}

#[test]
fn p1431_pdf_mapeia_caps_nos_caminhos_top_level_e_local() {
    for local in [false, true] {
        let butt_doc = matrix_document(Some(LineCap::Butt));
        let round_doc = matrix_document(Some(LineCap::Round));
        let square_doc = matrix_document(Some(LineCap::Square));
        let butt = page_stream(isolated_page(&butt_doc, local));
        let round = page_stream(isolated_page(&round_doc, local));
        let square = page_stream(isolated_page(&square_doc, local));
        assert!(
            !butt.contains("0 J") && !butt.contains("1 J") && !butt.contains("2 J"),
            "P36 Butt: {butt}"
        );
        assert_scoped(&round, "1 J");
        assert_scoped(&square, "2 J");
    }
}

#[test]
fn p1431_pdf_legado_butt_preserva_bytes_sem_zero_j() {
    let legacy = layout(&Content::equation(
        Content::math_frac(Content::text("x"), Content::text("y")),
        false,
    ));
    let first = export_pdf(&legacy, StreamMode::Verbose);
    let second = export_pdf(&legacy, StreamMode::Verbose);
    assert_eq!(first, second, "P36 legacy Butt bytes must be deterministic");
    assert!(!extract_page_content_streams_text(&first).contains("0 J"));
}

#[test]
fn p1431_export_opaco_falha_fechado() {
    let legacy = layout(&Content::equation(
        Content::math_frac(Content::text("x"), Content::text("y")),
        false,
    ));
    let stream =
        extract_page_content_streams_text(&export_pdf(&legacy, StreamMode::Verbose));
    let truncated = stream.get(..stream.len().min(2));
    let verdict = match truncated {
        Some(value) if value.contains("1 J") => ("PRESERVED_PDF_LINE_CAP", true, false),
        _ => ("UNKNOWN_EXPORT_OBSERVATION", false, false),
    };
    assert_eq!(verdict, ("UNKNOWN_EXPORT_OBSERVATION", false, false));
}
