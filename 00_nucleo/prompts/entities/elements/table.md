# Prompt L0 — `entities/elements/table` — `TableElem`
Hash do Código: 23ed1741

**Camada**: L1 · **Alvo**: `01_core/src/entities/elements/table.rs`
**Origem**: modelo D (ADR-0105), **Lote 12 P327** (bloco grid/table cell).
Trait: ver `entities/elements/_comum.md`. **Não-locatável**. Contentor —
`map_*` recursam em cada `children`.

---

## Struct

```rust
#[derive(Debug, Clone, PartialEq)]
pub struct TableElem {
    pub columns:  Vec<TrackSizing>,
    pub rows:     Vec<TrackSizing>,
    pub children: Vec<Content>,
    pub stroke:   Option<Stroke>,
    pub fill:     Option<Color>,
}
```

`Content::Table { … }` → `Content::Table(Arc<TableElem>)`. Construtor
ergonómico preservado: `Content::table(columns, rows, children)` (stroke/fill
default `None`).

> **`Hash` manual via Debug**: `TrackSizing` (colunas/linhas) e `Stroke`/`Color`
> carregam `f64` → `impl Hash { format!("{self:?}").hash(state) }`.
> `PartialEq` deriva.

## `impl Element for TableElem`

| método | comportamento (idêntico ao braço atual) |
|---|---|
| `plain_text` | `self.children.iter().map(\|c\| c.plain_text()).collect::<Vec<_>>().join(" ")` (`content.rs:1806`) |
| `is_empty` | **override**: `self.children.is_empty()` (`content.rs:1658`) |
| `map_content` | **recursivo** em cada `children`, preserva columns/rows/stroke/fill (`content.rs:2276`) |
| `map_text` | **recursivo** em cada `children`, preserva columns/rows/stroke/fill (`content.rs:2521`) |
| `get_field`/`element_kind`/`to_payload` | default |

## `eq`

`#[derive(PartialEq)]` compara os 5 campos (paridade `content.rs:1949`).
