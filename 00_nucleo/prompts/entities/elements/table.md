# Prompt L0 — `entities/elements/table` — `TableElem`
Hash do Código: f37f26ed

**Camada**: L1 · **Alvo**: `01_core/src/entities/elements/table.rs`
**Origem**: modelo D (ADR-0105), **Lote 12 P327** (bloco grid/table cell).
**P459**: campo `caption` opcional para numeração automática via `table.numbering`.
Trait: ver `entities/elements/_comum.md`. **Não-locatável**. Contentor —
`map_*` recursam em cada `children` e no `caption`.

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
    pub caption:  Option<Content>,
}
```

`Content::Table { … }` → `Content::Table(Arc<TableElem>)`. Construtores
ergonómicos: `Content::table(columns, rows, children)` (caption `None`) e
`Content::table_with_caption(columns, rows, children, caption)` (P459).

> **Nota (P661):** a tabela com `caption` numerada por P459 continua a ser
> um `Content::Table`, não uma `figure`. Logo, não responde a show rules de
> `figure.where(kind: table)`.

> **`Hash` manual via Debug**: `TrackSizing` (colunas/linhas) e `Stroke`/`Color`
> carregam `f64` → `impl Hash { format!("{self:?}").hash(state) }`.
> `PartialEq` deriva.

## `impl Element for TableElem`

| método | comportamento |
|---|---|
| `plain_text` | `children` concatenados com espaço; `caption` prefixado quando presente (P459) |
| `is_empty` | `children.is_empty() && caption.as_ref().is_none_or(\|c\| c.is_empty())` |
| `map_content` | **recursivo** em cada `children` e no `caption`, preserva columns/rows/stroke/fill |
| `map_text` | **recursivo** em cada `children` e no `caption`, preserva columns/rows/stroke/fill |
| `get_field`/`element_kind`/`to_payload` | default |

## `eq`

`#[derive(PartialEq)]` compara os 6 campos (paridade `content.rs`).
