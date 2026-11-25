# L0 — stdlib/layout/box_elem — box
Hash do Código: adb14139

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/set/declared-properties.toml sha256:b4b40ae19e8c22e62a22af00459e257fa4a6c9d8c518d101cd0798cbf4f97744

Layer: L1
Exclusive consumer: `01_core/src/compiler/stdlib/layout/box_elem.rs`

## Evidence before decision

The uncommitted working tree captured in `lab/results/construction/diagnostics/atomizacao-stdlib-layout-2026-09-30-before.json.gz` contains this family's logic in `01_core/src/compiler/stdlib/layout.rs`; the complete L1 suite passed with 6401 tests and 2 ignored before extraction. This is structural measurement, not a benchmark or new bilateral vanilla parity evidence. Historical decisions, measurements and pin qualifications remain in `00_nucleo/prompts/core/compiler/stdlib/layout.md`.

## Language contract

box preserves sizing, width_fraction, height, inset/outset, baseline, radius, fill, clip and stroke with existing defaults. width_fraction provenance is retained in the archived original for this owner.

Extraction preserves signatures, function identity, syntax, morphology, errors and validation precedence. The hub and registry continue to forward calls statically without duplicating algorithms. Preserve public contracts, default behavior and pipeline phase.

## Acceptance

The L1 suite and `crystalline-lint .` must remain regression-free. Parity is at the language level under ADR-0107/0108; mechanics may diverge. Preserve exclusive ownership under ADR-0129/0130.

## Approved shapes package

Approved on 2026-11-24 (“Aprovo A–F”): `00_nucleo/diagnostics/shapes-package-2026-11-24.pt-BR.md`.
Parse through one function shared by the constructor and declarations. Keep relative
parts of width, height, inset, outset and baseline, an aligned baseline, per-side and
dictionary strokes, and the lexical named arguments. `auto` sizes equal omission.
Gradient and tiling paint stay rejected here and warn in set. Own the pure projection
that applies declarations and resolves relative parts against given region sizes.
