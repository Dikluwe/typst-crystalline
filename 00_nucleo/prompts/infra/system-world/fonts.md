# L0 — infra/system-world/fonts
Hash do Código: d9c08969

**Layer:** L3
**Exclusive consumer:** `03_infra/src/world/fonts.rs`

## Current evidence

- `03_infra/src/world/fonts.rs:23` — `with_fonts`.
- `03_infra/src/world/fonts.rs:34` — `with_system_fonts`.
- `03_infra/src/world/fonts.rs:46` — `with_embedded_fonts`.
- `03_infra/src/world/fonts.rs:65` — `with_fonts_and_system`.
- `03_infra/src/world/fonts.rs:121` — `preload_coverage_if_needed`.

These are local source anchors, not a new upstream parity measurement.
Reproduce discovery with `rg -n "fn |struct |enum |mod |pub use" 03_infra/src/world/fonts.rs`.

## Owned contract

- Own font builders, lazy exact coverage and conservative preloading. Invalid FontInfo entries are dropped while book/slot indices remain aligned; absent system fonts are nonfatal.
- Preserve builder order: embedded text before embedded math/code; combined discovery uses embedded text, system, embedded math/code, project sources.
- System and combined builders capture discovery policy and owned project paths without discovering fonts. The first book, font or candidate request initializes the complete paired catalog once, safely under concurrent access, and retains it for the world lifetime. Explicit slot and embedded-only builders retain their existing acquisition behavior. Replacing builder configuration replaces its catalog and clears coverage belonging to the previous indices.
- Deferred discovery still obtains exact system coverage through the fontdb owner while temporary face bytes are available. Candidate queries must not map all system FontSlots to reconstruct coverage; preserve the P1410 resource bound.
- Prefer nonempty book coverage without loading font bytes. Only missing coverage triggers lazy cmap parsing/cache by slot; invalid faces yield empty coverage. Preserve exact codepoint filtering, candidate order and locks.
- The existing explicit preload method remains conservative for Text/MathText/RawTrimmed through embedded coverage union and first-uncovered candidate demand; calling it may initialize the catalog. Normal CLI compilation leaves activation to actual font demand. Post-evaluation content follows the same demand path.

## Verification and boundary

Verify this unit through its existing family/CLI tests when validating behavior.
Ownership is strictly prompt-to-consumer 1:1 (ADR-0129); referenced owners do not share this prompt.
Preserve existing public contracts, defaults and pipeline phases. Referenced child owners govern their own implementations; dependencies do not create shared ownership.
