# L0 — placement
Hash do Código: 04873fde

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/layout/coordinates.toml sha256:2ccbb1e5daf5f6806e58cd48272b1463fbc9107300c0bce6ea1c3ccf7b0b8748

**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/layout/placement.rs`
**Architecture:** ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129, ADR-0130.

## Evidence before specification

The current consumer implements the operations specified below at `01_core/src/compiler/layout/placement.rs:21`, `01_core/src/compiler/layout/placement.rs:277`, `01_core/src/compiler/layout/placement.rs:539`.

Source inspection establishes current behavior and mechanical ownership; it is not a new bilateral parity measurement or evidence of upstream intent. Observable text, morphology, geometry and errors remain language-level obligations; buffer layout and traversal structure are implementation mechanics. Historical probes and their snapshot-specific qualifications are recorded in `00_nucleo/diagnosticos/sanitization-layout-history.md`. Older vanilla pins must not be presented as proof against the current pinned reference `9f2b6e8715237cb086899a42873660fe744622e8`.

## Ownership and contract

This prompt owns only the named consumer. State storage and general coordination remain in `00_nucleo/prompts/compiler/layout.md`; the contract below defines this unit's operations, not ownership of that hub.

Own layout_align, non-float layout_place, resolve_alignment and deferred horizontal/vertical fixups. `00_nucleo/prompts/compiler/layout/align.md` and `00_nucleo/prompts/compiler/layout/place.md` own their element adapters. Use the existing absolute sublayout origin and add only the incremental alignment delta on recomposition, never the full origin twice. Width comes from already-laid-out items and FontMetrics; vertical alignment resolves cell-region first, then unconstrained sublayout fallback, then remaining page region.

For Place, preserve Page/Parent/Column scope, inherited per-axis cell alignment, resolved dx/dy and out-of-flow emission. The Place adapter supplies the current-flow correction when a pending line requires it; this geometry helper does not apply a global post-layout scan.

When auto dimensions are still infinite, use the current finite-content fallback and record path-based deferred fixups in hub storage. Nested Group/Link/Semantic paths and orphan rebase remain in the same physical/logical conventions. Items, decoration segments and alignment metadata receive their respective matching deltas, with baseline compensation applied only where the item convention requires it. Apply deferred fixups once against final dimensions and clear them. No page policy, cloned checkpoint or entity-to-render import is created.

## Acceptance and limits

Preserve the unit's current rendered content, morphology, causal positions/pages, styles and diagnostics, including the explicitly documented limits. Follow the existing static descendant interface without altering public signatures, defaults, compatibility or pipeline phase. This documentation does not authorize implementation of unresolved proposals.
