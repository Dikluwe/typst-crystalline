# Prompt L0 — `entities/elements/counter_display_callback` — `CounterDisplayCallbackElem`
Hash do Código: 5f98e175

**Layer:** L1
**Exclusive consumer:** `01_core/src/entities/elements/counter_display_callback.rs`
**Architecture:** ADR-0105. The trait contract is owned by
`00_nucleo/prompts/core/entities/elements/mod.md`.

## Evidence before specification

Original extraction anchors: `content.rs:1660`, `content.rs:2008`, `extract_payload.rs:67`.
These historical coordinates support the ownership boundary; they are not
fresh current-pin language-parity measurements.

## Data contract

```rust
#[derive(Debug, Clone, PartialEq)]
pub struct CounterDisplayCallbackElem {
    pub key:      String,
    pub callback: Option<Func>,
}
```

## Behavior and acceptance

Preserve `Content::CounterDisplayCallback(Arc<CounterDisplayCallbackElem>)`
and `Content::counter_display_callback(key, callback)`. Hash is manual via
Debug; derived PartialEq compares key and callback, using Func's Arc::ptr_eq.
The hub dispatches matching CounterDisplayCallback variants to entity equality.
Plain text is empty, emptiness defaults false, and maps are terminal.
Kind is CounterDisplay and payload CounterDisplay carries key/callback.
The element is locatable. Post-fixpoint `apply_counter_displays` consumes the
payload to pre-render the value; that algorithm belongs to its compiler owner.

### Preserved boundary ambiguity

The original says kind is shared with legacy CounterDisplay, while
`00_nucleo/prompts/core/entities/elements/counter_display.md` describes that
legacy carrier as non-locatable with default None kind. This cleanup preserves
the callback's CounterDisplay kind without silently resolving the legacy
claim. Verify callback identity, terminal maps, kind/payload, and hub equality.

## Historical record

The original specification and its construction chronology are preserved in
`lab/history/prompt-consolidation-2026-10-18/records/00_nucleo/prompts/core/entities/elements/counter_display_callback.md`. Historical observations are not a new parity certification.
