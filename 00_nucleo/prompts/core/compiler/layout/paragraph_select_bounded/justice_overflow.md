# L0 — core/compiler/layout/paragraph_select_bounded/justice_overflow
Hash do Código: 775de2b6

**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/layout/paragraph_select_bounded/justice_overflow.rs`
**Architecture:** ADR-0107/0108/0109/0127/0129/0130.

## Evidence before specification

`kitlangton/justice` at commit `86dbc6da8cc2059cabb2a67c0aa42d773d5023e2`, `src/engine.ts:340-400`, visits predecessors in reverse order and stops after overflow under a monotonicity guard. The earlier combined prototype in `lab/results/construction/diagnostics/paragraph-select-podas-vanilla-justice-2026-10-02.md` measured 1,000 words/240pt: 210.360 µs with a Simple upper bound and 196.900 µs without a seed, versus 120.770 and 122.730 µs for the respective exact DPs. Justice's cost function and states differ; this traces its search rule rather than porting it wholesale. These are historical prototype measurements, not new certification of the current reference.

## Experimental contract and acceptance

This private, test-only consumer owns descending start traversal for each end. The parent supplies prefix measurements, exact cost, active frontier and DP vectors. Measure the Simple-upper-bound and unseeded options separately. When words, gaps and tracking contribute finite nonnegative values, an overfull line proves that lines starting still earlier remain overfull; stop only if their minimum cost cannot improve this end's best cost. On the final line, exclude the `runt` penalty from the estimate for lines preceding a word start. Preserve the later start on ties through strict `<`. After searching, update the active frontier as the productive selector does. If uncertain, do not stop. Do not change cost, productive selection, default, API or phase. The parent's benchmark compares complete ranges and measures visits, pruning and time.
