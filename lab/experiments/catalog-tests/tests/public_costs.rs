use std::hint::black_box;
use std::mem::{align_of, size_of};
use std::time::Instant;
use typst_core::entities::{
    content::Content, layout_types::FrameItem, scope::Binding, syntax_kind::SyntaxKind,
    syntax_node::SyntaxNode, value::Value,
};

#[test]
fn carrier_layout_observation() {
    macro_rules! report {
        ($ty:ty) => {
            println!(
                "carrier={} size_bytes={} align_bytes={}",
                stringify!($ty),
                size_of::<$ty>(),
                align_of::<$ty>()
            );
            assert_eq!(size_of::<$ty>() % align_of::<$ty>(), 0);
        };
    }
    report!(Value);
    report!(Content);
    report!(FrameItem);
    report!(Binding);
    report!(SyntaxNode);
    println!(
        "live_variant_frequencies=Unknown heap_retention=Unknown target={}-{}",
        std::env::consts::ARCH,
        std::env::consts::OS
    );
}

fn nested_collections(n: usize) -> Value {
    let dict = Default::default();
    // Infer the public IndexMap/hasher from Value::Dict; no copied dictionary implementation.
    let mut value = Value::Dict(dict);
    if let Value::Dict(ref mut d) = value {
        for i in 0..n {
            d.insert(
                format!("key{i:05}").into(),
                Value::Array(vec![
                    Value::Int(i as i64),
                    Value::Str("long shared string for clone cost".into()),
                ]),
            );
        }
    }
    value
}

#[test]
fn collection_clone_mutation_isolation_and_order() {
    let original = Value::Array(vec![nested_collections(16)]);
    let mut changed = original.clone();
    let Value::Array(ref mut a) = changed else { unreachable!() };
    let Value::Dict(ref mut d) = a[0] else { unreachable!() };
    let before_keys: Vec<_> = d.keys().cloned().collect();
    let Value::Array(ref mut child) = d.get_mut("key00000").unwrap() else {
        unreachable!()
    };
    child[0] = Value::Int(-1);
    child.push(Value::Bool(true));
    d.insert("last".into(), Value::None);
    let Value::Array(ref orig_a) = original else { unreachable!() };
    let Value::Dict(ref orig_d) = orig_a[0] else { unreachable!() };
    assert_eq!(orig_d.keys().cloned().collect::<Vec<_>>(), before_keys);
    let Value::Array(ref orig_child) = orig_d["key00000"] else { unreachable!() };
    assert_eq!(
        orig_child,
        &[Value::Int(0), Value::Str("long shared string for clone cost".into())]
    );
    assert_eq!(d.keys().last().unwrap().as_str(), "last");
}

#[test]
#[ignore = "cost observation; execute alone in release with external allocation/RSS profiler"]
fn measure_collection_clones() {
    for count in [16, 256, 4096] {
        let original = nested_collections(count);
        let iterations = 100;
        let start = Instant::now();
        for _ in 0..iterations {
            black_box(black_box(&original).clone());
        }
        println!("operation=dict_nested_array_clone entries={count} iterations={iterations} elapsed_ns={} allocations=Unknown bytes_copied=Unknown", start.elapsed().as_nanos());
    }
}

fn cst(depth: usize, leaves: usize) -> (SyntaxNode, String) {
    let text = "áβ x\t\n";
    let mut node = SyntaxNode::inner(
        SyntaxKind::Markup,
        (0..leaves)
            .map(|_| SyntaxNode::leaf(SyntaxKind::Text, text))
            .collect(),
    );
    for _ in 0..depth {
        node = SyntaxNode::inner(SyntaxKind::Markup, vec![node]);
    }
    (node, text.repeat(leaves))
}
#[test]
fn cst_into_text_preserves_deep_unicode_text() {
    for depth in [0, 1, 32, 256] {
        let (node, expected) = cst(depth, 32);
        let retained = node.clone();
        assert_eq!(node.len(), expected.len());
        assert_eq!(node.into_text().as_str(), expected);
        assert_eq!(retained.into_text().as_str(), expected);
    }
}
#[test]
#[ignore = "cost observation; execute alone in release with external allocation/RSS profiler"]
fn measure_cst_into_text_depth() {
    for depth in [0, 16, 64, 256] {
        let (node, expected) = cst(depth, 1024);
        let start = Instant::now();
        let iterations = 100;
        for _ in 0..iterations {
            let text = black_box(node.clone()).into_text();
            assert_eq!(text.as_str(), expected);
            black_box(text);
        }
        println!("operation=cst_clone_into_text depth={depth} output_bytes={} iterations={iterations} elapsed_ns={} copied_bytes=Unknown productive_call_frequency=Unknown", expected.len(), start.elapsed().as_nanos());
    }
}

use typst_core::entities::{source::Source, source_result::SourceDiagnostic, span::Span};
use typst_shell::diagnostic::{format_diagnostic, DiagnosticSource};
fn diagnostic_fixture(lines: usize) -> (DiagnosticSource, SourceDiagnostic) {
    let text = "plain text áβ\tend\n".repeat(lines);
    let source = Source::detached(text).unwrap();
    let offset = source.text().find("áβ").unwrap();
    let diag = SourceDiagnostic::error(
        Span::from_range(source.id(), offset..offset + "áβ".len()),
        "catalog diagnostic",
    )
    .with_hint("retain this hint");
    (DiagnosticSource::new(source, "catalog-large.typ"), diag)
}
#[test]
fn diagnostic_formatter_unicode_hint_and_missing_source_control() {
    let (source, diag) = diagnostic_fixture(8);
    let formatted = format_diagnostic(&diag, std::slice::from_ref(&source), false);
    assert!(formatted.contains("catalog diagnostic"));
    assert!(formatted.contains("retain this hint"));
    assert!(formatted.contains("catalog-large.typ"));
    assert!(formatted.contains("áβ"));
    let missing = format_diagnostic(&diag, &[], false);
    assert!(missing.contains("catalog diagnostic"));
    assert!(missing.contains("retain this hint"));
    assert!(!missing.contains("catalog-large.typ"));
}
#[test]
#[ignore = "presentation-only observation; execute alone in release"]
fn measure_diagnostic_formatter_large_source() {
    for lines in [64, 4096, 65536] {
        let (source, diag) = diagnostic_fixture(lines);
        let input_bytes = source.source().len_bytes();
        let start = Instant::now();
        let iterations = 100;
        let mut output_bytes = 0;
        for _ in 0..iterations {
            output_bytes += black_box(format_diagnostic(
                black_box(&diag),
                std::slice::from_ref(&source),
                false,
            ))
            .len();
        }
        println!("operation=diagnostic_formatter source_bytes={input_bytes} iterations={iterations} output_bytes={output_bytes} elapsed_ns={} source_copy_calls=Unknown copied_bytes=Unknown", start.elapsed().as_nanos());
    }
}

use typst_core::contracts::world::World;
use typst_core::entities::{
    duration::Duration,
    file_id::FileId,
    font_book::FontBook,
    font_list::FontList,
    layout_types::{Page, PagedDocument, Point, Pt, TextStyle},
    world_types::{Bytes, Datetime, FileError, FileResult, Font, Library},
};
struct FontWorld {
    library: Library,
    book: FontBook,
    fonts: Vec<Font>,
}
impl FontWorld {
    fn embedded() -> Self {
        let embedded = typst_infra::embedded_fonts::load_embedded_fonts();
        let fonts = embedded
            .text_slots
            .iter()
            .map(|slot| slot.get().expect("embedded font must load"))
            .collect();
        Self {
            library: Library::new(),
            book: embedded.text_book,
            fonts,
        }
    }
    fn style(&self) -> TextStyle {
        TextStyle {
            size: Pt(12.0),
            font: Some(FontList::single(self.book.infos()[0].family.clone().into())),
            ..Default::default()
        }
    }
}
impl World for FontWorld {
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
        Err(FileError::NotFound)
    }
    fn file(&self, _: FileId) -> FileResult<Bytes> {
        Err(FileError::NotFound)
    }
    fn font(&self, index: usize) -> Option<Font> {
        self.fonts.get(index).cloned()
    }
    fn today(&self, _: Option<Duration>) -> Option<Datetime> {
        None
    }
}
fn text_document(world: &FontWorld, texts: &[String]) -> PagedDocument {
    PagedDocument::new(vec![Page {
        width: 600.0,
        height: 800.0,
        numbering: None,
        logical_number: 1,
        supplement: Content::Empty,
        bleed: Default::default(),
        fill: Default::default(),
        background: vec![],
        foreground: vec![],
        items: texts
            .iter()
            .enumerate()
            .map(|(i, text)| FrameItem::Text {
                pos: Point { x: Pt(0.0), y: Pt(i as f64 * 14.0) },
                text: text.as_str().into(),
                style: world.style(),
                bidi: None,
            })
            .collect(),
    }])
}
fn shaped_signature(
    doc: &PagedDocument,
) -> Vec<(String, Vec<typst_core::entities::layout_types::ShapedGlyph>)> {
    doc.pages[0]
        .items
        .iter()
        .map(|item| match item {
            FrameItem::TextShaped { text, glyphs, .. } => {
                assert!(!glyphs.is_empty(), "real shaping is required");
                (text.to_string(), glyphs.clone())
            }
            other => panic!("expected real TextShaped, got {other:?}"),
        })
        .collect()
}
#[test]
fn shaping_repetition_matches_fresh_single_run_and_metrics() {
    use typst_core::compiler::layout::FontMetrics;
    let world = FontWorld::embedded();
    let text = "office café 123".to_string();
    let repeated = shaped_signature(&typst_infra::shaper::shape_document(
        &world,
        text_document(&world, &[text.clone(), text.clone()]),
    ));
    let fresh = shaped_signature(&typst_infra::shaper::shape_document(
        &world,
        text_document(&world, &[text.clone()]),
    ));
    assert_eq!(repeated[0], fresh[0]);
    assert_eq!(repeated[1], fresh[0]);
    let metrics = typst_infra::font_metrics::FallbackFontMetrics::new(&world);
    let a = metrics.advance(&text, Pt(12.0), &world.style());
    assert!(a.0.is_finite() && a.0 > 0.0);
    assert_eq!(metrics.advance(&text, Pt(12.0), &world.style()), a);
    assert_eq!(
        typst_infra::font_metrics::FallbackFontMetrics::new(&world).advance(
            &text,
            Pt(12.0),
            &world.style()
        ),
        a
    );
}
#[test]
#[ignore = "cost observation; execute alone in release with external allocation/RSS profiler"]
fn measure_shaping_and_metrics_repeated_and_unique() {
    use typst_core::compiler::layout::FontMetrics;
    let world = FontWorld::embedded();
    for unique in [false, true] {
        let texts: Vec<_> = (0..256)
            .map(|i| {
                if unique {
                    format!("office café {i:04}")
                } else {
                    "office café 0000".into()
                }
            })
            .collect();
        let doc = text_document(&world, &texts);
        let start = Instant::now();
        let shaped = typst_infra::shaper::shape_document(&world, doc);
        let elapsed = start.elapsed().as_nanos();
        let signature = shaped_signature(&shaped);
        let glyphs: usize = signature.iter().map(|(_, glyphs)| glyphs.len()).sum();
        println!("operation=shape_document unique={unique} input_runs={} output_runs={} glyphs={glyphs} elapsed_ns={elapsed} cache_hits=Unknown key_allocated_bytes=Unknown", texts.len(), signature.len());
        let metrics = typst_infra::font_metrics::FallbackFontMetrics::new(&world);
        let style = world.style();
        let start = Instant::now();
        for _ in 0..32 {
            for text in &texts {
                black_box(metrics.advance(black_box(text), Pt(12.0), &style));
            }
        }
        println!("operation=metrics_advance unique={unique} calls={} elapsed_ns={} cache_hits=Unknown key_allocated_bytes=Unknown", 32 * texts.len(), start.elapsed().as_nanos());
    }
}
