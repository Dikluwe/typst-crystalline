# Prompt L0 — `entities/elements/grid_cell` — `GridCellElem`
Hash do Código: 27ac91b7

**Layer:** L1
**Exclusive consumer:** `01_core/src/entities/elements/grid_cell.rs`
**Architecture:** ADR-0105. The trait contract is owned by
`00_nucleo/prompts/core/entities/elements/mod.md`.

## Evidence before specification

Original extraction anchors: `content.rs:1654`, `content.rs:1802`, `content.rs:1939`, `content.rs:2262`, `content.rs:2507`.
These historical coordinates support the ownership boundary; they are not
fresh current-pin language-parity measurements.

## Data contract

```rust
#[derive(Debug, Clone, PartialEq)]
pub struct GridCellElem {
    pub body:      Content,
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

## Behavior and acceptance

Preserve `Content::GridCell(Arc<GridCellElem>)` and `Content::grid_cell(...)`.
This contract declares fields identical to TableCellElem, with separate hub
arms, no combined `|` dispatch and no split. The independent table-cell owner
is `00_nucleo/prompts/core/entities/elements/table_cell.md`; differing future
requirements require reconciliation rather than an implicit grid change.
Hash remains manual via Debug because Stroke/Color/Align2D/Sides<Length>
contain f64. Derived PartialEq compares all ten fields. Plain text and
emptiness delegate to body. Both maps recurse into body and preserve the nine
other fields. Field/kind/payload retain defaults; non-locatable. Verify
transparency, maps, all metadata, and structural equality.

## Historical record

The original specification and its construction chronology are preserved in
`lab/history/prompt-consolidation-2026-10-18/records/00_nucleo/prompts/core/entities/elements/grid_cell.md`. Historical observations are not a new parity certification.
