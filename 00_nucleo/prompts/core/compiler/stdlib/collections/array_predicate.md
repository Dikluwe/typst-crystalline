# L0 — stdlib/collections/array_predicate — Array predicates and ordering
Hash do Código: 990b8ae1

Layer: L1
Exclusive consumer: `01_core/src/compiler/stdlib/collections/array_predicate.rs`

## Evidence before decision

The uncommitted working tree captured in `lab/results/construction/diagnostics/atomizacao-stdlib-collections-2026-09-30-before.json.gz` contains this family's routines in `01_core/src/compiler/stdlib/collections.rs`; the complete L1 suite passed with 6401 tests and 2 ignored before extraction. This is structural measurement, not new bilateral vanilla parity evidence or a benchmark. The historical contract and vanilla pin qualifications remain in `00_nucleo/prompts/core/compiler/stdlib/collections.md`.

## Language contract

filter, map, find, position, any, all and dedup preserve iteration, short-circuiting and callback invocation; predicates and sorted(by:) require Bool; dedup retains the first global occurrence and sorted preserves stability with key/by.

Bound-method dispatch and type exposure remain in the parent owner; both must call this behavior without algorithm duplication. Preserve operation identity, syntax, morphology, errors, spans and validation precedence of the measured implementation. Internal extraction preserves the public contract, default behavior, pipeline phase and compatibility.

## Acceptance

The L1 suite and `crystalline-lint .` must remain regression-free. Vanilla comparison must measure semantics, syntax and morphology under ADR-0107/0108 without assuming mechanical equality. Ownership remains 1:1 under ADR-0129/0130.


## Decimal keys admitted by higher-order constructor conversion

### Evidence before decision

At HEAD `91ea6ee91105b97b1aedf172e34780dddde14c94`, uncommitted working tree,
`lab/results/v16-corrections-2026-10-19/batch-f/agents/adapters/decimal-ordering-repair.json`
pins the exact source/owner hashes, UTC, changed-file stat and candidate test log.
The existing `batch_f_higher_order_sorted_key` language test fails with
`cannot compare decimal and decimal` after Decimal constructor admission.
This owner's `value_cmp` at line 365 currently recognizes only Int/Float/Str.
By contrast, the existing domain comparison at
`01_core/src/compiler/eval/operators/ordering.rs:118` compares homogeneous
Decimal exactly. Pinned vanilla `foundations/ops.rs:477` does the same;
`agents/adapters/baseline.json` under that batch retains the bilateral sorted-key
probe, pinned binary/source identity and oracle result. This is a language
ordering correction, not a requirement to copy sorting mechanics.

### Exclusive correction and acceptance

Add exact homogeneous Decimal comparison to this private collection comparator,
using the existing decimal domain ordering without a floating-point conversion.
Preserve every existing numeric/string branch, unsupported-pair diagnostic and
operand order, stable sorting, callback count and key/by validation precedence.
The already failing sorted(key: decimal) assertion remains unchanged and must
pass after the correction. Do not expand mixed numeric coercions or claim all
collection ordering parity; other omitted comparable families remain separate
obligations. No public signature, default, phase or entity changes are authorized.
L0 precedes code; parent integration reseals this exact owner and validates
focused/workspace tests, paths and lineage under the same batch gates.


## Version key ordering after constructor admission

### Evidence before decision

At HEAD `91ea6ee91105b97b1aedf172e34780dddde14c94`, uncommitted working tree,
`lab/results/v16-corrections-2026-10-19/batch-f/agents/adapters/version-ordering-repair/before.json`
records UTC, changed-file stat, source/owner snapshots and the failed accepted
replay. Frozen `baseline.json::version-key` requires `(10, 2).sorted(key: version)`
to return `(2, 10)`, matching the pinned oracle. The candidate instead reports
`cannot compare version and version`; the frozen expectation is unchanged.
`entities/version.rs:161-178` already defines total component-wise zero-padded
ordering, and `compiler/eval/operators/ordering.rs:126-128` uses that relation.
Pinned `foundations/ops.rs:483` also compares homogeneous Version values.
This missing collection branch is language ordering, not sorting mechanics.

### Bounded correction and acceptance

Add only homogeneous Version comparison using the existing domain ordering.
Preserve trailing-zero semantics, existing Decimal/Int/Float/Str branches, stable
key sorting, all validation/callback behavior and unsupported-pair diagnostics.
The existing higher-order sorted-key test must retain Decimal and additionally
assert the frozen Version result; parent must observe that Version assertion
fail before implementation. No expected-result relaxation or sorting by repr.
Duration, Array and other omitted comparison families are outside this correction
and remain separate obligations. No public contract, default, phase or domain
Version change is authorized. Reseal and revalidate this exact owner only through
the coordinated parent gates.
