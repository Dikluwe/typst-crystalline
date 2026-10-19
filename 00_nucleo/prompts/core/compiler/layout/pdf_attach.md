# L0 — pdf attach
Hash do Código: 1803245e

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/layout/element-form-b.toml sha256:6dbf8faa56960845c60734f5e047685ec5b3a6f14c0ce3a48d333b8982d0baa5

**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/layout/pdf_attach.rs`
**Architecture:** ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129, ADR-0130.

## Evidence before specification

The current consumer implements the operations specified below at `01_core/src/compiler/layout/pdf_attach.rs:13`.

Source inspection establishes current behavior and mechanical ownership; it is not a new bilateral parity measurement or evidence of upstream intent. Observable text, morphology, geometry and errors remain language-level obligations; buffer layout and traversal structure are implementation mechanics. Historical probes and their snapshot-specific qualifications are recorded in `lab/results/construction/diagnostics/sanitization-layout-history.md`. Older vanilla pins must not be presented as proof against the current pinned reference `9f2b6e8715237cb086899a42873660fe744622e8`.

## Ownership and contract

This prompt owns only the named consumer. State storage and general coordination remain in `00_nucleo/prompts/core/compiler/layout/mod.md`; the contract below defines this unit's operations, not ownership of that hub.

Register one occurrence in document order in the Layouter attachment side-channel, with no visible item, cursor/extent/baseline advance or locator allocation. The current function receives `&PdfAttachElem` and stores an Arc around a cloned element; do not claim an Arc-argument signature or an O(1) handle clone that the source does not provide. The hub transfers the committed vector into PagedDocument. Duplicate-path validation, Filespec generation and bytes loading belong to their external owners. Speculative replay uses the hub's transaction boundary rather than a local transaction.

## Acceptance and limits

Preserve the unit's current rendered content, morphology, causal positions/pages, styles and diagnostics, including the explicitly documented limits. Follow the existing static descendant interface without altering public signatures, defaults, compatibility or pipeline phase. This documentation does not authorize implementation of unresolved proposals.
