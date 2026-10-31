# L0 — stdlib/structural/table_lines
Hash do Código: b09ae5f8

Layer: L1
Exclusive consumer: `01_core/src/compiler/stdlib/structural/table_lines.rs`

## Evidence and classification

`01_core/src/compiler/stdlib/structural/table_lines.rs:30`, `01_core/src/compiler/stdlib/structural/table_lines.rs:40`, `01_core/src/compiler/stdlib/structural/table_lines.rs:81` identifies the current ownership boundary. This inspection is not a new bilateral parity measurement or proof of upstream intent. Historical decision evidence retains its original snapshot qualifications in `lab/results/construction/diagnostics/sanitization-parser-stdlib-history.md`.

## Contract and ownership

Own grid/table hline/vline constructors, extract_line_range and default_hline_stroke. Pair table/grid validation and defaults: start integer >=0 default 0, end optional auto, stroke default 1pt and optional none, position auto|top|bottom for horizontal or auto|left|right for vertical. Wrong-axis values retain their exact diagnostic; errors use the existing detached span. Reject zero-thickness; stroke:none is represented as Option::None, not a zero-width Stroke.

Transport line placement based on child order, not absolute coordinates. Container parsing in table_grid resolves row/column; horizontal auto becomes top when first in a row, otherwise bottom, and vertical auto becomes left. Rendering skips None and remains with its render owner. table_grid reuses this unit's default stroke helper; preserve the existing sibling linkage and pure execution.

## Acceptance

Preserve the specified syntax, semantics, morphology, errors/hints/spans, defaults and public identities, with no productive-code change. Preserve pinned shared claims and the exclusive owner boundary under ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129 and ADR-0130. Historical evidence does not establish parity against a newer snapshot.


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
