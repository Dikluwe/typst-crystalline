# L0 — wiring/watch
Hash do Código: 89e4b093

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
- Capture Instant before compilation; report status before diagnostics. Map elapsed/warnings/exit into typed WatchStatus with local time or UTC fallback. Compiling uses one LF, other blocks two; write_all then flush with existing propagated errors/stderr expect.
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
