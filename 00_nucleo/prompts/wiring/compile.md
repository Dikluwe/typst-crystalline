# L0 — wiring/compile
Hash do Código: 561e2ab5

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/compiler-feature-gates.toml sha256:09b32666fb1b1fa7ba8f2ef7c979c60cbc64cc515b8d9c46818bec19d7471fb1
- 00_nucleo/prompts/_nuclei/export/format-option-precedence.toml sha256:9c4f3ce07c56762e4c588a0f223c3ea1e7e50d6c00f83ff457fad766b0cea3cf
- 00_nucleo/prompts/_nuclei/network/custom-ca-cert.toml sha256:4b9f69ff41790a55a5b2d6a7b9982da858092e4bb2f0c503d6332a14a510e3ec
- 00_nucleo/prompts/_nuclei/watch-font-generation.toml sha256:0f2985807cc3c7b4aeed9a5a26b77ad66dbbdb67ccfb43ba8336dbc8e8eea1a5
- 00_nucleo/prompts/_nuclei/watch-source-generation.toml sha256:c7f9b3d18b112b1d725e082b7da75b886bf7120346ffbca911c236ea65c52986
- 00_nucleo/prompts/_nuclei/wiring/cli-observables.toml sha256:e9d6623a18ffc35f36051b280d96a75a0e4c1bd1768361a3a50c98587c568aef

**Layer:** L4
**Exclusive consumer:** `04_wiring/src/compile.rs`

```text
```

## Current evidence

- `04_wiring/src/compile.rs:40` — `report_in_order`.
- `04_wiring/src/compile.rs:45` — `write_bundle_output`.
- `04_wiring/src/compile.rs:81` — `run_compile`.
- `04_wiring/src/compile.rs:86` — `run_compile_observed`.
- `04_wiring/src/compile.rs:546` — `p1323_html_experimental_warning_matches_ratified_envelope`.

These are local source anchors, not a new upstream parity measurement.
Reproduce discovery with `rg -n "fn |struct |enum |mod |pub use" 04_wiring/src/compile.rs`.

## Owned contract

L4 composes the existing L1 domain, L2 presentation and L3 infrastructure. Font/file/package acquisition, compilation and exporting remain delegated to L3. Existing stdout/stderr and output-tree publication calls are wiring orchestration (ADR-0049/0050); they do not authorize moving domain algorithms or resource acquisition into L4.

- Own run_compile/run_compile_observed, report_in_order, write_bundle_output and experimental-warning constants. Preserve observed return values (exit, dependencies, input kind, optional Published) and the no-op-observer wrapper.
- Keep pre-World validation order: bundle stdout/feature; explicit PDF pages+tagged; CLI-only accessible-standard/tagging with conditional pages hint; conflicting PdfStandards. Preserve early exits and callback order, without implicit feature activation.
- Construct SystemWorld with current root/main/font/input/CA/plugin configuration. Map compact only to StreamMode; preserve tagging legacy false/explicit Some/absent None and pretty/pages/standard/PPI typed overrides until L3 resolves them. PDF oracle selection is PDF-only.
- Leave font acquisition to L3 resource demand; normal compilation does not invoke explicit source coverage preloading. System/project fonts remain available through the unchanged builders and World contract.
- Keep exhaustive target dispatch Bundle/Pdf/Png/Svg/Html, every timing/non-timing path and all BundleCompileOptions fields. Map HTML serialization to the existing L3 enum only; never perform layout/export or reconstruct format defaults in L4.
- On successful output write bytes or complete BundleFile tree. Preserve normal-component checks, parent creation, canonical/symlink/escape checks and existing publication policy; do not add incidental hardening or atomicity changes.
- Compilation errors exit 1; write/timing failures exit 2; success exits 0 and alone produces Published. report_in_order invokes status then diagnostics once: errors, conditional experimental warnings, domain warnings, then operational error. Preserve opt-in CRYSTALLINE_MEASUREMENTS=1 through the existing L3 APIs.

## Verification and boundary

Verify this unit through its existing family/CLI tests when validating behavior.
Ownership is strictly prompt-to-consumer 1:1 (ADR-0129); referenced owners do not share this prompt.
Preserve existing public contracts, defaults and pipeline phases. Referenced child owners govern their own implementations; dependencies do not create shared ownership.

## Evidence supporting specialized contracts

The original clauses and construction limits are archived at
`lab/history/prompt-consolidation-2026-10-18/records/00_nucleo/prompts/wiring/compile.md`.
The following measurements are historical evidence, not fresh certification of
the current upstream pin.

- `lab/results/construction/diagnostics/pdf-standard-bundle-2026-09-30/results.json`
  observes missing-title direct PDF rejection under `--pdf-standard=ua-1` while a
  bundle PDF is published. `compile.rs:155` constructs standard_override only for
  direct PDF; `:325` therefore passes None into BundleCompileOptions. L3 bundle
  already resolves the field per entry. The missing-connection hypothesis is
  refuted by Some(PdfStandards) reaching bundle; the measured code sent None.
- `compile-stdin-2026-10-06/before.json.gz` retains HEAD/tree/UTC, sources/commands/
  binary SHA-256. With physical file `-` containing #unknown_physical and separate
  stdin, Tekt read the file; historical vanilla
  `9f2b6e8715237cb086899a42873660fe744622e8` read stdin. `compile.rs:223–270` always
  built SystemWorld::new; `world.rs:102–106` already offered for_stdin without a
  physical read. Upstream `args.rs:164` explicitly documents `-` as stdin,
  world.rs:222–242 validates root/selects stdin identity, :282–290 acquires bytes.
  The documented option establishes intent; FileId/lazy acquisition is mechanics.
  Origin/diagnostics/resources/results are observable. The World-selection cause
  is refuted if only that composition change fails to preserve control inputs/results.
  green.log records residual RED for stdout, where compile.rs:487–492 wrote all
  non-bundle outputs with std::fs::write. after.json.gz root_error refutes an
  unconditional `<stdin>` caption: root project under parent cwd displays
  `project/<stdin>:1:1`.
- `plano-debitos-2026-10-06/before.json.gz` measures warning/error order;
  upstream compile.rs:779 chains errors before warnings, contrary to the old
  local/ADR-0045 §8 order. Human `confirmo` on 2026-10-06 approved the amendment,
  candidates/hashes in
  `lab/history/construction/records/00_nucleo/materialization/warnings-cli-aprovacao-2026-10-06.json`.
  `warnings-cli-2026-10-06/before.json.gz` HTML warning/error/only controls refute
  placing experimental warnings after domain warnings. Internal warning order
  survives; only errors move ahead.
- `watch-retencao-2026-10-07.md` records HEAD/tree/binaries/UTC, actual edits,
  recovery and dynamic acquisition counts. `watch-validacao-2026-10-07/measurement.json`
  compares discovery validation versus full acquisition at the same state;
  controls.json covers added/removed/replaced/restored-mtime fonts, links and
  configuration. Costs are isolated, not a CLI speedup or upstream-intent proof.
  FontBook/slots/coverage must preserve language selection/content/diagnostics/
  render; Arc/duration are mechanics. Cost, stale resources, regression or unbounded
  retention refute the possible-benefit inference. Fixtures/provenance/limits
  remain in those reports. Owner `Confirmo` ratified
  `lab/history/construction/records/00_nucleo/materialization/watch-catalogo-2026-10-07/manifest.json`;
  receipt `watch-catalogo-execucao-2026-10-07/ratification.json`, under ADR-0127
  §2.1/§2.2 with 1:1 ownership.
- `incremental-prioridade6-2026-10-08/{before.json.gz,measurement.json.gz,control-measurement.json.gz,count.json}`
  records tree/HEAD/UTC/commands/releases. Source::new rebuilds CST and new World
  rebuilds main/import Sources per edit; a minimal import also costs eval_ms, so
  not all of that metric is parsing. Source/id/path identity follows existing
  owners; reuse is mechanics preserving language observables. Cost, diagnostics,
  effects, stale source, geometry or different identity refute the benefit
  hypothesis. No upstream intent is inferred; the source-session contract is
  ratified as recorded in the archive.

## Bundle-standard proposal: gate remains pending

If authorized, L4 validates explicit standards with PdfStandards::new for direct
PDF and Bundle, preserving absence as None, and passes Option<PdfStandards> to
BundleCompileOptions without rebuilding each entry's options. L3 applies
caller > document > default and PDF validation only to PDF entries; errors block
publication of the collection. Accessible-standard/tag CLI-only checks remain
for direct PDF; bundle validates documents per entry. Other pre-World order and
non-PDF targets stay the same. This changes the flag's observable effect and
requires owner ADR-0127 confirmation before implementation. After approval,
update/reseal the current owner and verify RED→GREEN. No other historical
unratified proposal is authorized by the session or carrier work below.

## Stdin and stdout composition

For input exactly `-`, validate/canonicalize the root already resolved by L2,
acquire stdin bytes once, accept partial BrokenPipe as the pin does and validate
UTF-8. Missing root exits 1 with `root directory not found (searched at ...)`;
invalid UTF-8 exits 1 with `file is not valid UTF-8` and the extensionless-file
hint. Other acquisition failures exit 2 with operational diagnostics. Invoke
the status callback once before diagnostics on exits in this branch.

Compose SystemWorld::for_stdin using acquired text and unchanged font/input/CA/
plugin builders. Never open physical `-` or create a document file. Imports/read
use the supplied root; stdin retains causal identity and cwd-relative diagnostics/
traces without renaming import origins. Feed the existing Source through all
format/timing branches, preserving publication/warnings/phases and public
signatures/flags. Root/caption uses the existing formatter and World's synthetic
root/<stdin>, not an overriding primary: cwd-root displays `<stdin>`, a child root
displays `project/<stdin>`. Preserve both bilateral controls and explicit-root
traces; correct expectations from the recorded upstream refutation.

Publish output exactly `-` through stdout.lock().write_all at the existing
non-bundle publication point using identical exported bytes/error handling.
This also applies to physical input. Bundle refuses stdout before World;
watch also refuses stdout. The physical file `-` must not be overwritten.

Black-box regressions distinguish stdin from file `-`, PDF file/stdout,
SVG/PNG/HTML, inputs/import/read under explicit root, causal error/trace,
`**` warning and invalid UTF-8. Assert success/status/stderr and PDF signature
on stdout; compare language/diagnostics, not entire PDF/SVG bytes or internal ids.
This internal parity repair follows continuous ADR-0127 without public/default/
phase changes. The original stdin slice explicitly left required-output parsing
incomplete; that separate L2 obligation is now specified by
`00_nucleo/prompts/shell/cli.md`, not implemented here. The earlier slice's
warning/error-order preservation is superseded only by the approved order below.

## Diagnostic order

Observed status precedes every diagnostic. Failure drains errors, experimental
format warnings, domain warnings and finally operational errors. Success keeps
experimental warnings before domain warnings. Preserve message/span/hint/trace,
color, exit and publication. The approved amendment changes only error placement;
warning order remains intact.

## Optional watch-resource sessions

Keep public run_compile and run_compile_observed compatible, delegating ordinary
compile without sessions. A private/pub(super) observed helper accepts optional
Arc<FontCatalogSession>; None retains with_fonts_and_system, Some uses
with_font_catalog_session with unchanged font_paths. Only watch supplies it.
Build a new SystemWorld every call; preserve validations/inputs/CA/plugins/source
before pipeline, existing deps/exit/input-kind/Published return, stdout/stdin,
all targets and option/export/PDF-publication precedence. No phase/diagnostic/
flag/timer change, global session or prior-World reuse.

Transport optional Arc<SourceSession> through the same private helper alongside
the catalog session. Public compile/observed signatures do not change and pass
no syntax session. Physical input with a session uses new_with_source_session
before eager reading; no session/transient input keeps the existing path. L4
implements no validation/cache algorithm. Preserve errors, inputs/font_paths/
plugins/cert/targets/staging/deps/timers and diagnostic order.

Acceptance: RED→GREEN; production counts separate from timing; three paired CLI
rounds against the archived prior release, startup/RSS and invalidation/identity/
effects/Unicode/diagnostic/assets/font/geometry controls. Reseal only this owner,
run relevant builds/suites and lint with zero errors. Do not close full
incrementality or call unrepeatable/regressing retention an optimization.

## Internal Rust expression and lint boundaries

### Evidence

`ci-simplificacoes-2026-10-14/before.json.gz` retains source/owner/spans, HEAD/
uncommitted tree/UTC/diffstat; red.log records selected lints as errors before
editing. compile.rs lines 523/547 contain the flagged expressions, with types/
uses checked under pinned toolchain 1.92.0. `type-complexity-2026-10-15/before.json.gz`
retains HEAD/tree/UTC/diffstat/sources/owners/hashes; red-clippy.log fails before
editing at compile.rs:174 (archived inventory item C128). Inspection distinguishes
positional tuples, repeated FontKey aliases and existing explicit static contracts.
These are Rust-representation maintenance, not new parity, functional bug or
AI-benefit measurements.

### Contracts and acceptance

Both timing-write paths already have &PathBuf: pass path directly to std::fs::write,
preserving calls/path/JSON/error handling/publication/status order. No acquisition/
pipeline change, new public API/default/phase/unsafe/dependency/lint suppression.
L0 first, owner-only resealing. Use existing functional controls without fixture/
expectation/tolerance changes or tests merely mirroring simplification. Require
selected-lint RED→GREEN then full suite/build/all-targets/Rustdoc/fmt/lineage/
full inventory.

Keep local ACCESSIBLE as its four existing PdfStandard/label-factory pairs without
a new L4 type. Only that const may carry
`#[expect(clippy::type_complexity, reason = "ACCESSIBLE: composição estática de standards e labels existentes")]`.
Preserve entries/order/lazy label evaluation/message/hint and pre-World CLI
validation. Type abbreviation is allowed only with provably identical expansion;
incompatibility/new public type requires another gate. This nominal exception is
only type_complexity, never module/impl/crate suppression or a higher threshold.
It narrowly supersedes prior generic bans only for the three justified historical
batch items; other correction cases remain unsuppressible. No resolved public
entity/signature/default/phase change. Existing functional expectations remain;
selected gate RED→GREEN includes unfulfilled_lint_expectations as error, followed
by build/all-target Clippy/workspace suite/fmt/Rustdoc/lineage and scoped resealing.
No tests merely mirroring aliases/field names.

## Publication carrier boundary

### Evidence

`tekt-v12-2026-10-16/before.json.gz` retains HEAD
`06848cdcb5450f2c555626bf116bc7acd82bd82b`, uncommitted tree/UTC/sources/L0/hashes/
status/diffstat. The hash-checked inventory reported 19 V12: one productive local
Published enum at main.rs:22, 18 auxiliary test declarations. Html(String) and
Bundle(Vec<BundleFile>) are produced at compile.rs:90/99/363/539/541 and dispatched
at watch.rs:103/104. This explicitly supersedes the former private-L4-carrier
permission, a Rust ownership decision without new parity/upstream-intent claim.

### Contract and acceptance

The internal return remains Option<Published>, now resolved through the parent's
private `typst_infra::watch::Published` import. Preserve Html/Bundle producers,
every success/error condition, moved payloads, order and absent None. Add no
conversion/method/publication logic or target inference from extensions.

Preserve real watch HTML/bundle HTTP/reload/flags/success/error/last-snapshot
controls and compile full-tree/byte-exact assets/stdout/warnings/exit/no-publication-
on-error controls specified in `00_nucleo/prompts/wiring/main.md`, including its
exact named watch tests and L3 HTTP controls. Require real V12 RED before and zero
productive V12 after, transfer/import-only diff, no owner-transfer syntax tests.
Validate build, strict workspace/all-target Clippy (-D warnings and
-D unfulfilled_lint_expectations), workspace suite/fmt, Rustdoc -D warnings/four
indexes, V1/V5/V15/V26/reverse lineage, SARIF and diff --check. Heavy runs are
sequential under MemoryMax=12 GiB, MemorySwapMax=0, CARGO_BUILD_JOBS=1; no release.
