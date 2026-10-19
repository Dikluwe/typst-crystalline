# L0 — core/compiler/layout/paragraph_select_bounded/vanilla_lower
Hash do Código: 77cee26e

**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/layout/paragraph_select_bounded/vanilla_lower.rs`
**Architecture:** ADR-0107/0108/0109/0127/0129/0130.

## Evidence before specification

`lab/typst-original/crates/typst-layout/src/inline/linebreak.rs:264-344` uses a lower bound for underfull lines after obtaining a valid upper bound. The earlier combined prototype in `lab/results/construction/diagnostics/paragraph-select-podas-vanilla-justice-2026-10-02.md` measured 1,000 words/240pt: 9,695 evaluations and 195.470 µs, versus 120.770 µs for exact DP. This synthetic measurement does not establish vanilla intent or certify the current reference.

## Experimental contract and acceptance

This private, test-only consumer owns ascending start traversal for each end. The parent supplies prefix measurements, exact cost, Simple upper bound, active frontier and DP vectors. After the first nonfinal underfull line, its cost may serve as a lower bound for later starts when words, gaps and tracking contribute finite nonnegative values. Discard a predecessor only if its accumulated cost plus that bound strictly exceeds the upper bound. Preserve active-frontier updates, `<=` ties favoring the later start, and the parent's fallback. Do not change the cost function, productive selection, default, API or phase. The parent's benchmark compares ranges against exact DP and measures visits, pruning and time.
