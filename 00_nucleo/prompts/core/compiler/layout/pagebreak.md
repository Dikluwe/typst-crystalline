# L0 — pagebreak
Hash do Código: 89c3acd1

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/layout/element-form-b.toml sha256:7510f3bba27256221d4447db608839af562ac71b4eacc0c042f693b531a95184
- 00_nucleo/prompts/_nuclei/layout/position-frame-carriers.toml sha256:1f0c020521a9654478f7ba3325005ba621fe5f26b307b56dc49e8c526fceb59f

**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/layout/pagebreak.rs`
**Architecture:** ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129, ADR-0130.

## Evidence before specification

The current consumer implements the operations specified below at `01_core/src/compiler/layout/pagebreak.rs:16`.

Source inspection establishes current behavior and mechanical ownership; it is not a new bilateral parity measurement or evidence of upstream intent. Observable text, morphology, geometry and errors remain language-level obligations; buffer layout and traversal structure are implementation mechanics. Historical probes and their snapshot-specific qualifications are recorded in `lab/results/construction/diagnostics/sanitization-layout-history.md`. The former reference `9f2b6e8715237cb086899a42873660fe744622e8` and older vanilla pins are historical snapshots. They do not certify the current ratified reference `bd5ebec285505889e8ff1d4f68804ab147a699f3` without a new measurement.

The composed-position candidate failed the existing explicit-footer page-reference test (historical probe P1159) (workspace.log archived in page-boundary-candidate.json.gz).
The CLI emits R8 instead of R7, while the heading itself remains on physical
page 1. pagebreak.rs:31-34 only flushes horizontal progress; a label captured
after a closed block has no horizontal advance and its private carrier stays
in current_line across the page transition. The page-reference regression
probe preserves both binaries and source. This is a capture-boundary defect,
not a reason to change logical numbering or the test expectation.

## Ownership and contract

This prompt owns only the named consumer. State storage and general coordination remain in `00_nucleo/prompts/core/compiler/layout/mod.md`; the contract below defines this unit's operations, not ownership of that hub.

Consume an already-open empty page-run boundary without an additional ordinary break. Otherwise close any horizontally pending line, or a line containing a private paged position carrier, before requesting the existing regional transition. A carrier-only flush must remain neutral for line advance and page dimensions, but must associate those captured positions with the page being closed. Preserve ordinary overflow handling: do not flush pending content at every regional transition. For `to`, request the additional transition only when the next physical page index does not match parity. Preserve the present behavior; the consumer does not inspect the element's weak flag.

## Acceptance and limits

Preserve the unit's current rendered content, morphology, causal positions/pages, styles and diagnostics, including the explicitly documented limits. Follow the existing static descendant interface without altering public signatures, defaults, compatibility or pipeline phase. This documentation does not authorize implementation of unresolved proposals.
