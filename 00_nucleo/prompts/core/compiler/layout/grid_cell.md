# L0 — grid cell
Hash do Código: 3c659b56

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/layout/element-form-b.toml sha256:7510f3bba27256221d4447db608839af562ac71b4eacc0c042f693b531a95184

**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/layout/grid_cell.rs`
**Architecture:** ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129, ADR-0130.

## Evidence before specification

The current consumer implements the operations specified below at `01_core/src/compiler/layout/grid_cell.rs:15`.

Source inspection establishes current behavior and mechanical ownership; it is not a new bilateral parity measurement or evidence of upstream intent. Observable text, morphology, geometry and errors remain language-level obligations; buffer layout and traversal structure are implementation mechanics. Historical probes and their snapshot-specific qualifications are recorded in `lab/results/construction/diagnostics/sanitization-layout-history.md`. The former reference `9f2b6e8715237cb086899a42873660fe744622e8` and older vanilla pins are historical snapshots. They do not certify the current ratified reference `bd5ebec285505889e8ff1d4f68804ab147a699f3` without a new measurement.

## Ownership and contract

This prompt owns only the named consumer. State storage and general coordination remain in `00_nucleo/prompts/core/compiler/layout/mod.md`; the contract below defines this unit's operations, not ownership of that hub.

Dispatch the cell body exactly once. Placement, spans, insets, style resolution and pagination belong to `00_nucleo/prompts/core/compiler/layout/grid.md` and `00_nucleo/prompts/core/compiler/layout/grid_placement.md`, not this adapter.

## Acceptance and limits

Preserve the unit's current rendered content, morphology, causal positions/pages, styles and diagnostics, including the explicitly documented limits. Follow the existing static descendant interface without altering public signatures, defaults, compatibility or pipeline phase. This documentation does not authorize implementation of unresolved proposals.
