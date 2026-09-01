# Prompt L0 — `entities/elements/table` — `TableElem`
Hash do Código: 0fe0fe94

**Camada**: L1 · **Alvo**: `01_core/src/entities/elements/table.rs`
**Origem**: modelo D (ADR-0105), **Lote 12 P327** (bloco grid/table cell).
**P459**: campo `caption` opcional para numeração automática via `table.numbering`.
Trait e glossário (§A.0): ver `entities/elements/_comum.md`. **Não-locatável**. Contentor —
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
Consumidos por `layout_grid` (`compiler/layout/grid.rs`) como row-group — mesmo
mecanismo de `GridElem.header`/`.footer`, ver
`00_nucleo/prompts/compiler/layout.md` secção `grid.header(...)`/`grid.footer(...)`.

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

## P1288 — summary semântico de acessibilidade (PROPOSTO; gate ADR-0127)

### Medição anterior à decisão

`01_core/src/entities/elements/table.rs:29-42` mede `TableElem` sem summary.
A fonte vanilla pinada mede `summary: Option<EcoString>` interno em
`typst-library/src/model/table.rs:271-277` e `pdf.table-summary` substituindo
somente esse valor em `pdf/accessibility.rs:137-143`.

### Decisão proposta

Adicionar a `TableElem` um summary semântico opcional, distinto de caption e
texto visível. Só a omissão pública é convertida em `None` interno;
`summary: none` explícito é rejeitado no cast antes de alcançar a entidade;
string vazia permanece presente. Todo
constructor, clone, hash, igualdade mecânica, `map_content` e `map_text`
preserva o campo. `plain_text`/`is_empty` o ignoram, pois summary não é texto
visual nem body. `pdf.table-summary` substitui só esse campo.

O storage Rust exato é mecânica; o observável é a preservação do summary até
a estrutura `/Table` no PDF tagueado sem alterar texto, páginas, boxes ou
geometria.
