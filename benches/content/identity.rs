use criterion::{black_box, Criterion};

use crate::fixtures;

pub fn register(c: &mut Criterion) {
    let mut group = c.benchmark_group("content/identity");
    let catalog = fixtures::representative();

    group.bench_function("elem_name_catalog", |b| {
        b.iter(|| {
            let total = catalog
                .iter()
                .map(|content| black_box(content).elem_name().len())
                .sum::<usize>();
            black_box(total)
        });
    });
    group.bench_function("debug_catalog", |b| {
        b.iter(|| {
            let output = catalog
                .iter()
                .map(|content| format!("{content:?}"))
                .collect::<Vec<_>>();
            black_box(output)
        });
    });
    group.finish();
}
