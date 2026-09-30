# L0 — figure
Hash do Código: ed7dd7a6

**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/layout/figure.rs`
**Architecture:** ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129, ADR-0130.

## Evidence before specification

The current consumer implements the operations specified below at `01_core/src/compiler/layout/figure.rs:19`, `01_core/src/compiler/layout/figure.rs:70`.

Source inspection establishes current behavior and mechanical ownership; it is not a new bilateral parity measurement or evidence of upstream intent. Observable text, morphology, geometry and errors remain language-level obligations; buffer layout and traversal structure are implementation mechanics. Historical probes and their snapshot-specific qualifications are recorded in `00_nucleo/diagnosticos/sanitization-layout-history.md`. Older vanilla pins must not be presented as proof against the current pinned reference `9f2b6e8715237cb086899a42873660fe744622e8`.

## Ownership and contract

This prompt owns only the named consumer. State storage and general coordination remain in `00_nucleo/prompts/compiler/layout.md`; the contract below defines this unit's operations, not ownership of that hub.

Read `figure.numbering` from the active chain: absence selects `1`, String supplies the pattern, None or another value disables numbering. For an enabled kind, consume the per-kind figure index and obtain the injected introspector number, with the existing index-plus-one fallback. Register the current page for LoF in the enabled-numbering branch, even if no caption is present. Format the number or decimal fallback and use the language owner's kind-specific supplement. Dispatch the body first; if a caption exists, emit a linebreak and dispatch the optional `supplement number: ` prefix plus rich caption. Do not own supplement tables, counter construction or outline fixpoint iteration.

## Acceptance and limits

Preserve the unit's current rendered content, morphology, causal positions/pages, styles and diagnostics, including the explicitly documented limits. Follow the existing static descendant interface without altering public signatures, defaults, compatibility or pipeline phase. This documentation does not authorize implementation of unresolved proposals.
