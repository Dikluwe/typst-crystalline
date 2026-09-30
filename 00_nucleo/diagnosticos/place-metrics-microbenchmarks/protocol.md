# Place composition and metric-instance reuse microbenchmarks

Scope: this diagnostic directory only. No production APIs, implementation copies,
existing benchmark edits, optimization, global formatting or hash reseal.

Fifteen new cases:

- Four metric APIs (vertical_metrics, text_edges, advance, advance_shaped), each
  with a retained warmed instance and a fresh instance over the same warmed World.
  Each operation is public component composition, not isolated face parsing,
  fallback selection, cmap, cache lookup or shaping-engine instruction cost.
- FallbackFontMetrics constructor plus destruction; warmed metric Clone plus
  destruction; batched-reference control. These are separately measured operations.
- Complete FixedMetrics layout of one rectangle with Place nesting depths 0/1/4/16.
  Includes introspection, temporary layouters, layout, result allocation and result
  destruction. Does not isolate a Place helper and does not use real font discovery
  or shaping. Depth is the sole fixture change; do not infer a universal per-level cost.

Font setup: embedded-only SystemWorld, explicit Libertinus Serif at 11pt, text
"Hello world". World/library construction, embedded metadata/book acquisition,
selected FontSlot loading, style/fixture construction and correctness checks are
outside all timed closures. Font bytes/faces are fingerprinted at setup. The
retained instance warms every API before measuring. Fresh metric instances start
with empty per-instance caches but inherit warm World font handles; this is not
cold-process or cold-filesystem performance. Fresh constructor and input destruction
are excluded with Criterion iter_batched_ref; metric-query allocations, cache
population, resolution and first parsing/shaping inside the API remain included.
Retained calls use iter, so their primitive output disposal is included. Clone
shares caches; its measured case includes clone/drop, not subsequent queries.

Shape Content/Arc/Value/Place construction is outside timing. The layout interval
borrows an unchanged Content, includes production internal copies, and destroys
the returned document. No setup I/O or PDF export is included. The fixtures avoid
the known RED image clipping and text-baseline cases. A single-shape smoke check
proves materialization only, not all nested placement geometry.

Build sequentially, release opt-level 3, no debug assertions, cleared Rust flags,
offline locked dependencies, one Cargo job. Verify the Cargo JSON artifact profile
before executing it; runtime additionally refuses debug assertions. Compile and
untimed smoke complete before the quiet gate. No tests/builds/other benchmarks may
run during measurement. Poll known competing processes every 100ms. Record HEAD,
exact dirty/untracked state, clock, machine/toolchain, commands and executable hash.

Three independent Criterion rounds, 50 samples per case, 500ms warmup, 1s measurement,
10,000 analysis resamples. Reverse metric/pair/depth ordering in round two. Warmups
and statistical analysis are outside the sample intervals. Preserve all normalized
sample distributions and outliers. Stability criterion: all normalized samples
CV <=20% and round medians within 10% of the pooled median. Unstable cases remain
exploratory; no selective retry. Fingerprint consumers, inputs, diagnostic source,
lock and executable before/after each round; discard a changed/contaminated round.

OS caches are warm; no cache-drop operation. PerIteration batching has a timer/
bookkeeping floor: record the control but do not subtract it from another case.
Do not add these costs to earlier pipeline/CLI medians or subtract different fixture
populations. No vanilla equivalent is measured here: the public boundaries differ.
These results test current Tekt cache lifetime/component behavior, not a Tekt/vanilla
speed ratio. Existing RED regressions remain unresolved.
