# L0 — counters
Hash do Código: bf43f367

**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/layout/counters.rs`
**Architecture:** ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129, ADR-0130.

## Evidence before specification

The consumer contains only its lineage/compatibility comments; no productive counter function is present. See `01_core/src/compiler/layout/counters.rs:7`.

Source inspection establishes current behavior and mechanical ownership; it is not a new bilateral parity measurement or evidence of upstream intent. Observable text, morphology, geometry and errors remain language-level obligations; buffer layout and traversal structure are implementation mechanics. Historical probes and their snapshot-specific qualifications are recorded in `00_nucleo/diagnosticos/sanitization-layout-history.md`. Older vanilla pins must not be presented as proof against the current pinned reference `9f2b6e8715237cb086899a42873660fe744622e8`.

## Ownership and contract

This prompt owns only the named consumer. State storage and general coordination remain in `00_nucleo/prompts/compiler/layout.md`; the contract below defines this unit's operations, not ownership of that hub.

The current consumer is an empty compatibility module with a lineage header. It exposes no counter-update, heading-numbering, equation-numbering or display implementation. Do not specify removed CounterStateLegacy helpers as behavior of this owner and do not repopulate the module. CounterUpdate remains a central no-op after introspection; display behavior belongs to `00_nucleo/prompts/compiler/layout/counter_display.md` and `00_nucleo/prompts/compiler/layout/counter_display_callback.md`.

## Acceptance and limits

The module remains behaviorally empty. No legacy counter mutation or display logic is reintroduced.
