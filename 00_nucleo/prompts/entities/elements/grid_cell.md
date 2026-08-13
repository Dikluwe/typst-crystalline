# Prompt L0 — `entities/elements/grid_cell` — `GridCellElem`
Hash do Código: 36d4c9dd

**Camada**: L1 · **Alvo**: `01_core/src/entities/elements/grid_cell.rs`
**Origem**: modelo D (ADR-0105), **Lote 12 P327** (bloco grid/table cell).
Trait e glossário (§A.0): ver `entities/elements/_comum.md`. **Não-locatável**. Contentor —
`map_*` recursam no `body`. **Campos idênticos a `TableCellElem`** (estruturas
gémeas; arms separados no hub, sem `|`-combinado — sem split).

---

## Struct

```rust
#[derive(Debug, Clone, PartialEq)]
pub struct GridCellElem {
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

`Content::GridCell { … }` → `Content::GridCell(Arc<GridCellElem>)`.
Construtor ergonómico: `Content::grid_cell(...)`.

> **`Hash` manual via Debug**: `Stroke`/`Color`/`Align2D`/`Sides<Length>` com
> `f64` → `impl Hash { format!("{self:?}").hash(state) }`. `PartialEq` deriva.

## `impl Element for GridCellElem`

| método | comportamento (idêntico ao braço atual) |
|---|---|
| `plain_text` | `self.body.plain_text()` (transparente, `content.rs:1802`) |
| `is_empty` | **override**: `self.body.is_empty()` (`content.rs:1654`) |
| `map_content` | **recursivo** no `body`, preserva os 9 cosméticos (`content.rs:2262`) |
| `map_text` | **recursivo** no `body`, preserva os 9 cosméticos (`content.rs:2507`) |
| `get_field`/`element_kind`/`to_payload` | default |

## `eq`

`#[derive(PartialEq)]` compara os 10 campos (paridade `content.rs:1939`).
