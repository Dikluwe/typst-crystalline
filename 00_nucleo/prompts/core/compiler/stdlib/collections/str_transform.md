# L0 — stdlib/collections/str_transform — String transformations
Hash do Código: 55095eee

Layer: L1
Exclusive consumer: `01_core/src/compiler/stdlib/collections/str_transform.rs`

## Evidence before decision

The uncommitted working tree captured in `lab/results/construction/diagnostics/atomizacao-stdlib-collections-2026-09-30-before.json.gz` contains this family's routines in `01_core/src/compiler/stdlib/collections.rs`; the complete L1 suite passed with 6401 tests and 2 ignored before extraction. This is structural measurement, not new bilateral vanilla parity evidence or a benchmark. The historical contract and vanilla pin qualifications remain in `00_nucleo/prompts/core/compiler/stdlib/collections.md`.

## Language contract

normalize, replace, trim, split, repeat, to-upper, to-lower, to-unicode and rev preserve defaults, pattern validation, Unicode forms and language results.

Bound-method dispatch and type exposure remain in the parent owner; both must call this behavior without algorithm duplication. Preserve operation identity, syntax, morphology, errors, spans and validation precedence of the measured implementation. Internal extraction preserves the public contract, default behavior, pipeline phase and compatibility.

## Acceptance

The L1 suite and `crystalline-lint .` must remain regression-free. Vanilla comparison must measure semantics, syntax and morphology under ADR-0107/0108 without assuming mechanical equality. Ownership remains 1:1 under ADR-0129/0130.


## Unused internal helper boundary

### Evidence before decision

compiler/stdlib/collections/str_transform.rs:72 contains the compiler-flagged
unused/visibility item. Before-edit usage research and active dispatchers are
preserved in
`lab/results/construction/diagnostics/caminhos-residuais-ci-2026-10-13/before.json.gz`,
with sources/L0, HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, uncommitted
working tree, UTC, status and diffstat. This is not new bilateral parity evidence.

### Contract and acceptance

Remove only the uncalled internal str_trim helper. trim still uses str_trim_dispatch with existing defaults, patterns, at/repeat and diagnostics.

This ADR-0127 internal continuous-flow correction introduces no public signature,
product default or phase. Preserve language results and test expectations.
Revalidate all targets, core suite, build, Clippy and lineage; do not substitute
warning suppression for the correction.
