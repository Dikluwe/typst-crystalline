use criterion::{black_box, BatchSize, Criterion};
use typst_core::entities::content::Content;
use typst_core::entities::style::Styles;

pub fn register(c: &mut Criterion) {
    let mut group = c.benchmark_group("content/construction");

    group.bench_function("text", |b| {
        b.iter_batched(
            || String::from("crystalline"),
            |text| black_box(Content::text(text)),
            BatchSize::SmallInput,
        );
    });
    group.bench_function("styled", |b| {
        b.iter_batched(
            || Content::text("body"),
            |body| black_box(Content::styled(Box::new(body), Styles::new())),
            BatchSize::SmallInput,
        );
    });
    group.bench_function("heading", |b| {
        b.iter_batched(
            || Content::text("heading"),
            |body| black_box(Content::heading(2, body)),
            BatchSize::SmallInput,
        );
    });
    group.bench_function("math_frac", |b| {
        b.iter_batched(
            || (Content::MathIdent("a".into()), Content::MathIdent("b".into())),
            |(num, den)| black_box(Content::math_frac(num, den)),
            BatchSize::SmallInput,
        );
    });
    group.finish();
}
