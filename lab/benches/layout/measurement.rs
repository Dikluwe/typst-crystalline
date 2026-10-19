use criterion::{black_box, BenchmarkId, Criterion};
use typst_core::compiler::layout::measure_content_real;
use typst_core::compiler::layout::FixedMetrics;
use typst_core::entities::content::Content;
use typst_core::entities::style_chain::StyleChain;

use super::fixtures;

pub fn register(c: &mut Criterion) {
    let chain = StyleChain::default_chain();
    let cases: [(&str, Content); 5] = [
        ("text_short", fixtures::short_text()),
        ("text_long", fixtures::long_text()),
        ("sequence_64", fixtures::wide_sequence(64)),
        ("nested_strong_16", fixtures::nested_strong(16)),
        ("vertical_stack_64", fixtures::vertical_stack(64)),
    ];

    for (name, content) in &cases {
        let measured = measure_content_real(content, &chain, &FixedMetrics);
        assert!(
            measured.0.is_finite() && measured.1.is_finite(),
            "measurement fixture {name} must produce finite dimensions"
        );
    }

    let mut group = c.benchmark_group("layout/measurement");
    for (name, content) in &cases {
        group.bench_with_input(BenchmarkId::new("real", name), content, |b, content| {
            b.iter(|| {
                black_box(measure_content_real(
                    black_box(content),
                    black_box(&chain),
                    black_box(&FixedMetrics),
                ))
            })
        });
    }
    group.finish();
}
