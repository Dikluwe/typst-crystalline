# Reproducible performance baseline protocol

Prepared before timing. Authority: repository instructions and ADR-0030,
ADR-0107/0108, ADR-0115, ADR-0119, ADR-0121 and ADR-0126. Existing benchmark
reports are historical evidence; their concurrent builds and short private
measurement windows do not establish a new performance comparison.

## Admission and identity

`run.py prepare` verifies vanilla's ratified SHA-256 and compares every upstream
tracked blob against pin `9f2b6e8715237cb086899a42873660fe744622e8` (tree
`6ab8bb9434b1bd4d44e1a10fbf1e407b3b86a1a4`). The local lockfile overlay is
recorded. It does not trust a version string alone. It records machine,
toolchain, font paths, explicit environment, fixtures and source identities.

Before sampling, a 30-second quiet gate requires identical consumer/input fingerprints and no
other Cargo, Rust compiler, test artifact, Typst, Criterion or lint workloads.
The gate also rejects observed C/C++ build tools and Go build/test processes,
including unrelated projects on the same machine. All measurements are sequential. Each build/target/CLI block has exact HEAD,
expanded dirty/untracked inventory, diff-stat and binary patch before and after.
Fingerprints include all L1-L4 files, benchmark files, manifests/lockfile, Cargo
configuration, diagnostic scripts/fixtures, vanilla tracked source and accessible
system fonts/fontconfig configuration. New/deleted consumer paths change the
fingerprint. Executables are checked against their recorded SHA-256 before and
after targets and before each CLI invocation. Build identity is the Cargo JSON
profile, requiring optimization and disabled debug assertions.

A process monitor polls throughout execution. Changed fingerprints, changed
executable identities, failed commands or observed competing builds/tests/benches
invalidate the whole affected block. Raw rejected records are retained and never
pooled. Polling is evidence of observed isolation, not a guarantee against a
short process between polls or a file changed and restored between snapshots.
Consumer archives and exact patches permit reconstructing the dirty source state.
No source optimization, benchmark changes, formatting or hash reseal is part of
this protocol. No system process is killed or global cache dropped.

## Build and repetition

```sh
PYTHONDONTWRITEBYTECODE=1 python3 00_nucleo/diagnosticos/performance-baseline/prepare_fonts.py
PYTHONDONTWRITEBYTECODE=1 python3 00_nucleo/diagnosticos/performance-baseline/run.py prepare
PYTHONDONTWRITEBYTECODE=1 python3 00_nucleo/diagnosticos/performance-baseline/run.py build
PYTHONDONTWRITEBYTECODE=1 python3 00_nucleo/diagnosticos/performance-baseline/run.py micro
PYTHONDONTWRITEBYTECODE=1 python3 00_nucleo/diagnosticos/performance-baseline/run.py e2e
PYTHONDONTWRITEBYTECODE=1 python3 00_nucleo/diagnosticos/performance-baseline/run.py e2e --instrumented
```

The build uses `cargo bench --manifest-path benches/Cargo.toml --no-run` plus
`--offline --locked --message-format=json`, then a release CLI build, then a
release core library test build without running tests. Builds have one Cargo job;
no sampling overlaps them. Optimization uses unmodified Cargo bench/release
profiles. Encoded/plain rustflags are empty; product/profile overrides are
removed. No custom target-cpu, LTO or allocator flags are added. Private launchers
use the suite's guarded runner and cached release test artifact; their build
startup is outside Criterion sampling.

All eight existing targets run in three sequential rounds. Public Criterion
settings: sample size 30, warmup 0.5 seconds, requested measurement 1.5 seconds,
10,000 bootstrap resamples, no plots. Private observation/stdlib retain their
unmodifiable 30-sample, 150-ms warmup, 300-ms requested windows and default
bootstrap settings. Criterion may extend measurement windows. Every target/round
has its own `CRITERION_HOME`; historical Criterion baselines are never reused.

Report each case's per-round estimates, confidence intervals, raw batch iteration
counts/times and normalized ns/op distribution. Cross-round range/median and
mean-versus-median discrepancy are reported separately. The ADR-0115 criterion
(mean-versus-median <3%) is shown, along with the stricter range/median <3% flag;
a failure labels a noisy case rather than an optimization regression. No
outlier trimming, fastest-run selection or automatic speedup claim is allowed.

## Cache, CPU and CLI comparison

Affinity is Linux logical CPU 4; its sibling and governor are captured in
`host.json`. Boost/governor are observed, not changed. Rayon uses one thread and
vanilla `--jobs 1`. This is a single-CPU baseline, not a maximum-throughput test.
Desktop/OS background work remains a limitation. Process and load telemetry is
retained; compilation/test/benchmark overlap causes rejection.

Microbenchmarks are warm-process measurements: fixtures and smoke checks precede
sampling, comemo/shaping/font caches persist as the existing code permits. The
protocol adds no eviction. CLI measurements launch a fresh process each time,
with warm OS file cache and no inherited process-level comemo cache. Two untimed
runs of each engine precede each fixture/round. Disk cache is not purged; this
baseline does not measure physical cold-disk startup. CLI font discovery and
source loading/parsing, output I/O and process teardown belong to wall time.

Five qualified diagnostic fixtures cover the pipeline's exact single-font literal, dense
text, positioned shapes, context and the existing tiny JPEG. Inputs are
identical on both sides. Both use default embedded/system font discovery; explicit
text fixtures select Libertinus Serif. The lockfiles pin different typst-assets revisions; per-face byte identity
is checked and recorded separately before comparing equivalent fonts. Font inventories and PDF font identities are checked before
interpreting ratios. No packages, downloads or generated timestamps appear in
source. `SOURCE_DATE_EPOCH=0`, PDF tagging false and pretty false are explicit;
Tekt's production Verbose stream mode is retained. Options requiring different
production behavior are not introduced to force faster output.

Three CLI rounds contain twelve alternating paired invocations per fixture.
Raw spawn-to-wait wall time, child user/system CPU time, RSS, context switches,
stdout/stderr and PDF identities are retained. Input/output hashing and report
writing happen outside wall timing. PDF byte equality is not language parity.
Page geometry, text morphology and selected font families are checked separately;
known output differences limit interpretation as equal completed work.

Instrumented CLI rounds are separate: six pairs per fixture, with Tekt
`--timings-json` and vanilla `--timings`. Their wall distributions quantify
instrumentation overhead. Tekt phase counters are additive but `total_ms` is not
process wall: parse is not populated by this caller; world construction, source
parse, PDF font resource resolution before the export timer, output writing and
teardown are outside phase sums. `render_ms` covers full export excluding its
nested subset timer; it is not stream time. Context/layout/shape boundaries can
contain stabilization and metrics work.

Vanilla traces contain nested spans. Use unions/exclusive intervals on the
single thread, never sum nested spans as elapsed time. Compare only broad
compatible compile/eval-layout/export envelopes and retain unattributed residuals.
No microbenchmark with a different fixture is added to an end-to-end total. A
phase's participation is descriptive attribution within its measured boundary;
a causal explanation needs a controlled intervention, proposed in the analysis.

External system-font digests are reused only when device, inode, size, mtime
and ctime remain identical; a changed stat signature forces byte rehashing.
This avoids rereading several GiB before every short target and is recorded
in `font-digest-cache.json`. Production source/input files are always byte-hashed.
Patches are content-addressed gzip files; each snapshot links its exact patch.

Libertinus and DejaVu embedded face bytes match across asset revisions; NewCM
faces do not. Qualification rejected the attempted math control: Tekt does not
support the vanilla math.equation show selector, and a document text-font setting
is overridden by equation defaults on both sides. The uniquely named diagnostic
`Baseline Math` face is supplied through the same font path to both engines but
is unused by the qualified fixtures. Its hashes, creation recipe and unsuccessful
qualification are retained. No matched math end-to-end ratio is reported; the
existing Tekt math microbenchmarks still run. The initial flow-shape candidate
also produced different pagination; the active shape fixture uses fixed positions
inside one page. These are diagnostic fixture changes, never benchmark edits.

The font inventory is rediscovered at every fingerprint boundary (fontconfig
and known font directories); added/deleted faces invalidate a block. The process
monitor also rejects other Python reproduction/probe/test/benchmark scripts.
Only measurement/preparation scripts enter the timing fingerprint; later analysis
scripts and Markdown results cannot alter executable behavior.

Preparation refinement: builds collect no performance samples. They may overlap
other preparation work if consumers/inputs remain unchanged; Cargo serialization,
JSON profiles and final executable fingerprints identify their products. Build
receipts explicitly record concurrent preparation. Actual sampling still requires
the quiet gate and rejects any observed overlapping build/test/benchmark/lint.
Earlier preparation receipts rejected under the initial stricter rule are kept.

Resuming the same command preserves accepted blocks only when their source
fingerprint still matches the built state. Failed/incomplete blocks and their
receipts/logs/snapshots move to `rejected/` before repetition. Every block checks
the build fingerprint, including changes between targets; all build products
must share the final source fingerprint. Analysis does not pool retired records.

The convenience controller retries only isolation/contamination failures:
`python3 -u 00_nucleo/diagnosticos/performance-baseline/collect.py micro`,
then `collect.py e2e`, then `collect.py e2e --instrumented`. It starts no parallel
work and collects no timing itself. Exact attempts are in `collection-*.json`.
`python3 00_nucleo/diagnosticos/performance-baseline/analyze.py` recomputes all
summaries; `validate.py` performs untimed output checks after collection.

The detector covers `ctest`, Go `.test` executables and benchmark executable
names including hyphenated names. When interference is observed during a
microbenchmark, the collector terminates only its own process group and retains
the incomplete, rejected raw data. It never interrupts another task.

The CLI numbers apply to the default embedded/system font universe **plus one
unused project face supplied by the diagnostic font path**. Its acquisition
remains part of wall time even though it is not selected for visible text.
The effect of this extra face is not isolated in the primary comparison; an
option-only A/B control is specified in `controlled-experiments.md`.

Collection adaptation: `finish_micro.py` runs the unchanged short targets first,
then inventories Content through its existing Criterion `--list` mode and runs
each existing group with a positional regex filter. All original Content cases
must appear in the inventory and accepted output for all three rounds. Each
group is a separate process and receipt, with identical sample/warmup/bootstrap
options. There is no benchmark-source or operation/timing-loop change. Inter-group
warm-process history differs from an unfiltered invocation and is not a causal
comparison with historical full-target results. `content-filter-plan.json` records
the exact cases/groups. Each receipt additionally captures the new ordering
script's SHA-256 before/after and rejects drift. Original source/input fingerprints
and executable hashes still match the recorded build. This change of scheduling
preserves the completed CLI populations and their exact original script identities.

The same existing-group filtering is applied to pipeline, whose full-target
attempts were all rejected. Observation and stdlib remain their original fixed
whole-test entrypoints; they run before public targets. No accepted case mixes
full-target and group-filtered invocations between its three rounds. Driver
revisions are archived by SHA-256 under `driver-revisions/`. Pipeline and Content
inventories are recorded in their respective `*-filter-plan.json` files.

After all sampling, bulky state JSON records are compressed losslessly to
`.json.gz`; `packed-records.json` maps original and stored SHA-256 values.
`MANIFEST.sha256` covers stored raw data, scripts and the sibling analysis.
The final execution recipe uses `finish_micro.py` for the microbenchmark matrix.
The original `run.py micro` full-target recipe remains available as historical
protocol preparation, with its rejected records retained; it is not required to
recreate the accepted filtered pipeline/Content observations.
