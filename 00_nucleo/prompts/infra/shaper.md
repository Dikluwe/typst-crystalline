# L0 — infra/shaper
Hash do Código: ff4d04c9

**Layer:** L3
**Exclusive consumer:** `03_infra/src/shaper.rs`

## Current evidence

- `03_infra/src/shaper.rs:1` — lineage and facade module declarations/reexports.

These are local source anchors, not a new upstream parity measurement.
Reproduce discovery with `rg -n "fn |struct |enum |mod |pub use" 03_infra/src/shaper.rs`.

## Owned contract

- Own only private module declarations and facade reexports. Preserve public shape_document/fix_line_positions and the existing crate-private FaceCache/CachedFace/shaped_width import paths.
- The paged pipeline owner places the pass: `00_nucleo/prompts/infra/pipeline/paged.md`. Text traversal and fallback are governed by `00_nucleo/prompts/infra/shaper/traversal.md`; glyph materialization is governed by `00_nucleo/prompts/infra/shaper/shaping.md`. This facade does not repeat those contracts.
- Traversal, bidi, font selection, shaping, caching and reconciliation are owned independently below. Keep static delegation and existing visibility; do not add a registry, dynamic dispatcher, public contract or L1 font-byte access.

## Child owners

- `00_nucleo/prompts/infra/shaper/bidi.md` owns only `03_infra/src/shaper/bidi.rs`.
- `00_nucleo/prompts/infra/shaper/cache.md` owns only `03_infra/src/shaper/cache.rs`.
- `00_nucleo/prompts/infra/shaper/font_selection.md` owns only `03_infra/src/shaper/font_selection.rs`.
- `00_nucleo/prompts/infra/shaper/reconciliation.md` owns only `03_infra/src/shaper/reconciliation.rs`.
- `00_nucleo/prompts/infra/shaper/shaping.md` owns only `03_infra/src/shaper/shaping.rs`.
- `00_nucleo/prompts/infra/shaper/tests.md` owns only `03_infra/src/shaper/tests.rs`.
- `00_nucleo/prompts/infra/shaper/traversal.md` owns only `03_infra/src/shaper/traversal.rs`.

## Verification and boundary

Verify this unit through its existing family/CLI tests when validating behavior.
Ownership is strictly prompt-to-consumer 1:1 (ADR-0129); referenced owners do not share this prompt.
Preserve existing public contracts, defaults and pipeline phases. Referenced child owners govern their own implementations; dependencies do not create shared ownership.
