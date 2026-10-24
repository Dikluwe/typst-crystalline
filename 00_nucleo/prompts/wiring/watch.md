# L0 — wiring/watch
Hash do Código: cea521c7

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/watch-font-generation.toml sha256:0f2985807cc3c7b4aeed9a5a26b77ad66dbbdb67ccfb43ba8336dbc8e8eea1a5
- 00_nucleo/prompts/_nuclei/watch-source-generation.toml sha256:c7f9b3d18b112b1d725e082b7da75b886bf7120346ffbca911c236ea65c52986
- 00_nucleo/prompts/_nuclei/wiring/cli-observables.toml sha256:e9d6623a18ffc35f36051b280d96a75a0e4c1bd1768361a3a50c98587c568aef

**Layer:** L4
**Exclusive consumer:** `04_wiring/src/watch.rs`

```text
```

## Current evidence

- `04_wiring/src/watch.rs:14` — `run_watch`.
- `04_wiring/src/watch.rs:113` — `write_watch_block`.
- `04_wiring/src/watch.rs:118` — `emit_watch_block`.
- `04_wiring/src/watch.rs:124` — `watch_line_at`.
- `04_wiring/src/watch.rs:128` — `watch_block_at`.

These are local source anchors, not a new upstream parity measurement.
Reproduce discovery with `rg -n "fn |struct |enum |mod |pub use" 04_wiring/src/watch.rs`.

## Owned contract

- Own run_watch and local status/write/time/cycle helpers. Reuse the compile owner’s private observed helper with the session below, L3 watch/server operations and the L4 eviction wrapper (`00_nucleo/prompts/wiring/eviction.md`); no local polling, fingerprinting or HTTP algorithm.
- Reject stdout output. Serve semantic Html/Bundle unless no_serve, preserving port/no_reload/URL/PID staging and the existing event loop. Never infer HTML from an asset extension.
- Capture Instant before compilation; report status before diagnostics. Map elapsed/warnings/exit into typed WatchStatus with local time or UTC fallback. Compiling uses one LF, other blocks two; write_all then flush with the fallible stderr contract below; the earlier stderr expect is superseded.
- Preserve dependency fallback then arm, successful publish/publish_directory or failed abandon/abandon_directory. Publication failure exits 2; HTTP state changes only on successful publish and the last disk/HTTP artifact survives errors.
- Keep evict(10) and wait(100ms) call placement. Published is privately imported by the parent from typst_infra::watch. Local helper tests characterize status/order, not a new watcher or timing policy.

## Verification and boundary

Verify this unit through its existing family/CLI tests when validating behavior.
Ownership is strictly prompt-to-consumer 1:1 (ADR-0129); referenced owners do not share this prompt.
Preserve existing public contracts, defaults and pipeline phases. Referenced child owners govern their own implementations; dependencies do not create shared ownership.


## Retained resources and watcher lifetime

### Evidence and approvals

`lab/results/construction/diagnostics/watch-retencao-2026-10-07.md` records
HEAD/tree/binaries/UTC, real edits/recovery and dynamic acquisition counts per
cycle. `watch-validacao-2026-10-07/measurement.json` compares discovery validation
and full acquisition at the same state; controls.json covers addition/removal/
replacement/restored mtime/links/configuration. These are isolated costs, not CLI
speedup or upstream-intent proof. FontBook/slots/coverage must preserve language
selection/content/diagnostics/render; duration/Arc are mechanics. Potential benefit
is an inference refuted by cost, stale resources, regression or unbounded retention.
Fixtures/provenance/limits remain in the report.

The owner ratified the catalog package under ADR-0127 §2.1/§2.2 with `Confirmo`:
`lab/history/construction/records/00_nucleo/materialization/watch-catalogo-2026-10-07/manifest.json`;
receipt `lab/results/construction/diagnostics/watch-catalogo-execucao-2026-10-07/ratification.json`.
Every owner remains 1:1.

`incremental-prioridade6-2026-10-08/{before.json.gz,measurement.json.gz,control-measurement.json.gz,count.json}`
records tree/HEAD/UTC/commands/releases. Source::new rebuilds CST; new World builds
main/import Source on each edit. Even a minimal import consumes resources in
eval_ms; do not attribute the whole metric to parsing. Existing owners govern
Source/id/path causal identity; reuse is mechanics preserving language observables.
Cost, diagnostics/effects, stale source, geometry or identity changes refute the
benefit hypothesis; no upstream intent is inferred. The source-session contract
was ratified as stated in the archived owner record.

### Session contracts and acceptance

Create Arc<FontCatalogSession> once per run_watch, outside the loop, without font
discovery. Pass clones to compile's private observed helper every cycle, retaining
CompileIntent/staging. Each call still constructs a new World; source/error recovery
does not require resetting it. End the session with the watcher, never global
retention. Preserve status-before-diagnostics, Instant/timers, deps/fallback,
arm-before-publication, evict(10), 100ms polling and last successful HTTP state.
Do not observe fonts as new watch events or change server policy/modes/flags.

Real-process acceptance covers main/import/read/PNG edits, error/creation-after-
negative/recovery and font addition/removal/replacement on the next recompilation.
Measure startup separately from cycles, CPU/latency/RSS and distinct counters;
compare the frozen prior binary and pinned vanilla. If improvement is not repeatable
or any regression occurs, retain negative evidence and reevaluate the productive
path; do not claim optimization complete.

Create Arc<SourceSession> once per run_watch outside the loop, independently of
the catalog session. Pass clones to compile's private helper per cycle while World
remains new. The watcher call bounds lifetime/retention. Preserve staging/HTTP/last
success, recovery/deps/fallback, arm-before-publication, evict(10) and 100ms polling;
no new public flag/mode. Acceptance requires RED→GREEN, production counts separate
from timing, three paired CLI rounds against the archived prior release, startup/
RSS and invalidation/identity/effects/Unicode/diagnostic/assets/font/geometry
controls. Reseal only this owner, run relevant builds/suites and lint with zero
errors; this does not close full incrementality.

## Publication and internal expression boundaries

### Evidence

`clippy-expressions-2026-10-16/before.json.gz` retains sources/L0, HEAD/uncommitted
tree/status/diffstat/UTC; initial-diagnostics.json retains suggestions/spans.
`watch.rs:94` triggered clippy::collapsible_else_if; uses/types were inspected.
This is internal Rust representation, no new parity/performance claim.

`tekt-v12-2026-10-16/before.json.gz` records HEAD
`06848cdcb5450f2c555626bf116bc7acd82bd82b`, uncommitted tree/UTC/sources/L0/hashes/
status/diffstat and the hash-checked inventory: 19 V12, one productive Published
enum in main.rs and 18 auxiliary test declarations. Html(String) and
Bundle(Vec<BundleFile>) are produced in compile.rs and dispatched in watch.rs.
The earlier L4-local carrier permission is explicitly superseded by L3 ownership;
this is representation, not upstream intent or parity. Full historical clauses:
`lab/history/prompt-consolidation-2026-10-18/records/00_nucleo/prompts/wiring/watch.md`.

### Contract and acceptance

Flatten only failed-publication discard `else { if Bundle ... }` into `else if
Bundle`, retaining abandon_directory versus abandon, staging, successful-only
publication, return and previous state. Continuous ADR-0127: no public API,
default, phase, dependency or suppression. L0 precedes code, resealing is scoped,
existing control inputs/results/tolerances remain exact. Require selected-lint
RED→GREEN, workspace suite/build/all-target Clippy/fmt/Rustdoc/lineage; no tests
merely mirroring Rust syntax.

The static Html/Bundle match stays in place: Html calls publish_html, Bundle calls
publish with the same borrows. Never infer target from filename or change
publication/wait/cache/server/snapshots/status. Carrier ownership is
`00_nucleo/prompts/infra/watch.md`, privately imported by main; its transfer must
preserve the real watch HTTP/reload/flags/error/last-snapshot and compile full-tree/
byte-exact-assets/stdout/warnings/exit/no-publication-on-error controls specified
in `00_nucleo/prompts/wiring/main.md`. Require real V12 RED and zero productive
V12 afterward, diff-only transfer/import proof, build, strict workspace/all-target
Clippy (-D warnings, -D unfulfilled_lint_expectations), workspace suite/fmt,
Rustdoc -D warnings/four indexes, V1/V5/V15/V26/reverse lineage/SARIF/diff --check.
No syntax-only owner-transfer tests. Heavy runs sequential, MemoryMax=12 GiB,
MemorySwapMax=0, CARGO_BUILD_JOBS=1; no release publication.


## Approved failed dependency graph handling — approved 2026-10-22

Consume the private compile result without flattening graph failure into empty.
Only Ok(empty) retains the existing input fallback; Ok(nonempty) retains the
original graph. Err is fatal operational exit 2: do not arm, publish, update
HTTP state, evict/wait as a successful cycle, or use previous/invented graph.
Dispose of any current staging artifact through an existing fallible discard
operation without publishing; preserve the last successful disk/HTTP state.
Report any discard failure explicitly as well, without panic or suppression.
Healthy cycles keep status/order, sessions, arming/publication, recovery,
retention, HTTP controls and timings. This deliberate exceptional behavior
change is part of the approval, not a new mode/flag or eval/layout phase.


Approval of this exact metadata correction is retained in `lab/results/world-metadata-errors-2026-10-22/approval.json`, review SHA-256 `31d632a54d40ca7df4139081c5297af774970a93b08cd605986b53e75e0ccc62`. The approved clauses supersede prior infallible signature/return preservation only for the two public APIs and the described private composition interfaces.

Private dependency gating may accept discard/arm composition closures for tests. Exercise existing disk/HTTP state under failed graph, verify no arming/publication, exact original and cleanup errors, healthy empty fallback, and a separate healthy invocation. Missing staging is a legitimate successful discard; other cleanup errors remain explicit. New discard composition may use the existing std::fs fallible remove operations; do not add a public L3 function.


## Fallible stderr/watch boundary — authorized 2026-10-22

### Measurement before decision

The immutable baseline is `lab/results/stderr-watch-errors-2026-10-22/agents/measurement/before/receipt.json`
(SHA-256 e49aa4458f4cb413995af35f8297ecf4fcafd20141cbdab41067fc7ed5b3895c), with the later real-cycle control in
`lab/results/stderr-watch-errors-2026-10-22/agents/measurement/late-before/receipt.json`
(SHA-256 07ddcb85110604877f56d369dec0d2ac3c7d96ba9ac95f27d9e4e49469f3435c). Each receipt retains binary/source state, HEAD, changed-file
stat, UTC, commands and outcome. Before modification, `watch.rs:59` writes the
compiling status, `:60-64` installs the unit-return terminal callback, `:171`
expects successful stderr write/flush. URL/errors at `:17`, `:32`, `:36`, `:80`
and `:95` use panic-capable stderr macros. The compile owner's `compile.rs:620`
reports terminal status only after staging writes. The source inventory and its
hashes are retained in the watch agent audit/provenance under that result root.
The measured failure is operational I/O, not language semantics or inferred
upstream intent. A remaining panic, postfailure action, changed healthy diagnostic,
status ordering or last-output regression refutes the correction.

### Private contract and acceptance

User authorization to correct stderr/watch applies to this internal error
propagation, under ADR-0127 continuous flow: no public API, new flag/default mode,
eval/layout phase or compatibility change. Supersede the earlier stderr expect
only; preserve all healthy contracts and the separately approved metadata gate.

`run_watch` returns private `Result<ExitCode, Vec<io::Error>>`; all stderr emission
uses fallible shared composition or the existing write-all/flush status helper.
Retain the actual first sink error; do not stringify/replace its kind, custom
payload or OS metadata. Main terminates with operational exit 2 and must not retry
the failed sink or write human diagnostics to stdout. Compiling status failure
returns before compile. Private compile callback returns `io::Result<()>`; status
remains before diagnostics and no diagnostics run after its error.

A production-used private postcompile gate accepts compile's outer `io::Result`.
On Err, discard the current file/directory staging through existing fallible
remove operations, then return actual errors in primary/cleanup order. Missing
staging is legitimate absence; every other discard error remains an actual
`io::Error`. Never arm dependencies, publish, update HTTP/reload, evict, or wait
after sink failure. Keep the prior disk artifact and prior live HTTP snapshot
unchanged until normal server teardown on return. Do not claim a terminated
watcher continues serving. No error latch, panic, suppression, poisoned-state
recovery, fake success, invented graph or fallback resource is permitted.

Keep Instant capture, exact status bytes/one-two LF, local time fallback, sessions,
healthy dependency capture/fallback, arm-before-publish, Html/Bundle dispatch,
error/recovery and no-serve/no-reload/port behavior, eviction(10), wait(100ms) and
existing L3 snapshot/finalization APIs. URL and error messages retain healthy text
and order. Existing L3 best-effort publication cleanup is not redefined here.

Tests precede implementation. Use the production-used private gate with actual
write/flush failures, file and bundle staging, prior disk content and real local
HTTP state; assert no continuation actions and retain primary/cleanup errors,
then verify a separate healthy invocation. Preserve prior status helper tests
unchanged. Runtime `/dev/full` and closed-pipe cases must terminate with exit 2,
no leaked staging/replaced valid destination/stdout diagnostics; ordinary healthy
and semantic-failure controls retain their results. Require workspace build/tests,
strict Clippy/rustdoc, parity, paths, hashes and lineage without weakened criteria.


## Exact output manifest on failed stderr — measured continuation

### Measurement before decision

`lab/results/stderr-watch-errors-2026-10-22/agents/measurement/templates-before/receipt.json`
(SHA-256 76364c29509b7269c8d54451472ee25e04a7c57e3e80d1e42bc4af66e2aa6072)
records the preserved baseline binary/build receipt, HEAD, uncommitted source/hash
state, exact commands and UTC. One-page and two-page SVG output templates expand
staging into exact sibling paths. The prior stderr-closed cases terminate with
101 and leave those files; healthy-stderr controls also expose the existing
literal-template publication mismatch with exit 2. Sentinels at the final
literal/expanded destinations remain unchanged. This receipt separates the
failed-stderr cleanup cause from the independently open healthy publication debt.
The current source anchors are `03_infra/src/pipeline/svg.rs:210` (expanded output
paths) and `04_wiring/src/watch.rs:77-85` (compile error cleanup only knows literal
staging). Source inventory/provenance already precedes this decision.

### Narrow private correction

Consume a per-cycle `Vec<PathBuf>` populated by compile immediately before each
output write attempt, using exact existing L3 destination paths for SVG and the
existing staging root for ordinary file/bundle output. Pass that vector through
a private out-parameter only; this authorizes no new public API or L3 entity.
The observed private tuple drops its unused returned warning bool; terminal status
still receives the actual warning bool through its existing fallible callback.

On compile outer I/O Err, cleanup iterates all exact recorded output paths in
manifest order, using existing format-appropriate fallible removal. Never glob,
scan sibling directories, reconstruct page templates, infer other paths, or target
unrecorded files/final destinations. Continue attempting the remaining exact paths
after a cleanup error. Missing recorded paths are valid absence; retain every
other actual `io::Error`. Return actual errors ordered primary sink error followed
by all cleanup errors, without stringification or replacement. The postcompile
gate takes cleanup's `Result<(), Vec<io::Error>>` and extends its original error
vector. No arm/publish/HTTP/reload/eviction/wait follows a sink failure.

Preserve all healthy behavior and prior tests. Healthy SVG-template publication
remains explicitly OPEN; do not silently broaden this correction into multi-output
publication, new phase ordering or a public API. Tests first must prove exact
multiple-output cleanup, continuation after independent cleanup errors, preserved
primary/custom/OS metadata, missing recorded path handling, untouched final and
unrecorded neighboring sentinel files, plus prior file/bundle/live HTTP controls.
Runtime remeasurement must show closed stderr gives normal exit 2 with all exact
recorded staging removed and unchanged final sentinels; healthy-template controls
retain their open debt. Standard tests/parity/hash/lineage gates remain unchanged.


## Fatal publication error cleanup and fallible reporting

The same immutable template baseline receipt cited above (SHA-256
76364c29509b7269c8d54451472ee25e04a7c57e3e80d1e42bc4af66e2aa6072)
measures a publication error with expanded staging left behind even when stderr
is healthy. In the first stderr correction, `watch.rs:110-121` reports that fatal
publication error through a fallible sink, but does not yet discard the exact
written-output manifest. A failure of that report can therefore retain the same
staging. This is a measured continuation of the existing failure boundary, not
proof that healthy template publication works.

Before returning from a fatal publication error, attempt cleanup of every exact
recorded output using the same manifest helper, including when its error report
will fail. Preserve the original typed publication `io::Error`, every actual
cleanup error in manifest order, and any subsequent report `io::Error`; never
replace their kind, custom payload or OS metadata with text. Return this private
error vector to the existing main exit-2 boundary, whether reporting succeeds or
fails. Emit the original publication error text exactly as before when stderr
works, then explicit cleanup-error messages if any. Stop at the first reporting
failure and never retry that sink. Cleanup must already have run before reporting.
A private composition helper may accept a fallible message emitter for tests.

This continuation removes owned staging after failed publication with healthy
stderr as well; it still does not fix the healthy SVG-template publication
mismatch, which remains OPEN with normal exit 2 and preserved final sentinels.
No template expansion, glob, unrecorded-path cleanup, public API or phase change
is authorized. Tests first exercise actual L3 ArmedWatch publication failure,
exact written manifests and previous valid destinations, proving cleanup before
reporting, unchanged successful-report primary text, retained publication/cleanup/
reporting errors, and no sink retry after reporting failure. Retain every previous
acceptance condition, runtime measurement and full project validation gate.
