# Prompt L0 — `entities/elements/table_hline` — `TableHLineElem`

**Camada**: L1 · **Alvo**: `01_core/src/entities/elements/table_hline.rs`
**Origem**: Passo 512 (linhas em grid/table). Trait: ver `entities/elements/_comum.md`. **Não-locatável**.

---

## Struct

```rust
#[derive(Debug, Clone, PartialEq, Hash)]
pub struct TableHLineElem {
    pub start:    usize,
    pub end:      Option<usize>, // None = até à última coluna
    pub row:      usize,          // linha da tabela onde a hline se posiciona
    pub stroke:   Stroke,
    pub position: EcoString,      // "top" | "bottom"
}
```

`Content::TableHLine(TableHLineElem)`.
Construtor ergonómico: `Content::table_hline(start: usize, end: Option<usize>, row: usize, stroke: Stroke, position: EcoString)`.

O campo `row` não é argumento público de `table.hline()`: é calculado por `native_table()` a partir da ordem dos children durante o parsing.

## `impl Element for TableHLineElem`

| método | comportamento |
|---|---|
| `plain_text` | string vazia |
| `is_empty` | `false` (linha é conteúdo visual) |
| `map_content` | devolve `Content::TableHLine(Arc::new(self.clone()))` (terminal) |
| `map_text` | devolve `Content::TableHLine(Arc::new(self.clone()))` (terminal) |
| `get_field`/`element_kind`/`to_payload` | default `None` |

## `eq` estrutural

`#[derive(PartialEq)]` compara `start + end + stroke + position`.

## Critério

`plain_text` vazio; `map_*` terminal; igualdade estrutural.
