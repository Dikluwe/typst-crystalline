# L0 — helpers
Hash do Código: e633079e

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/layout/coordinates.toml sha256:2ccbb1e5daf5f6806e58cd48272b1463fbc9107300c0bce6ea1c3ccf7b0b8748

**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/layout/helpers.rs`
**Architecture:** ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129, ADR-0130.

## Evidence before specification

The current consumer implements the operations specified below at `01_core/src/compiler/layout/helpers.rs:18`, `01_core/src/compiler/layout/helpers.rs:54`, `01_core/src/compiler/layout/helpers.rs:94`.

Source inspection establishes current behavior and mechanical ownership; it is not a new bilateral parity measurement or evidence of upstream intent. Observable text, morphology, geometry and errors remain language-level obligations; buffer layout and traversal structure are implementation mechanics. Historical probes and their snapshot-specific qualifications are recorded in `00_nucleo/diagnosticos/sanitization-layout-history.md`. Older vanilla pins must not be presented as proof against the current pinned reference `9f2b6e8715237cb086899a42873660fe744622e8`.

## Ownership and contract

This prompt owns only the named consumer. State storage and general coordination remain in `00_nucleo/prompts/compiler/layout.md`; the contract below defines this unit's operations, not ownership of that hub.

Own the existing pure FrameItem position/extent/translation/offset/path-slice helpers, the narrow legacy shape/sequence measurement fallback, heading-scale mapping and dimension projection. For a Semantic wrapper, horizontal position is the minimum non-marker child x and width is rightmost child end minus that same left edge; empty/nonvisual extent is zero. Keep existing vertical conventions and Group/Link behavior, without child reorder or automatic render semantics.

Exclude direct private context markers from visual right/bottom extents but preserve their point under all translation/offset helpers. Descendant path-slice traversal supports Group, Link and Semantic, rejects invalid/overflowing paths, and does not create geometry. Heading scales remain level one 1.4, level two 1.2, other levels 1.0.

Retain the source's existing translation/projection conventions rather than claiming fully general affine bounds or clipping: Semantic translation uses its first-child anchor; item_bottom_y retains its descender approximation; Image reconstruction in translate_frame_item drops clip_rect. Those limitations require a separate behavior decision, not silent cleanup repairs.

## Acceptance and limits

Preserve the unit's current rendered content, morphology, causal positions/pages, styles and diagnostics, including the explicitly documented limits. Follow the existing static descendant interface without altering public signatures, defaults, compatibility or pipeline phase. This documentation does not authorize implementation of unresolved proposals.
