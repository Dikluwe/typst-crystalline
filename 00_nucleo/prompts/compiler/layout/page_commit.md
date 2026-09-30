# L0 — page commit
Hash do Código: 7ba0eddd

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/layout/coordinates.toml sha256:2ccbb1e5daf5f6806e58cd48272b1463fbc9107300c0bce6ea1c3ccf7b0b8748

**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/layout/page_commit.rs`
**Architecture:** ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129, ADR-0130.

## Evidence before specification

The current consumer implements the operations specified below at `01_core/src/compiler/layout/page_commit.rs:18`, `01_core/src/compiler/layout/page_commit.rs:174`.

Source inspection establishes current behavior and mechanical ownership; it is not a new bilateral parity measurement or evidence of upstream intent. Observable text, morphology, geometry and errors remain language-level obligations; buffer layout and traversal structure are implementation mechanics. Historical probes and their snapshot-specific qualifications are recorded in `00_nucleo/diagnosticos/sanitization-layout-history.md`. Older vanilla pins must not be presented as proof against the current pinned reference `9f2b6e8715237cb086899a42873660fe744622e8`.

## Ownership and contract

This prompt owns only the named consumer. State storage and general coordination remain in `00_nucleo/prompts/compiler/layout.md`; the contract below defines this unit's operations, not ownership of that hub.

Own intermediate Page assembly only, receiving dimensions already chosen by its caller. Drain emitted items; apply par.line, equation, horizontal-align and vertical-align fixups in their current order. Obtain explicit marginal layers from `00_nucleo/prompts/compiler/layout/page_running.md`, and background/foreground canvas layers from `00_nucleo/prompts/compiler/layout/page_canvas.md`; append marginal items to foreground. Preserve fill, resolved supplement and binding/parity-dependent bleed. Initialize logical_number to the physical page index; the later pipeline logical overlay is not this owner's work.

When the selected marginal is Auto, format a unary pattern using the known logical current number if available, otherwise the physical index. Defer compound patterns with the current marginal geometry to the hub's terminal total-number coordinator. Function numbering uses the already-realized visible PageStore Content, never Func execution. Append the completed Page without draining floats/footnotes, advancing a region or resetting the next cursor. Final-page composition remains in `00_nucleo/prompts/compiler/layout.md`; its terminal lifetime is not a second consumer of this prompt.

## Acceptance and limits

Preserve the unit's current rendered content, morphology, causal positions/pages, styles and diagnostics, including the explicitly documented limits. Follow the existing static descendant interface without altering public signatures, defaults, compatibility or pipeline phase. This documentation does not authorize implementation of unresolved proposals.
