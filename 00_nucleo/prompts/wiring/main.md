# L0 — wiring/main
Hash do Código: bd7358aa

**Layer:** L4
**Exclusive consumer:** `04_wiring/src/main.rs`

Pinned Tekt dependencies (canonical linter metadata):

```text
Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/wiring/cli-observables.toml sha256:e9d6623a18ffc35f36051b280d96a75a0e4c1bd1768361a3a50c98587c568aef
```

## Current evidence

- `04_wiring/src/main.rs` — private import of `typst_infra::watch::Published` (the historical local declaration is superseded below).
- `04_wiring/src/main.rs:27` — `main`.
- `04_wiring/src/main.rs:43` — `main_is_only_a_closed_command_dispatcher`.

These are local source anchors, not a new upstream parity measurement.

## Owned contract

- Own only binary composition, the closed exhaustive RunIntent dispatcher and the private import of the L3 Published carrier (Html string or Bundle files). Preserve all command arms and static delegation to private child handlers.
- Compose L2 intents, L3 operations and L1 values; no CLI parser, serializer, language policy, rendering, TLS, polling or watcher algorithm belongs here.
- Handler bodies, diagnostic drain and eviction belong to their respective owners. The import remains private in L4; the transient carrier is owned by 00_nucleo/prompts/infra/watch.md; no new public API, flag, default or phase is authorized.

## Child owners

- `00_nucleo/prompts/wiring/compile.md` owns only `04_wiring/src/compile.rs`.
- `00_nucleo/prompts/wiring/completions.md` owns only `04_wiring/src/completions.rs`.
- `00_nucleo/prompts/wiring/diagnostics.md` owns only `04_wiring/src/diagnostics.rs`.
- `00_nucleo/prompts/wiring/eval.md` owns only `04_wiring/src/eval.rs`.
- `00_nucleo/prompts/wiring/eviction.md` owns only `04_wiring/src/eviction.rs`.
- `00_nucleo/prompts/wiring/fonts.md` owns only `04_wiring/src/fonts.rs`.
- `00_nucleo/prompts/wiring/info.md` owns only `04_wiring/src/info.rs`.
- `00_nucleo/prompts/wiring/init.md` owns only `04_wiring/src/init.rs`.
- `00_nucleo/prompts/wiring/query.md` owns only `04_wiring/src/query.rs`.
- `00_nucleo/prompts/wiring/watch.md` owns only `04_wiring/src/watch.rs`.

## Verification and boundary

Verify this unit through its existing family/CLI tests when validating behavior.
Ownership is strictly prompt-to-consumer 1:1 (ADR-0129); referenced owners do not share this prompt.
Preserve the existing public contracts, defaults and phases; documentation cleanup
does not authorize productive changes or approve archived proposals. Validate lineage with
`crystalline-lint --checks v5,v15,v26 .`.


## Publication carrier ownership and verification

### Evidence

`lab/results/construction/diagnostics/tekt-v12-2026-10-16/before.json.gz` retains
HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, uncommitted tree/UTC, sources/L0/
hashes/status/diffstat. The hash-checked inventory reported 19 V12: one productive
enum at `04_wiring/src/main.rs:22` and 18 auxiliary test declarations. The private
Published enum had exactly Html(String) and Bundle(Vec<BundleFile>), produced at
compile.rs:90/99/363/539/541 and dispatched at watch.rs:103/104. The former owner
permitted this private L4 carrier; its transfer explicitly supersedes that local
ownership decision. This is Rust ownership/representation, not new language
parity or upstream intent evidence. The complete construction record remains in
`lab/history/prompt-consolidation-2026-10-18/records/00_nucleo/prompts/wiring/main.md`.

### Current responsibility and acceptance

Remove the local Published declaration and the BundleFile import used only by it;
privately import `typst_infra::watch::Published`. The root remains composition and
closed dispatch without a new local type/public L4 API. Child handlers may retain
super::Published. The carrier contract belongs exclusively to
`00_nucleo/prompts/infra/watch.md`.

Preserve real watch HTML/bundle HTTP/reload controls, success/error, no-serve/
no-reload and last valid snapshot, including
`p1485_watch_bundle_http_reload_e_preserva_snapshot_apos_erro` and
`p1485_watch_flags_no_reload_no_serve_e_porta_ocupada` in `04_wiring/tests/cli.rs`,
and L3 watch HTTP controls. Preserve existing compile HTML/bundle full-tree,
byte-exact assets, stdout/warnings/exit and no-publication-on-error expectations.
Require real V12 RED before transfer and zero productive V12 afterward; the diff
must show only enum transfer/import adjustments, never extension-based target
inference. Do not add tests merely mirroring owner-transfer syntax.

Validate build, workspace/all-target Clippy with -D warnings and
-D unfulfilled_lint_expectations, workspace suite, formatting, Rustdoc -D warnings
and four indexes, V1/V5/V15/V26/reverse lineage, SARIF and diff --check. Heavy runs
remain sequential under MemoryMax=12 GiB, MemorySwapMax=0 and CARGO_BUILD_JOBS=1.
No release publication is authorized by this transfer.


## Checked stderr composition — authorized 2026-10-22

### Measured evidence

`lab/results/stderr-watch-errors-2026-10-22/agents/measurement/baseline.json` (SHA-256
`9c6da7587761dd242cddf07abcf512169ad45d4bde63e3040dc1577e02e67b15`)
retains exact HEAD, uncommitted source/archive, UTC, dirty stat and binary identity.
`lab/results/stderr-watch-errors-2026-10-22/agents/measurement/before/receipt.json` (SHA-256
`e49aa4458f4cb413995af35f8297ecf4fcafd20141cbdab41067fc7ed5b3895c`)
retains the real failed-sink and healthy controls before this correction. The
source map is `lab/results/stderr-watch-errors-2026-10-22/agents/stderr/audit.md`.
These observations precede the narrowly owned correction below; a panic is not
required behavior or evidence of upstream intent.

### Owned correction and acceptance

Keep the closed exhaustive `match cli::parse()` and every existing command
arm. Private finite handlers now return `io::Result<ExitCode>`; normalize their
errors to one-element `Vec<io::Error>`. Watch returns the same private result
carrier with a vector retaining original status/write and cleanup failures.
The final process boundary returns existing successful/language/operational
codes unchanged and maps actual terminal emission failure to operational exit 2.
Do not retry printing to the failed stderr, invent a fallback stdout channel,
store a global error latch or claim the diagnostic was emitted. Errors remain
typed and unchanged until this terminal boundary; only the process exit can be
observed once its diagnostic sink has failed. No command body or new local L4
entity belongs here; no public parse/handler API is added.

The existing pinned CLI-observables Nucleus governs shared output/exit obligations;
this owner does not redefine it. The user's explicit request authorizes replacing
stderr panic with exceptional operational exit 2. This changes private composition
only, without public signature/flag/default mode or eval/layout phase change
(ADR-0127). Retain healthy message bytes/order and all existing alerts/contracts.
Require measured RED→GREEN, healthy parity, tests, hashes and direct/reverse lineage;
no suppression or weakened assertion is authorized.


## Release resource validation — 2026-10-23

### Measurement before decision

`lab/results/pdf-resources-2026-10-23/{before,recipe}-build/receipt.json` and
`{before,recipe}-measure/receipt.json` freeze identical productive/L0 inputs,
HEAD/uncommitted stat/UTC/recipes/binaries. Cargo.toml had no release profile;
reference lab/typst-original/Cargo.toml:164–169 uses ThinLTO and codegen-units1.
The same-source experimental control with those two settings reduces normalized
size and preserves all seven original text/page/word geometry gates (0.001pt).
Strip is applied only to disposable copies of both executables; symbols account
for part of the raw-size difference. No architectural cause or upstream intent
follows from these mechanical measurements.

### Resource boundary and acceptance

Build this binary with the workspace release metadata `lto = "thin"` and
`codegen-units = 1`, retaining the existing optimization level, symbol/debug
policy, CLI surfaces, features, dependencies, runtime diagnostics, semantics,
default output modes and all compiler phases. Root Cargo.toml is build metadata,
not an additional productive consumer of this prompt; main.rs remains the sole
consumer and its composition/dispatch gains no business logic. This reversible
compiler-recipe optimization is continuous under ADR-0127; it publishes nothing.

Require same-source recipe control and same-recipe loader control, normalized
size on disposable copies, source-bound frozen builds and uninstrumented paired
fresh-process RSS against previous and pinned oracle. Keep allocation profiles
separate from RSS/time and retain noisy/negative/residual results. Language gates
remain complete text/lines/pages/word geometry at 0.001pt, diagnostics, symbol
observations and independent raster; no byte identity with vanilla is required.
Keep full build/workspace tests/all-target Clippy/fmt/Rustdoc, 1:1 and reverse
lineage and full crystalline-lint with zero violations. No mutable global cache,
L1 impurity, new mode/flag/public API, compatibility break or phase move is
licensed by a smaller executable. This is a local measurement, not a universal
memory/speed guarantee.


## R6 release resource boundary — 2026-10-24

### Measurements before decision

`lab/results/construction/diagnostics/pdf-resources-r6-2026-10-24/baseline.json`
freezes received R5 HEAD, full source archive, dirty stat/UTC and binary identities.
`sections-before.json` measures .text26,768,901 and .rodata18,277,029 bytes;
`before-profile/receipt.json` locates font-catalog allocation requests, which are
not live heap or RSS. Prior G reader prototypes did not improve CLI RSS.
Recipe-only s, single-font-transfer+s, and recipe-only z fail the fixed RSS3%
gate in their respective candidate/candidate2/candidate3 resource receipts.
The single-font source and owner are restored exactly per
`single-transfer-rejected.json`; its experimental tests are not shipped.

`candidate4-resources/receipt.json` measures z plus private immutable-coverage
sharing at normalized ratio0.81308, aggregate RSS ratio0.93553, all workloads
within3% and repeatable. The same-window z-only control separates storage from
recipe. Math elapsed medians0.12→0.24s reveal a CPU tradeoff. The completed
`z-cycle` retains full source-bound validation, all A–H/R1–R5 replays, resources
and historical stress observations verbatim, with relocation hashes.
These are mechanical resource costs, not language-parity criteria or upstream
intent. Smaller code and shared runs are hypotheses qualified by actual paired
measurements, not universal memory/speed claims. A failed gate refutes adoption.

### Owned recipe and acceptance

Stage and validate opt-level = "s", ThinLTO, one codegen unit, retaining all
symbols/features/dependencies/public CLI behavior and compiler phases. This
compares s plus the same immutable-coverage representation against validated
z plus shared coverage to choose a lower-CPU-cost passing resource recipe.
The private coverage storage/clone contract belongs exclusively to
`00_nucleo/prompts/core/entities/font_book.md`; this owner does not authorize its
implementation. main.rs remains the only productive consumer here; root
Cargo.toml is build metadata and adds no productive owner or business logic.
Internal reversible recipe optimization follows ADR-0127 continuous flow.

Keep the fixed resource regression: normalized executable <=95% of received R5
and <=pinned vanilla; geometric mean of seven peak-RSS median ratios <=97%; no
workload above103%; received/current round-median variation <=3% on every case.
Measure three rounds ×three randomized fresh processes/role/workload with two
warmups, GNU maxRSS, no competing build; retain elapsed time and all negative
results. Include received R5, pinned oracle, recipe-only s/z and z+shared storage
as same-window controls. Preserve actual resource RED→GREEN, original binary
symbols, source/recipe archives and oracle provenance. Strip disposable copies
only. Do not weaken criteria or remove functionality to reach a byte target.

Retain s only if it passes every resource/language/final gate and has lower
measured math elapsed cost than z+shared storage; otherwise preserve the validated
z alternative and record the decision. No speed superiority over R5 is promised.
Preserve every existing A–H/R1–R5 outcome/tolerance, complete font inventories,
canonical coverage semantics/metadata, diagnostics, geometry, independent raster
and known limits. Keep the received PDF single-font allocation exception.
Require complete frozen workspace build/tests/all-target Clippy/fmt/Rustdoc,
direct/reverse lineage, V15/V26 before resealing and full crystalline-lint zero.
Heavy Cargo remains serial/offline/locked/one-job, MemoryMax12GiB, High8GiB,
SwapMax0. No publication/new mode/public API/phase/compatibility change.
