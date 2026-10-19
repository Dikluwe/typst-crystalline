# Place composition and font metric cache lifetime microbenchmarks

Fifteen diagnostic cases were collected in three accepted release rounds; fourteen
passed the predeclared stability criterion. No production code, L0, ADR, or existing
benchmark was changed by this study. The five preceding Place regression tests
remain RED; this collection does not close those defects.

## Provenance and measurement boundaries

Measured HEAD: `17f13b1df79b87ea065ee62e78164b18e477da3d`, with a dirty working tree.
The exact tracked/untracked inventory and tracked diff stat are in
[provenance.json](place-metrics-microbenchmarks/provenance.json). Consumers include
the preceding regression tests and preexisting changes from other work; these are
not measurements of a clean HEAD alone. Machine: Dell-G15-Linuxmint, AMD Ryzen 7
5800H, Linux 6.17.0-42-generic, Rust/Cargo 1.92.0, x86_64-unknown-linux-gnu.
Collection receipts are UTC on 2026-09-28 (local 2026-09-27): accepted round completion
00:04:01.165223, 00:04:27.014519, and 00:04:52.914420. Build and process commands,
source fingerprints, and selected embedded font hashes are retained alongside raw
results. Executable SHA-256: `96bee016bc8b99e434d9ef94e0fb5b9434c5ec3d0908cc18902283fe8c1a411d`.

The build uses release opt-level 3, no debug assertions or overflow checks, empty
RUSTFLAGS/CARGO_ENCODED_RUSTFLAGS, locked offline dependencies, and one Cargo job.
Both the Cargo artifact profile and a runtime assertion reject debug measurements.
The executable is archived in `bin/microbench`; the measured source overlay is in
`measured-sources.tar.gz`. Builds and the untimed smoke check finished before the
five-second quiet gate. Every round retained identical fingerprints of productive
sources, L0s, benchmark sources, root build inputs, diagnostic inputs, and executable.
The 100ms process monitor recorded no known competing build/test/benchmark process.
It cannot prove CPU idleness or detect every short-lived/unrecognized process.

The metric cohort uses embedded fonts only, Libertinus Serif, 11pt, and `Hello world`.
World creation, book acquisition, font loading, fixture construction, and consistency
checks are untimed. This is a warm World, not fresh-process font discovery. Each
retained metric instance has all four query caches warmed. Each fresh instance starts
with empty metric caches, but uses the same warm World. Fresh construction and input
cache destruction are excluded with `iter_batched_ref(PerIteration)`; query resolution,
parsing, cache population, allocations, and shaping inside the public API are included.
Fresh scalar output disposal is also outside that interval. Retained `iter` includes
scalar disposal. Constructor/drop and clone/drop are separately measured, with their
destruction included. Clone shares caches. Metric results agree across retained,
fresh, and cloned instances; this smoke check is not a full language parity proof.

The layout cohort builds one 80×40 rectangle and 0/1/4/16 nested nonfloating top-left
Place elements, each translated by (3,5), before timing. The interval calls the public
`layout(&Content)` with FixedMetrics and default page sizing: introspection, temporary
layouters, internal copies, allocations, and returned document destruction are included.
It excludes source parsing, evaluation, font discovery, real shaping, I/O, and PDF
export. It measures layout composition, not an isolated Place helper. Each smoke check
finds one materialized shape; complete nested geometry parity is not established.

## Distribution

Each case has 150 normalized samples (`time / iterations`) across three independent
50-sample rounds. Warmup is 500ms and requested measurement is 1s per case. Round two
reverses metric/pair/depth order. All samples and outliers are preserved. These are
medians of normalized batch samples, not individual request latency percentiles.
Stability requires pooled CV ≤20% and every round median within 10% of the pooled
median. No outlier removal, selective retry, or control subtraction was applied.

All times below are microseconds. Complete min/max/mean/standard deviation and sample
distributions are in [summary.json](place-metrics-microbenchmarks/summary.json);
Criterion raw samples and estimates are under `round0`, `round1`, and `round2`.

| Case | Pooled median µs | CV | Round medians µs (0,1,2) | Admission |
|---|---:|---:|---|---|
| vertical_retained_metric_instance | 0.269746 | 3.09% | 0.270657, 0.266299, 0.270534 | stable |
| vertical_fresh_metric_instance | 1.115589 | 2.13% | 1.153124, 1.113404, 1.110901 | stable |
| text_edges_retained_metric_instance | 0.258872 | 3.55% | 0.272992, 0.256563, 0.258593 | stable |
| text_edges_fresh_metric_instance | 1.108173 | 3.08% | 1.135859, 1.105980, 1.104955 | stable |
| advance_retained_metric_instance | 0.150426 | 5.76% | 0.161354, 0.149358, 0.149392 | stable |
| advance_fresh_metric_instance | 4.183305 | 1.61% | 4.156281, 4.282657, 4.172670 | stable |
| advance_shaped_retained_metric_instance | 0.150403 | 1.93% | 0.150985, 0.150644, 0.149862 | stable |
| advance_shaped_fresh_metric_instance | 20.499379 | 1.38% | 20.516697, 20.424945, 20.516299 | stable |
| constructor_and_drop | 0.092641 | 0.73% | 0.092778, 0.092705, 0.091989 | stable |
| shared_cache_clone_and_drop | 0.015809 | 1.33% | 0.015784, 0.015849, 0.015809 | stable |
| control_batched_ref | 0.023675 | 2.56% | 0.024308, 0.023674, 0.023525 | stable |
| nested_place/0 | 1.876979 | 16.59% | 1.860702, 2.040311, 1.857496 | stable |
| nested_place/1 | 2.267191 | 11.17% | 2.241242, 2.456641, 2.237312 | stable |
| nested_place/4 | 3.343106 | 8.49% | 3.334912, 3.506845, 3.286893 | stable |
| nested_place/16 | 8.829059 | 10.26% | 8.809579, 10.158675, 8.758464 | **exploratory** |

## What this supports

For this identical text/style/World, retained query caches materially reduce public
metric API cost. Fresh versus retained medians are approximately 4.14× for vertical
metrics, 4.28× for text edges, 27.81× for advance, and 136.30× for shaped advance.
The shaped-query pair is 20.499µs versus 0.150µs. The unit batched-reference control
is 0.0237µs; the two paths have different timing overhead, so ratios are observed API
comparisons, not pure cache lookup versus Rustybuzz instruction costs. Constructor
plus drop is 0.0926µs, and shared-cache clone plus drop is 0.0158µs. Neither value
was added to the fresh-query interval or subtracted from another result.

Depth 0/1/4 layout medians are 1.877/2.267/3.343µs. Additional Place composition costs
more for this one-shape fixture, but these medians do not isolate a universal cost
per level. Depth 16 is exploratory: round two's 10.159µs median is about 15.1% above
the pooled 8.829µs median, exceeding the admission rule despite CV below 20%.
The accepted depth-0 cohort also has appreciable variation (16.59% CV).

No vanilla counterpart was executed in this collection: its public metric/layout
boundaries differ. The existing pinned vanilla evidence remains in the previous
studies. These microseconds cannot be added to earlier CLI/pipeline medians from
different fixtures and cache policies, or used to explain the entire Tekt–vanilla
time gap. Real pipeline frequency, metric instance lifetime, catalog acquisition,
and output-equivalence effects still require compatible measurements.

## Hypotheses and controlled follow-ups

1. **Metric instance lifetime causes avoidable first-query work.** These pairs show
   the consequence of empty versus warm caches for a fixed public query. Confirm
   its contribution in a full pipeline by holding input, fonts, options and output
   invariant, changing only metric-instance retention across equivalent layouts.
   Measure whole-interval time and query/cache-miss counts. Little total-time change
   or few repeated misses would refute a major contribution for that workload.
2. **Shaping dominates the fresh shaped-query composition.** Fresh shaped advance
   exceeds fresh advance, but the APIs have different semantics/cache paths; that
   difference alone does not isolate shaping. A diagnostic-only harness should
   measure resolution/face setup and actual shaping separately with identical bytes,
   axes, language, features, text and glyph-output checks. A small isolated shaping
   cost would refute this explanation. Do not expose production APIs for this purpose.
3. **Place nesting creates accumulating layout overhead.** Confirm with independent
   stable rounds, identical document geometry checks, a wider controlled depth range,
   and matching unwrapped controls. This collection's depth-16 instability prevents
   a firm deep-nesting estimate. This is separate from the previously demonstrated
   unnecessary font acquisition in nontext Place layouts.

## Reproduction and limits

See [protocol.md](place-metrics-microbenchmarks/protocol.md) and
[README.md](place-metrics-microbenchmarks/README.md) for commands and evidence layout.
Host caches were warm; no OS cache flush, CPU affinity, governor lock, thermal control,
or exclusive desktop session was used. RAYON_NUM_THREADS=1 was set. Other environment
and host variation can affect a future run. The selected family face hashes and
query results match across all rounds. Source fingerprints and process checks admit
this block under its protocol, not a claim of noise-free timing. Source archives are
overlays on the repository, not complete dependency/toolchain distributions.
