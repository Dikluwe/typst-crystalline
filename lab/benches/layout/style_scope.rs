use criterion::{black_box, BenchmarkId, Criterion};
use typst_core::compiler::layout::layout;
use typst_core::entities::content::Content;

use super::fixtures;

pub fn register(c: &mut Criterion) {
    let cases: [(&str, Content); 7] = [
        ("plain", fixtures::short_text()),
        ("strong", Content::strong(fixtures::short_text())),
        ("emph", Content::emph(fixtures::short_text())),
        ("smallcaps", Content::smallcaps(fixtures::short_text())),
        ("styled_once", fixtures::styled_once()),
        ("styled_nested_8", fixtures::nested_styled(8)),
        ("styled_nested_32", fixtures::nested_styled(32)),
    ];

    for (name, content) in &cases {
        let expected = if *name == "smallcaps" {
            "CRYSTALLINE LAYOUT"
        } else {
            "crystalline layout"
        };
        assert_eq!(
            layout(content).plain_text(),
            expected,
            "style fixture {name} must preserve its expected morphology"
        );
    }

    let mut group = c.benchmark_group("layout/style_scope");
    for (name, content) in &cases {
        group.bench_with_input(
            BenchmarkId::from_parameter(name),
            content,
            |b, content| b.iter(|| black_box(layout(black_box(content)))),
        );
    }
    group.finish();
}
