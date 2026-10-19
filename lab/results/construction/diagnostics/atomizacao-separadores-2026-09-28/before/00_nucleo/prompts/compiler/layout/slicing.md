# L0 — slicing
Hash do Código: ddbb10f7

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/layout/coordinates.toml sha256:2ccbb1e5daf5f6806e58cd48272b1463fbc9107300c0bce6ea1c3ccf7b0b8748

**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/layout/slicing.rs`
**Architecture:** ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129, ADR-0130.

## Evidence before specification

The current consumer implements the operations specified below at `01_core/src/compiler/layout/slicing.rs:39`, `01_core/src/compiler/layout/slicing.rs:61`, `01_core/src/compiler/layout/slicing.rs:84`.

Source inspection establishes current behavior and mechanical ownership; it is not a new bilateral parity measurement or evidence of upstream intent. Observable text, morphology, geometry and errors remain language-level obligations; buffer layout and traversal structure are implementation mechanics. Historical probes and their snapshot-specific qualifications are recorded in `00_nucleo/diagnosticos/sanitization-layout-history.md`. Older vanilla pins must not be presented as proof against the current pinned reference `9f2b6e8715237cb086899a42873660fe744622e8`.

## Ownership and contract

This prompt owns only the named consumer. State storage and general coordination remain in `00_nucleo/prompts/compiler/layout.md`; the contract below defines this unit's operations, not ownership of that hub.

Partition items by their existing projected starting y against a height threshold, preserve order in both outputs, and rebase the tail by minus that threshold. Preserve the current projection conventions: Link projects to zero and Semantic to the maximum child start; this is not arbitrary geometric clipping of a wrapper crossing the threshold. Translate tail coordinates recursively through Semantic/Link, and shift only the origin of a Group whose children are local. Deferred fixup transport is coordinated by callers, not a second registry here. The current Image rebase reconstructs clip_rect as None; preserving clipped-image masks across slicing remains an unresolved discrepancy rather than an authorized behavior change.

## Acceptance and limits

Preserve the unit's current rendered content, morphology, causal positions/pages, styles and diagnostics, including the explicitly documented limits. Follow the existing static descendant interface without altering public signatures, defaults, compatibility or pipeline phase. This documentation does not authorize implementation of unresolved proposals.
