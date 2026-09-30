# L0 — infra/font_metrics
Hash do Código: d87e162e

**Layer:** L3
**Exclusive consumer:** `03_infra/src/font_metrics.rs`

## Current evidence

- `03_infra/src/font_metrics.rs:27` — `FontBookMetrics`.
- `03_infra/src/font_metrics.rs:40` — `from_bytes`.
- `03_infra/src/font_metrics.rs:49` — `FallbackFontMetrics`.
- `03_infra/src/font_metrics.rs:68` — `new`.
- `03_infra/src/font_metrics.rs:80` — `clone`.

These are local source anchors, not a new upstream parity measurement.
Reproduce discovery with `rg -n "fn |struct |enum |mod |pub use" 03_infra/src/font_metrics.rs`.

## Owned contract

- Own FontBookMetrics/FallbackFontMetrics state, lifetimes, constructors, Clone and existing internal reexports only. Metric algorithms, caches, fallback, MATH and resolution belong to their child owners.
- FontBookMetrics::from_bytes parses face index zero, returns None for invalid bytes, uses 1000 for zero units-per-em and eagerly builds its reverse glyph map through the math child.
- FallbackFontMetrics::new borrows World and creates lazy Arc/Mutex caches, including the shared shaper face cache. Clone shares every cache and the same World; no global state or I/O enters L1.
- Preserve the L1 FontMetrics contract and existing crate-private edge_offset_pt/ssty/reverse-map access. ttf-parser/rustybuzz types stay inside L3; do not add public signatures or broaden private child visibility.

## Child owners

- `00_nucleo/prompts/infra/font_metrics/basic.md` owns only `03_infra/src/font_metrics/basic.rs`.
- `00_nucleo/prompts/infra/font_metrics/caches.md` owns only `03_infra/src/font_metrics/caches.rs`.
- `00_nucleo/prompts/infra/font_metrics/fallback.md` owns only `03_infra/src/font_metrics/fallback.rs`.
- `00_nucleo/prompts/infra/font_metrics/math.md` owns only `03_infra/src/font_metrics/math.rs`.
- `00_nucleo/prompts/infra/font_metrics/resolution.md` owns only `03_infra/src/font_metrics/resolution.rs`.

## Verification and boundary

Run `cargo test -p typst-infra --lib font_metrics:: -- --test-threads=1` when validating behavior.
Ownership is strictly prompt-to-consumer 1:1 (ADR-0129); referenced owners do not share this prompt.
Preserve existing public contracts, defaults and pipeline phases. Referenced child owners govern their own implementations; dependencies do not create shared ownership.
