# Prompt L0 — `entities/elements/grid` — `GridElem`
Hash do Código: e7f36621

**Layer:** L1
**Exclusive consumer:** `01_core/src/entities/elements/grid.rs`
**Trait owner:** `00_nucleo/prompts/core/entities/elements/mod.md`

## Evidence before specification

The archived extraction record preserves the original source observations: `content.rs:1648`, `content.rs:1795`, `content.rs:1925`, `content.rs:2237`, `content.rs:2490`.
These are historical anchors, not a fresh current-pin parity measurement.

## Data contract

```rust
#[derive(Debug, Clone, PartialEq)]
pub struct GridElem {
    pub columns: Vec<TrackSizing>,
    pub rows:    Vec<TrackSizing>,
    pub cells:   Vec<Content>,
    pub gutter:  Option<Length>,
    pub align:   Option<Align2D>,
    pub inset:   Sides<Length>,
    pub header:  Option<Content>,
    pub footer:  Option<Content>,
    pub stroke:  Option<Stroke>,
    pub fill:    Option<Color>,
}
```

## Behavior and acceptance

Preserve `Content::Grid(Arc<GridElem>)` and `Content::grid(...)`.
Manual Hash uses Debug because TrackSizing, Length, Align2D, Sides<Length>,
Stroke, and Color contain f64; PartialEq compares all ten fields.
Plain text joins cell plain texts with one space. Emptiness checks only whether
the cells vector is empty. Both maps recurse through cells, header, and footer,
preserving columns, rows, gutter, align, inset, stroke, and fill; map_content
collects SourceResult. Header/footer may themselves be Content::GridHeader or
GridFooter, with ordinary transparent element dispatch. Field/kind/payload
retain defaults; non-locatable. Verify all fields, cell order/text joining,
vector-based emptiness, nested header/footer mapping, and structural equality.

## Historical record

The original specification and its construction chronology are preserved in
`lab/history/prompt-consolidation-2026-10-18/records/00_nucleo/prompts/core/entities/elements/grid.md`. Historical observations are not a new parity certification.
