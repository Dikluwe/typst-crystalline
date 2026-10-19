# L0 — paragraph_select_bounded
Hash do Código: 9588ae1d

**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/layout/paragraph_select_bounded.rs`
**Architecture:** ADR-0107/0108/0109/0127/0129/0130.

## Evidence before specification

`01_core/src/compiler/layout/paragraph_select.rs:101-176` uses exact DP over all word ends, prefix measurements for O(1) intervals, safe overfull-predecessor discard and ties favoring the later predecessor. `01_core/src/compiler/layout/paragraph_plan.rs` calls only that selection. Historical pinned vanilla `lab/typst-original/crates/typst-layout/src/inline/linebreak.rs:227-381` combines an approximate bound and bounded search. This is mechanical source inspection, without timing or a vanilla-equivalence claim.

Prototype differential tests establish identical breaks and effective pruning, not timing. Both paths are accessible in this consumer under cfg(test). `lab/results/construction/diagnostics/microbench-paragraph-select-bounded-2026-09-30.md` measured only 40/160 words: pruning won at 160 words/240pt and 1200pt, but not 40 words/76pt. Those cases do not establish 1,000-word scaling.

Vanilla `lab/typst-original/crates/typst-layout/src/inline/linebreak.rs:264-344` uses underfull-line cost as a lower bound for later attempts without negative widths. `kitlangton/justice` at `86dbc6da8cc2059cabb2a67c0aa42d773d5023e2`, `src/engine.ts:340-400`, visits predecessors in reverse and stops after overflow only with a monotonicity guard. `paragraph_select_bounded.rs:91-143` has only a global upper bound and visits other pairs before pruning. External discovery suggests mechanics, neither a language contract nor local gain. `lab/results/construction/diagnostics/paragraph-select-podas-vanilla-justice-2026-10-02.md` measured both variants in one file: ranges were preserved, but synthetic cases were slower than exact DP.

## Experimental ownership and contract

This prototype is private and test-only. It receives the exact word selector's prepared input, width, size, justification and costs, introducing no API/mode/flag/default/pipeline phase or production replacement. It does not cover hyphenation candidates, complete UAX14 or variable width.

Compute a valid upper bound by summing exact costs of the existing Simple solution's lines, measuring them with the same prefix formula as Optimized. Each line cost is a nonnegative square; an accumulated prefix strictly greater than the bound cannot occur in a better or tied solution, and only those prefixes may be pruned. Retain exact DP's overfull discard and `<=` later-predecessor tie rule. Nonfinite bound/result delegates to exact selection. Do not apply an approximate margin or change cost.

This owner retains only common preparation, reconstruction, the upper-bound variant and comparative tests. Lower-bound search belongs exclusively to `00_nucleo/prompts/core/compiler/layout/paragraph_select_bounded/vanilla_lower.md`, owning `paragraph_select_bounded/vanilla_lower.rs`; reverse-overflow search belongs exclusively to `00_nucleo/prompts/core/compiler/layout/paragraph_select_bounded/justice_overflow.md`, owning `paragraph_select_bounded/justice_overflow.rs`. Share measurements/cost only; each owner retains traversal/pruning logic. All remain private under cfg(test); differential tests/benchmarks remain here as the comparison bench.

## Acceptance and benchmark limits

Differential tests compare complete range sequences to exact selection across empty, overlong, justified/unjustified, widths/gaps, tracking and negative runt cost. Candidate/prune counts are exposed only to tests and do not prove timing gain. Build and crystalline-lint must have no violations.

Keep a default-ignored release Criterion microbenchmark reusing identical Prepared instances across variants. Measure complete selection, including greedy-bound construction, on deterministic short/narrow, medium and wide inputs. Consume output with black_box and establish range equivalence before timing. Per pair, keep cost, font size, justification and width identical. Extend to 1,000 words in all three width regimes with the same generator, font, gaps, costs, differential check and full-selection timing; record separate results/provenance rather than extrapolating the earlier cases. Record commit/tree, command, environment and uncertainty. Local synthetic results do not establish whole-compiler gain.
