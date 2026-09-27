use criterion::{black_box, Criterion};
use typst_core::entities::content::Content;
use typst_core::entities::style::Styles;

use crate::fixtures;

fn mixed_tree() -> Content {
    let nodes = (0..128).flat_map(|index| {
        let text = Content::text(index.to_string());
        let styled = Content::styled(Box::new(text), Styles::new());
        [styled, Content::SpaceFromNewline]
    });
    Content::sequence(nodes.collect())
}

pub fn register(c: &mut Criterion) {
    let mut group = c.benchmark_group("content/morphology");
    let mixed = mixed_tree();
    let catalog = fixtures::representative();

    group.bench_function("canonicalize_mixed_tree", |b| {
        b.iter(|| black_box(mixed.morph_canon()));
    });
    group.bench_function("canonicalize_catalog", |b| {
        b.iter(|| {
            let output = catalog.iter().map(Content::morph_canon).collect::<Vec<_>>();
            black_box(output)
        });
    });
    group.finish();
}
