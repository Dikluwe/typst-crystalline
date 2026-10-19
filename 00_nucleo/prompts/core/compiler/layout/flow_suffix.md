# L0 — flow suffix
Hash do Código: 9fddfb20

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/layout/coordinates.toml sha256:3921c80230cfcc66a807429fbf8bc07ee090bae4988c351b6524023038bab84b

**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/layout/flow_suffix.rs`
**Architecture:** ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129, ADR-0130.

## Evidence before specification

The current consumer implements the operations specified below at `01_core/src/compiler/layout/flow_suffix.rs:15`, `01_core/src/compiler/layout/flow_suffix.rs:36`, `01_core/src/compiler/layout/flow_suffix.rs:97`.

Source inspection establishes current behavior and mechanical ownership; it is not a new bilateral parity measurement or evidence of upstream intent. Observable text, morphology, geometry and errors remain language-level obligations; buffer layout and traversal structure are implementation mechanics. Historical probes and their snapshot-specific qualifications are recorded in `lab/results/construction/diagnostics/sanitization-layout-history.md`. The former reference `9f2b6e8715237cb086899a42873660fe744622e8` and older vanilla pins are historical snapshots. They do not certify the current ratified reference `bd5ebec285505889e8ff1d4f68804ab147a699f3` without a new measurement.

## Ownership and contract

This prompt owns only the named consumer. State storage and general coordination remain in `00_nucleo/prompts/core/compiler/layout/mod.md`; the contract below defines this unit's operations, not ownership of that hub.

Own acceptance/rejection and replay of the speculative flow suffix. Accumulate top-level occurrences in order while nested dispatch participates in the same unit. Ask Place's existing fitting helper whether newly queued floats fit; preserve the current acceptance conditions and boundary clearing rather than inventing a general fixpoint.

An accepted suffix clears its transaction/checkpoint/replay state before ordinary transition. A rejected suffix takes the hub checkpoint and replay list, restores the full captured tail through `00_nucleo/prompts/core/compiler/layout/mod.md`, requests an ordinary Cursor transition, and re-dispatches the same occurrences through the non-reopening entry once. No prefix float, post-marker item, counter, location or attachment may be duplicated by replay. Do not peek at future Content, copy state into this module, translate only current_line or special-case Block/Place geometry. Region-progress policy remains in `00_nucleo/prompts/core/compiler/layout/cursor.md`.

## Acceptance and limits

Preserve the unit's current rendered content, morphology, causal positions/pages, styles and diagnostics, including the explicitly documented limits. Follow the existing static descendant interface without altering public signatures, defaults, compatibility or pipeline phase. This documentation does not authorize implementation of unresolved proposals.
