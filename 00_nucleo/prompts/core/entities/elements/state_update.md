# Prompt L0 — `entities/elements/state_update` — `StateUpdateElem`
Hash do Código: 7febc4ae

**Layer:** L1
**Exclusive consumer:** `01_core/src/entities/elements/state_update.rs`
**Architecture:** ADR-0105. The trait contract is owned by
`00_nucleo/prompts/core/entities/elements/mod.md`.

## Evidence before specification

Original extraction anchors: `content.rs:1658`, `extract_payload.rs:49`, `state_update.rs:53`.
These historical coordinates support the ownership boundary; they are not
fresh current-pin language-parity measurements.

## Data contract

```rust
#[derive(Debug, Clone, PartialEq)]
pub struct StateUpdateElem {
    pub key:    String,
    pub update: StateUpdate,   // entities::state_update::StateUpdate
}
```

## Behavior and acceptance

Preserve `Content::StateUpdate(Arc<StateUpdateElem>)` and
`Content::state_update(key, update)`. Hash is manual through Debug because
StateUpdate has no Hash; derived PartialEq uses StateUpdate's manual equality.
Plain text is empty, emptiness defaults false, and both maps are terminal.
The element is locatable: kind StateUpdate and payload StateUpdate preserve
key/update. The `from_tags` consumer applies Set/Func to StateRegistry through
that payload; this owner does not perform registry application.

### Preserved cross-owner discrepancy

The earlier contract explicitly requires the hub's StateUpdate equality arm
to remain absent (`_ => false`), despite derived entity equality. This cleanup
does not revoke that obligation or add dispatch. Reconciliation belongs to
`00_nucleo/prompts/core/entities/content/equality.md`; do not infer language
equality from the entity's Rust equality. Verify terminal transport, payload,
and the existing hub distinction.

## Historical record

The original specification and its construction chronology are preserved in
`lab/history/prompt-consolidation-2026-10-18/records/00_nucleo/prompts/core/entities/elements/state_update.md`. Historical observations are not a new parity certification.
