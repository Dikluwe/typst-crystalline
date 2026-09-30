# L0 — compiler/eval/table
Hash do Código: d3dcb76f

**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/eval/table.rs`
**Governance:** ADR-0107, ADR-0108, ADR-0127, ADR-0129, ADR-0130.

## Source evidence

The current exclusive consumer defines the following owned boundaries:

- `01_core/src/compiler/eval/table.rs:15` — `fn normalize_table_cell`.

Prior source readings supporting these clauses (snapshot provenance is
retained in the assigned history diagnostic, not asserted as a fresh measurement):

- `crates/typst-library/src/model/table.rs:494`.
- `crates/typst-library/src/model/table.rs:524`.
- `crates/typst-library/src/model/table.rs:732`.
- `model/table.rs:496-498`.
- `model/table.rs:734-780`.
- `model/table.rs:746-748`.
- `model/table.rs:750-752`.
- `model/table.rs:781-784`.

## Contract

Own only normalize_table_cell, the content-or-table.cell cast used by its
callers. An existing Content::TableCell clones the complete cell, preserving all
visual, geometric and accessibility fields. Other Value::Content becomes a cell
with that body, unset x/y/colspan/rowspan/stroke/fill/align/inset/breakable and
TableCellKind::Auto. Other values fail with `expected content, found <long type>`
at the existing detached span using the canonical type-name formatter.

This helper neither constructs a table namespace nor evaluates set rules,
numbering, captions or PDF headers. Those behaviors retain their respective
stdlib/rules/PDF owners; caller-specific argument diagnostics are not duplicated
in this cast. Do not discard fields by unwrapping an already constructed cell.
