# L0 — paragraph_select_forward
Hash do Código: 0f2924e6

**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/layout/paragraph_select_forward.rs`
**Architecture:** ADR-0107/0108/0109/0127/0129/0130.

## Evidence before specification

`01_core/src/compiler/layout/paragraph_select.rs:151-180` fixes line end and visits active starts, storing total cost/predecessor; prefix interval measurements are O(1). `01_core/src/compiler/layout/paragraph_select_bounded.rs:91-143` uses Simple's upper bound but still visits/measures every pair before pruning. The 2026-10-01 microbenchmark found this slower for 1,000 words/240pt and faster at 1,200pt, without proving loop inversion better.

`lab/results/construction/diagnostics/microbench-paragraph-select-forward-2026-10-02.md` found Simple gave the lowest bound in all four scenarios, while two extra seeds cost time. `paragraph_select.rs:110-122` recomputes full interval measure for every candidate word in Simple; `paragraph_select_forward.rs:49-101` already computes prefix measures. Simple-only still loses to exact selection at 1,000 words/240pt; prefix arithmetic may round differently.

`lab/results/construction/diagnostics/microbench-paragraph-select-forward-ablation-2026-10-02.md` found prefixed Simple faster than exact at justified 160 words/240pt and 1,000 words/1,200pt, but slower at 40 words/76pt and 1,000 words/240pt. No universal winner exists in that matrix. First adaptive measurement (`2026-10-02 18:50:57 UTC`, working tree over `39a74d477`, two changed files): at 160 words/76pt, choosing forward on word count alone cost 10.744 µs versus exact's 8.338 µs; relative width must also participate. This synthetic calibration is not a production threshold.

`lab/results/construction/diagnostics/microbench-paragraph-select-local-seed-2026-10-02.md` measured 500 words/240pt: prefixed forward 2,932 candidates/43.289 µs versus one-pass local repair 2,305/39.529 µs. At 1,000 words/240pt the counts/times were 10,300/134.19 µs versus 7,895/114.82 µs. Wide-case repair reduced neither candidates nor time. All-cut pair search and a second pass were measured/rejected in the diagnostic.

Internal iteration evidence: selected Clippy warning at `01_core/src/compiler/layout/paragraph_select_forward.rs:309`; sources/spans/owners/references/binary/UTC, HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, uncommitted tree/status/diffstat are in `lab/results/construction/diagnostics/iteracao-parametros-ci-2026-10-13/before.json.gz`. This measures mechanics/uses, not new bilateral parity or performance gain.

## Experimental ownership and algorithm

This consumer is private under cfg(test), with no API/mode/default/phase or production-path change. Receive Prepared, width, font-size, justification and word-selector costs; return Optimized's ranges on the prepared subset, without representing hyphenation or all UAX14.

Compute identical prefix measurements and base cost. Build two valid initial partitions: (a) fullest line within ratio limit, preserving indivisible-word progress; (b) valid line end nearest estimated target width from remaining material divided by estimated remaining lines. Score both with exact cost, retaining the best upper bound; existing Simple is a control. Line-count estimates guide candidates only and never discard solutions.

Reverse DP traversal: for each start with known cost, visit eligible ends and relax their prefix costs. Precompute each end's first active start exactly as the productive selector; this frontier never decreases. Stop a start's end vector when the start falls below the active frontier. Skip an entire start only when its partial cost exceeds the best bound, since future costs are nonnegative squares. Preserve `<=` ties favoring later starts. Nonfinite bounds or incomplete output delegate to exact selection.

Keep comparative experimental variants: full packed/balanced seeds; Simple-only without those seeds or their accumulated-width vector; prefixed Simple-only constructing the same greedy partition through available O(1) prefixes, preserving advancement/indivisible words. Do not change the productive path.

Local seed repair starts from prefixed Simple. Visit adjacent line pairs once, left to right; test only moving the cut one word left/right with both lines nonempty. Accept only finite strict improvement of the two exact line costs. Repaired partition supplies an upper bound only; the heuristic itself excludes no DP candidate. Preserve prefix measure, active frontier and ties.

Adaptive choice is deterministic and low-cost, using only paragraph size, width relative to mean advance and justification; never run both paths to choose. Expose chosen path to tests. Select exact below 64 words; at 64–256 choose prefixed forward if at least seven mean advances fit per line; above 256 choose prefixed forward if at least 24 fit with justification or 32 without. Otherwise, within 257–1,200 words and at least seven mean advances, choose forward with local repair; outside those ranges choose exact. Mean advance sums word widths and preceding gaps. These are test-only thresholds requiring real-corpus validation before promotion; retain additional wide-unjustified and medium-narrow calibration scenarios.

In cfg(test) active_before precomputation, enumerate mutable slots starting at index one. Store active before the same while; retain n+1 slots and untouched slot zero. Preserve measurement calls, order, bound, ties, fallback and existing experiment thresholds. Do not add lint suppression.

## Acceptance and limits

Differentially compare complete breaks to exact for empty, overlong, widths/gaps, justification, tracking and negative runt. Release benchmarks reuse identical inputs, establish range equality before timing and measure complete selection. Compare exact baseline, complete forward and Simple-only in one Criterion run to isolate seed cost from loop inversion; compare the three forward variants and baseline together for prefix-measure ablation, requiring exact final ranges in differential and timed matrices despite rounding. Compare whole adaptive selection, including choice cost, to isolated paths on the same inputs. Repeat calibration after including relative width. Record exact-state provenance; synthetic gains do not authorize production/default changes or general-gain claims.

Build and crystalline-lint must have no violations. Internal equivalent maintenance follows continuous flow under ADR-0127 without public/default/phase changes. Preserve existing expected outputs; do not add tests merely mirroring loops. Validate core suite including conflicts, spans, fixups, markers and differential selection, all targets, build, documentation, Clippy and lineage with restricted resealing.
