# L0 — cell tail commit
Hash do Código: 7ea0449b

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/layout/coordinates.toml sha256:2ccbb1e5daf5f6806e58cd48272b1463fbc9107300c0bce6ea1c3ccf7b0b8748

**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/layout/cell_tail_commit.rs`
**Architecture:** ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129, ADR-0130.

## Evidence before specification

The current consumer implements the operations specified below at `01_core/src/compiler/layout/cell_tail_commit.rs:25`, `01_core/src/compiler/layout/cell_tail_commit.rs:133`.

Source inspection establishes current behavior and mechanical ownership; it is not a new bilateral parity measurement or evidence of upstream intent. Observable text, morphology, geometry and errors remain language-level obligations; buffer layout and traversal structure are implementation mechanics. Historical probes and their snapshot-specific qualifications are recorded in `00_nucleo/diagnosticos/sanitization-layout-history.md`. Older vanilla pins must not be presented as proof against the current pinned reference `9f2b6e8715237cb086899a42873660fe744622e8`.

## Ownership and contract

This prompt owns only the named consumer. State storage and general coordination remain in `00_nucleo/prompts/compiler/layout.md`; the contract below defines this unit's operations, not ownership of that hub.

Drain deferred cell tails only after Cursor opens the next region. Preserve the current forwarding cutoff: a tail with forwarded_count at least three is skipped. This is a bounded existing limitation, not a guarantee for arbitrary overflow. Determine local extent using the current item-coordinate projection, emit fill, body items, then stroke, and advance cursor_y to the greatest emitted bottom. Rebase orphaned align paths to the final insertion point and apply the same origin delta to their geometry as to items. The buffer belongs to `00_nucleo/prompts/compiler/layout.md`; pure slicing/rebase belongs to `00_nucleo/prompts/compiler/layout/slicing.md`. Do not reinterpret Grid/Table or decide a page transition.

## Acceptance and limits

Preserve the unit's current rendered content, morphology, causal positions/pages, styles and diagnostics, including the explicitly documented limits. Follow the existing static descendant interface without altering public signatures, defaults, compatibility or pipeline phase. This documentation does not authorize implementation of unresolved proposals.
