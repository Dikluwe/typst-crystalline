# L0 — grid
Hash do Código: 59b58793

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/layout/coordinates.toml sha256:2ccbb1e5daf5f6806e58cd48272b1463fbc9107300c0bce6ea1c3ccf7b0b8748

**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/layout/grid.rs`
**Architecture:** ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129, ADR-0130.

## Evidence before specification

The current consumer implements the operations specified below at `01_core/src/compiler/layout/grid.rs:125`, `01_core/src/compiler/layout/grid.rs:154`, `01_core/src/compiler/layout/grid.rs:536`, `01_core/src/compiler/layout/grid.rs:30`, `01_core/src/compiler/layout/grid.rs:31`, `01_core/src/compiler/layout/grid.rs:32`.

Source inspection establishes current behavior and mechanical ownership; it is not a new bilateral parity measurement or evidence of upstream intent. Observable text, morphology, geometry and errors remain language-level obligations; buffer layout and traversal structure are implementation mechanics. Historical probes and their snapshot-specific qualifications are recorded in `00_nucleo/diagnosticos/sanitization-layout-history.md`. Older vanilla pins must not be presented as proof against the current pinned reference `9f2b6e8715237cb086899a42873660fe744622e8`.

## Ownership and contract

This prompt owns only the named consumer. State storage and general coordination remain in `00_nucleo/prompts/compiler/layout.md`; the contract below defines this unit's operations, not ownership of that hub.

Own Grid dispatch and the shared visual grid/table engine. Delegate cell-row emission and pagination to `00_nucleo/prompts/compiler/layout/grid/cell_emission.md`. Delegate column/row track sizing and its pagination point to `00_nucleo/prompts/compiler/layout/grid/tracks.md`. Delegate header/footer normalization and collision detection to `00_nucleo/prompts/compiler/layout/grid/row_groups.md`, and visual h/v rules to `00_nucleo/prompts/compiler/layout/grid/rules.md`. Delegate row-border coalescing and h/v segment emission to `00_nucleo/prompts/compiler/layout/grid/borders.md`, retaining their invocation order and geometry. Preserve track resolution (fixed/auto/fractional), gutters, per-cell overrides and span measurement, row-group extraction/repetition, h/v rules, fill/stroke and row-border coalescing. Obtain occupied placement from `00_nucleo/prompts/compiler/layout/grid_placement.md`; no second placement algorithm is defined in the hub or cell wrappers.

Measure cell content with injected metrics/styles in the existing finite column widths, apply per-axis inherited alignment and inset, and lay out each cell once in its absolute cell origin/region. Recompose by incremental deltas, keeping parent bbox, cell alignment/context, decorations and orphan fixup paths consistent and restored. Preserve breakable-row splitting, repeated header/footer visual output and deferred cell tails within the source's bounded support; `00_nucleo/prompts/compiler/layout/slicing.md` performs partition/rebase and `00_nucleo/prompts/compiler/layout/cell_tail_commit.md` later emits tails. Table semantic metadata belongs to `00_nucleo/prompts/compiler/layout/table.md`; do not derive it from visual strokes/fills. Unsupported multipage/span/container combinations remain limited rather than silently promoted to parity.

## Acceptance and limits

Preserve the unit's current rendered content, morphology, causal positions/pages, styles and diagnostics, including the explicitly documented limits. Follow the existing static descendant interface without altering public signatures, defaults, compatibility or pipeline phase. This documentation does not authorize implementation of unresolved proposals.
