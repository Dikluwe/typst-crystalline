# Prompt L0 — `entities/elements/table_cell` — `TableCellElem`
Hash do Código: f7d16310

**Camada**: L1 · **Alvo**: `01_core/src/entities/elements/table_cell.rs`
**Origem**: modelo D (ADR-0105), **Lote 12 P327** (bloco grid/table cell).
Trait: ver `entities/elements/_comum.md`. **Não-locatável**. Contentor —
`map_*` recursam no `body`.

---

## Struct

```rust
#[derive(Debug, Clone, PartialEq)]
pub struct TableCellElem {
    pub body:      Content,               // era Box<Content>
    pub x:         Option<usize>,
    pub y:         Option<usize>,
    pub colspan:   Option<usize>,
    pub rowspan:   Option<usize>,
    pub stroke:    Option<Stroke>,
    pub fill:      Option<Color>,
    pub align:     Option<Align2D>,
    pub inset:     Option<Sides<Length>>,
    pub breakable: Option<bool>,
}
```

`Content::TableCell { … }` → `Content::TableCell(Arc<TableCellElem>)`.
Construtor ergonómico preservado: `Content::table_cell(...)`.

> **`Hash` manual via Debug**: `Stroke`/`Color`/`Align2D`/`Sides<Length>`
> carregam `f64` → `impl Hash { format!("{self:?}").hash(state) }`
> (paridade `content_hash`; ressalva `-0.0`≠`0.0`). `PartialEq` deriva.

## `impl Element for TableCellElem`

| método | comportamento (idêntico ao braço atual) |
|---|---|
| `plain_text` | `self.body.plain_text()` (transparente; spans runtime-only, `content.rs:1813`) |
| `is_empty` | **override**: `self.body.is_empty()` (`content.rs:1662`) |
| `map_content` | **recursivo** no `body`, preserva os 9 cosméticos (`content.rs:2289`) |
| `map_text` | **recursivo** no `body`, preserva os 9 cosméticos (`content.rs:2530`) |
| `get_field`/`element_kind`/`to_payload` | default |

## `eq`

`#[derive(PartialEq)]` compara os 10 campos (paridade `content.rs:1953`).
