# L0 — introspect/fixpoint
Hash do Código: 312d6fdc

## Ownership and evidence

Layer: L1. Sole consumer: `01_core/src/compiler/introspect/fixpoint.rs`.
`run_fixpoint` at `:75` returns `Result<TagIntrospector, FixpointError>`;
`introspect_to_fixpoint` at `:149` is its direct wrapper.
The current L3 consumers use single-walk runtime entrypoints from
`00_nucleo/prompts/core/compiler/introspect.md`, not this full reevaluation loop.
Source call-site evidence and its reproduction are recorded in
`lab/results/construction/diagnostics/sanitization-introspection-columns-history.md`.

## API and bounded loop

Preserve MAX_FIXPOINT_ITERATIONS = 5 and these signatures:

```rust
pub fn run_fixpoint<F>(
    engine: &mut Engine<'_>,
    ctx: &mut EvalContext,
    eval_step: F,
) -> Result<TagIntrospector, FixpointError>
where
    F: FnMut(&mut Engine<'_>, &mut EvalContext) -> SourceResult<Content>;

pub fn introspect_to_fixpoint<F>(
    engine: &mut Engine<'_>,
    ctx: &mut EvalContext,
    eval_step: F,
) -> Result<TagIntrospector, FixpointError>
where
    F: FnMut(&mut Engine<'_>, &mut EvalContext) -> SourceResult<Content>;
```

Begin with an empty previous introspector and no previous tag hash. On each
bounded iteration, clone that introspector into ctx.introspector, evaluate the
closure, then walk its content with a fresh Locator, tag vector, introspector,
auto-label counter and default StyleChain; the language argument is None.
No separate CounterStateLegacy value is created or returned.

Compute the tag hash and coordinate the child-owned passes in current order:
counter functions, fallible runtime State displays, counter displays,
equation numbering, equation supplements, equation realization, and Heading
finalization before the convergence check. The ratified demand-driven State
contract below supersedes the historical eager state-function pass; readers
with Engine resolve recorded actions. Preserve the relative precedence of all
other passes.
After those passes, equality with the preceding iteration's tag hash returns
the current introspector. Otherwise retain its hash/introspector and repeat.
The first hash alone cannot establish convergence. Exhaustion returns
FixpointError::NotConverged; evaluation and fallible postprocessing errors
return FixpointError::Eval immediately, not a partial success.

## Boundaries and acceptance

Hashing is owned by `00_nucleo/prompts/core/compiler/introspect/convergence.md`;
callback semantics by `00_nucleo/prompts/core/compiler/introspect/from_tags.md`;
the causal walk by `00_nucleo/prompts/core/compiler/introspect.md`.
This helper does not independently add the parent's bibliography conversion,
outline-entry synthesis or parent-index preparation. Do not infer equivalence
of every prepared store with the production entrypoint.

Keep the linear bounded mechanism without new memoization or I/O.
Test stable successive evaluations, dependence on the preceding introspector,
bounded non-convergence, evaluation/runtime errors and wrapper equivalence.
Hash equality is this helper's mechanical stopping rule, not proof of all
language observables or authorization to substitute it for pipeline
stabilization. No promised future migration or production adoption is implied.

## Test-helper cleanup evidence and contract

The Clippy inventory identifies `01_core/src/compiler/introspect/fixpoint.rs:170`
at HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b` with an uncommitted working
tree. Sources, owners, references, UTC, status and diffstat are preserved in
`lab/results/construction/diagnostics/helpers-testes-ci-2026-10-13/before.json.gz`.
Call-site research and productive paths precede classification; this is neither
a new parity measurement nor performance evidence.

Remove only the uncalled test helper `ck`. Preserve `sel`, fixtures, tests,
assertions and the complete fixpoint algorithm. Keep productive routes, existing
test names and bodies, external APIs, defaults and phase. This is internal
continuous flow under ADR-0127. Revalidate all workspace targets with build and
check, the core suite, test inventory, Clippy and lineage. Do not remove
expectations to pass the gate.

## Heading callback evidence and responsibility

The 2026-10-17 evidence records HEAD
`b1b486f3d42431f8ea360bc5712637ee1e3d4f6c` and working-tree/UTC/hash provenance
in `lab/results/construction/diagnostics/lote-a1-callbacks-reflexao-seletores-2026-10-17/before.json.gz`.
`before-pdf-probe.json.gz` records `heading-supplement-panic` succeeding in Tekt
and producing `A1_HEADING` in vanilla
`bd5ebec285505889e8ff1d4f68804ab147a699f3`. Both accept
`heading-supplement-valid`, but only vanilla uses `Custom` in the reference.
Upstream `model/heading.rs:249–289` synthesizes supplement per occurrence;
`model/reference.rs:393–415` applies the function and display.
`Set.rs:286–320` ignores supplement and clears numbering;
`snapshot_fields` at `heading.rs:55–68` fixes translation. These are observable
callbacks/values, not required upstream mechanics.

Finalize Heading alongside the existing Engine postprocessors before checking
convergence. Propagate failure as `FixpointError::Eval`. Preserve the cap,
criterion and pass count; do not adopt the complete loop in new callers. This
parity correction follows ADR-0127 continuous flow: no new Rust API, public field,
or transfer of existing logic between eval and layout. The existing post-walk
Engine boundary applies callbacks. L0 first, RED and bilateral controls; no
callbacks in encoders, walk or layout, and no claim of parity for every field.

## Ratified demand-driven State readers

### Evidence and approval

HEAD `b1b486f3d42431f8ea360bc5712637ee1e3d4f6c`, uncommitted working tree;
UTC/status/diffstat/sources are in `before.json.gz` and `final-freeze.json.gz`
under `lab/results/construction/diagnostics/lote-a1-callbacks-reflexao-seletores-2026-10-17`.
`state-counterproof.json.gz` records bilateral RED: readers initialized with
1/10 for the same key after +1 require 2/11; an unread callback does not execute;
a reused update executes per occurrence. The pin is
`bd5ebec285505889e8ff1d4f68804ab147a699f3`, upstream
`introspection/state.rs:226–245, 419–455, 477–520`. The reader's init starts the
sequence; document order, not the Location integer, determines ordering.
The measured method treated `Func` as `Set`, and the eager postpass lost this
semantics. These are language values/errors, not Rust equality or a mandated
algorithm. The earlier reader proposal was withdrawn. The owner confirmed the
replacement A1-S2 gate on 2026-10-17 in
`lab/history/construction/records/00_nucleo/materialization/state-reader-runtime-2026-10-17/review.md`,
SHA-256 `6d6a176b8e27cc53c38fe6650c03c217448059f4e7287fd8848fe54e80247fa4`.

### Current responsibility and acceptance

Remove automatic eager application of `StateUpdate::Func`; Engine readers
resolve recorded actions. Coordinate the internal fallible display and preserve
cap=5, the hash/convergence criterion, other passes' precedence and APIs.
Tests observing functional values must use runtime reads with Engine; they must
not assume the legacy trait without Engine automatically materializes `Func`.

The gate ratifies five signatures and the read phase. L0 first, functional RED
before code; GREEN against the named pin for language and independent controls
for Tekt extensions. Do not expand other public contracts. Require workspace
build/doctests, strict Clippy/Rustdoc, fmt, V1/V5/V15/V26 and reverse lineage;
V16 must not increase. Use MemoryMax 12 GiB/High 8 GiB/swap0/jobs1, with heavy tasks
sequential. Measure consumption; representation alone proves no global gain.

## Ratified Heading finalization before queries

### Evidence and approval

HEAD `b1b486f3d42431f8ea360bc5712637ee1e3d4f6c`, uncommitted working tree;
`final-freeze.json.gz` and `target-controls.json.gz` in
`lab/results/construction/diagnostics/lote-a1-callbacks-reflexao-seletores-2026-10-17`
record UTC/diffstat/sources/argv. Pin:
`bd5ebec285505889e8ff1d4f68804ab147a699f3`.
`heading-raw-constructor-func` passes on baseline/vanilla and fails on the
candidate because transport `Styled` changes `func`. Query also observes a
temporary causal wrapper; HTML ignores a panic callback. Sources:
`walk.rs:930–945`, `pipeline/paged.rs:419–427`, `pipeline/html.rs:101–106`
prepare queries through the pure walk. These are language values/errors, not a
requirement to copy vanilla's structure or algorithm. The owner confirmed A1-H2
on 2026-10-17 in
`lab/history/construction/records/00_nucleo/materialization/heading-supplement-runtime-2026-10-17/review.md`,
SHA-256 `66739b13596013f4343bdc3ffacbdb9081151e3d9863e5d962e8ff13fd68775c`.

### Responsibility, acceptance and limits

Update only the internal Heading-finalizer call to use the snapshot's index and
pending data. Preserve the limit, convergence and separately approved removal
of eager State processing. No new API belongs to this owner.

Require H1–H8 in the ratified review, with bilateral RED before code. No public
contract beyond the Heading facade is authorized by H2; A1-S2 separately covers
five State helpers. No new public fields. Ordinary `Styled` and other callbacks
retain their boundaries. Require workspace/doctests/build/Clippy/Rustdoc, fmt,
V1/V5/V15/V26/reverse lineage and no V16 increase; use
12GiB/High 8 GiB/swap0/jobs1 with heavy commands sequential. RAM gain requires
measurement, not intention.

## State error-regression acceptance

Earlier measurements are preserved in `pre-repair-validation.json.gz`,
`complete-workspace.log`, HEAD `b1b486f3d42431f8ea360bc5712637ee1e3d4f6c`
and the working-tree/UTC/diffstat freeze above.
`fixpoint_propaga_erro_state_update_callback` expected an error without a reader,
but unread State does not execute a callback at pin
`bd5ebec285505889e8ff1d4f68804ab147a699f3` (`state-counterproof.json.gz`).
That historical expectation conflicts with the ratified reader contract.
Preserve its `FixpointError::Eval` assertion and callback message, adding an
actual `StateDisplay` read after the update to the fixture. The independent
`a1_state_unused_callback_waits_for_a_read` control must still require no
execution without a read. Do not restore the eager postpass or change the
resolver to satisfy the old expectation. This is an internal test correction,
with no new contract.


## Approved Figure/Grid occurrence counter correction

### Evidence before decision

Baseline HEAD `731f0ffdf4c7c152f590551e72ff63f7bd85ef85`; the reviewed current-pin measurements and exact source anchors are retained in `lab/results/v16-corrections-2026-10-19/figure-kind/review.md`, `agents/counter/design.md` and `agents/counter/counter-probes.json`. In that evidence, captionless/numbered Figures count; manual Set/Func actions make caption ordinal indexing incorrect; typed element kinds and custom names require distinct keys. These are language values/errors, not Rust or rendering-byte equality. The human explicitly approved the complete Figure/Grid contracts, phases and compatibility changes in the current implementation session. This approval satisfies ADR-0127 for this exact scope. Historical conflicting caption-only or productive string-projection clauses are superseded only as described here; unrelated contracts remain.

### Exclusive responsibility

Invoke the same Figure finalizer after complete iteration indexing and existing passes. Its Engine clone uses an iteration-local Sink; publish this sink only for the returned successful iteration. Preserve five-iteration cap, stopping criterion, signatures, evaluation/other callback ordering, immediate original errors. No shared cache or new transaction abstraction.

### Acceptance and limits

L0 precedes functional RED and implementation. Require independent fixed expectations for complete demanded histories, captionless/hidden/None demand controls, manual actions before/after requested Locations, empty Set state, typed aliases/homonyms/custom names, actual Paged caption/reference text, both-target explicit reads, bundle metadata rejection before effects and candidate diagnostic retention. No callback count as parity oracle. Preserve the independent filtered-read/replay/Sink algorithms and HTML implicit-rendering debt. Require exhaustive real-enum extension probes at new owned decisions, all twelve established gates, full differential lint, canonical owner paths, V1/V5/V15/V26 and reverse lineage with scoped resealing. No lint suppression, contract weakening, new defaults, memoization or public changes beyond the reviewed approval. Record HEAD/dirty file list, UTC, argv and executable identity for measurements.
