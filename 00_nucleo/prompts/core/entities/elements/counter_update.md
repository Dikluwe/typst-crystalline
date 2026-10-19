# Prompt L0 — `entities/elements/counter_update` — `CounterUpdateElem`
Hash do Código: 2805c367

**Layer:** L1
**Exclusive consumer:** `01_core/src/entities/elements/counter_update.rs`
**Trait owner:** `00_nucleo/prompts/core/entities/elements/mod.md`

## Evidence before specification

Historical anchors are content.rs:1704,1868, extract_payload.rs:129 and
counter_update.rs:20. The observation record establishes that element and
payload clone key/action and that a Func action already transports the real
Func through Content, maps, and walk; no extra element field is needed for
that measured fixture.

## Data contract

```rust
#[derive(Debug, Clone, PartialEq, Hash)]
pub struct CounterUpdateElem {
    pub key:    String,
    pub action: CounterAction,
}
```

## Behavior and acceptance

Preserve `Content::CounterUpdate(Arc<CounterUpdateElem>)` and
`Content::counter_update(key, action)`. Derived equality compares key/action;
the hub dispatches matching CounterUpdate variants to it. Derive Hash when
CounterAction supports it; the original conditional fallback to manual Debug
hash if action gains Value/f64 remains a future review condition, not an
unconditional algorithm change.
Plain text is empty, is_empty defaults false, and maps are terminal. This
carrier is locatable: kind CounterUpdate and payload CounterUpdate preserve
key/action. from_tags applies the payload to CounterRegistry through apply_at
or apply_hierarchical_at; this owner does not implement registry application.

Under exact `cfg(p1339_observation)`, a Func action may carry the private
observation carrier defined by `00_nucleo/prompts/core/entities/func.md`.
Clone, both maps, and to_payload preserve that same nested carrier by cloning
Func. Do not extract/recreate it or index it through a side table. Set/Step
have no such carrier. This is test-only transport for the specified observation
fragment; the normal struct, fields, payload, Eq/Hash, locatability and result
stay unchanged. Verify terminal transport, equality/hash, payload identity,
and the nested test-only carrier where enabled.

## Historical record

The original specification and its construction chronology are preserved in
`lab/history/prompt-consolidation-2026-10-18/records/00_nucleo/prompts/core/entities/elements/counter_update.md`. Historical observations are not a new parity certification.
