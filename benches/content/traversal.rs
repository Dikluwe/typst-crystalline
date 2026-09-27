use criterion::{black_box, Criterion};
use typst_core::entities::content::Content;
use typst_core::entities::source_result::SourceResult;

use crate::fixtures;

fn wide_tree() -> Content {
    Content::sequence((0..256).map(|index| Content::text(index.to_string())).collect())
}

fn deep_tree(depth: usize) -> Content {
    (0..depth).fold(Content::text("leaf"), |body, _| Content::par(body))
}

pub fn register(c: &mut Criterion) {
    let mut group = c.benchmark_group("content/traversal");
    let wide = wide_tree();
    let deep = deep_tree(64);
    let catalog = fixtures::representative();

    group.bench_function("map_content_noop_wide", |b| {
        b.iter(|| {
            let mut transform =
                |_: &Content| -> SourceResult<Option<Content>> { Ok(None) };
            black_box(wide.map_content(&mut transform).unwrap())
        });
    });
    group.bench_function("map_content_noop_deep", |b| {
        b.iter(|| {
            let mut transform =
                |_: &Content| -> SourceResult<Option<Content>> { Ok(None) };
            black_box(deep.map_content(&mut transform).unwrap())
        });
    });
    group.bench_function("map_text_identity_wide", |b| {
        b.iter(|| {
            let mut identity = |text: &str| text.to_owned();
            black_box(wide.map_text(&mut identity))
        });
    });
    group.bench_function("map_text_transform_wide", |b| {
        b.iter(|| {
            let mut uppercase = |text: &str| text.to_uppercase();
            black_box(wide.map_text(&mut uppercase))
        });
    });
    group.bench_function("map_content_noop_catalog", |b| {
        b.iter(|| {
            let mut transform =
                |_: &Content| -> SourceResult<Option<Content>> { Ok(None) };
            let output = catalog
                .iter()
                .map(|content| content.map_content(&mut transform).unwrap())
                .collect::<Vec<_>>();
            black_box(output)
        });
    });
    group.bench_function("map_text_identity_catalog", |b| {
        b.iter(|| {
            let mut identity = |text: &str| text.to_owned();
            let output = catalog
                .iter()
                .map(|content| content.map_text(&mut identity))
                .collect::<Vec<_>>();
            black_box(output)
        });
    });
    group.finish();
}
