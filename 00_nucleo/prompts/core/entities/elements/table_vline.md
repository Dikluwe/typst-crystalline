# Prompt L0 — `entities/elements/table_vline` — `TableVLineElem`
Hash do Código: bc8240a6

**Layer:** L1
**Exclusive consumer:** `01_core/src/entities/elements/table_vline.rs`
**Trait owner:** `00_nucleo/prompts/core/entities/elements/mod.md`

## Evidence before specification

The archived source records the measured distinction between `stroke: none`
and zero thickness: zero thickness becomes a PDF hairline, whereas None must
skip drawing. This is historical evidence, not a fresh current-pin probe.

## Data contract

```rust
#[derive(Debug, Clone, PartialEq, Hash)]
pub struct TableVLineElem {
    pub start:    usize,
    pub end:      Option<usize>,
    pub col:      usize,
    pub stroke:   Option<Stroke>,
    pub position: EcoString,
}
```

`end: None` extends through the last row; position uses left/right.
Keep the `Content::TableVLine` variant and ergonomic `Content::table_vline` constructor
with arguments start, end, col, stroke, position in that order and the
field types above. The `col` field is computed by `native_table()` from
child order, not a public argument of `table.vline()`.
`stroke: None` preserves `stroke: none`; the layout consumer skips drawing.

## Behavior and acceptance

Plain text is empty. `is_empty` is false because this is structural visual
content. Both maps are terminal and reconstruct
`Content::TableVLine(Arc::new(self.clone()))`. Field/kind/payload default to None;
non-locatable. Verify empty text, terminal maps, and structural equality.

## Pending documentary reconciliation

The original specifies derived PartialEq but enumerates only start, end,
stroke, position in its equality summary, omitting col. Preserve the
structural derive and all fields; this editorial cleanup does not authorize
ignoring col in equality. Its shorter summary is not a new contract change.

## Historical record

The original specification and its construction chronology are preserved in
`lab/history/prompt-consolidation-2026-10-18/records/00_nucleo/prompts/core/entities/elements/table_vline.md`. Historical observations are not a new parity certification.
