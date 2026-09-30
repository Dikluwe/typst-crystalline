# L0 — Paper and page binding
Hash do Código: e9e7a288

**Layer:** L1
**Exclusive consumer:** `01_core/src/entities/page_geometry.rs`

## Evidence before specification

Historical upstream `a51e02804` specifies paper shorthand at
`layout/page.rs:54-103`, its millimetre catalog at `page.rs:818-1032`,
binding at `pages/run.rs:149-155`, and parity at `layout/page.rs:724-756`.
These are retained snapshot-specific anchors, not a current-pin parity probe.

## Owned contract

`Paper` has private canonical name, width and height in millimetres. Preserve
`Paper::A4`, `from_name`, `name`, `width_pt`, `height_pt`, the complete source
catalog including Unicode names, case-insensitive parsing and invalid-name
rejection. A4 is 210 by 297 mm. Convert exactly with `pt = mm * 72 / 25.4`;
rounded render/export dimensions are not normative catalog values.
`PageBinding` is Auto, Left or Right. Auto resolves LTR to Left and RTL to Right.
`swap` takes a nonzero one-based physical page: Left swaps on even pages,
Right on odd pages. These operations are pure L1, independent of Value,
layout frames, exporters, I/O and global state.

## Delegation

Margin storage belongs to `00_nucleo/prompts/entities/layout_types.md`;
geometry application to `00_nucleo/prompts/compiler/layout/page_geometry.md`;
regional transitions to `00_nucleo/prompts/compiler/layout.md`.
Former cross-owner obligations and incomplete handoffs are preserved in the
sanitation diagnostic; removing their duplication does not waive them.
Canvas, running matter and supplement have separate entity owners.
Verify catalog completeness, exact conversion, invalid names, automatic binding
and both parities without changing public contracts, defaults or pipeline phase.
