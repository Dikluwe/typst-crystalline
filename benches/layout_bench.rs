//! Criterion entry point for behavior-focused layout microbenchmarks.

#[path = "support/profile.rs"]
mod profile;

use criterion::{criterion_group, criterion_main, Criterion};

#[path = "layout/fixtures.rs"]
mod fixtures;
#[path = "layout/measurement.rs"]
mod measurement;
#[path = "layout/style_scope.rs"]
mod style_scope;

fn layout_benches(c: &mut Criterion) {
    profile::require_optimized();
    measurement::register(c);
    style_scope::register(c);
}

criterion_group!(benches, layout_benches);
criterion_main!(benches);
