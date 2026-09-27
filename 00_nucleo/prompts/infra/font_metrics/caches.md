# L0 — infra/font_metrics/caches
Hash do Código: 74e4321c

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
