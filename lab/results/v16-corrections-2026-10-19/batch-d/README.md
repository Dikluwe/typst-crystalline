# V16 batch D correction evidence

Status: twenty selected V16 findings validated and closed; 44 remain open.
This document records evidence and does not authorize productive code.
Baseline commit: `a9f03073809fa15ff11d86c532f2c803ee892e47`, branch
`codex/tekt-mature-layout`. `cases.json` and `scope.json` identify twenty findings
(029–047 and 077) across fifteen exclusive source/L0 pairs. No Nucleus changes.

## Authority and provenance

The fresh baseline ended at `2026-10-19T16:01:26.079736+00:00` and exactly
matches the accepted batch C scan: 64 V16 errors, 1,195 V19 notes and 878 V20
notes. `baseline-state.json.gz` and `baseline.sarif.gz` preserve its state and
result. All measurements on an uncommitted tree include UTC, checkpoint, exact
changed-file stat and input hashes. A version string is never binary identity.

The pinned linter is commit `12797073972e375d9ab078cf6750c8b30b71e2b9`, executable
SHA-256 `421d0797e7ae1a5102e9c5bcd87b488fea70a1997394c38765a03239fe579121`.
Language probes use vanilla commit `bd5ebec285505889e8ff1d4f68804ab147a699f3`, tree
`cac836c07e4dec2a89219370428292393a3216b5`, executable SHA-256
`9b2373155e9c05bbb0daf5755bf4be88a11e64bd6b4f8b376648fa27303373b9`.
Its verified recipe is retained under
`../../construction/diagnostics/vanilla-resync-2026-10-17/relatorio.md`.
The baseline product is the batch C build, SHA-256
`7c4369a2c6b7e5c9fd7da6ab007aa9707867ba7d811ee6a61fa81c0b819198e5`;
batch C's validation/build/replay receipts bind it to the committed sources.
It is not a build of concurrent batch D edits.

Three agents worked on disjoint grid, layout and measurement owners, with
cross-review. L0 updates and sealed tests preceded implementation. These are
internal classification and measured parity corrections under ADR-0127's
continuous flow. Public signatures, defaults, pipeline phases, dependencies,
linter rules, severities and scan configuration remain unchanged.

## Tests before implementation

`preimplementation-final.json` and `.patch.gz` freeze the final test/L0 state.
`baseline-final/` records 27 new controls: 24 preservation passes and three
semantic RED failures. `baseline-test-classification.json` names each result.
The RED failures demonstrate lost visible image extent, lost constrained
dimensions through transport, and zero positive pure-math measurement. A loop
stops at its first failed assertion: this receipt does not claim independent
RED for every case inside a test. Separate bilateral CLI probes establish the
image, equation, figure and wrapper losses.

Earlier attempts are retained and excluded from semantic evidence. The tracks
fixture's `&raw` binding caused a parser failure; `preseal-parser-attempt.json`
explicitly describes the later-header reconstruction limit. A missing test
Introspector import and a call to a private grid helper caused E0405/E0624 in
`baseline-run/`. Test-only corrections avoided widening production visibility.
Initial SVG-image input failed in product; relative-path axis probes hit the
product sandbox. Valid PNG and absolute-path replacements are identified in
the measurement report. None of these infrastructure failures counts as RED.

## Corrections and preserved boundaries

Grid classification keeps raw-cell inheritance distinct from authoritative
prepared-cell values, including resolved absence. Placement-neutral content is
not permission to cache: token, capture and take guards remain independent.
Guard refusal remains refusal; cache misses still render. Nested Table remains
an atomic projection barrier. Table document order and grid explicit-first
placement retain their distinct overflow/error behavior and causal spans.
Insets retain their internal Length/partial-dictionary schema and atomic
rejection, without inventing new source casts.

Layout classification keeps line-number admission, suppression, deferred pattern
emission and callback requests separate. Layout does not execute the callback.
Source/Located dispatch retains one locator event; quote observation stays
separate from full dispatch. Alignment decisions cover both finite axes and
optional float alignment. CounterKey, Selector and ElementKind supplements keep
their existing labels and refusal behavior; nested decisions are independently
checked rather than hidden behind another wildcard.

Constrained image measurement reuses existing dimension calculation and the
injected image sizer, preserving occupied target bounds. Captionless Figure
delegates to body measurement. Finite-positive-width inline Equation can reuse
isolated subframe layout only after a direct recursive positive certificate of
its simple math body. It inspects Source/Located bodies and Realized.visual,
never consumed originals. Effects, references, ShowRecipe, metadata and
unaudited mathematical families are rejected. Generic visitors were unsuitable:
they can skip transport roots. The certificate has its own evolution witness.
Independent review confirms subframes block ambient line-number requests and
do not share the emitter's callback passes.

The legacy helper remains a bounded last-resort projection after real transform
layout. Unsupported projections are documented as unresolved approximations,
not as language objects with semantically zero extent. `open-obligations.json`
retains required behavior, observations and refuters for captioned/numbered
figures, block/contextual/uncertified math, image axes and diagnostics, SVG input,
nested-table origin, and missing intermediate contextual-cell output.

## Proof and validation method

`../workers/batch_d_structural.py` extends real enum declarations in disposable
complete workspace copies, using actual locked dependencies. Baseline uses the
immutable pre-edit workspace. Selected decisions require a primary E0004 at the
exact file, line and byte span; unrelated errors do not count. Twenty original
targets become twenty-three with the nested supplement decisions and new math
certificate. Three additional HAlign experiments independently isolate new/old,
old/new and new/new pairs. The original baseline has no selected witness.

Grid replay compiles eighteen cases in both binaries, requires successful text
and geometry extraction, and preserves the baseline product observations. It
does not claim all old grid cases match vanilla: contextual intermediate output,
extraction order and small existing geometry differences stay explicit. The
nine selected 100-cell corpus variants are correctness probes, not a runtime
benchmark or certification of larger corpus sizes.

Measurement replay freezes source, binary, runner and build-receipt hashes.
It requires successful compilation and exactly one red next-column marker in
each width probe, transforming local SVG coordinates through ancestor matrices.
Transform probes require the expected painted geometry or explicit absence.
Four source assertions verify that contextual state updates occur once.
Matching next-column x does not certify every y coordinate or complete render
parity. Earlier weaker replay instrumentation remains historical, not accepted.

Heavy commands run serially, offline and locked, with one Cargo job, 12 GiB
maximum, 8 GiB high threshold and zero swap. Actual cgroup limits, peak and OOM
counters are recorded. After copied-workspace mutation experiments, the four
product packages are cleaned before full validation in the real checkout. The
twelve gates include paths, strict forward and reverse lineage, ownership and
Nuclei, builds, domain/infra/wiring tests, doctests, all targets including lab
benches, strict Clippy, formatting and Rustdoc. Source/L0 inputs must remain
frozen throughout. The full linter is expected to remain nonzero for other
batches; structural exhaustiveness does not certify complete language parity.

## Acceptance receipts

The candidate core suite passed 7,512 tests with five preexisting ignored and
no failures; all 27 new tests passed. Eighteen bilateral grid cases preserved
the baseline product observations. All 27 measurement checks passed, including
four effect-once source assertions. Source, binary and build-receipt freezes
remain recorded with each replay. The scoped Auto image/figure origins are now
40pt, and the simple inline equation origin is 23.124444444pt, equal to the
pinned oracle in these fixtures.

The full pinned scan ended at `2026-10-19T16:19:05.400082+00:00`: **64 → 44 V16**,
exactly the selected twenty removed, with no new error or warning and identical
diagnostics outside the fifteen affected consumers. V19 is 1,222 and V20 is
876. `agents/grid/note-review.md` and `.json` distinguish line moves, equivalent
Value-path spelling and substantive notes. No V19 content disappeared; three
nested V20 patterns became separate finite matches with their own proof sites.
Notes remain enabled and do not substitute for semantic coverage.

The complete candidate run finished at `2026-10-19T16:27:01.146586+00:00`
with unchanged frozen inputs and no OOM event. All twenty-three decision
witnesses and three isolated HAlign tuple witnesses passed. The independent
`structural-witness-review.json` and `.md` reparse raw compiler messages and
actual archived mutation bytes; they confirm the twenty original baseline
absences and three tuple absences separately from new helper decisions.
`agents/measurement/candidate-replay-review.json` and `.md` independently
recompute SVG geometry, verify body presence and distinguish grid preservation
from unresolved upstream differences.

All twelve final gates completed at `2026-10-19T16:40:10.165167+00:00`, with
2,131 unchanged frozen inputs. The four-package clean preceded the actual
checkout rebuild and full tests. Core reports 7,512 passed and five preexisting
ignored; infrastructure reports 1,377 passed and two preexisting ignored.
Every added test is observed passing in the fresh full run; none was ignored.
`validation/` preserves command logs, exact input hashes and uncommitted state.

Both accepted replays completed at `2026-10-19T16:40:40.910463+00:00`:
all eighteen grid cases preserve baseline product observations, and all
twenty-seven measurement cases pass against the fixed oracle. The four outer
jobs and their memory/freeze receipts are in `validation-run/`. Each replay
binds its product bytes to `validation/validation-state.json.gz` and verifies
source/binary stability. The measured outcomes do not close the eight explicit
obligations in `open-obligations.json`.

`../workers/batch_d_close.py` verified the actual receipts before emitting
`closure-ledger.json` and `new-test-execution.json`. The ledger binds each of the
twenty original IDs to its canonical owner, source/owner hashes, prompt seal
and exact structural witness. It also checks independent reviews, all retained
open-obligation evidence hashes, and absence of new suppressions or ignored
tests. No rule, severity, exception or scan-scope change was used.

The full pinned linter still exits 1 for 44 V16 errors in the remaining batches.
Post-document path validation and the evidence manifest record the final
reviewable package. This closure is local to batch D, not complete language
parity or a zero-violation repository.

One automatic permission review timed out before Git staging. The permitted
single retry succeeded. This was not a test failure or a security finding.
