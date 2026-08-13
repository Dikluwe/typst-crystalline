# Prompt L0 — `entities/elements/hide` — `HideElem`
Hash do Código: 0b7dfddb

**Camada**: L1 · **Alvo**: `01_core/src/entities/elements/hide.rs`
**Origem**: modelo D (ADR-0105), **Lote 7 P322** (por largura). Trait e glossário (§A.0): ver
`entities/elements/_comum.md`. **Não-locatável** (confirmado P322). Contentor —
`map_*` recursam no body.

---

## Struct

```rust
#[derive(Debug, Clone, PartialEq, Hash)]
pub struct HideElem {
    pub body: Content,   // era Box<Content>
}
```

`Content::Hide { body }` → `Content::Hide(Arc<HideElem>)`.
Construtor ergonómico preservado: `Content::hide(body)`. **Deriva `Hash`**
(`Content: Hash`).

## `impl Element`

| método | comportamento (idêntico ao braço atual) |
|---|---|
| `plain_text` | `String::new()` — hide é invisível (`content.rs:1812`) |
| `is_empty` | **override**: `self.body.is_empty()` (`content.rs:1640`) |
| `map_content` | **recursivo** no body (`content.rs:2155`) |
| `map_text` | **recursivo** no body (`content.rs:2445`) |
| `get_field`/`element_kind`/`to_payload` | default |

## `eq`

`#[derive(PartialEq)]` compara `body` (paridade `content.rs:1985`).
