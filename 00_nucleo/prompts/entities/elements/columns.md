# Prompt L0 — `entities/elements/columns` — `ColumnsElem`
Hash do Código: a03ab2b3

**Camada**: L1 · **Alvo**: `01_core/src/entities/elements/columns.rs`
**Origem**: modelo D (ADR-0105), **Lote 8 P323** (por largura). Trait: ver
`entities/elements/_comum.md`. **Não-locatável**. Contentor — `map_*` recursam
no `body`.

---

## Struct

```rust
#[derive(Debug, Clone, PartialEq)]
pub struct ColumnsElem {
    pub count:  usize,
    pub gutter: Option<Length>,
    pub body:   Content,            // era Box<Content>
}
```

`Content::Columns { count, gutter, body }` →
`Content::Columns(Arc<ColumnsElem>)`. Construtor ergonómico preservado:
`Content::columns(count, gutter, body)`.

> **`Hash` manual via Debug** (precedente Lote 4/5/7): `Length` carrega `f64` e
> não implementa `Hash` → `impl Hash { format!("{self:?}").hash(state) }`
> (paridade `content_hash`; ressalva `-0.0`≠`0.0`; `…Elem` não é chave de mapa —
> revisitar se passar a ser). `PartialEq` deriva (`usize`/`Length`/`Content`
> implementam `PartialEq`).

## `impl Element for ColumnsElem`

| método | comportamento (idêntico ao braço atual) |
|---|---|
| `plain_text` | `self.body.plain_text()` (`content.rs:1834`) |
| `is_empty` | **override**: `self.body.is_empty()` (`content.rs:1665`) |
| `map_content` | **recursivo** no `body`, preserva `count`/`gutter` (`content.rs:2143`) |
| `map_text` | **recursivo** no `body`, preserva `count`/`gutter` (map_text) |
| `get_field`/`element_kind`/`to_payload` | default |

## `eq`

`#[derive(PartialEq)]` compara `count`/`gutter`/`body` (paridade
`content.rs:2020`).
