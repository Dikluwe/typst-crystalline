# L0 — pdf artifact
Hash do Código: 0117f9d4

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/layout/element-form-b.toml sha256:6dbf8faa56960845c60734f5e047685ec5b3a6f14c0ce3a48d333b8982d0baa5

**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/layout/pdf_artifact.rs`
**Architecture:** ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129, ADR-0130.

## Evidence before specification

The current consumer implements the operations specified below at `01_core/src/compiler/layout/pdf_artifact.rs:14`, `01_core/src/compiler/layout/pdf_artifact.rs:23`, `01_core/src/compiler/layout/pdf_artifact.rs:35`.

Source inspection establishes current behavior and mechanical ownership; it is not a new bilateral parity measurement or evidence of upstream intent. Observable text, morphology, geometry and errors remain language-level obligations; buffer layout and traversal structure are implementation mechanics. Historical probes and their snapshot-specific qualifications are recorded in `lab/results/construction/diagnostics/sanitization-layout-history.md`. Older vanilla pins must not be presented as proof against the current pinned reference `9f2b6e8715237cb086899a42873660fe744622e8`.

## Ownership and contract

This prompt owns only the named consumer. State storage and general coordination remain in `00_nucleo/prompts/compiler/layout.md`; the contract below defines this unit's operations, not ownership of that hub.

Dispatch the body once and preserve each committed visual fragment inside a transparent Artifact semantic envelope with the element's kind and `alt: None`. Use the existing private start/end markers to delimit fragments across completed pages, emitted items and the pending line, removing those markers during wrapping. Preserve paint order, geometry, pagination and plain text; do not reuse Formula or force a single isolated frame. Direct private HTML context markers do not themselves trigger a material-line boundary. MCID, structure nodes, property lists and tag-disabled serialization belong to exporter owners, not layout; this contract does not claim AT, reflow or PDF/UA conformance.

## Acceptance and limits

Preserve the unit's current rendered content, morphology, causal positions/pages, styles and diagnostics, including the explicitly documented limits. Follow the existing static descendant interface without altering public signatures, defaults, compatibility or pipeline phase. This documentation does not authorize implementation of unresolved proposals.
