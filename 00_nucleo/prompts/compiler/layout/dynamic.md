# L0 — dynamic
Hash do Código: d495df96

**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/layout/dynamic.rs`
**Architecture:** ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129, ADR-0130.

## Evidence before specification

The current consumer implements the operations specified below at `01_core/src/compiler/layout/dynamic.rs:26`.

Source inspection establishes current behavior and mechanical ownership; it is not a new bilateral parity measurement or evidence of upstream intent. Observable text, morphology, geometry and errors remain language-level obligations; buffer layout and traversal structure are implementation mechanics. Historical probes and their snapshot-specific qualifications are recorded in `lab/results/construction/diagnostics/sanitization-layout-history.md`. Older vanilla pins must not be presented as proof against the current pinned reference `9f2b6e8715237cb086899a42873660fe744622e8`.

## Ownership and contract

This prompt owns only the named consumer. State storage and general coordination remain in `00_nucleo/prompts/compiler/layout.md`; the contract below defines this unit's operations, not ownership of that hub.

Retain the existing E1 user-element boundary: resolve settable properties from `<kind>.<prop>` in the active chain, honoring the element's resolution rules. If the result is Dynamic, inspect that resolved element; otherwise retain the original. Dispatch a Content-valued body, or non-empty plain text as fallback. This existing DynElement interface does not authorize dynamic dispatch for native Content variants, a vtable-based layout refactor or render methods on domain entities.

## Acceptance and limits

Preserve the unit's current rendered content, morphology, causal positions/pages, styles and diagnostics, including the explicitly documented limits. Follow the existing static descendant interface without altering public signatures, defaults, compatibility or pipeline phase. This documentation does not authorize implementation of unresolved proposals.
