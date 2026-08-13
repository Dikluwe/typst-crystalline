# Prompt L0 — `entities/elements/quote` — `QuoteElem`
Hash do Código: e75db3d8

**Camada**: L1 · **Alvo**: `01_core/src/entities/elements/quote.rs`
**Origem**: modelo D (ADR-0105), **Lote 8 P323** (por largura). Trait e glossário (§A.0): ver
`entities/elements/_comum.md`. **Não-locatável**. Contentor — `map_*` recursam
em `body` **e** `attribution`.

---

## Struct

```rust
#[derive(Debug, Clone, PartialEq, Hash)]
pub struct QuoteElem {
    pub body:        Content,           // era Box<Content>
    pub attribution: Option<Content>,   // era Option<Box<Content>>
    pub block:       bool,
    pub quotes:      bool,
}
```

`Content::Quote { body, attribution, block, quotes }` →
`Content::Quote(Arc<QuoteElem>)`. Construtor ergonómico preservado:
`Content::quote(body, attribution, block, quotes)`. **Deriva `Hash`**
(`Content: Hash` via impl manual `content.rs:2039`; `bool: Hash`).

## `impl Element for QuoteElem`

| método | comportamento (idêntico ao braço atual) |
|---|---|
| `plain_text` | **custom** (`content.rs:1835`): `let txt = body.plain_text(); let w = if quotes { format!("\"{txt}\"") } else { txt }; match attribution { Some(a) => format!("{w} — {}", a.plain_text()), None => w }` |
| `is_empty` | **override**: `self.body.is_empty()` (`content.rs:1632`) |
| `map_content` | **recursivo** em `body` e `attribution`, preserva `block`/`quotes` (`content.rs:2132`) |
| `map_text` | **recursivo** em `body` e `attribution`, preserva `block`/`quotes` (`content.rs:2419`) |
| `get_field`/`element_kind`/`to_payload` | default |

## `eq`

`#[derive(PartialEq)]` compara os 4 campos (paridade `content.rs:1975`).
