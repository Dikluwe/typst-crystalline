# L0 — infra/font_metrics/basic
Hash do Código: fffeb8c0

**Layer:** L3
**Exclusive consumer:** `03_infra/src/font_metrics/basic.rs`

## Current evidence

- `03_infra/src/font_metrics/basic.rs:24` — `typo_metrics`.
- `03_infra/src/font_metrics/basic.rs:53` — `edge_offset_pt`.
- `03_infra/src/font_metrics/basic.rs:110` — `advance`.
- `03_infra/src/font_metrics/basic.rs:164` — `vertical_metrics`.
- `03_infra/src/font_metrics/basic.rs:178` — `cap_height`.

These are local source anchors, not a new upstream parity measurement.
Reproduce discovery with `rg -n "fn |struct |enum |mod |pub use" 03_infra/src/font_metrics/basic.rs`.

## Owned contract

- Own typo_metrics, edge_offset_pt and FontMetrics for FontBookMetrics. Convert design units proportionally by size/upem, preserving missing-glyph/default fallback and zero-unit guards.
- Use OS/2 metrics before hhea and respect cap-height/baseline edge policy. Explicit Length edges resolve at the supplied size; unknown metrics preserve current defaults.
- Text ink unions use max(0,ymax) and max(0,-ymin). Glyph-by-id ink retains signed ymax/-ymin, so a glyph entirely above baseline can have negative bottom extent. Bbox methods use the requested codepoint or glyph id in this face, without remapping.
- Apply math italic correction only to the singular base outside TextItem provenance; variation selectors are not extra bases. Delegate MATH/ssty extraction to the math child and glyph_to_char to the facade's precomputed map.

## Verification and boundary

Run `cargo test -p typst-infra --lib font_metrics:: -- --test-threads=1` when validating behavior.
Ownership is strictly prompt-to-consumer 1:1 (ADR-0129); referenced owners do not share this prompt.
Preserve existing public contracts, defaults and pipeline phases. Referenced child owners govern their own implementations; dependencies do not create shared ownership.


## Explicit fixture initialization

### Evidence

`lab/results/construction/diagnostics/field-reassign-2026-10-15/before.json.gz`
retains source/spans/prior L0/HEAD/working tree/UTC/diffstat; initial-diagnostics.json
records defaults followed by test-local field assignments at basic.rs:505,513,520.
This is Rust representation maintenance, no new parity measurement or product
default change.

### Contract and acceptance

Construct the three local TextStyle values with unchanged top_edge/bottom_edge:
pt/em lengths and baseline metric, inheriting remaining defaults. Preserve supplied
size, unit conversion, text_edges calls and existing tolerances.

Use ..Type::default() for remaining fields; remove mut only where no mutations
remain. No new types/builders/APIs/lint suppression. L0 first and scoped resealing,
continuous ADR-0127. Selected gate must fail before and pass after; preserve test
names/inputs/expectations/tolerances. Revalidate all-target Clippy/workspace suite/
build/fmt/Rustdoc/lineage. No tests merely mirroring initializer syntax.
