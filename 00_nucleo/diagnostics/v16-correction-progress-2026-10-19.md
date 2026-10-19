# V16 correction progress

Status: batches A–E are closed; batch F has 17 validated repairs and one
explicit pending item (Figure V16-055). The remaining 20 V16 findings remain open in the [cause-correction plan](v16-cause-correction-plan-2026-10-19.md).
This diagnostic is not a new L0 contract or a claim of complete Typst parity.

## Batch A: evidence before closure

The baseline is commit `841c9855f8ad6695f4e140d1ee4a33e4f4a46689` on
`codex/tekt-mature-layout`. The final lint measurement ended at
2026-10-19T10:52:22Z on the uncommitted implementation. Its exact changed-file
list, source/L0 digests, command and timestamps are in
[the lint receipt](../../lab/results/v16-corrections-2026-10-19/batch-a/lint-state.json.gz).
The pinned linter is unchanged: commit
`12797073972e375d9ab078cf6750c8b30b71e2b9`, executable SHA-256
`421d0797e7ae1a5102e9c5bcd87b488fea70a1997394c38765a03239fe579121`.

It reports **105 → 91 V16 errors**. Exactly the 14 selected occurrences disappear;
there is no new error or warning. Outside the eleven affected consumers, all
messages and locations are identical. V19 notes change from 1,140 to 1,157 and
V20 from 879 to 878. [The substantive review](../../lab/results/v16-corrections-2026-10-19/batch-a/note-review.md)
retains the grouped-arm coverage reminders and their limits; no severity,
suppression, scan scope or linter change was used.

## Batch A: implementation

The [scope ledger](../../lab/results/v16-corrections-2026-10-19/batch-a/scope.json)
contains 048, 071–073, 078–082, 086, 087, 090, 104 and 105: 14 findings in
11 exclusive source/L0 pairs. Three agents handled disjoint owners; integration,
resealing and validation stayed centralized. Owning prompts were updated first.
The changes classify existing token, value, color, syntax-node, PDF, font-format
and JSON alternatives explicitly, preserving rejection, delegation, recursion,
identity, errors and numeric behavior. No public contract, default, dependency,
pipeline phase or compatibility surface changes.

Behavior controls pass against the original implementations. The structural
baseline lacks E0004 at the selected decisions; repaired copies produce it at
all 15 sites, with Pattern and its nested Expr measured separately. Additional
NodeKind probes independently isolate new/old, old/new and new/new, with exact
compiler witnesses. An extra PDF control exercises all nine continuations at
the collector root and retains the nested-table integration. These are actual
workspace copies and real locked dependencies, never substitute implementations.

[Thirty-one added tests](../../lab/results/v16-corrections-2026-10-19/batch-a/new-tests.json)
are observed passing in the fresh checkout's full domain/infra run. The final
workflow cleans the four workspace packages after copied-workspace probes to
prevent cached test-artifact reuse. It validates paths, forward/reverse lineage,
ownership/Nuclei, build, workspace tests, doctests, all targets including lab
benches, strict Clippy, formatting and Rustdoc. Complete commands, frozen states,
failed attempts, resource limits and results remain in
[the evidence receipt](../../lab/results/v16-corrections-2026-10-19/batch-a/README.md).

All twelve integrated gates completed successfully at 2026-10-19T11:09:36Z,
with no change in the 2,130 frozen inputs. The [closure ledger](../../lab/results/v16-corrections-2026-10-19/batch-a/closure-ledger.json)
binds each selected ID to its final source/L0 hashes. At that checkpoint, the full pinned linter still
exited 1 for the other 91 V16 errors; it was not an overall zero-violation result.

## Batch B: test observers and storage qualification

The baseline is commit `059d1a0ffa9523dbe21af6860a215acaeef55d73` on the same
branch. The final pinned scan at `2026-10-19T11:58:06.809650+00:00` reports
**91 → 83 V16 errors**: exactly IDs 013–016, 025, 075, 076 and 091 disappear,
across eight exclusive source/L0 pairs. No new error or warning appears; outside
these consumers even diagnostic positions stay identical. Nine V19 grouping
notes are added and retained (1,157 → 1,166); V20 stays at 878, and no previous
informational message content disappears. See the
[complete diagnostic review](../../lab/results/v16-corrections-2026-10-19/batch-b/note-review.md)
and [exact measured state](../../lab/results/v16-corrections-2026-10-19/batch-b/lint-state.json.gz).

Three agents updated owning L0s first, implemented disjoint changes and then
cross-reviewed one another. Test observers now classify the current Content
domain explicitly. Existing hit/order/opacity policies remain: the mathematical
root finder stays narrower than the other math finders; the style collector
stays bounded; no-replacement callbacks retain processed descendants. Storage
qualification retains all 34 Arc comparisons and closes both tuple axes.
No productive behavior, public contract, Nucleus, phase, default, linter or scan
configuration changes, and no new suppression or ignored test is introduced.

Actual sec_27 measurement found all four required equations through existing
Sequence/Styled paths; that evidence did not justify broader carrier traversal.
The original count, selected equation, fonts, ascent and tolerance are unchanged.
The temporary measurement instrument remains in lab evidence and was removed
from active tests. Cross-review also strengthened the new storage control so a
traversal that misses every child cannot certify reuse.

[Twenty-four new tests](../../lab/results/v16-corrections-2026-10-19/batch-b/new-tests.json)
and one expanded existing bottom-up control pass against original and repaired
decisions. Actual copied-workspace experiments show baseline absence and repaired
E0004 at all eight selected decisions, with new/old, old/new and new/new storage
pairs proved separately. The infrastructure boundary uses the owning L0's paired
Styled-arm omission proof with real dependencies; unrelated compiler failures
never count as closure. See
[the witness audit](../../lab/results/v16-corrections-2026-10-19/batch-b/structural-witness-review.json).

All twelve integrated gates completed at **2026-10-19T12:16:50.708042+00:00**, with
no change in 2130 frozen inputs. After cleaning the four product packages to
separate copied-workspace cache artifacts, the real checkout passed paths, strict
forward/reverse lineage, ownership/Nuclei, workspace builds/tests, wiring,
doctests, all targets including lab benches, Clippy, formatting and Rustdoc.
All 24 new tests are observed passing in that fresh full run. Full receipts,
source/L0 hashes, measurements and review limits are in the
[batch B evidence](../../lab/results/v16-corrections-2026-10-19/batch-b/README.md) and
[closure ledger](../../lab/results/v16-corrections-2026-10-19/batch-b/closure-ledger.json).
The full linter still exits 1 for the remaining 83 errors in batches C–G.

## Batch C: selectors, rich display and discarded updates

The baseline is commit `370d78355bd646d1abead5bcacef215288cc8834` on the same
branch. The final pinned scan ended at `2026-10-19T13:39:03.479605+00:00` on the
uncommitted implementation: **83 → 64 V16 errors**, exactly the nineteen selected
findings removed, with no new error or warning. Its complete changed-file stat,
UTC and input hashes are in the
[lint receipt](../../lab/results/v16-corrections-2026-10-19/batch-c/lint-state.json.gz).
Diagnostics outside the twenty-two source/L0 pairs are identical. One retained
V16 inside that scope moves by two lines after helper-signature formatting;
its message and severity remain unchanged. V19 notes change from 1,166 to 1,195;
V20 remains 878 with one content addition and one removal. All notes remain
enabled and receive a [substantive review](../../lab/results/v16-corrections-2026-10-19/batch-c/note-review.md).

Three agents updated owning L0s first, implemented disjoint repairs and reviewed
one another. Closed nominal decisions preserve existing validation order,
delegation, errors, snapshots and nonexecution of rejected arguments. Semantic
repairs cover selector operands and contextual boundaries, typed native and
Format identity, rich LocatedContent insertion, and static detection of discarded
updates. A new shared Nucleus connects the Heading constructor's private explicit
supplement snapshot to its diagnostic consumer; the existing Format Nucleus is
newly pinned without changing its contents. Public signatures, defaults and
pipeline phases remain unchanged.

An initial candidate introduced a forbidden direct regex dependency. The full
scan caught V14; the repair uses a pure local escaping helper specified by its
owner, verified against locked dependency sources. No dependency allowlist,
linter rule, severity, scan scope, suppression or ignored test was changed.

The [52 new tests](../../lab/results/v16-corrections-2026-10-19/batch-c/new-tests.json)
include 23 semantic RED controls and 29 preservation controls, with their actual
baseline runs retained. All 52 and the existing Heading reflection control pass
in the final fresh full domain run. Bilateral checks against the ratified oracle
pass 60 selector cases and 23 warning cases, plus rich content from an actual
query producer in three insertion routes. Real enum-extension experiments and
independent span audits confirm 25 selected/helper decision witnesses and six
isolated Selector/FuncRepr tuple classes. Unrelated compiler errors do not count
as proof, and exhaustiveness does not establish complete language parity.

All twelve integrated gates completed at **2026-10-19T13:58:58.141405+00:00**, with no change in
2,131 frozen inputs. Cleaning the four product packages before the final run
ensured the actual checkout rebuilt and tested its sources. Paths, strict direct
and reverse lineage, ownership/Nuclei, builds, domain/infra/wiring tests, doctests,
all targets including lab benches, Clippy, formatting and Rustdoc passed.
Receipts and hashes are in the
[batch C evidence](../../lab/results/v16-corrections-2026-10-19/batch-c/README.md) and
[closure ledger](../../lab/results/v16-corrections-2026-10-19/batch-c/closure-ledger.json).

The full linter still exits 1 for the other 64 V16 errors. Figure/Equation
supplement provenance, paragraph staging and completed-carrier reentry, generic
rich-value display and ambiguous native identities remain explicit
[open obligations](../../lab/results/v16-corrections-2026-10-19/batch-c/open-obligations.json).
Their required behavior and acceptance criteria are preserved; this batch does
not certify those cases or weaken their contracts.

## Batch D: grid, layout and constrained measurement

The baseline is commit `a9f03073809fa15ff11d86c532f2c803ee892e47` on the same
branch. The pinned scan ended at `2026-10-19T16:19:05.400082+00:00` on the
uncommitted implementation: **64 → 44 V16 errors**, exactly IDs 029–047 and 077
removed across fifteen exclusive source/L0 pairs. No new error or warning
appears; diagnostics outside these consumers are identical. Exact changed-file
stat, UTC and hashes are in the
[lint receipt](../../lab/results/v16-corrections-2026-10-19/batch-d/lint-state.json.gz).
V19 changes from 1,195 to 1,222 and V20 from 878 to 876. The
[independent note review](../../lab/results/v16-corrections-2026-10-19/batch-d/agents/grid/note-review.md)
separates line moves, equivalent Value-path spelling and substantive changes.
No V19 content disappeared; nested supplement patterns became separate finite
decisions with independent evolution checks. All notes remain enabled.

Three agents updated owning prompts first and cross-reviewed disjoint changes.
Grid fixes preserve raw/prepared inheritance, cache refusal, nested-table
barriers, placement policy and atomic inset rejection. Layout fixes preserve
occurrence identity, quote observation, line-number requests, alignment and
supplement behavior. Constrained measurement now preserves image target bounds,
captionless figure body extent and certified simple inline math extent through
existing isolated layout. A direct recursive certificate rejects hidden effects
and unaudited math before measurement. Public contracts, defaults, phases,
dependencies and Nuclei remain unchanged.

The [27 new tests](../../lab/results/v16-corrections-2026-10-19/batch-d/new-tests.json)
include three observed semantic RED failures and 24 preservation controls.
All pass in the final fresh full run. Eighteen bilateral grid cases preserve
baseline product behavior; 27 measurement cases pass the scoped geometry and
single-effect checks against the ratified oracle. Image and captionless figure
Auto-column origins recover from zero to 40pt, and the simple equation from
zero to 23.124444444pt in the recorded sources. These are scoped occupied-space
measurements, not complete render or language parity. Independent reviewers
verified actual SVG geometry/body presence, source/build links and the comparator
failure conditions.

Actual enum-extension experiments produce 23 exact decision witnesses plus
three isolated HAlign tuple witnesses. The
[independent structural audit](../../lab/results/v16-corrections-2026-10-19/batch-d/structural-witness-review.md)
reparses compiler logs and archived mutations, verifying absence at the twenty
original baseline sites and the three tuple classes. Unrelated compiler errors
never count as proof.

All twelve integrated gates completed at **2026-10-19T16:40:10.165167+00:00**,
with 2,131 unchanged frozen inputs. Cleaning the four product packages before
validation separated mutation-workspace caches from the actual checkout.
Paths, strict direct/reverse lineage, ownership/Nuclei, builds, domain/infra/wiring
tests, doctests, all targets including lab benches, Clippy, formatting and Rustdoc
passed. Both accepted replays completed at `2026-10-19T16:40:40.910463+00:00`.
The [evidence report](../../lab/results/v16-corrections-2026-10-19/batch-d/README.md)
and [closure ledger](../../lab/results/v16-corrections-2026-10-19/batch-d/closure-ledger.json)
bind every selected ID to the validated sources, owners and seals.

The full linter still exits 1 for the remaining 44 V16 errors. Captioned figures,
block/contextual/uncertified math, relative image axes and source diagnostics,
SVG image input, nested-table vertical origin and contextual-cell output remain
[explicit open obligations](../../lab/results/v16-corrections-2026-10-19/batch-d/open-obligations.json).
Their required behavior and refuters are retained; unsupported approximations
are not ratified as zero language extent. No suppression or weaker contract was
used to close this batch.

## Batch E: reflection, serialization and realized Equation snapshots

The baseline is commit `5e1a01066b9eef0eb3c478e8b49e15e1da15b0b9` on the same
branch. The final pinned lint comparison completed at
**2026-10-19T19:51:53.979959+00:00** on the uncommitted implementation:
**44 → 37 V16 errors**, exactly IDs 001–004 and 083–085 removed.
No new error or warning appears; outside the ten source/owner pairs,
diagnostics are identical. V19 changes from 1,222 to 1,237 and V20 from 876
to 877; no substantive note disappears. The retained encoding V16 moves by
two lines without changing its content. Exact state and note dispositions are in
[the lint receipt](../../lab/results/v16-corrections-2026-10-19/batch-e/lint-state.json.gz)
and [note review](../../lab/results/v16-corrections-2026-10-19/batch-e/note-review.md).

Three agents analyzed and repaired the bounded reflection and serialization
causes, measured language behavior and reviewed the results. Owning L0s were
updated first. Closed nominal decisions preserve authoritative snapshots,
location precedence, label order and opacity. A private constructor-origin
invariant distinguishes explicit Equation options from external styling;
canonical native identity and the lexical style boundary are retained.
No public API, default or pipeline phase changes.

The first full suite exposed a real realized-Equation regression beyond the
initial corpus. The correction retains semantic content and publishes the
existing ordered captured-field snapshot at its owning producer. Existing
snapshots remain authoritative, including empty snapshots. Selector productive
logic stays unchanged; its existing test now invokes the real producer while
retaining the original expectations. Failed attempts and superseded candidate
receipts remain available rather than being presented as successful validation.

All [34 new tests](../../lab/results/v16-corrections-2026-10-19/batch-e/new-test-execution.json)
pass in the final fresh full run: 18 observed semantic RED failures and 16
preservation controls. The existing selector regression and getter controls also
pass. Twelve exact compiler-negative witnesses receive independent raw-span and
archived-mutation review. Every new realized-query case matches the ratified
oracle: 38 repaired and one preserved, across 39 frozen sources/cases.

The original 173-case corpus has 87 complete oracle matches. Its raw comparator
still exits 1; 26 changed observations are independently verified partial repairs
with exact pre-existing residual differences. Scoped acceptance does not change
those raw results or claim full parity. Presence distinctions, richer reflection,
generated-caption selection and supplement diagnostics remain explicit
[open obligations](../../lab/results/v16-corrections-2026-10-19/batch-e/open-obligations.json).

All twelve integrated gates completed at **2026-10-19T20:10:53.280629+00:00**,
with 2,132 unchanged frozen inputs. Paths, strict direct/reverse lineage,
ownership/Nuclei, workspace builds/tests, doctests, all targets including lab
benches, Clippy, formatting and Rustdoc pass. Both accepted replays completed by
`2026-10-19T20:11:57.424281+00:00`. The
[evidence report](../../lab/results/v16-corrections-2026-10-19/batch-e/README.md)
and [closure ledger](../../lab/results/v16-corrections-2026-10-19/batch-e/closure-ledger.json)
bind the selected findings to the exact validated sources, owners and seals.
The full linter still exits 1 for the remaining 37 V16 errors; no suppression,
severity reduction, linter/configuration change or weakened contract was used.

## Batch F: argument conversion and exhaustive decisions — partial

The baseline is commit `91ea6ee91105b97b1aedf172e34780dddde14c94` on the same branch.
The final pinned lint comparison ended at **2026-10-19T21:22:37.852499+00:00**
on the uncommitted implementation: **37 → 21 V16 errors**, exactly 16 selected
findings removed across 19 exclusive source/L0 pairs. IDs 055 and 065 remain
open; batch F as a whole is not closed. The exact changed-file list, source
hashes, UTC and command are in [the lint receipt](../../lab/results/v16-corrections-2026-10-19/batch-f/lint-state.json.gz).
No new error or warning appears. Outside-scope diagnostics are unchanged.
V19 remains visible at 1,259 notes (previously 1,237); V20 stays at 877.
[The note review](../../lab/results/v16-corrections-2026-10-19/batch-f/note-review.md)
accounts for every substantive addition/removal without changing severity or scope.

Three agents implemented and cross-reviewed the bounded corrections. L0 updates
preceded implementation and centralized resealing. Constructor adapters preserve
lazy calls and causal Arguments; Decimal and Version keys use existing exact
domain ordering. Grid/table reject malformed tracks; place/line retain the
supplied value's diagnostic origin. Counter projections, selector cardinality,
HTML/stroke/alignment conversions, encoders, locale and math/outline decisions
now enumerate their nominal domains. No public API, default or phase changed.

All 52 new tests pass: 22 original semantic RED cases and 30 preservation
controls. The existing adapter admission test and strengthened malformed-grid
test also pass. The sorted-key test additionally demonstrated a genuine Version
RED before its private comparator correction. Failed Circle attempts, the old
grid acceptance regression, cache reuse and the failed Version replay remain
archived. No expected result was weakened to pass. Seventeen exact compiler
witnesses and two arity witnesses received independent raw-span review.

All twelve integrated gates completed at **2026-10-19T21:40:03.769355+00:00**,
with 2,132 unchanged frozen inputs. Paths, strict direct/reverse lineage,
ownership/Nuclei, builds, domain/infra/wiring tests, doctests, all targets,
Clippy, formatting and Rustdoc passed. The 335 strict replay cases also passed
their frozen, case-specific policies: 102 adapters, 182 geometry and 51 encoding.
Preserved baseline debt and explicit residuals are not full oracle parity.
The [evidence report](../../lab/results/v16-corrections-2026-10-19/batch-f/README.md)
and [scoped closure ledger](../../lab/results/v16-corrections-2026-10-19/batch-f/closure-ledger.json)
bind the accepted results to their exact source/L0 and executable identities.

Figure055 needs a coherent typed-kind, selection/finalization and counter-demand
contract. The [readiness analysis](../../lab/results/v16-corrections-2026-10-19/batch-f/fg1-proposal/readiness.md)
and [lifecycle refutation](../../lab/results/v16-corrections-2026-10-19/batch-f/fg1-proposal/counter-lifecycle-followup.md)
explain why the current draft is not ready for approval. At that checkpoint, Circle065 had an
[independently reviewed proposal](../../lab/results/v16-corrections-2026-10-19/batch-f/agents/geometry/circle-deferred/proposal/review.md)
for typed constructor origin and full explicit radius, pending ADR-0127 approval.
The approved follow-up below supersedes that pending status; the original batch F
receipts remain unchanged.
[Open obligations](../../lab/results/v16-corrections-2026-10-19/batch-f/open-obligations.json)
retain Auto/default, relative-dimension and native-constructor gaps. The full
linter still exits 1 for 21 V16 errors; no suppression or weaker contract was used.

## Circle follow-up: approved origin and full explicit radius

The user approved ShapeOrigin and ShapeElem.origin after the original batch F
checkpoint. Baseline HEAD is `954d13185e26a1ed90f6d7c60f6ba27760ca9889` on the
same branch. Native Circle now retains constructor identity and full abs/em
radius through the existing layout resolver; generic shapes retain Geometry
origin. Numeric/Length equivalence, fill/stroke and explicit Auto-as-zero debt
remain. Malformed values are rejected by an exhaustive decision with causal spans.

The frozen shared-occurrence replay additionally exposed Label-selector
first-only projection. Independent controls reproduced it without Circle or
Context. Its compiler-owned branch now preserves all distinct causal occurrences,
while query_by_label keeps its first-result API. This internal parity repair had
its own L0 and semantic RED; no Context or phase change was introduced.

The full pinned scan at `2026-10-19T22:28:29.489373+00:00` reports **21 → 20 V16**,
exactly V16-065 removed, no new error or warning and identical diagnostics outside
fourteen source/owner pairs. V19 is 1262 and V20 is 877; all prior notes and the
three new grouping reminders remain visible. The bounded test observer's complete
runtime alternative coverage is not claimed. State, hashes and independent review
are in [the follow-up report](../../lab/results/v16-corrections-2026-10-19/circle-origin/README.md).

All **23 new tests** pass in the fresh full suite: eight observed semantic REDs,
nine preservation controls and six additional post-API/domain controls. Five
exact compiler witnesses passed independent raw-span review. All twelve integrated
gates finished at **2026-10-19T22:42:48.099173+00:00**, with **2132 unchanged inputs**. Paths, strict
direct/reverse lineage, V15/V26, builds, tests, Clippy, formatting and Rustdoc pass.
The 224 frozen language cases, six exported-ink controls and eighteen Label
controls pass their case-specific policies against the ratified oracle. The first
223/224 replay and observer-lint failure remain preserved, with no weaker expected
result or suppression. Accepted source/owner hashes are in
[the closure ledger](../../lab/results/v16-corrections-2026-10-19/circle-origin/closure-ledger.json).

Batch F now has 17 repairs and Figure V16-055 pending. Auto/default sizing,
Circle reflection/selectors/show/set and the other explicitly deferred surfaces
remain obligations; this is not complete Circle or language parity.

## Figure V16-055: contract proposal awaiting decision

The follow-up at baseline `d4e72102df01f9c248dffac761662d0f3cbf699e` identifies
typed-kind loss, premature constructor inference, lost Grid child order and
inconsistent typed counter histories. Three agents audited the carrier,
counter lifecycle and independent language probes. Their
[concrete review package](../../lab/results/v16-corrections-2026-10-19/figure-kind/review.md)
specifies the public fields, occurrence phase, bundle facade, compatibility
changes, exclusive owners and required acceptance without suppressing V16-055.
This supersedes the earlier incomplete design candidate as a proposal only.

The unchanged baseline passed 128 existing Figure unit tests, CI tooling tests,
canonical paths, strict V1/V5/V15/V26 and reverse lineage. The full pinned scan
still reports 20 V16 errors, 1262 V19 notes and 877 V20 notes. Exact UTC, commands
and unchanged inputs are in the package's validation receipts. These are baseline
results, not evidence that the proposed implementation passes. No productive
source or active L0 was changed in this follow-up. ADR-0127 approval is required
before the new public contracts and phase are materialized; batch F remains open.

## Remaining obligations and organizational result

The pending Figure item in F and batch G remain open. The mathematical parser's old grouping comment differs
from the observed tree for `a^b'_c^d`; batch A preserved the observed tree and
recorded the intent question separately. It does not ratify that comment or close
parity through Rust exhaustiveness. Three callback font formats still lack
independent fixtures, as explicitly retained by their owner; no new corpus or
variation-sequence certification is claimed.

The reorganization helped trace each selected finding directly to its canonical
owner, keep active contracts separate from historical evidence, and isolate the
parser ownership/comment questions without moving implementation between layers.
It did not itself fix these structural causes. No runtime or development-speed
improvement was measured. History is preserved; changes stay on the existing
branch, with probes, benchmarks, builds and detailed receipts under `lab/`.
