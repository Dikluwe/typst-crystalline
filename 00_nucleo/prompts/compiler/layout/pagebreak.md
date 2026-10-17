# L0 — pagebreak
Hash do Código: 90a92cdf

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/layout/element-form-b.toml sha256:6dbf8faa56960845c60734f5e047685ec5b3a6f14c0ce3a48d333b8982d0baa5
- 00_nucleo/prompts/_nuclei/layout/position-frame-carriers.toml sha256:ff71dfc6648d6deb4b2b8054e3d58a93777d17837129f0f1b1e7e138ffb6943c

**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/layout/pagebreak.rs`
**Architecture:** ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129, ADR-0130.

## Evidence before specification

The current consumer implements the operations specified below at `01_core/src/compiler/layout/pagebreak.rs:16`.

Source inspection establishes current behavior and mechanical ownership; it is not a new bilateral parity measurement or evidence of upstream intent. Observable text, morphology, geometry and errors remain language-level obligations; buffer layout and traversal structure are implementation mechanics. Historical probes and their snapshot-specific qualifications are recorded in `00_nucleo/diagnosticos/sanitization-layout-history.md`. Older vanilla pins must not be presented as proof against the current pinned reference `9f2b6e8715237cb086899a42873660fe744622e8`.

The composed-position candidate failed the existing P1159 explicit-footer
page-reference test (workspace.log archived in page-boundary-candidate.json.gz).
The CLI emits R8 instead of R7, while the heading itself remains on physical
page 1. pagebreak.rs:31-34 only flushes horizontal progress; a label captured
after a closed block has no horizontal advance and its private carrier stays
in current_line across the page transition. The page-reference regression
probe preserves both binaries and source. This is a capture-boundary defect,
not a reason to change logical numbering or the test expectation.

## Ownership and contract

This prompt owns only the named consumer. State storage and general coordination remain in `00_nucleo/prompts/compiler/layout.md`; the contract below defines this unit's operations, not ownership of that hub.

Consume an already-open empty page-run boundary without an additional ordinary break. Otherwise close any horizontally pending line, or a line containing a private paged position carrier, before requesting the existing regional transition. A carrier-only flush must remain neutral for line advance and page dimensions, but must associate those captured positions with the page being closed. Preserve ordinary overflow handling: do not flush pending content at every regional transition. For `to`, request the additional transition only when the next physical page index does not match parity. Preserve the present behavior; the consumer does not inspect the element's weak flag.

## Acceptance and limits

Preserve the unit's current rendered content, morphology, causal positions/pages, styles and diagnostics, including the explicitly documented limits. Follow the existing static descendant interface without altering public signatures, defaults, compatibility or pipeline phase. This documentation does not authorize implementation of unresolved proposals.
