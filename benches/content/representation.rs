use std::hash::{DefaultHasher, Hash, Hasher};

use criterion::{black_box, Criterion};
use typst_core::entities::content::Content;

use crate::fixtures;

fn text_sequence(size: usize) -> Content {
    Content::sequence((0..size).map(|index| Content::text(index.to_string())).collect())
}

pub fn register(c: &mut Criterion) {
    let mut group = c.benchmark_group("content/representation");
    let leaf = Content::text("crystalline");
    let sequence = text_sequence(256);
    let equal_sequence = text_sequence(256);
    let different_sequence = Content::sequence(
        (0..256)
            .map(|index| {
                if index == 255 {
                    Content::text("different")
                } else {
                    Content::text(index.to_string())
                }
            })
            .collect(),
    );
    let catalog = fixtures::representative();
    let equal_catalog = fixtures::representative();

    group.bench_function("clone_leaf", |b| {
        b.iter(|| black_box(leaf.clone()));
    });
    group.bench_function("clone_sequence", |b| {
        b.iter(|| black_box(sequence.clone()));
    });
    group.bench_function("equality_sequence_equal", |b| {
        b.iter(|| black_box(&sequence) == black_box(&equal_sequence));
    });
    group.bench_function("equality_sequence_late_difference", |b| {
        b.iter(|| black_box(&sequence) == black_box(&different_sequence));
    });
    group.bench_function("rust_hash_sequence", |b| {
        b.iter(|| {
            let mut state = DefaultHasher::new();
            black_box(&sequence).hash(&mut state);
            black_box(state.finish())
        });
    });
    group.bench_function("clone_catalog", |b| {
        b.iter(|| black_box(catalog.clone()));
    });
    group.bench_function("equality_catalog_equal", |b| {
        b.iter(|| {
            let all_equal = catalog
                .iter()
                .zip(equal_catalog.iter())
                .all(|(left, right)| black_box(left) == black_box(right));
            black_box(all_equal)
        });
    });
    group.bench_function("rust_hash_catalog", |b| {
        b.iter(|| {
            let mut state = DefaultHasher::new();
            for content in black_box(&catalog) {
                content.hash(&mut state);
            }
            black_box(state.finish())
        });
    });
    group.finish();
}
