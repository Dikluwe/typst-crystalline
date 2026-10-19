# L0 — stdlib/collections/arguments — Arguments methods
Hash do Código: 423d9ed5

Layer: L1
Exclusive consumer: `01_core/src/compiler/stdlib/collections/arguments.rs`

## Evidence before decision

The uncommitted working tree captured in `lab/results/construction/diagnostics/atomizacao-stdlib-collections-2026-09-30-before.json.gz` contains this family's routines in `01_core/src/compiler/stdlib/collections.rs`; the complete L1 suite passed with 6401 tests and 2 ignored before extraction. This is structural measurement, not new bilateral vanilla parity evidence or a benchmark. The historical contract and vanilla pin qualifications remain in `00_nucleo/prompts/core/compiler/stdlib/collections.md`.

## Language contract

len, pos, named, at, filter and map operate on the causal occurrence sequence; preserve legal duplicate names, spans, order, one invocation per occurrence and detachment of value_span in map.

Bound-method dispatch and type exposure remain in the parent owner; both must call this behavior without algorithm duplication. Preserve operation identity, syntax, morphology, errors, spans and validation precedence of the measured implementation. Internal extraction preserves the public contract, default behavior, pipeline phase and compatibility.

## Acceptance

The L1 suite and `crystalline-lint .` must remain regression-free. Vanilla comparison must measure semantics, syntax and morphology under ADR-0107/0108 without assuming mechanical equality. Ownership remains 1:1 under ADR-0129/0130.
