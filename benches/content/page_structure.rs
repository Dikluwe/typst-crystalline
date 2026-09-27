use std::sync::Arc;

use criterion::{black_box, BatchSize, Criterion};
use typst_core::entities::content::{Content, SetPageDelta};

fn without_columns() -> Content {
    Content::sequence((0..128).map(|index| Content::text(index.to_string())).collect())
}

fn with_columns() -> Content {
    let set_page = Content::SetPage(Arc::new(SetPageDelta {
        columns: Some(3),
        ..SetPageDelta::default()
    }));
    let body = Content::sequence(
        (0..128).map(|index| Content::text(index.to_string())).collect(),
    );
    Content::sequence(vec![set_page, body, Content::pagebreak(false, None)])
}

pub fn register(c: &mut Criterion) {
    let mut group = c.benchmark_group("content/page_structure");
    let fast_path = without_columns();
    let rewrite = with_columns();

    group.bench_function("wrap_columns_fast_path", |b| {
        b.iter_batched(
            || fast_path.clone(),
            |content| black_box(content.wrap_page_columns()),
            BatchSize::SmallInput,
        );
    });
    group.bench_function("wrap_columns_rewrite", |b| {
        b.iter_batched(
            || rewrite.clone(),
            |content| black_box(content.wrap_page_columns()),
            BatchSize::SmallInput,
        );
    });
    group.finish();
}
