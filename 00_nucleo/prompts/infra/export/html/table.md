# L0 — html/table
Hash do Código: e0d03431

**Layer:** L3
**Exclusive consumer:** `03_infra/src/export/html/table.rs`
**Governance:** ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129, ADR-0130

## Consumer evidence

`03_infra/src/export/html/table.rs:12` identifies the current exclusive consumer. The clauses below
are checked against that source and its delegated calls. They specify current
export observables and ownership, not full upstream byte or algorithm equality.

## Table encoding

Own html_table and flatten/row helpers. Use columns.len().max(1); a present header
emits thead/th and ordinary children tbody/td. Flatten Sequence/TableHeader/TableFooter
bodies under the existing rules. Cell colspan/rowspan default to one and attributes
are emitted only above one. Preserve chunk-based row grouping, including its current
span-occupancy limitation; do not invent a separate footer or expand supported tables.
Delegate cell content to inline with the same mode, resolver and StyleChain.

## Verification

Check empty/ordinary/header tables, flattened wrappers, spans and style/frame
propagation without paginated layout.
