# Prompt L0 — `entities/elements/table_vline` — `TableVLineElem`
**P739A** — `stroke` passou a `Option<Stroke>`: `stroke: none` compila e a linha não é desenhada (paridade vanilla, medido; zero-thickness seria hairline em PDF — achado P726). O render (`engine/layout/grid.rs`) salta linhas com `stroke: None`.

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
    pub stroke:   Option<Stroke>, // **P739A** — None = `stroke: none` (linha não desenhada; paridade vanilla)
    pub position: EcoString,      // "left" | "right"
}
```

`Content::TableVLine(TableVLineElem)`.
Construtor ergonómico: `Content::table_vline(start: usize, end: Option<usize>, col: usize, stroke: Option<Stroke>, position: EcoString)`.

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
