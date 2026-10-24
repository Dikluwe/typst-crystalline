# Current Tekt vs pinned vanilla

Current source-bound results and remaining divergences: [comparison after R8–R10](vanilla-comparison-after-r8-r10-2026-10-24.md). The observations below retain the original pre-A–H snapshot.

The current optimized Tekt candidate has measured local advantages in expression
evaluation and checked CLI I/O failures. Three exact-JSON evaluation workloads
show approximately 1.3–1.8× paired speed and 26–38% lower peak RSS, stable in both
default-affinity and single-CPU controls. Failed stdout in eval/fonts/info/
completions gives an ordinary Tekt error where the pinned vanilla has captured
panics in the measured cases.

This is not general render superiority. PDF peak RSS is higher in all seven
tested document workloads. New controls expose a trailing weak-pagebreak blank
page and prose/math continuation differences in both debug and release. `init`
still has a captured stdout-failure panic. Six broader parity profiles and the
established 55-case corpus retain their bounded successful criteria, while
16 public symbol/text carrier differences remain explicit morphology debt.

No productive source, L0 contract, comparator threshold, warning policy or oracle
pin was changed. Both executables are optimized, source-bound and hash-verified.
The experiment compares actual build recipes and does not attribute runtime gains
causally to architecture or recent refactors.

The [full report](../../lab/results/tekt-vs-vanilla-2026-10-23/README.md) contains
timings, confidence intervals, memory, raster/geometry evidence, failed-harness
metadata recovery, exact source state, provenance and prioritized follow-ups.

The [divergence handoff](vanilla-divergence-handoff-2026-10-23.md) separates
confirmed defects from observer/shared-boundary investigations and gives
reproducers, proposed batches and acceptance criteria for the next conversation.
