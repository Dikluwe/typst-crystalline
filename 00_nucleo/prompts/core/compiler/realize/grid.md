# L0 — realize/grid
Hash do Código: 2a457252

**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/realize/grid.rs`
**Governance:** ADR-0107, ADR-0109, ADR-0127, ADR-0129, ADR-0132.

## Contract

Prepare a `grid` built by its constructor once, at its insertion point.

- Properties not given in the call take their `set grid` declaration. When any does,
  the grid is rebuilt by its constructor from the merged arguments.
- `fill`, `stroke`, `align` and `inset` given as an array or function apply per cell:
  an array by column, cycling; a function called with the cell's column and row.
  `set grid.cell` declarations apply first. A cell's own value always wins.
- Free slots of the occupied rows become empty cells so that they are painted.
- The result is marked prepared and is not prepared again.

## Acceptance

Unit controls for preparation state; CLI controls for arrays, functions, `set grid`
and `set grid.cell` against the pinned reference.

## Evidence

Approved on 2026-11-25 (“Aprovo A–F”): `00_nucleo/diagnostics/grid-package-2026-11-25.pt-BR.md`.
