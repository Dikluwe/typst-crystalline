# L0 — stdlib/collections/array_sequence — Array sequence transformations
Hash do Código: f9130f0c

Layer: L1
Exclusive consumer: `01_core/src/compiler/stdlib/collections/array_sequence.rs`

## Evidence before decision

The uncommitted working tree captured in `lab/results/construction/diagnostics/atomizacao-stdlib-collections-2026-09-30-before.json.gz` contains this family's routines in `01_core/src/compiler/stdlib/collections.rs`; the complete L1 suite passed with 6401 tests and 2 ignored before extraction. This is structural measurement, not new bilateral vanilla parity evidence or a benchmark. The historical contract and vanilla pin qualifications remain in `00_nucleo/prompts/core/compiler/stdlib/collections.md`.

## Language contract

rev, zip, enumerate, chunks, windows, flatten, join, intersperse, split and to-dict preserve order, bounds, defaults, result shape and the exact/truncation criteria specified by the parent L0.

Bound-method dispatch and type exposure remain in the parent owner; both must call this behavior without algorithm duplication. Preserve operation identity, syntax, morphology, errors, spans and validation precedence of the measured implementation. Internal extraction preserves the public contract, default behavior, pipeline phase and compatibility.

## Acceptance

The L1 suite and `crystalline-lint .` must remain regression-free. Vanilla comparison must measure semantics, syntax and morphology under ADR-0107/0108 without assuming mechanical equality. Ownership remains 1:1 under ADR-0129/0130.

## Value equality

`split` compares elements with the language equality owned by
`00_nucleo/prompts/core/compiler/eval/operators/equality.md`, never Rust equality.
Measured and approved in
`00_nucleo/diagnostics/value-comparison-package-2026-11-24.pt-BR.md`.
