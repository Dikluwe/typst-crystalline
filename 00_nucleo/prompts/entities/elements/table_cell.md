# Prompt L0 — `entities/elements/table_cell` — `TableCellElem`
Hash do Código: 53580625

**Camada**: L1 · **Alvo**: `01_core/src/entities/elements/table_cell.rs`
**Origem**: modelo D (ADR-0105), **Lote 12 P327** (bloco grid/table cell).
Trait e glossário (§A.0): ver `entities/elements/_comum.md`. **Não-locatável**. Contentor —
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

## P1288 — classificação semântica explícita da célula (PROPOSTO; gate ADR-0127)

### Medição anterior à decisão

`01_core/src/entities/elements/table_cell.rs:20-33` mede apenas body e campos
geométrico-visuais. A fonte vanilla pinada mede o carrier fechado
`TableCellKind::{Header(NonZeroU32, TableHeaderScope), Footer, Data}` e scopes
Both/Column/Row em `pdf/accessibility.rs:274-310`, enquanto `table.cell.kind`
começa em auto em `model/table.rs:772-778`.

### Decisão proposta

Adicionar classificação explícita capaz de distinguir `Auto`,
`Header { level positivo, scope: Column|Row|Both }` e `Data`. `Auto` permite
que o contexto `table.header`/`table.footer` derive a classe; Header/Data
explícitos vencem essa derivação. Em particular, `pdf.data-cell` dentro de
header permanece Data e nunca é promovido de volta a Header.

`map_content`, `map_text`, constructors, clone e hash preservam kind/level/
scope junto dos campos existentes. `plain_text`, body e render visual não são
alterados. A representação Rust pode usar enum fechado em vez de três campos;
não pode codificar a classe por string, style visual ou posição inferida.
