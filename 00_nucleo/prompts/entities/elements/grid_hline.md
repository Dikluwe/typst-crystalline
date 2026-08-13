# Prompt L0 — `entities/elements/grid_hline` — `GridHLineElem`
**P739A** — `stroke` passou a `Option<Stroke>`: `stroke: none` compila e a linha não é desenhada (paridade vanilla, medido; zero-thickness seria hairline em PDF — achado P726). O render (`compiler/layout/grid.rs`) salta linhas com `stroke: None`.

**Camada**: L1 · **Alvo**: `01_core/src/entities/elements/grid_hline.rs`
**Origem**: Passo 512 (linhas em grid/table). Trait e glossário (§A.0): ver `entities/elements/_comum.md`. **Não-locatável**.

---

## Struct

```rust
#[derive(Debug, Clone, PartialEq, Hash)]
pub struct GridHLineElem {
    pub start:    usize,
    pub end:      Option<usize>, // None = até à última coluna
    pub row:      usize,          // linha do grid onde a hline se posiciona
    pub stroke:   Option<Stroke>, // **P739A** — None = `stroke: none` (linha não desenhada; paridade vanilla)
    pub position: EcoString,      // "top" | "bottom"
}
```

`Content::GridHLine(GridHLineElem)`.
Construtor ergonómico: `Content::grid_hline(start: usize, end: Option<usize>, row: usize, stroke: Option<Stroke>, position: EcoString)`.

O campo `row` não é argumento público de `grid.hline()`: é calculado por `native_grid()` a partir da ordem dos children durante o parsing.

## `impl Element for GridHLineElem`

| método | comportamento |
|---|---|
| `plain_text` | string vazia |
| `is_empty` | `false` (linha é conteúdo visual) |
| `map_content` | devolve `Content::GridHLine(Arc::new(self.clone()))` (terminal) |
| `map_text` | devolve `Content::GridHLine(Arc::new(self.clone()))` (terminal) |
| `get_field`/`element_kind`/`to_payload` | default `None` |

## `eq` estrutural

`#[derive(PartialEq)]` compara `start + end + stroke + position`.

## Critério

`plain_text` vazio; `map_*` terminal; igualdade estrutural.
