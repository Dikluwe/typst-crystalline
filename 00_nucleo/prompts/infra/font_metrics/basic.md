# L0 — infra/font_metrics/basic
Hash do Código: e974652a

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
