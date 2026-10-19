# L0 — stdlib/collections/validation — Shared validation and casts
Hash do Código: b1d888f8

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

## Exhaustive higher-order constructor admission

### Measured evidence

At baseline HEAD `91ea6ee911`, `compiler/stdlib/collections/validation.rs:91-103`
admits nine Type constructors while `compiler/eval/call_dispatch/invoke.rs:110-148`
already admits additional native constructors. The frozen bilateral receipt
`lab/results/v16-corrections-2026-10-19/batch-f/agents/adapters/baseline.json`
records exact HEAD, UTC, working-tree diff and binary identities against pinned
upstream `bd5ebec285505889e8ff1d4f68804ab147a699f3`. Decimal, Regex, Selector,
Stroke, Tiling, Version, Label, Array(Bytes), Arguments and SpotColorant enter
existing constructors when supplied as higher-order functions upstream; Duration
also admits the constructor on an empty map and preserves its own argument errors
when invoked. These are language-admission observations, not constructor-mechanics
requirements. Native duration diagnostics and tiling representation already differ
in direct calls; extending admission does not certify those independent contracts.

### Contract and acceptance

Preserve the existing nine constructor identities and add the existing native
Decimal, Duration, Regex, Selector, Stroke, Tiling, Version, Label, Array(Bytes)
and SpotColorant callbacks with their native names. Adapt Arguments privately to
the same native ABI by returning the original cloned Args value, preserving all
positional and named data and occurrence spans. Do not execute a constructor while
converting Type to Func. Collection map/filter/key/by must receive the same native
function and forward constructor results, effects and diagnostics unchanged.
Array remains the existing Bytes-only conversion, not new general Array casting.

Explicitly classify every remaining Type as unsupported by this adapter, not
intrinsically non-callable. In particular Path has an observed higher-order
filesystem-context error upstream, while Dictionary has constructor behavior not
yet admitted by Tekt's main dispatcher; both remain separate measured obligations.
No invented callback, filesystem access, public signature, phase, or new default
is authorized. Use an exhaustive Type match without wildcard suppression. Require
semantic RED→GREEN for newly admitted constructors and preservation controls for
existing constructors, unsupported types, aliases, errors, empty input, sorted key
and predicate result validation; a new Type must require a decision at this match.
This is ADR-0127 continuous flow for measured mapping-table/parity corrections.


### Sorted-key constructor integration regression

The failed accepted replay archived in
`lab/results/v16-corrections-2026-10-19/batch-f/agents/adapters/version-ordering-repair/before.json`
pins the source/owner state and shows that Version admission reaches a missing
comparison branch. Keep the existing Decimal sorted-key assertion and add
`(10, 2).sorted(key: version) == (2, 10)` to that same test function. The frozen
oracle expectation remains unchanged. Comparison belongs exclusively to
`00_nucleo/prompts/core/compiler/stdlib/collections/array_predicate.md`; this
owner adds integration acceptance without duplicating sorting or conversion.
Require focused semantic RED before the comparator correction and exact replay
afterward. This does not change constructor identity, admission or diagnostics.
