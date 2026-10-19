# L0 — stdlib/layout/place — place and flush
Hash do Código: 498810d7

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/place-coordinate-origin.toml sha256:b9012d07e2d22b7c3984963a6d4e90eedbb9b925280f4136f8db8aaa9459bbc8

Layer: L1
Exclusive consumer: `01_core/src/compiler/stdlib/layout/place.rs`

## Evidence before decision

The uncommitted working tree captured in `lab/results/construction/diagnostics/atomizacao-stdlib-layout-2026-09-30-before.json.gz` contains this family's logic in `01_core/src/compiler/stdlib/layout.rs`; the complete L1 suite passed with 6401 tests and 2 ignored before extraction. This is structural measurement, not a benchmark or new bilateral vanilla parity evidence. Historical decisions, measurements and pin qualifications remain in `00_nucleo/prompts/core/compiler/stdlib/layout.md`.

## Language contract

place preserves alignment, dx/dy, scope, float and relative clearance default 1.5em. flush rejects every argument and produces Content::flush without layout.

Extraction preserves signatures, function identity, syntax, morphology, errors and validation precedence. The hub and registry continue to forward calls statically without duplicating algorithms. Preserve public contracts, default behavior and pipeline phase.

## Acceptance

The L1 suite and `crystalline-lint .` must remain regression-free. Parity is at the language level under ADR-0107/0108; mechanics may diverge. Preserve exclusive ownership under ADR-0129/0130.

## Preserve vertical omission

### Evidence before decision

compiler/stdlib/layout/place.rs:42–48 converts omission into Some(Top).
`lab/results/construction/diagnostics/place-origem-local-2026-10-14/{before,extra-before}/results.json.gz`
retains HEAD, dirty tree/diffstat, UTC, binaries and SVGs against historical pin
`9f2b6e8715237cb086899a42873660fe744622e8`: after text A, omitted alignment anchors
at baseline while explicit Top anchors in the region. Historical upstream
flow/distribute.rs:703–715 distinguishes explicit axis from flow offset. This is
language geometry, not Rust equality. The tests
place_origin_pending_line_and_explicit_top_are_distinct reproduce the divergence
before correction.

### Contract and acceptance

Use the existing Option to preserve v=None for omitted vertical axis; retain
explicit alignment. Add no field or signature. ADR-0127 internal parity flow
introduces no mode/flag/phase. Layout chooses the anchor under the pinned
place-coordinate-origin nucleus. Preserve clearance/scope/validation/floats and
validate pending-line, explicit-Top, geometry, float and full-suite controls.
The separate anchor-after-committed-flow work remains open in the local-origin
diagnostic; this does not close it.
