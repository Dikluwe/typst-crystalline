# L0 — shape block behaviour
Hash do Código: a6ffe8e1

**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/layout/shape.rs`
**Architecture:** ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129, ADR-0130.

## Evidence before specification

The current consumer implements the operations specified below at `01_core/src/compiler/layout/shape.rs:25`, `01_core/src/compiler/layout/shape.rs:35`.

Source inspection establishes current behavior and mechanical ownership; it is not a new bilateral parity measurement or evidence of upstream intent. Observable text, morphology, geometry and errors remain language-level obligations; buffer layout and traversal structure are implementation mechanics. Historical probes and their snapshot-specific qualifications are recorded in `00_nucleo/diagnosticos/sanitization-layout-history.md`. Older vanilla pins must not be presented as proof against the current pinned reference `9f2b6e8715237cb086899a42873660fe744622e8`.

## Ownership and contract

This prompt owns only the named consumer. State storage and general coordination remain in `00_nucleo/prompts/compiler/layout.md`; the contract below defines this unit's operations, not ownership of that hub.

Own Shape rendering in `shape.rs`. Resolve the current Rect/RoundedRect/Ellipse/Path dimensions or Line dx/dy, establish the initial typography reference, close pending text and retain the main-flow block-spacing/collapse protocol. Shape position follows the source's distinct first/post-block cap-height versus post-text baseline-plus-above branches; subframes use direct positioning without main-flow above/below. Request ordinary progress only from the candidate shape's actual bottom, so a fitting shape is not rejected by unrelated typographic leading. Resolve radius against effective size, preserve Paint, stroke, fill rule and parent bbox. Delegate supported content-backed tiling fills to `00_nucleo/prompts/compiler/layout/tiling.md`; when materialized, preserve the separate unfilled stroke shape and paint order; otherwise retain the declarative fill. Advance using the current shape extent/below protocol and leave parent state coherent. Do not rasterize, sample all paints into a single solid color, shift Place through a global scan or move this render logic into ShapeElem.

## Acceptance and limits

Preserve the unit's current rendered content, morphology, causal positions/pages, styles and diagnostics, including the explicitly documented limits. Follow the existing static descendant interface without altering public signatures, defaults, compatibility or pipeline phase. This documentation does not authorize implementation of unresolved proposals.
