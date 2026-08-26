# Prompt L0 — `entities/elements/state_update` — `StateUpdateElem`
Hash do Código: 3083bb9e

**Camada**: L1 · **Alvo**: `01_core/src/entities/elements/state_update.rs`
**Origem**: modelo D (ADR-0105), **Lote 6 P321** (família state/counter). Trait:
ver `entities/elements/_comum.md`. Comportamento idêntico (P171/P173).

> **Fronteira: LOCATÁVEL**. `element_kind` → `ElementKind::StateUpdate`;
> `to_payload` → `ElementPayload::StateUpdate`. `from_tags` aplica ao
> `StateRegistry` (Set/Func; consumo por payload, inalterado).

---

## Struct

```rust
#[derive(Debug, Clone, PartialEq)]
pub struct StateUpdateElem {
    pub key:    String,
    pub update: StateUpdate,   // entities::state_update::StateUpdate
}
```

`Content::StateUpdate { key, update }` → `Content::StateUpdate(Arc<…Elem>)`.
Construtor: `Content::state_update(key, update)`.

> **`Hash` manual via Debug**: `state_update::StateUpdate` deriva só
> `Debug, Clone` (sem `Hash`) → `impl Hash { format!("{self:?}")… }`. **`PartialEq`
> deriva** — `StateUpdate` tem `impl PartialEq` manual (`state_update.rs:53`).

## `impl Element`

| método | comportamento |
|---|---|
| `plain_text` | `String::new()` (`content.rs:1658`) |
| `is_empty` | default `false` |
| `map_*` | **terminais** |
| `element_kind` | `Some(ElementKind::StateUpdate)` |
| `to_payload` | `Some(ElementPayload::StateUpdate { key, update })` (absorve `extract_payload.rs:49`) |

## `eq` — quirk pré-existente preservado

**`StateUpdate` NÃO tem arm de `eq` no hub** (cai em `_ => false`). **Preservar**
— não adicionar dispatch (o `derive` existe pelo contrato; hub mantém `_ => false`).
