# V16 correction progress

Status: batches A and B validated and closed. The other
83 V16 findings remain open in the [cause-correction plan](v16-cause-correction-plan-2026-10-19.md).
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

## Remaining obligations and organizational result

Batches C–G remain open. The mathematical parser's old grouping comment differs
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
