# Prompt L0 — `entities/elements/table_vline` — `TableVLineElem`

**Camada**: L1 · **Alvo**: `01_core/src/entities/elements/table_vline.rs`
**Origem**: Passo 512 (linhas em grid/table). Trait: ver `entities/elements/_comum.md`. **Não-locatável**.

---

## Struct

```rust
#[derive(Debug, Clone, PartialEq, Hash)]
pub struct TableVLineElem {
    pub start:    usize,
    pub end:      Option<usize>, // None = até à última linha
    pub col:      usize,          // coluna da tabela onde a vline se posiciona
    pub stroke:   Stroke,
    pub position: EcoString,      // "left" | "right"
}
```

`Content::TableVLine(TableVLineElem)`.
Construtor ergonómico: `Content::table_vline(start: usize, end: Option<usize>, col: usize, stroke: Stroke, position: EcoString)`.

O campo `col` não é argumento público de `table.vline()`: é calculado por `native_table()` a partir da ordem dos children durante o parsing.

## `impl Element for TableVLineElem`

| método | comportamento |
|---|---|
| `plain_text` | string vazia |
| `is_empty` | `false` (linha é conteúdo visual) |
| `map_content` | devolve `Content::TableVLine(Arc::new(self.clone()))` (terminal) |
| `map_text` | devolve `Content::TableVLine(Arc::new(self.clone()))` (terminal) |
| `get_field`/`element_kind`/`to_payload` | default `None` |

## `eq` estrutural

`#[derive(PartialEq)]` compara `start + end + stroke + position`.

## Critério

`plain_text` vazio; `map_*` terminal; igualdade estrutural.
