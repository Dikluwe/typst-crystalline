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


## Stack image repair: stable x86-64 code alignment — 2026-10-27

### Measurements before classification

`lab/results/construction/diagnostics/stack-image-flow-fix-2026-10-27/full-attempt3/final-state.json`
freezes the uncommitted source state and UTC. Its full twelve gates pass, but
`jpeg-regression-attempt1/resource-gates.json` rejects the JPEG timing result.
`received-jpeg/receipt.json` independently reproduces the regression against the
received executable. `codegen-evidence/receipt.json` compares ten hot decoder
functions: instructions and sizes are unchanged, while function placement shifts.
This is observable resource cost, not a change to JPEG semantics or intended
upstream behavior. Causation by alignment is an experimentally supported inference,
not a guarantee for other processors.

Same-source final-link controls retain full source/recipe/binary identities in
`alignment-trial/receipt.json`, `decoder-opt3-trial/receipt.json` and
`alignment32-trial/receipt.json`. Decoder opt-level3 and 32-byte alignment do not
recover the received timing and are rejected. The 16-byte control restores uniform
grayscale/RGB timing; `alignment16-qualification/resource-gates.json` passes the
frozen 936-process JPEG campaign: gradient ratio0.991044 versus pre-SSE control,
paired bootstrap95% upper0.995939, ratio0.908706 versus pre-native-Luma control,
and all nine RSS ratios<=1.03. Additional loop alignment emits identical bytes
(`alignment-loop-trial/receipt.json`) and is unnecessary.

### Mechanical recipe and acceptance

For the measured `x86_64-unknown-linux-gnu` target, stage LLVM function alignment
16 bytes (`-C llvm-args=-align-all-functions=4`) as target-scoped Cargo build
metadata in `.cargo/config.toml`. Keep the existing release s/ThinLTO/one-unit
recipe, full diagnostics/symbols, portable target CPU, dependencies and complete
JPEG decode/dedup/streaming behavior. This compiler tuning adds no user-facing
mode/flag/default compiler path, public contract, compatibility requirement or
pipeline phase/order change; ADR-0127 continuous flow applies. main.rs remains
the sole productive consumer of this recipe owner; Cargo metadata adds no owner.
Do not add artificial padding, dead logic, target-CPU feature requirements or a
new JPEG cache to control addresses. Other targets retain their existing recipe.

The same-source final-link result only licenses a candidate, not promotion of the
actual target-config build. Freeze/rebuild that exact default recipe; rerun all
twelve validation gates, full unfiltered lint, debug/release language comparisons,
strict Stack image PDF/SVG geometry and raster controls, R6 resource bounds,
all 17922 JPEG compatibility probes and JPEG output observations. Require the
unchanged JPEG gradient timing/CI/Luma and nine-case RSS thresholds above; retain
all failed controls and compare timing/RSS directly with the received executable.
Report any residual received-relative timing difference explicitly. Keep decoder
productive bytes unchanged. A failed resource or language gate refutes promotion;
never weaken a threshold or reuse an earlier binary's observations as final proof.


## Display-math continuation: qualify the decoder recipe with alignment retained

### Measurements before classification — 2026-10-27

`lab/results/construction/diagnostics/block-math-vertical-advance-fix-2026-10-27/timing-attempt1/relocation.json`
preserves the frozen first geometrically accepted release. The unchanged JPEG
source failed its936-process timing gates: gradient/pre-SSE1.04879649,
bootstrap95%[1.04240172,1.05191282], native-Luma0.96381619; all RSS limits pass.
`jpeg-layout-pilot/receipt.json` independently reproduces slow uniform grayscale.
`codegen-validator/receipt.json` observes identical normalized hot instructions
and sizes with shifted placement. `gdb-profile/receipt.json` localizes approximately
6ms to validation. Placement sensitivity is an inference supported by controls,
not a universal CPU claim. Do not change JPEG semantics to hide this resource RED.

`alignment64-trial/receipt.json` and `alignment64-pilot/receipt.json` reject forced
64-byte alignment: uniform ratio1.168889 against received. The earlier Stack
opt3 control disabled target alignment through empty encoded rustflags; it did
not measure opt3 combined with actual16-byte alignment.
`decoder-opt3-aligned-trial/receipt.json` keeps target16 and tests opt3 only for
zune-jpeg. Its pilot ratios against received are1.012273 uniform,1.007013 RGB,
1.023494 gradient, all RSS<=1.03. `decoder-opt3-qualification/resource-gates.json`
passes936 processes: gradient/pre-SSE0.99279706, paired95% upper0.99616115,
native-Luma0.90443092 and nine RSS<=1.03. This licenses a candidate recipe only.

### Authorized internal compiler recipe and final gates

Keep release s/ThinLTO/one unit and the existing target16-byte alignment. Add
`[profile.release.package.zune-jpeg] opt-level = 3` in root Cargo metadata. This
supersedes the earlier blanket other-target unchanged clause only for this
portable package optimization; target-specific flags remain unchanged. No CPU
feature requirement is added. Performance measurements certify only the measured
x86_64 host, not other architectures. Decoder source, pinned dependency revision,
full decoding/streaming/dedup behavior, symbols, diagnostics and public behavior
remain unchanged. This is private compiler mechanics, ADR-0127 continuous flow;
main.rs remains this owner's sole consumer, with only lineage header resealed.
No new public contract/default user mode/pipeline phase/compatibility break.

Before promotion, freeze and rebuild the actual default Cargo recipe, rerun
all12 validation gates, language/geometry/raster/preservation corpora, R6 size/RSS,
17922 JPEG compatibility probes,936 paired timing/CI/Luma/RSS bounds and nine-case
direct received comparison. Keep every failed campaign/control and exact source,
recipe, binary, UTC and resources. Do not reuse the experimental binary as final
proof, relax any threshold, strip product symbols or change unrelated inputs.


## Equation-index continuation: repair the measured JPEG placement regression (2026-10-29)

### Measurements before classification

`lab/results/construction/diagnostics/readiness-fixes-2026-10-28/jpeg-recipe-evidence20.json`
freezes HEAD212aed0449150dd296ceea13fc2be665cc5db01a, uncommitted diffstat,
UTC, input/source/build/observer hashes and the unchanged actual default19
binary1932303a2a0a2c67662537bd8a5edd0e776bc3213cb9ab1acf10dbdd8dc7e421.
`jpeg-alignment-before20.json.gz` preserves this owner's prior text and metadata.
The fresh balanced61-pair four-role `jpeg-cause20b/receipt.json` measures
current19/pre-index10 gray-large1.171382, while pre-index10/task-received1.004417.
Empty/text and RGB controls lack that loss. The only compiled source changes
between those builds are the four index/request-test files; JPEG consumers and
recipe bytes are unchanged. GDB diagnostic intervals localize the loss to
ImageValidator, not semantic index computation.

`03_infra/src/export/images/scan.rs:318-367` instantiates the pinned generic
native grayscale path, preserving full Huffman/IDCT reconstruction. The
913-byte `copy_raw_planes_for_mcu_stripe` has the same instruction shapes after
normalizing relocation addresses and private linker-generated LLVM suffixes.
Its sixteen-byte scalar loop at offset0x288 moves from offset24 to56 modulo64,
crossing a measured64-byte instruction-cache line in the latter placement.
`jpeg-copy-placement20` creates diagnostic executable copies ONLY, with no
registered unwind for the relocated routine; these are never product artifacts.
Same-length paths and identical unchanged function bodies at the two placements
isolate the variable. `jpeg-copy-placement-measure20/receipt.json` measures
crossing/noncrossing gray-large1.167097, paired95%[1.165792,1.177393], and
horizontal1.139006; each pair emits identical PDF bytes. This byte identity is
only a controlled diagnostic, not a vanilla-language acceptance requirement.
Placement causation is supported on this measured host, not a universal CPU claim.

The converted-borrowed full-decoding alternative was slower in every measured
case (`jpeg-algorithm-probe20`). Cargo1.92 local profiles.html:503-522 explains
that generic instantiations may use the consumer's optimization level. Both
consumer opt2 and opt1 controls retain target16 and full decoding; their exact
1916-process and R6 receipts reject them on unchanged memory/resource gates.
They are not adopted. Preserve their binaries, sources, failures and receipts.
The earlier alignment32 rejection above belongs to its historical code snapshot;
it does not decide this new loop geometry without a current measurement.

### Internal recipe decision and unchanged acceptance

Supersede ONLY the target16 setting with function alignment32 bytes
(`-C llvm-args=-align-all-functions=5`) for x86_64-unknown-linux-gnu in
.cargo/config.toml. For this measured loop offset/length, aligned32 entries place
it at8 or40 modulo64 instead of the bad56. Keep release s/ThinLTO/one codegen unit,
portable zune-jpeg opt3, all other target settings, symbols, dependencies, source
JPEG/native helper boundaries, full reconstruction/limits/fallback/error policy,
Arc deduplication, allocation bounds, output, public APIs and pipeline phases.
Do not adopt either consumer optimization override. No padding/dead logic,
hand-patched executable, extra CPU feature, kernel change, cache, decoder shortcut
or new user mode is authorized. This owner still has exactly one productive
consumer, main.rs; its only source update is lineage. Private compiler mechanics
remain ADR-0127 continuous flow, with no new public/default-user/phase gate.

The actual default config build must be freshly frozen, built and archived;
never promote diagnostic copies or reuse old timing as new acceptance. Require
all12 technical gates, strict lint and direct/reverse lineage; retain every
existing test and all four ratified readiness approvals. Repeat the full
index/622-case PDF/SVG/0.001pt/independent-paint and historical1720 preservation,
language/feature matrix and original resource suites. Execute the full17922 JPEG
compatibility corpus with the current release test library, exact legacy errors,
all20 JPEG PDF/page/text/raster/payload observations,936 paired and1916 direct
process campaigns. Keep every original time/pairedCI/native-Luma/nine-RSS bound,
noise median<=1.02/95% upper<=1.03 versus fast prior, recovery<=.98 versus received,
and all nine times/RSS<=1.03 against fast/received/task-received. Retain R6
normalized size<=95% R5 and<=oracle, geometric mean RSS<=.97, every case<=1.03,
round variation<=3% and math time below validated z+shared. Any failure refutes
promotion; do not relax thresholds or recategorize native-valid positives.
Keep compiler effects separate from language semantics and record limits of the
measured host. No commit, push, dependency or oracle resync is authorized.


## Nested-equation continuation: reduce forced alignment — 2026-10-30 UTC

### Measurements before classification

`lab/results/construction/diagnostics/readiness-fixes-2026-10-28/jpeg-alignment-before24.json.gz`
preserves HEAD212aed0449150dd296ceea13fc2be665cc5db01a, the uncommitted
working-tree stat, complete input hashes, exact UTC and prior recipe/owner/source
texts. All measurements below retain their own source and binary identities.
The preceding placement experiment isolates the unchanged native copy routine;
this is compiler resource cost, not a change to JPEG or equation semantics.

The actual target32 candidate20 passed the twelve technical gates but failed
936 RSS (gray-horizontal ratio1.030519>1.03) and six direct RGB timing criteria.
It is rejected. The allblocks16 pilot21 failed RSS and R6 size/RSS. Consumer2
with target16 failed R6 and RGB RSS; consumer1 also failed. Pilot22's late
alignment0 did NOT establish automatic dependency alignment; its consumer2
result also fails RSS/R6. Pilot23 is byte-identical to actual20: lowering the
last-crate LLVM option cannot undo alignment already emitted in dependencies.
`jpeg-late-alignment-refutation23.json` invalidates that intended control rather
than treating it as a real function8 measurement. Preserve every raw failure.

The corrected `jpeg-global-function8-pilot24/receipt.json` rebuilds ALL dependencies
with explicit CARGO_ENCODED_RUSTFLAGS `-C` plus
`llvm-args=-align-all-functions=3`, while source and actual default config stay
frozen. Its executable SHA256 is
2d5f9f565178554d2f40f9a8918854ef86d73076430565ecfe05c3b5ac292beb.
The same913-byte native copy routine starts0x266ce90; its sixteen-byte scalar
loop starts0x266d118, offset24 modulo64, without crossing the instruction-cache
line. `copy-assembly.txt` preserves actual instructions. No decoder source,
algorithm, reconstruction, fallback, validation limit or user contract changes.
`jpeg-global-function8-direct24-gate.json` passes all1916 direct process criteria,
including noise/CI/recovery and all nine timing/RSS cases against three controls.
For example gray-large/fast-prior time0.992800 and RSS1.00864 recover the loss;
the exact unrounded ratios and paired samples remain in that receipt.
`jpeg-global-function8-r6-resources24/receipt.json` passes every fixed R6 check:
normalized size ratio0.867819123 and geometric mean RSS ratio0.959343610,
all seven per-case RSS/round variation bounds and math elapsed below z+shared.
The pilot's resource wrapper ends2026-10-30T01:16:56.496329+00:00 with exit0.
These results certify this diagnostic source and measured host only.

### Owned mechanical decision and unchanged final gates

Supersede the preceding target32 candidate setting with forced function alignment
8 bytes (`-C llvm-args=-align-all-functions=3`) in .cargo/config.toml for
x86_64-unknown-linux-gnu. Apply it to the full dependency compilation, not only
the final crate. Preserve release s/ThinLTO/one unit, portable zune-jpeg opt3,
all other targets and profiles. Do not adopt consumer optimization overrides,
allblocks alignment, patched binaries, padding/dead logic, extra CPU features,
cache, skipped decoding, changed limits, new modes, public signatures or phases.
main.rs remains the sole productive consumer, with lineage-only source changes.
This reversible private recipe correction follows ADR-0127 continuous flow.

The pilot licenses staging only. Freeze, reseal and freshly build the ACTUAL
default configuration with no rustflag environment override. Require all twelve
technical gates, strict full lint and forward/reverse lineage; repeat complete
index,622-case PDF/SVG/0.001pt/paint, historical1720, language/feature matrix,
R6 resources,17922 current-release JPEG compatibility, all20 native/task JPEG
PDF/text/page/raster/payload observations,936 paired and1916 direct campaigns.
Retain ALL original bounds specified above and all four existing approvals.
A failed actual-build resource/language gate refutes promotion. No threshold
relaxation, historical result substitution, commit, publication or resync.
