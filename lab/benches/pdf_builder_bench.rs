//! PDF export cohorts. Fixtures are prepared outside the timed closure.
//! These measure complete exports, not isolated internal phase durations.
#![allow(deprecated)] // Intentionally exercises the Type1 text fallback.

#[path = "support/profile.rs"]
mod profile;

use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use std::sync::Arc;
use typst_core::entities::content::Content;
use typst_core::entities::font_book::FontVariant;
use typst_core::entities::font_list::FontList;
use typst_core::entities::font_variations::FontVariations;
use typst_core::entities::layout_types::{
    FrameItem, LinkTarget, Page, PagedDocument, Point, Pt, Size, TextStyle,
};
use typst_infra::export::{
    export_pdf_multifont, export_pdf_with_font, export_pdf_with_tags_and_serialization,
    PdfSerializationMode, PdfTags, StreamMode,
};

fn document(pages: usize, rows: usize) -> PagedDocument {
    PagedDocument::new(
        (0..pages)
            .map(|index| Page {
                width: 595.28,
                height: 841.89,
                numbering: None,
                logical_number: index + 1,
                supplement: Content::Empty,
                bleed: Default::default(),
                fill: Default::default(),
                background: vec![],
                foreground: vec![],
                items: (0..rows)
                    .map(|row| FrameItem::Text {
                        pos: Point { x: Pt(36.0), y: Pt(48.0 + row as f64 * 12.0) },
                        text: "PDF builder: repeated text and resource reuse.".into(),
                        style: TextStyle { size: Pt(10.0), ..Default::default() },
                        bidi: None,
                    })
                    .collect(),
            })
            .collect(),
    )
}

fn bench_builder(c: &mut Criterion) {
    profile::require_optimized();
    let small = document(1, 1);
    let dense = document(8, 48);
    let mut streams =
        c.benchmark_group("pdf_export_cohorts/builder/text_serialization_modes");
    for (name, doc) in [("small", &small), ("dense", &dense)] {
        for serialization in [PdfSerializationMode::Compact, PdfSerializationMode::Pretty]
        {
            streams.bench_with_input(
                BenchmarkId::new(name, format!("{serialization:?}")),
                doc,
                |b, doc| {
                    b.iter(|| {
                        black_box(export_pdf_with_tags_and_serialization(
                            black_box(doc),
                            StreamMode::Verbose,
                            PdfTags::Disabled,
                            serialization,
                        ))
                    })
                },
            );
        }
    }
    streams.finish();

    let mut serialization = c.benchmark_group("pdf_export_cohorts/builder/empty_pages");
    for pages in [1, 16, 64] {
        let doc = document(pages, 0);
        serialization.bench_with_input(
            BenchmarkId::from_parameter(pages),
            &doc,
            |b, doc| {
                b.iter(|| {
                    black_box(export_pdf_with_tags_and_serialization(
                        black_box(doc),
                        StreamMode::Compact,
                        PdfTags::Disabled,
                        PdfSerializationMode::Pretty,
                    ))
                })
            },
        );
    }
    serialization.finish();

    let mut metadata_doc = small.clone();
    metadata_doc.document_info.title = Some("Title <XML> & PDF (Unicode: café)".into());
    metadata_doc.document_info.author = Some("Author".into());
    metadata_doc.document_info.keywords = Some("PDF, export, metadata".into());
    let mut navigation_doc = dense.clone();
    for page in &mut navigation_doc.pages {
        page.items.extend((0..32).map(|index| FrameItem::Link {
            target: LinkTarget::Url(
                format!("https://example.invalid/link/{index}").into(),
            ),
            items: vec![],
            pos: Point { x: Pt(36.0), y: Pt(48.0 + index as f64 * 12.0) },
            size: Size { width: Pt(120.0), height: Pt(10.0) },
        }));
    }
    let mut resource_doc = small.clone();
    let image = Arc::new(
        include_bytes!("../../03_infra/fixtures/p307b/sources/tiny.jpg").to_vec(),
    );
    resource_doc.pages[0]
        .items
        .extend((0..32).map(|index| FrameItem::Image {
            pos: Point { x: Pt(36.0 + index as f64 * 4.0), y: Pt(80.0) },
            data: image.clone(),
            alt: None,
            width: Pt(16.0),
            height: Pt(16.0),
            intrinsic_width: 1,
            intrinsic_height: 1,
            clip_rect: None,
            orientation: 1,
        }));
    for (name, doc) in [
        ("metadata", &metadata_doc),
        ("navigation", &navigation_doc),
        ("resources", &resource_doc),
    ] {
        c.bench_function(&format!("pdf_export_cohorts/builder/{name}"), |b| {
            b.iter(|| {
                black_box(export_pdf_with_tags_and_serialization(
                    black_box(doc),
                    StreamMode::Verbose,
                    PdfTags::Disabled,
                    PdfSerializationMode::Compact,
                ))
            })
        });
    }

    let font = typst_assets::fonts().next().expect("embedded font");
    c.bench_function("pdf_export_cohorts/builder/fonts/single", |b| {
        b.iter(|| {
            black_box(export_pdf_with_font(
                black_box(&dense),
                black_box(font),
                StreamMode::Verbose,
            ))
        })
    });
    let fonts: Vec<_> = typst_assets::fonts()
        .take(2)
        .enumerate()
        .map(|(index, data)| {
            (
                (
                    FontList::single(format!("Bench Font {index}").into()),
                    FontVariant::default(),
                    FontVariations::default(),
                ),
                data.to_vec(),
            )
        })
        .collect();
    assert_eq!(fonts.len(), 2, "multifont fixture requires two embedded fonts");
    let mut multifont_doc = dense.clone();
    for page in &mut multifont_doc.pages {
        for (index, item) in page.items.iter_mut().enumerate() {
            if let FrameItem::Text { style, .. } = item {
                style.font = Some(fonts[index % fonts.len()].0 .0.clone());
            }
        }
    }
    c.bench_function("pdf_export_cohorts/builder/fonts/multifont_path", |b| {
        b.iter(|| {
            black_box(export_pdf_multifont(
                black_box(&multifont_doc),
                black_box(&fonts),
                StreamMode::Verbose,
            ))
        })
    });
}

criterion_group!(benches, bench_builder);
criterion_main!(benches);
