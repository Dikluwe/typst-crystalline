//! Criterion entry point for behavior-focused `Content` microbenchmarks.

#[path = "support/profile.rs"]
mod profile;

use criterion::{criterion_group, criterion_main, Criterion};

#[path = "content/construction.rs"]
mod construction;
#[path = "content/coverage.rs"]
mod coverage;
#[path = "content/fixtures.rs"]
mod fixtures;
#[path = "content/identity.rs"]
mod identity;
#[path = "content/morphology.rs"]
mod morphology;
#[path = "content/page_structure.rs"]
mod page_structure;
#[path = "content/projections.rs"]
mod projections;
#[path = "content/representation.rs"]
mod representation;
#[path = "content/sequence.rs"]
mod sequence;
#[path = "content/traversal.rs"]
mod traversal;

fn content_benches(c: &mut Criterion) {
    profile::require_optimized();
    coverage::assert_catalog_families();
    construction::register(c);
    representation::register(c);
    sequence::register(c);
    identity::register(c);
    traversal::register(c);
    morphology::register(c);
    projections::register(c);
    page_structure::register(c);
}

criterion_group!(benches, content_benches);
criterion_main!(benches);
