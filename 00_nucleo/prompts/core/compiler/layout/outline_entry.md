# L0 — outline entry
Hash do Código: 3a3d5f7f

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/layout/coordinates.toml sha256:3921c80230cfcc66a807429fbf8bc07ee090bae4988c351b6524023038bab84b
- 00_nucleo/prompts/_nuclei/layout/physical-page-occurrences.toml sha256:b33f2e3c6238da01aa48783aecee6a8b6153b4f36ddcdc23edfe78434bae5b42

**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/layout/outline_entry.rs`
**Architecture:** ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129, ADR-0130.

## Evidence before specification

The current consumer implements the operations specified below at `01_core/src/compiler/layout/outline_entry.rs:10`, `01_core/src/compiler/layout/outline_entry.rs:24`.

Source inspection establishes current behavior and mechanical ownership; it is not a new bilateral parity measurement or evidence of upstream intent. Observable text, morphology, geometry and errors remain language-level obligations; buffer layout and traversal structure are implementation mechanics. Historical probes and their snapshot-specific qualifications are recorded in `lab/results/construction/diagnostics/sanitization-layout-history.md`. The former reference `9f2b6e8715237cb086899a42873660fe744622e8` and older vanilla pins are historical snapshots. They do not certify the current ratified reference `bd5ebec285505889e8ff1d4f68804ab147a699f3` without a new measurement.

## Ownership and contract

This prompt owns only the named consumer. State storage and general coordination remain in `00_nucleo/prompts/core/compiler/layout/mod.md`; the contract below defines this unit's operations, not ownership of that hub.

Dispatch `elem.body()` only when it returns `Ok`; ignore `Err` as the current consumer does. Do not synthesize an outline, prefix, filler or destination link. Outline construction belongs to `00_nucleo/prompts/core/compiler/layout/outline.md`.

## Acceptance and limits

Preserve the unit's current rendered content, morphology, causal positions/pages, styles and diagnostics, including the explicitly documented limits. Follow the existing static descendant interface without altering public signatures, defaults, compatibility or pipeline phase. This documentation does not authorize implementation of unresolved proposals.


## Planned source-child transport

The approved physical-page protocol is recorded in `lab/results/set-page-header-2026-10-21/review.md`; baseline source/owner evidence and actual semantic RED are preserved in `lab/results/set-page-header-implementation-2026-10-21/`. This consumer already dispatches the source child named below; ordinary layout_content intentionally isolates generated content and therefore cannot transport its source packet.

In a planned pass, descend by OutlineEntryBody when the existing body() succeeds. Preserve one actual source occurrence, its source authority and inherited deferred anchor. Known generated render output remains unplanned and cannot acquire source membership. Measurements and repeated visual copies do not execute or admit the source again. Preserve all existing geometry, wrappers, diagnostics, cache behavior, zero-render cases and unplanned entrypoints. No new public API/default or phase is authorized here.
