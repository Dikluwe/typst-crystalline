# V16 batch B correction evidence

Status: batch B validated and closed; eight findings resolved and 83 V16 errors remain.
Implementation checkpoint: `059d1a0ffa9523dbe21af6860a215acaeef55d73`, branch
`codex/tekt-mature-layout`. The eight IDs and exclusive source/L0 pairs are in
`scope.json`. This is diagnostic evidence, not an implementation specification.

## Baseline and source authority

`baseline-state.json.gz` records the fresh pinned-linter measurement, checkpoint,
UTC, working-tree state, selected source/owner hashes and baseline archive
identity. `baseline.sarif.gz` has 91 V16 errors, 1,157 V19 notes and 878 V20 notes.
Linter SHA-256 remains
`421d0797e7ae1a5102e9c5bcd87b488fea70a1997394c38765a03239fe579121`, from pinned
commit `12797073972e375d9ab078cf6750c8b30b71e2b9`. Neither the rule nor its scan,
severity, configuration, exclusions or exceptions is changed by this batch.

Three agents own disjoint source/L0 pairs; the coordinator owns resealing,
compiler experiments, integration and final validation. They audited actual
callers, fixture boundaries, all current Content variants and full L0 contracts
before classification. Their English reviews are under `agents/`.

## Runtime preservation before implementation

`preimplementation.patch.gz` and its JSON receipt preserve the first L0-first
test-only state. All six jobs in `preservation-before/` passed with frozen inputs:
new controls, existing mathematical evaluation cases, rules tests, content tests,
the infrastructure observer controls and the unchanged real-font geometry case.

The temporary equation-tree instrument explicitly inspected causal carriers;
Debug and elem_name alone cannot establish their absence. The four required
equations occur under Sequence and two Styled wrappers. No Source, ShowRecipe,
Located or Realized carrier appears on these observed equation paths. Opaque
semantic leaves were not searched. This does not establish global carrier
absence or authorize a general equation search. The original four-equation,
third-body ascent and 0.001pt tolerance assertions pass unchanged. The bounded
observer remains appropriate for this measured fixture.

The instrument's nonempty assertion was only a measurement aid, not a useful
permanent regression test. Its exact implementation and output remain in the
first patch and `preservation-before/new-infra.log`; it was removed after the
measurement. Three meaningful equation-observer controls remain.

Cross-review found a possible vacuous success in the new storage test: deriving
expected reuse from the callback's own replacement count could accept traversal
that missed every child. The final test independently identifies MathPrimes and
MathAlignPoint as the only childless fixtures. Every other fixture must visit and
replace a seed and preserve a nonempty unchanged-sibling witness. This strengthens
the stated contract without changing product behavior or an old expectation.

`preimplementation-final.patch.gz` and its JSON receipt preserve the revised,
sealed baseline: 24 new tests and one expanded existing traversal control, with
all eight original outcome classifications retained (the equation helper is extracted
without changing its decision). `preservation-before-final/` revalidates all new
controls and original geometry. No old test was removed. `new-tests.json` is the
explicit inventory; earlier instrumentation is not counted as a delivered test.

## Structural acceptance

`../workers/batch_b_structural.py` compiles complete copied workspaces using the
actual locked dependencies, original declarations and real test targets. The
baseline includes the final new controls and unchanged selected decisions.
Each phase retains complete original Rust sources, input and fixture hashes,
exact mutations, commands, timestamps and full compiler JSON.

For the seven L1 test decisions, a sentinel is added only to the copied Content
enum. Only E0004 at the selected match's exact file, line and byte span counts.
Unrelated exhaustiveness errors are expected but cannot satisfy an obligation.
The storage tuple has separate new/old, old/new and new/new experiments whose
temporary arms cover the other pair classes; neither axis may stay open.

Changing Content also prevents L1 dependency metadata from reaching the L3 test.
The infrastructure owner therefore permits the established paired arm-omission
proof: remove the same unique Styled arm in both real copied observers, keeping
all declarations and dependency types unchanged. The old wildcard absorbs the
omission; the repaired exhaustive observer must report E0004 at that decision.
This evidence is separate from runtime preservation and does not introduce a
productive sentinel, stub, new public API or permissive default.

## Validation environment and evidence boundaries

Heavy commands run serially in a local service with one Cargo job, a 12 GiB
maximum, an 8 GiB high threshold and zero swap. Runner receipts read the actual
cgroup settings and peak/events. Commands use offline locked dependencies.
The first permission review for the final baseline expired before launch; the
single retry launched successfully. That timeout is not a failed test run.

Copied-workspace experiments share the existing product target cache. Before
final integrated acceptance, clean only the four product workspace packages and
build/test the real checkout afresh. The oracle and dependency cache are retained.
Accepted final receipts must show real-checkout compilation, all delivered tests,
strict paths and forward/reverse lineage, ownership/Nuclei, full builds/tests,
all targets including lab benches, Clippy, formatting and Rustdoc, with frozen
inputs. Full lint must remove exactly these eight V16 findings; unrelated errors,
warnings and out-of-scope notes must remain unchanged. Grouped-arm informational
notes require substantive review and remain visible.

These checks concern test observers and explicit enum decisions. They do not
certify broader Typst parity, universal Content identity, all carrier traversal,
or a runtime/development-speed gain. Remaining batches and earlier recorded
parser-intent/font-fixture obligations remain separate.

## Accepted closure

All six candidate preservation jobs and all five structural experiments passed.
The seven core decisions and the infrastructure decision have exact compiler
witnesses; the storage pair has independent witnesses for both mixed directions
and the new/new case. `structural-witness-review.json` and the read-only review
in `agents/final-cross-review.md` retain the independent attribution audit.

The final twelve integrated gates completed at **2026-10-19T12:16:50.708042+00:00**.
The 2130 frozen inputs did not change. Full core-library results are
7,433 passed / five existing ignored; infrastructure-library results are
1,377 passed / two existing ignored. All 24 delivered new tests and the expanded
existing callback control are observed passing in the fresh full run;
`new-test-execution.json` binds them to the exact log. No new ignore was added.
Wiring, integration tests, doctests, all targets, strict Clippy, formatting and
Rustdoc also pass. Complete commands, provenance and freeze checks are in
`validation/` and the enclosing `candidate-run/` receipt.

The worker's recorded memory peak is 5516689408 bytes, with zero
OOM events. These resource readings are validation provenance, not a product
performance measurement. `closure-ledger.json` binds all eight IDs to the accepted
source/L0 digests. The complete pinned linter still exits 1 for the other 83 V16
errors; this is a batch closure, not an overall zero-violation result. The full
diagnostic and retained-note review are in `lint-delta.json`,
`note-content-delta.json` and `note-review.md`.
