# L0 — bibliography
Hash do Código: 98bdbe17

**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/layout/bibliography.rs`
**Architecture:** ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129, ADR-0130.

## Evidence before specification

The current consumer implements the operations specified below at `01_core/src/compiler/layout/bibliography.rs:25`.

Source inspection establishes current behavior and mechanical ownership; it is not a new bilateral parity measurement or evidence of upstream intent. Observable text, morphology, geometry and errors remain language-level obligations; buffer layout and traversal structure are implementation mechanics. Historical probes and their snapshot-specific qualifications are recorded in `00_nucleo/diagnosticos/sanitization-layout-history.md`. Older vanilla pins must not be presented as proof against the current pinned reference `9f2b6e8715237cb086899a42873660fe744622e8`.

## Ownership and contract

This prompt owns only the named consumer. State storage and general coordination remain in `00_nucleo/prompts/compiler/layout.md`; the contract below defines this unit's operations, not ownership of that hub.

Render an optional title and close its line. When the precomputed CSL cache supplies bibliography Content, dispatch it directly. Otherwise stably order entries by first citation, leave uncited entries in original order, add sequential `[N]` prefixes and the existing back-reference arrows. Own the `format_bib_entry_body` compatibility helper and its conditional field composition. The hub delegates fallback formatting here; preserve the existing field order and organization-as-publisher rule. This fallback is a local compatibility renderer, not evidence of complete vanilla CSL parity. Citation rendering belongs only to `00_nucleo/prompts/compiler/layout/cite.md`; cache construction belongs to `00_nucleo/prompts/compiler/layout/bib_csl.md`.

## Acceptance and limits

Preserve the unit's current rendered content, morphology, causal positions/pages, styles and diagnostics, including the explicitly documented limits. Follow the existing static descendant interface without altering public signatures, defaults, compatibility or pipeline phase. This documentation does not authorize implementation of unresolved proposals.
