# L0 — html frame
Hash do Código: f0ebc1b7

**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/layout/html_frame.rs`
**Architecture:** ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129, ADR-0130.

## Evidence before specification

The current consumer implements the operations specified below at `01_core/src/compiler/layout/html_frame.rs:22`, `01_core/src/compiler/layout/html_frame.rs:25`, `01_core/src/compiler/layout/html_frame.rs:30`.

Source inspection establishes current behavior and mechanical ownership; it is not a new bilateral parity measurement or evidence of upstream intent. Observable text, morphology, geometry and errors remain language-level obligations; buffer layout and traversal structure are implementation mechanics. Historical probes and their snapshot-specific qualifications are recorded in `00_nucleo/diagnosticos/sanitization-layout-history.md`. Older vanilla pins must not be presented as proof against the current pinned reference `9f2b6e8715237cb086899a42873660fe744622e8`.

## Ownership and contract

This prompt owns only the named consumer. State storage and general coordination remain in `00_nucleo/prompts/compiler/layout.md`; the contract below defines this unit's operations, not ownership of that hub.

In ordinary paginated dispatch, delegate once to the body without a visual wrapper or default page margin. Intrinsic entrypoints create an isolated Layouter with supplied chain/metrics/sizer, unbounded region and transparent Page cropped to supported visual dimensions, logical_number one, without joining the global paginated sequence. Nesting remains transparent inside the same island.

The detailed variant captures ContextBlock ids using private zero-size carriers in the existing FrameItem tree, not a public variant or HtmlElem sentinel. Transport carriers through box/place/grid/link/group/transform geometry; after all composition, extract deterministic id/HtmlPosition pairs in the island's causal reference frame before private vertical visual cropping. Then normalize only the visual Page, remove all carriers, and derive final visual dimensions without them. Missing id/Location correspondence does not fabricate a position.

Own the shared marker identity/material predicates: only a direct private marker is non-material. Existing Semantic, Link and Group wrappers retain ordinary legacy material classification even if they contain only a carrier; do not flatten them. Extent helpers ignore a direct marker but transformations still reach its point. No SVG encoder, World, PageStore or logical PDF-label behavior is introduced. The hub owns public reexports, this unit owns intrinsic implementation.

## Acceptance and limits

Preserve the unit's current rendered content, morphology, causal positions/pages, styles and diagnostics, including the explicitly documented limits. Follow the existing static descendant interface without altering public signatures, defaults, compatibility or pipeline phase. This documentation does not authorize implementation of unresolved proposals.
