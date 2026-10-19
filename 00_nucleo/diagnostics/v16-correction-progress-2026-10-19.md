# V16 correction progress

Status: batch A validated and closed. The other
91 V16 findings remain open in the [cause-correction plan](v16-cause-correction-plan-2026-10-19.md).
This diagnostic is not a new L0 contract or a claim of complete Typst parity.

## Evidence before closure

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

## Implemented batch

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
binds each selected ID to its final source/L0 hashes. The full pinned linter still
exits 1 for the other 91 V16 errors; this is not an overall zero-violation result.

## Remaining obligations and organizational result

Batches B–G remain open. The mathematical parser's old grouping comment differs
from the observed tree for `a^b'_c^d`; this batch preserves the observed tree and
records the intent question separately. It does not ratify that comment or close
parity through Rust exhaustiveness. Three callback font formats still lack
independent fixtures, as explicitly retained by their owner; no new corpus or
variation-sequence certification is claimed.

The reorganization helped trace each selected finding directly to its canonical
owner, keep active contracts separate from historical evidence, and isolate the
parser ownership/comment questions without moving implementation between layers.
It did not itself fix these structural causes. No runtime or development-speed
improvement was measured. History is preserved; changes stay on the existing
branch, with probes, benchmarks, builds and detailed receipts under `lab/`.
