# Prompt L0 — `entities/elements/state_display` — `StateDisplayElem`
Hash do Código: ced154b4

**Camada**: L1 · **Alvo**: `01_core/src/entities/elements/state_display.rs`
**Origem**: modelo D (ADR-0105), **Lote 6 P321** (família state/counter). Trait:
ver `entities/elements/_comum.md`. Comportamento idêntico (P240).

> **Fronteira: LOCATÁVEL**. `element_kind` → `ElementKind::StateDisplay`;
> `to_payload` → `ElementPayload::StateDisplay`. Valor pré-renderizado em
> `apply_state_displays` pós-fixpoint.

---

## Struct

```rust
#[derive(Debug, Clone, PartialEq)]
pub struct StateDisplayElem {
    pub key:      String,
    pub callback: Option<Func>,
}
```

`Content::StateDisplay { key, callback }` → `Content::StateDisplay(Arc<…Elem>)`.
Construtor: `Content::state_display(key, callback)`.

> **`Hash` manual via Debug** (`Func`); `PartialEq` deriva (Func ptr_eq).

## `impl Element`

| método | comportamento |
|---|---|
| `plain_text` | `String::new()` (`content.rs:1659`) |
| `is_empty` | default `false` |
| `map_*` | **terminais** |
| `element_kind` | `Some(ElementKind::StateDisplay)` |
| `to_payload` | `Some(ElementPayload::StateDisplay { key, callback })` (absorve `extract_payload.rs:58`) |

## `eq`

`#[derive(PartialEq)]` compara `key + callback` (paridade `content.rs:2003`) →
**dispatch** `(StateDisplay(a), StateDisplay(b)) => a == b`.
