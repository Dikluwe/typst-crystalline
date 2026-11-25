# L0 — stdlib/collections/array_predicate — Array predicates and ordering
Hash do Código: 19041925

Layer: L1
Exclusive consumer: `01_core/src/compiler/stdlib/collections/array_predicate.rs`

## Evidence before decision

The uncommitted working tree captured in `lab/results/construction/diagnostics/atomizacao-stdlib-collections-2026-09-30-before.json.gz` contains this family's routines in `01_core/src/compiler/stdlib/collections.rs`; the complete L1 suite passed with 6401 tests and 2 ignored before extraction. This is structural measurement, not new bilateral vanilla parity evidence or a benchmark. The historical contract and vanilla pin qualifications remain in `00_nucleo/prompts/core/compiler/stdlib/collections.md`.

## Language contract

filter, map, find, position, any and all preserve iteration, short-circuiting and
callback invocation; predicates and sorted(by:) require Bool.

`dedup` keeps the first global occurrence, comparing values or keys with the
language equality owned by `00_nucleo/prompts/core/compiler/eval/operators/equality.md`.

`sorted` evaluates `key` once per element in order and is stable. Without `by` it
uses the language ordering owned by
`00_nucleo/prompts/core/compiler/eval/operators/ordering.md`, later element first, and
reports the first comparison failure at the argument span. With `by` the callback
receives (earlier, later) and true keeps the order. When it returns false, call it
with (later, earlier) and swap only on true: both strict and inclusive callbacks are
then stable. The reference reverses equal elements under a strict callback, an order
it documents as unspecified; that is not reproduced. An inconsistent callback or an
incomparable list never panics; every callback or comparison error is returned at
once. The result is a permutation of the input.

No second value comparison lives in this consumer. Bound-method dispatch and type
exposure remain in the parent owner. Preserve operation identity, argument
validation, its precedence and its diagnostics. No signature, default or phase change.

## Acceptance

RED→GREEN for every comparable family, Int with Decimal, nested arrays, stability
with <, <=, > and >=, key with by, large mixed and inconsistent lists, callback
errors, and dedup across Int/Float/Decimal and relative lengths, with and without
key. Preserve Decimal and Version key sorting and all existing controls.

## Evidence

Package approved on 2026-11-24 (“Aprovo A–F”):
`00_nucleo/diagnostics/value-comparison-package-2026-11-24.pt-BR.md`. The earlier Decimal and Version
corrections are in this file at commit `f2c737125`.
