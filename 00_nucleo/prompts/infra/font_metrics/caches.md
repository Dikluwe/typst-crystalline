# L0 — infra/font_metrics/caches
Hash do Código: e559f648

**Layer:** L3
**Exclusive consumer:** `03_infra/src/font_metrics/caches.rs`

## Current evidence

- `03_infra/src/font_metrics/caches.rs:20` — `CachedFace`.
- `03_infra/src/font_metrics/caches.rs:29` — `new`.
- `03_infra/src/font_metrics/caches.rs:41` — `face`.
- `03_infra/src/font_metrics/caches.rs:58` — `ShapedWidthKey`.
- `03_infra/src/font_metrics/caches.rs:75` — `AdvanceWidthKey`.

These are local source anchors, not a new upstream parity measurement.
Reproduce discovery with `rg -n "fn |struct |enum |mod |pub use" 03_infra/src/font_metrics/caches.rs`.

## Owned contract

- Own CachedFace, ShapedWidthKey/AdvanceWidthKey and cache helpers. Font bytes own the face lifetime; preserve the existing encapsulated unsafe and Arc sharing.
- Load faces lazily by slot and share successful parses across clones. Failed reads/parses are not cached. Never replace per-instance state with a global cache.
- Width keys preserve text, effective measurement size bits, family, bold/italic/weight, direction, language, variation hash and explicit math mode (which changes the primary/fallback chain); advance keys additionally distinguish TextItem provenance and math size.
- Advance cache helpers receive the explicit `FontMetrics::advance` size; never substitute `style.size` for that argument. Shaped keys use the effective style supplied by the fallback owner.
- Local regressions compare shared-cache and fresh-instance widths in both request orders, using real embedded fonts with discriminating math fallback and explicit-size inputs; they characterize correctness rather than performance or new upstream parity.
- Preserve tracking bypass in shaped-width caching; None results are not inserted. Read under the original lock scope, compute outside it and insert afterward; do not alter lock order or failure retry behavior.

## Verification and boundary

Run `cargo test -p typst-infra --lib font_metrics:: -- --test-threads=1` when validating behavior.
Ownership is strictly prompt-to-consumer 1:1 (ADR-0129); referenced owners do not share this prompt.
Preserve existing public contracts, defaults and pipeline phases. Referenced child owners govern their own implementations; dependencies do not create shared ownership.


## Local bindings and fixture initialization

### Evidence

`lab/results/construction/diagnostics/ci-helpers-testes-2026-10-14/before.json.gz`
retains source/owner/HEAD/uncommitted tree/UTC/diffstat for caches.rs:129,210.
references-before.json.gz records calls/imports/modules/references; previous suite/
inventory state was hash-checked and test list captured before editing. Private
calls/compilation were audited; dead_code alone proves no functional uselessness.
`field-reassign-2026-10-15/before.json.gz` retains source/spans/prior L0/HEAD/tree/
UTC/diffstat, and initial-diagnostics.json records default-then-assignment test
styles at caches.rs:282,285. This is Rust representation, no new parity or
product-default change.

### Contracts and acceptance

Remove only two unused pure local variant assignments and their exclusive import.
axis_variations_for_text_style already calls text_style_to_font_variant internally
and remains the axis-hash source with unchanged explicit variations/precedence.
Preserve all fields/keys/hash order/sizes/math/tracking bypass/locks/cache policy;
no underscore-local substitution. Continuous ADR-0127 adds no public field/API,
default, phase, dependency or suppression. Selected diagnostic gate RED→GREEN is
not a functional test/parity proof; keep test inventory/expectations, full suite,
lib/all-target compilation/build/Rustdoc/fmt/lineage/full diagnostic inventory;
no cfg/binding-removal mirror tests. Reseal only this owner.

Construct style_a/style_b with weights 400/700 in TextStyle literals. Preserve
axis_hash/key comparisons and repeated same-weight control; do not change caches,
locks or variation derivation. Use ..Type::default() for remaining fields; remove mut only where no mutations
remain. No new types/builders/APIs/lint suppression. L0 first and scoped resealing,
continuous ADR-0127. Selected gate must fail before and pass after; preserve test
names/inputs/expectations/tolerances. Revalidate all-target Clippy/workspace suite/
build/fmt/Rustdoc/lineage. No tests merely mirroring initializer syntax.
