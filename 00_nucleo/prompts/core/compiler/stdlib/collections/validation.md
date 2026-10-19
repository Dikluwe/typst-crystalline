# L0 — stdlib/collections/validation — Shared validation and casts
Hash do Código: 229b2653

Layer: L1
Exclusive consumer: `01_core/src/compiler/stdlib/collections/validation.rs`

## Evidence before decision

The uncommitted working tree captured in `lab/results/construction/diagnostics/atomizacao-stdlib-collections-2026-09-30-before.json.gz` contains this family's routines in `01_core/src/compiler/stdlib/collections.rs`; the complete L1 suite passed with 6401 tests and 2 ignored before extraction. This is structural measurement, not new bilateral vanilla parity evidence or a benchmark. The historical contract and vanilla pin qualifications remain in `00_nucleo/prompts/core/compiler/stdlib/collections.md`.

## Language contract

ensure_no_args, array_no_args, dict_no_args, str_no_args, optional_default and expect_* casts preserve the first error, extra-argument rejection and Args spans. type_as_callable preserves a type function as a callable value.

Bound-method dispatch and type exposure remain in the parent owner; both must call this behavior without algorithm duplication. Preserve operation identity, syntax, morphology, errors, spans and validation precedence of the measured implementation. Internal extraction preserves the public contract, default behavior, pipeline phase and compatibility.

## Acceptance

The L1 suite and `crystalline-lint .` must remain regression-free. Vanilla comparison must measure semantics, syntax and morphology under ADR-0107/0108 without assuming mechanical equality. Ownership remains 1:1 under ADR-0129/0130.


## Unused internal helper boundary

### Evidence before decision

compiler/stdlib/collections/validation.rs:140 contains the compiler-flagged
unused/visibility item. Before-edit usage research and active dispatchers are
preserved in
`lab/results/construction/diagnostics/caminhos-residuais-ci-2026-10-13/before.json.gz`,
with sources/L0, HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, uncommitted
working tree, UTC, status and diffstat. This is not new bilateral parity evidence.

### Contract and acceptance

Remove only the uncalled internal expect_one_array helper. Preserve active validators and collection-method error precedence.

This ADR-0127 internal continuous-flow correction introduces no public signature,
product default or phase. Preserve language results and test expectations.
Revalidate all targets, core suite, build, Clippy and lineage; do not substitute
warning suppression for the correction.
