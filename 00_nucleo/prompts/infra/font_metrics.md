# L0 — infra/font_metrics
Hash do Código: d0406708
Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/fonts/primary-resolution-cache.toml sha256:0ac6e8504aee9a06e2ae9ffc717bb4f878a3b71d37d1851fda9b93a93aacd39c


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

## Bounded primary-resolution reuse

### Evidence

`lab/results/construction/diagnostics/content-cost-profile-2026-10-10/{resolution-calls,cpu-style-current,checked-phases}.json.gz`
retains HEAD/tree/UTC, scripts/sources/commands and release SHA
`9ae0a3d9285b8d0b4bae0da1db058483e7f5543003fabff46bb7d93b17e474f2`.
Tables of 1, 600 cells call select_pattern 48, 019 times, resolve_primary 6, 401,
text_edges/vertical_metrics 3, 200/3, 201; selection appears in 25–29% of inclusive
CPU samples. resolution.rs:27–85 forms the base chain from FontList/effective
variant/face readability, without size/variations; :105–124 adds MATH separately;
font_metrics.rs:49–86 already shares clone-local caches. Inference: last-resolution
reuse may amortize scans without growing retention. Identity/chain divergence
or lock/retention cost eliminating benefit refutes it.

### Contract and acceptance

Add only a private last-complete-primary-resolution carrier, initially empty in
new and shared by Clone. Key/entry types and algorithm belong exclusively to
`00_nucleo/prompts/infra/font_metrics/resolution.md`. Consume the pinned nucleus;
add no public fields/signatures. Continuous ADR-0127 requires repeated-query RED
through clones; family/regex/order/variant controls, independent instances,
transient failures and math/variations against fresh instances; quantitative
allocation/paired-time gate, documents equivalent to the preceding release and
many-identity refuters; core/infra/build/lint. Language acceptance observes
selection/metrics/origin/effects/geometry, never vanilla mechanics or inferred
hash hits. Remove on divergence or robust regression.


## R8 — approved font-backed superscript integration

Evidence before decision: the source-bound `footnote-layout-r8-2026-10-24/before/receipt.json`,
`font-tables.json` and `red-geometry.json` under `lab/results/construction/diagnostics/`
record wrong 0.65 scaling/positive baseline and font-dependent pinned geometry at
bd5ebec285505889e8ff1d4f68804ab147a699f3. `approval.json` binds the human-approved
FontMetrics::text_superscript_metrics addition. OS/2/whole-run GSUB are text metrics;
MATH constants and fixture/font-name coefficients are not substitutes. This private
integration follows ADR-0127 continuous parity flow, superseding only superscript
scaling/feature assumptions below. Keep subscript behavior and unrelated folds.
Expose the existing private single-face superscript helper within L3 for shaping;
no new public API. Preserve resolution, caches and all unrelated behavior.

Require actual RED→GREEN tests, fixed-pin PDF/SVG controls, unchanged geometry tolerance and complete workspace/lineage validation.
