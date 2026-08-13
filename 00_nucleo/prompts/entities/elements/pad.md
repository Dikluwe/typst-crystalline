# Prompt L0 — `entities/elements/pad` — `PadElem`
Hash do Código: ade8e6e5

**Camada**: L1 · **Alvo**: `01_core/src/entities/elements/pad.rs`
**Origem**: modelo D (ADR-0105), **Lote 10 P325** (por largura). Trait e glossário (§A.0): ver
`entities/elements/_comum.md`. **Não-locatável**. Contentor — `map_*` recursam
no `body`.

---

## Struct

```rust
#[derive(Debug, Clone, PartialEq)]
pub struct PadElem {
    pub body:  Content,                  // era Box<Content>
    pub sides: Sides<Option<Length>>,
}
```

`Content::Pad { body, sides }` → `Content::Pad(Arc<PadElem>)`. Construtor
ergonómico preservado: `Content::pad(body, sides)`.

> **`Hash` manual via Debug**: `Sides<Option<Length>>` carrega `f64` (`Length`)
> → `impl Hash { format!("{self:?}").hash(state) }` (paridade `content_hash`;
> ressalva `-0.0`≠`0.0`). `PartialEq` deriva.

## `impl Element for PadElem`

| método | comportamento (idêntico ao braço atual) |
|---|---|
| `plain_text` | `self.body.plain_text()` (`content.rs:1843`) |
| `is_empty` | **override**: `self.body.is_empty()` (`content.rs:1677`) |
| `map_content` | **recursivo** no `body`, preserva `sides` (`content.rs:2148`) |
| `map_text` | **recursivo** no `body`, preserva `sides` (`content.rs:2395`) |
| `get_field`/`element_kind`/`to_payload` | default |

## `eq`

`#[derive(PartialEq)]` compara `body`/`sides` (paridade `content.rs:1995`).
