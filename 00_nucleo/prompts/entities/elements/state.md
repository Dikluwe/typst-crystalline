# Prompt L0 — `entities/elements/state` — `StateElem`
Hash do Código: 18ffda4e

**Camada**: L1 · **Alvo**: `01_core/src/entities/elements/state.rs`
**Origem**: modelo D (ADR-0105), **Lote 6 P321** (família state/counter). Trait:
ver `entities/elements/_comum.md`. Comportamento idêntico (P171).

> **Fronteira: LOCATÁVEL**. `element_kind` → `ElementKind::State`; `to_payload`
> → `ElementPayload::State`. `from_tags` inicia o `StateRegistry` (consumo por
> payload, inalterado).

---

## Struct

```rust
#[derive(Debug, Clone, PartialEq)]
pub struct StateElem {
    pub key:  String,
    pub init: Box<Value>,
}
```

`Content::State { key, init }` → `Content::State(Arc<StateElem>)`.
Construtor: `Content::state(key, init: Value)`.

> **`Hash` manual via Debug** (`Value` tem `f64`); `PartialEq` deriva.

## `impl Element`

| método | comportamento |
|---|---|
| `plain_text` | `String::new()` (`content.rs:1657`) |
| `is_empty` | default `false` |
| `map_*` | **terminais** |
| `element_kind` | `Some(ElementKind::State)` |
| `to_payload` | `Some(ElementPayload::State { key, init })` (absorve `extract_payload.rs:43`) |

## `eq` — quirk pré-existente preservado

**`State` NÃO tem arm de `eq` no hub** (cai em `_ => false`, sempre desigual).
**Preservar** — não adicionar dispatch (igual a `metadata.md`).
