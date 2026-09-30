# L0 — tiling
Hash do Código: ad7561b7

**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/layout/tiling.rs`
**Architecture:** ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129, ADR-0130.

## Evidence before specification

The current consumer implements the operations specified below at `01_core/src/compiler/layout/tiling.rs:15`, `01_core/src/compiler/layout/tiling.rs:124`, `01_core/src/compiler/layout/tiling.rs:144`.

Source inspection establishes current behavior and mechanical ownership; it is not a new bilateral parity measurement or evidence of upstream intent. Observable text, morphology, geometry and errors remain language-level obligations; buffer layout and traversal structure are implementation mechanics. Historical probes and their snapshot-specific qualifications are recorded in `00_nucleo/diagnosticos/sanitization-layout-history.md`. Older vanilla pins must not be presented as proof against the current pinned reference `9f2b6e8715237cb086899a42873660fe744622e8`.

## Ownership and contract

This prompt owns only the named consumer. State storage and general coordination remain in `00_nucleo/prompts/compiler/layout.md`; the contract below defines this unit's operations, not ownership of that hub.

Materialize only supported Content-backed tiling fills into existing Groups. Reject invalid/nonpositive/nonfinite target, explicit cell dimensions, pitches, offsets or angle without a loop; None means unsupported, not a successful transparent paint. Lay out the body once in an isolated region, derive auto cell dimensions from its actual width/height, and compute pitch as cell size plus resolved spacing. Relative offset uses pitch; apply offset before clockwise angle through the existing matrix order. Shapes' Auto/Itself phase is local; explicit Parent cancels the shape-origin translation before pattern placement. Preserve cell clipping and the enclosing shape clip, clone the already-laid-out cell in the bounded lattice (current maximum 10000 cells), and keep default/unsupported text-parent cases outside this owner. Do not execute body per copy, rasterize or change public Paint types. Shape owns invocation and stroke behavior.

## Acceptance and limits

Preserve the unit's current rendered content, morphology, causal positions/pages, styles and diagnostics, including the explicitly documented limits. Follow the existing static descendant interface without altering public signatures, defaults, compatibility or pipeline phase. This documentation does not authorize implementation of unresolved proposals.
