use criterion::{black_box, BatchSize, Criterion, Throughput};
use typst_core::entities::content::Content;

fn parts(size: usize) -> Vec<Content> {
    (0..size).map(|index| Content::text(index.to_string())).collect()
}

pub fn register(c: &mut Criterion) {
    let mut group = c.benchmark_group("content/sequence");

    for size in [0_usize, 1, 32, 256] {
        if size > 0 {
            group.throughput(Throughput::Elements(size as u64));
        }
        group.bench_function(format!("normalize/{size}"), |b| {
            b.iter_batched(
                || parts(size),
                |input| black_box(Content::sequence(input)),
                BatchSize::SmallInput,
            );
        });
    }

    group.finish();
}
