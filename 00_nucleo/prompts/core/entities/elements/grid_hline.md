# Prompt L0 — `entities/elements/grid_hline` — `GridHLineElem`
Hash do Código: aab808b3

**Layer:** L1
**Exclusive consumer:** `01_core/src/entities/elements/grid_hline.rs`
**Trait owner:** `00_nucleo/prompts/core/entities/elements/mod.md`

## Evidence before specification

The archived source records the measured distinction between `stroke: none`
and zero thickness: zero thickness becomes a PDF hairline, whereas None must
skip drawing. This is historical evidence, not a fresh current-pin probe.

## Data contract

```rust
#[derive(Debug, Clone, PartialEq, Hash)]
pub struct GridHLineElem {
    pub start:    usize,
    pub end:      Option<usize>,
    pub row:      usize,
    pub stroke:   Option<Stroke>,
    pub position: EcoString,
}
```

`end: None` extends through the last column; position uses top/bottom.
Keep the `Content::GridHLine` variant and ergonomic `Content::grid_hline` constructor
with arguments start, end, row, stroke, position in that order and the
field types above. The `row` field is computed by `native_grid()` from
child order, not a public argument of `grid.hline()`.
`stroke: None` preserves `stroke: none`; the layout consumer skips drawing.

## Behavior and acceptance

Plain text is empty. `is_empty` is false because this is structural visual
content. Both maps are terminal and reconstruct
`Content::GridHLine(Arc::new(self.clone()))`. Field/kind/payload default to None;
non-locatable. Verify empty text, terminal maps, and structural equality.

## Pending documentary reconciliation

The original specifies derived PartialEq but enumerates only start, end,
stroke, position in its equality summary, omitting row. Preserve the
structural derive and all fields; this editorial cleanup does not authorize
ignoring row in equality. Its shorter summary is not a new contract change.

## Historical record

The original specification and its construction chronology are preserved in
`lab/history/prompt-consolidation-2026-10-18/records/00_nucleo/prompts/core/entities/elements/grid_hline.md`. Historical observations are not a new parity certification.


## Issue 8894 — approved Stroke migration

Measurement: the source inventory and previous owner bytes in
`lab/results/construction/diagnostics/issue-8894-2026-10-31/` bind exact
HEAD/dirty diffstat/UTC and each affected source/owner before migration.
`stdlib/layout/stroke.rs:430–434` erases em; geometry.rs:81 carries only f64.
The explicit user approval in approval.json ratifies the public Length field
and occurrence-layout resolution, as specified by
`00_nucleo/prompts/core/entities/geometry.md`. This consumer retains its
exclusive responsibility and all other ratified clauses and controls.

Migrate this consumer's declarative Stroke literal thickness from scalar
points to Length::pt without changing their value or other fields. FrameStroke
remains scalar. Keep all existing semantic assertions; update only their
representation as necessary. Producers preserve full Length; layout consumers
resolve against effective font size before scalar geometry. Repr preserves
absolute/em components. RED→GREEN, build/tests and scoped lineage are required.
