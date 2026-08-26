# Prompt L0 — `entities/elements/counter_display_callback` — `CounterDisplayCallbackElem`
Hash do Código: 88f024ba

**Camada**: L1 · **Alvo**: `01_core/src/entities/elements/counter_display_callback.rs`
**Origem**: modelo D (ADR-0105), **Lote 6 P321** (família state/counter). Trait:
ver `entities/elements/_comum.md`. Comportamento idêntico (P241).

> **Fronteira: LOCATÁVEL**. `element_kind` → `ElementKind::CounterDisplay`
> (kind **partilhado** com o legacy `CounterDisplay`); `to_payload` →
> `ElementPayload::CounterDisplay`. Valor pré-renderizado em
> `apply_counter_displays` pós-fixpoint (consumo por payload, inalterado).

---

## Struct

```rust
#[derive(Debug, Clone, PartialEq)]
pub struct CounterDisplayCallbackElem {
    pub key:      String,
    pub callback: Option<Func>,
}
```

`Content::CounterDisplayCallback { key, callback }` →
`Content::CounterDisplayCallback(Arc<…Elem>)`. Construtor:
`Content::counter_display_callback(key, callback)`.

> **`Hash` manual via Debug**: `Func` não implementa `Hash` (tem `Arc<FuncRepr>`)
> → `impl Hash { format!("{self:?}")… }`. `PartialEq` deriva (`Func` tem
> `impl PartialEq` manual via `Arc::ptr_eq`).

## `impl Element`

| método | comportamento |
|---|---|
| `plain_text` | `String::new()` (`content.rs:1660`) |
| `is_empty` | default `false` |
| `map_*` | **terminais** |
| `element_kind` | `Some(ElementKind::CounterDisplay)` |
| `to_payload` | `Some(ElementPayload::CounterDisplay { key, callback })` (absorve `extract_payload.rs:67`) |

## `eq`

`#[derive(PartialEq)]` compara `key + callback` (Func via ptr_eq; paridade
`content.rs:2008`) → **dispatch** `(CounterDisplayCallback(a), …(b)) => a == b`.
