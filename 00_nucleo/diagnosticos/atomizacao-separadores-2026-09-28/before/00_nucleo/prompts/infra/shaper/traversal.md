# L0 — infra/shaper/traversal
Hash do Código: 220c26b6

**Layer:** L3
**Exclusive consumer:** `03_infra/src/shaper/traversal.rs`

## Current evidence

- `03_infra/src/shaper/traversal.rs:17` — `shape_document`.
- `03_infra/src/shaper/traversal.rs:34` — `shape_page`.
- `03_infra/src/shaper/traversal.rs:40` — `shape_layer`.
- `03_infra/src/shaper/traversal.rs:59` — `shape_item`.

These are local source anchors, not a new upstream parity measurement.
Reproduce discovery with `rg -n "fn |struct |enum |mod |pub use" 03_infra/src/shaper/traversal.rs`.

## Owned contract

- Own shape_document/page/item traversal. Create run and face caches per document and visit background, items and foreground in that order.
- Descend into Group, Link and Semantic without changing wrapper metadata, positions or child order. Preserve the closed static dispatch and nontext items.
- Delegate Text with a requested font to shaping::try_shape; preserve original text on absent font, failed resolution or whitespace. Do not reshape existing TextShaped or implement selection in this walker.

## Verification and boundary

Verify this unit through its existing family/CLI tests when validating behavior.
Ownership is strictly prompt-to-consumer 1:1 (ADR-0129); referenced owners do not share this prompt.
Preserve existing public contracts, defaults and pipeline phases. Referenced child owners govern their own implementations; dependencies do not create shared ownership.
