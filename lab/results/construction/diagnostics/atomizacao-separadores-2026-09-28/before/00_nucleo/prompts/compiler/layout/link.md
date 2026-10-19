# L0 — link
Hash do Código: dd8206d9

**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/layout/link.rs`
**Architecture:** ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129, ADR-0130.

## Evidence before specification

The current consumer implements the operations specified below at `01_core/src/compiler/layout/link.rs:18`, `01_core/src/compiler/layout/link.rs:51`, `01_core/src/compiler/layout/link.rs:141`.

Source inspection establishes current behavior and mechanical ownership; it is not a new bilateral parity measurement or evidence of upstream intent. Observable text, morphology, geometry and errors remain language-level obligations; buffer layout and traversal structure are implementation mechanics. Historical probes and their snapshot-specific qualifications are recorded in `00_nucleo/diagnosticos/sanitization-layout-history.md`. Older vanilla pins must not be presented as proof against the current pinned reference `9f2b6e8715237cb086899a42873660fe744622e8`.

## Ownership and contract

This prompt owns only the named consumer. State storage and general coordination remain in `00_nucleo/prompts/compiler/layout.md`; the contract below defines this unit's operations, not ownership of that hub.

Render the rich body, collect the newly produced pending-line and emitted-item ranges, and wrap them in the existing URL Link without changing style to mandatory blue/underline. Preserve nested content and current cursor behavior. Own link_bbox: measure text via injected advance/vertical metrics, glyphs via their current advance/size projection, shape/image/line extents and Group/Link bounds; recurse through Semantic. An empty measured set has zero bounds. Ignore direct private HTML context markers for bounds but retain them among children. Internal destination links are assembled by `00_nucleo/prompts/compiler/layout_outline.md`/`00_nucleo/prompts/compiler/layout_references.md` using this helper, not by adding a new LinkElem target here. The current unguarded pending-tail drain has a possible body-flush boundary limitation; multipage/rich-link safety and QuadPoints are not proven by this cleanup.

## Acceptance and limits

Preserve the unit's current rendered content, morphology, causal positions/pages, styles and diagnostics, including the explicitly documented limits. Follow the existing static descendant interface without altering public signatures, defaults, compatibility or pipeline phase. This documentation does not authorize implementation of unresolved proposals.
