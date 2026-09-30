# L0 — stdlib/structural/table_grid
Hash do Código: d4d8c07e

Layer: L1
Exclusive consumer: `01_core/src/compiler/stdlib/structural/table_grid.rs`

## Evidence and classification

`01_core/src/compiler/stdlib/structural/table_grid.rs:50`, `01_core/src/compiler/stdlib/structural/table_grid.rs:302`, `01_core/src/compiler/stdlib/structural/table_grid.rs:337` identifies the current ownership boundary. This inspection is not a new bilateral parity measurement or proof of upstream intent. Historical decision evidence retains its original snapshot qualifications in `00_nucleo/diagnosticos/sanitization-parser-stdlib-history.md`.

## Contract and ownership

Own table/grid container, cell and header/footer argument parsing and carriers, including the local usize/align/inset/bool extractors. Preserve paired table/grid validation except the explicitly specified semantic table-header level exception. Cells preserve x/y/colspan/rowspan and placement carriers; table/grid geometry resolution belongs to layout. Auto or None becomes None for optional usize placement; Align is direct or parsed from Str with Align2D::from_string; uniform Length inset becomes Sides<Length>; Bool retains the requested default. Preserve fill:none and stroke:none on their supported container/cell routes; zero-thickness remains rejected rather than PDF hairline.

Headers/footers preserve repeat=true default and explicit false. table.header accepts strictly positive integer level (default 1) and cells: raw content becomes Header Column-scope TableCell, an Auto cell is promoted likewise, and explicit Header/Data classification is retained. table combines distinct levels into one logical group in order and rejects duplicate levels, preserving the default-two-header diagnostic. This semantic PDF exception does not authorize level on grid.header. Keep geometry/repeat compatibility.

Reuse default_hline_stroke from `00_nucleo/prompts/compiler/stdlib/structural/table_lines.md`; do not duplicate line construction/rendering. Keep the table/grid symmetrical surface except for the documented header semantics, with no I/O or dynamic dispatch.

## Acceptance

Preserve the specified syntax, semantics, morphology, errors/hints/spans, defaults and public identities, with no productive-code change. Preserve pinned shared claims and the exclusive owner boundary under ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129 and ADR-0130. Historical evidence does not establish parity against a newer snapshot.
