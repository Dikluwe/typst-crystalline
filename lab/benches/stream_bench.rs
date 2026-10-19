//! Complete PDF export cohorts organized by page/text/shape/draw fixtures.
//! Times include resource discovery, builder and serialization; not isolated emitters.
//! Fixtures and correctness smoke checks are outside the timed closure.
#![allow(deprecated)] // Type1 fallback intentionally avoids font subsetting here.

#[path = "support/profile.rs"]
mod profile;

use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use typst_core::entities::content::Content;
use typst_core::entities::frame_paint::FramePaint as Paint;
use typst_core::entities::geometry::{FillRule, PathItem, ShapeKind};
use typst_core::entities::layout_types::{
    Color, FrameItem, LinkTarget, Page, PagedDocument, Point, Pt, SemanticKind,
    SemanticPlacement, Size, TextStyle, TransformMatrix,
};
use typst_infra::export::{
    export_pdf_with_tags_and_serialization, PdfSerializationMode, PdfTags, StreamMode,
};

fn point(x: f64, y: f64) -> Point {
    Point { x: Pt(x), y: Pt(y) }
}

fn text(index: usize) -> FrameItem {
    FrameItem::Text {
        pos: point(24.0, 24.0 + index as f64 * 4.0),
        text: "Stream benchmark".into(),
        style: TextStyle { size: Pt(10.0), ..Default::default() },
        bidi: None,
    }
}

fn shape(kind: ShapeKind<Pt>) -> FrameItem {
    FrameItem::Shape {
        pos: point(24.0, 24.0),
        kind,
        width: 40.0,
        height: 30.0,
        fill: Some(Paint::Solid(Color::rgb(20, 80, 160))),
        stroke: None,
        fill_rule: FillRule::NonZero,
        parent_bbox_at_emit: None,
    }
}

fn document(items: Vec<FrameItem>) -> PagedDocument {
    PagedDocument::new(vec![Page {
        width: 595.28,
        height: 841.89,
        numbering: None,
        logical_number: 1,
        supplement: Content::Empty,
        bleed: Default::default(),
        fill: Default::default(),
        background: vec![],
        foreground: vec![],
        items,
    }])
}

fn bench_stream(c: &mut Criterion) {
    profile::require_optimized();
    let path = ShapeKind::Path(vec![
        PathItem::MoveTo(point(0.0, 0.0)),
        PathItem::CubicTo(point(10.0, 30.0), point(30.0, 30.0), point(40.0, 0.0)),
        PathItem::ClosePath,
    ]);
    let group = FrameItem::Group {
        paint_root: false,
        pos: point(50.0, 50.0),
        matrix: TransformMatrix::rotate(0.2),
        clip_mask: Some(ShapeKind::Rect),
        inner_width: 100.0,
        inner_height: 100.0,
        items: vec![text(0), shape(ShapeKind::Rect)],
    };
    let semantic = FrameItem::Semantic {
        kind: SemanticKind::Formula,
        placement: SemanticPlacement::Inline,
        alt: Some("formula".into()),
        items: vec![text(0)],
    };
    let link = FrameItem::Link {
        target: LinkTarget::Url("https://example.invalid/stream".into()),
        items: vec![text(0)],
        pos: point(24.0, 24.0),
        size: Size { width: Pt(100.0), height: Pt(12.0) },
    };
    let cases = [
        ("page/empty", document(vec![]), PdfTags::Disabled),
        ("text/single", document(vec![text(0)]), PdfTags::Disabled),
        ("text/dense", document((0..128).map(text).collect()), PdfTags::Disabled),
        (
            "shape/rectangles",
            document(vec![shape(ShapeKind::Rect); 128]),
            PdfTags::Disabled,
        ),
        ("shape/bezier", document(vec![shape(path); 128]), PdfTags::Disabled),
        ("draw/clipped_groups", document(vec![group; 32]), PdfTags::Disabled),
        (
            "draw/semantic_untagged",
            document(vec![semantic.clone(); 32]),
            PdfTags::Disabled,
        ),
        ("draw/semantic_tagged", document(vec![semantic; 32]), PdfTags::Enabled),
        ("draw/links", document(vec![link; 32]), PdfTags::Disabled),
    ];
    let mut group = c.benchmark_group("pdf_export_cohorts/pretty_serialization");
    for (name, doc, tags) in &cases {
        for mode in [StreamMode::Compact, StreamMode::Verbose] {
            let smoke = export_pdf_with_tags_and_serialization(
                doc,
                mode,
                *tags,
                PdfSerializationMode::Pretty,
            );
            assert!(smoke.starts_with(b"%PDF-"), "{name}/{mode:?}: invalid PDF header");
            assert!(
                smoke.windows(5).any(|bytes| bytes == b"%%EOF"),
                "{name}/{mode:?}: missing trailer"
            );
            group.bench_with_input(
                BenchmarkId::new(*name, format!("{mode:?}")),
                doc,
                |b, doc| {
                    b.iter(|| {
                        black_box(export_pdf_with_tags_and_serialization(
                            black_box(doc),
                            mode,
                            *tags,
                            PdfSerializationMode::Pretty,
                        ))
                    })
                },
            );
        }
    }
    group.finish();
}
criterion_group!(benches, bench_stream);
criterion_main!(benches);
