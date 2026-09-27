use criterion::{black_box, Criterion};
use typst_core::entities::content::Content;

use crate::fixtures;

fn wide_text() -> Content {
    Content::sequence(
        (0..256)
            .flat_map(|index| [Content::text(index.to_string()), Content::Space])
            .collect(),
    )
}

fn deep_empty(depth: usize) -> Content {
    (0..depth).fold(Content::Empty, |body, _| Content::par(body))
}

pub fn register(c: &mut Criterion) {
    let mut group = c.benchmark_group("content/projections");
    let text = wide_text();
    let empty = deep_empty(64);
    let heading = Content::heading(2, Content::text("Heading"));
    let catalog = fixtures::representative();

    group.bench_function("is_empty_deep", |b| {
        b.iter(|| black_box(empty.is_empty()));
    });
    group.bench_function("plain_text_wide", |b| {
        b.iter(|| black_box(text.plain_text()));
    });
    group.bench_function("get_field_hit", |b| {
        b.iter(|| black_box(heading.get_field(black_box("body"))));
    });
    group.bench_function("get_field_miss", |b| {
        b.iter(|| black_box(heading.get_field(black_box("missing"))));
    });
    group.bench_function("is_empty_catalog", |b| {
        b.iter(|| {
            let empty = catalog.iter().filter(|content| content.is_empty()).count();
            black_box(empty)
        });
    });
    group.bench_function("plain_text_catalog", |b| {
        b.iter(|| {
            let output = catalog.iter().map(Content::plain_text).collect::<Vec<_>>();
            black_box(output)
        });
    });
    group.bench_function("get_field_miss_catalog", |b| {
        b.iter(|| {
            let output = catalog
                .iter()
                .map(|content| content.get_field("missing"))
                .collect::<Vec<_>>();
            black_box(output)
        });
    });
    group.finish();
}
