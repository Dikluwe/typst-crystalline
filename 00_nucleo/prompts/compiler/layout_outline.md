# L0 — outline
Hash do Código: a9be52e8

**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/layout/outline.rs`
**Architecture:** ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129, ADR-0130.

## Evidence before specification

The current consumer implements the operations specified below at `01_core/src/compiler/layout/outline.rs:23`, `01_core/src/compiler/layout/outline.rs:78`, `01_core/src/compiler/layout/outline.rs:192`.

Source inspection establishes current behavior and mechanical ownership; it is not a new bilateral parity measurement or evidence of upstream intent. Observable text, morphology, geometry and errors remain language-level obligations; buffer layout and traversal structure are implementation mechanics. Historical probes and their snapshot-specific qualifications are recorded in `00_nucleo/diagnosticos/sanitization-layout-history.md`. Older vanilla pins must not be presented as proof against the current pinned reference `9f2b6e8715237cb086899a42873660fe744622e8`.

## Ownership and contract

This prompt owns only the named consumer. State storage and general coordination remain in `00_nucleo/prompts/compiler/layout.md`; the contract below defines this unit's operations, not ownership of that hub.

For Figures and Tables targets retain the existing LoF/LoT paths, optional title or English default, numbered caption text and page suffix from the corresponding known-page vector by document-order index. For Headings, dispatch an available realized body rather than rebuilding entries; close a material exterior line before it and the realized terminal line after it, rearming the block boundary. The current consumer also retains a legacy fallback when no realized body exists: localized title, depth/active-indent filtering, stored heading number without supplement, cloned rich body and known label page, under the existing read-only flag. Do not remove this fallback under the guise of documentation. The private destination-link helper dispatches its body, requests fractional/repeat expansion through `00_nucleo/prompts/compiler/layout/line_commit.md` before draining, computes bounds through `00_nucleo/prompts/compiler/layout/link.md`, and preserves all copies in the same internal destination link. It does not implement repeat allocation itself. The realized-outline fail-closed proposal and the surviving fallback are a contract ambiguity requiring separate resolution.

## Acceptance and limits

Preserve the unit's current rendered content, morphology, causal positions/pages, styles and diagnostics, including the explicitly documented limits. Follow the existing static descendant interface without altering public signatures, defaults, compatibility or pipeline phase. This documentation does not authorize implementation of unresolved proposals.
