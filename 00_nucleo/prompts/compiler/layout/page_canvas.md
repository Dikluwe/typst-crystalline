# L0 — page canvas
Hash do Código: 03c714e2

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/layout/page-canvas.toml sha256:442dcd9884551a8bfc99600298aea9d1f6591a5d0ccaf61a811de075c5b435d6

**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/layout/page_canvas.rs`
**Architecture:** ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129, ADR-0130.

## Evidence before specification

The current consumer implements the operations specified below at `01_core/src/compiler/layout/page_canvas.rs:17`.

Source inspection establishes current behavior and mechanical ownership; it is not a new bilateral parity measurement or evidence of upstream intent. Observable text, morphology, geometry and errors remain language-level obligations; buffer layout and traversal structure are implementation mechanics. Historical probes and their snapshot-specific qualifications are recorded in `00_nucleo/diagnosticos/sanitization-layout-history.md`. Older vanilla pins must not be presented as proof against the current pinned reference `9f2b6e8715237cb086899a42873660fe744622e8`.

## Ownership and contract

This prompt owns only the named consumer. State storage and general coordination remain in `00_nucleo/prompts/compiler/layout.md`; the contract below defines this unit's operations, not ownership of that hub.

For optional background/foreground Content, return no layer when absent. Otherwise lay out once in an isolated canvas sized trim plus resolved bleed, and place the resulting group at negative left/top bleed with identity transform and no clip. Preserve finite canvas-relative dimension resolution and rich body content. Do not treat this region as the main-flow content rectangle, rasterize it, execute page functions or alter trim/PageConfig. Typed bleed resolution belongs to the existing page-canvas entity owner.

## Acceptance and limits

Preserve the unit's current rendered content, morphology, causal positions/pages, styles and diagnostics, including the explicitly documented limits. Follow the existing static descendant interface without altering public signatures, defaults, compatibility or pipeline phase. This documentation does not authorize implementation of unresolved proposals.
