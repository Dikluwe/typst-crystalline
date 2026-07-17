# Prompt L0 — `entities/elements/table` — `TableElem`
Hash do Código: b6c4c930

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
    pub hlines:   Vec<TableHLineElem>,  // P512
    pub vlines:   Vec<TableVLineElem>,  // P512
    pub header:   Option<Content>,      // P772v — row-group real, mesmo mecanismo de GridElem (P772i)
    pub footer:   Option<Content>,      // P772v
    pub stroke:   Option<Stroke>,
    pub fill:     Option<Color>,
    pub caption:  Option<Content>,
}
```

**P772v**: `header`/`footer` guardam `Content::TableHeader`/`Content::TableFooter`
(um de cada, no máximo — erro explícito em duplicado, mesma regra de `GridElem`)
extraídos do loop de resolução de `native_table` (`stdlib/structural.rs`), não
argumentos nomeados (paridade vanilla: `table.header(...)`/`table.footer(...)`
são elementos-filho — ver `lab/typst-original/.../model/table.rs:495,525`).
Consumidos por `layout_grid` (`rules/layout/grid.rs`) como row-group — mesmo
mecanismo de `GridElem.header`/`.footer`, ver
`00_nucleo/prompts/rules/layout.md` secção `grid.header(...)`/`grid.footer(...)`.

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
| `map_content` | **recursivo** em cada `children`, `header`, `footer` e no `caption`, preserva columns/rows/hlines/vlines/stroke/fill |
| `map_text` | **recursivo** em cada `children`, `header`, `footer` e no `caption`, preserva columns/rows/hlines/vlines/stroke/fill |
| `get_field`/`element_kind`/`to_payload` | default (excepto `element_kind`/`to_payload`, P459 — ver código) |

## `eq`

`#[derive(PartialEq)]` compara todos os 10 campos (paridade `content.rs`).
