# V16 correction progress

Status: batches A, B, C and D validated and closed. The other
44 V16 findings remain open in the [cause-correction plan](v16-cause-correction-plan-2026-10-19.md).
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

## Remaining obligations and organizational result

Batches E–G remain open. The mathematical parser's old grouping comment differs
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
