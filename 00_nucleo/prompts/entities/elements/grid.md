# Prompt L0 — `entities/elements/grid` — `GridElem`
Hash do Código: 5ca6d6fc

**Camada**: L1 · **Alvo**: `01_core/src/entities/elements/grid.rs`
**Origem**: modelo D (ADR-0105), **Lote 12 P327** (bloco grid/table cell).
Trait e glossário (§A.0): ver `entities/elements/_comum.md`. **Não-locatável**. Contentor —
`map_*` recursam em `cells` (Vec), `header` e `footer`.

---

## Struct

```rust
#[derive(Debug, Clone, PartialEq)]
pub struct GridElem {
    pub columns: Vec<TrackSizing>,
    pub rows:    Vec<TrackSizing>,
    pub cells:   Vec<Content>,
    pub gutter:  Option<Length>,
    pub align:   Option<Align2D>,
    pub inset:   Sides<Length>,
    pub header:  Option<Content>,         // era Option<Box<Content>>
    pub footer:  Option<Content>,         // era Option<Box<Content>>
    pub stroke:  Option<Stroke>,
    pub fill:    Option<Color>,
}
```

`Content::Grid { … }` → `Content::Grid(Arc<GridElem>)`. Construtor ergonómico:
`Content::grid(...)`.

> **`Hash` manual via Debug**: `TrackSizing`/`Length`/`Align2D`/`Sides<Length>`/
> `Stroke`/`Color` carregam `f64` → `impl Hash { format!("{self:?}").hash(state) }`.
> `PartialEq` deriva.

> **Interação com Lote 5**: `header`/`footer` contêm `Content` que pode **ser**
> um `Content::GridHeader`/`GridFooter` (já `Arc<…Elem>` desde o Lote 5 P320).
> `map_*` recursam neles de forma transparente (despacho normal ao elemento).

## `impl Element for GridElem`

| método | comportamento (idêntico ao braço atual) |
|---|---|
| `plain_text` | `self.cells.iter().map(\|c\| c.plain_text()).collect::<Vec<_>>().join(" ")` (`content.rs:1795`) |
| `is_empty` | **override**: `self.cells.is_empty()` (`content.rs:1648`) |
| `map_content` | **recursivo** em `cells` (collect `SourceResult`), `header`, `footer`; preserva columns/rows/gutter/align/inset/stroke/fill (`content.rs:2237`) |
| `map_text` | **recursivo** em `cells`, `header`, `footer`; preserva os mesmos cosméticos (`content.rs:2490`) |
| `get_field`/`element_kind`/`to_payload` | default |

## `eq`

`#[derive(PartialEq)]` compara os 10 campos (paridade `content.rs:1925`).
