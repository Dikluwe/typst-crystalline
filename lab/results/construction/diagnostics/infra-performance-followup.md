# Post-correction infra measurements and remaining defects

The measured first acquisition of the full font catalog dominates the instrumented Tekt control: 95.21–97.38% of fresh pipeline time for three fixtures. The shapes fixture reaches this acquisition through vertical metrics requested by placement layout. Vanilla records no font-discovery span for the shapes/image fixtures in this run. This identifies a concrete demand-path difference worth investigating; it does not establish that all layout time is layout-algorithm work or that complete PDF export is a stream cost.

The performance battery is **exploratory, not an accepted stable absolute baseline**. Only 6 of 235 microbenchmark cases pass the predeclared three-round rule; 229 fail. All ten plain CLI engine/fixture populations fail. Accepted receipts prove source stability and absence of detected concurrent builds/tests/benchmarks, not quiet hardware, thermal stability or stable timing. Raw results are retained rather than replaced by a favorable subset. No production optimization or correction was made during this study.

## Provenance and reproduction

HEAD: `0fea906e176acf93ae6301e4cd17ace6aa4fe349`. This is a dirty working tree, not HEAD alone. `frozen-status.json` preserves the exact status including untracked files and `git diff HEAD --stat`; `incoming.patch.gz` preserves the full tracked binary patch. `frozen-state.json` records actual bytes of consumers, prompts, nuclei, existing benchmarks, manifests/locks, producer scripts, inputs, upstream and font/config files. Aggregate SHA-256: `cf17c505d750ed3415eeffdb3ff086bad75155e50030bf3eae4a03db0e2fba7b`. Fingerprints match before/after every accepted primary cohort and all three demand-control rounds. Source archive `consumers-inputs.tar.gz` preserves the repository-local frozen files; absolute external dependencies/fonts remain identified by paths/hashes and reconstruction provenance. The final audit checks existing changes are preserved and prior evidence seals remain intact.

Primary measurement window: 2026-09-27 17:58:03.126462–18:17:22.149973 UTC. The separate demand control records its own timestamps in `demandprobe/r*/receipt.json` and raw rows. Machine: Dell-G15-Linuxmint, x86_64, Linux 6.17.0-42-generic, Ryzen 7 5800H, 8 cores/16 logical CPUs. rustc 1.92.0 (ded5c06cf), LLVM 21.1.3; Cargo 1.92.0. Full host/toolchain/profile/flags are in `build-provenance.json`. Measurement processes use CPU 4, driver CPU 6, Rayon one, vanilla `--jobs 1`. Governor remains powersave and boost enabled. Background load and frequency snapshots vary; the machine is not exclusively reserved. These observations do not prove the cause of the inter-round drift.

Vanilla is pinned to `9f2b6e8715237cb086899a42873660fe744622e8`, tree `6ab8bb9434b1bd4d44e1a10fbf1e407b3b86a1a4`, ratified CLI SHA-256 `7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea`. `vanilla-identity.json` checks actual original/staged source bytes and the CLI against ratified provenance; version text is not the proof. The previously documented local Cargo.lock overlay remains explicit. The combined font harness links actual pinned vanilla libraries with an independent dependency graph and lockfile; it is not the ratified CLI build.

Preparation commands ran serially before samples (complete commands, clocks and Cargo JSON in build logs):

```sh
cargo build --release --offline --locked --message-format=json
cargo bench --manifest-path benches/Cargo.toml --no-run --offline --locked --message-format=json
cargo test -p typst-core --release --lib --no-run --offline --locked --message-format=json
cargo build --manifest-path 00_nucleo/diagnosticos/infra-performance-followup/fontbench/Cargo.toml --release --offline --locked --target-dir target --message-format=json
```

`prepare.py`/`run.py` archive and run the measured executables; use `PYTHONDONTWRITEBYTECODE=1`. Every actual invocation and relevant environment overrides are stored in cohort `command.json` or CLI raw rows. Cargo profile evidence shows optimization 3, debug assertions false, overflow checks false. Rust flags are empty and Cargo profile overrides removed. Observation/stdlib use the supported guarded launchers and a prebuilt release core-test executable; their preparatory build/profile checks occur outside Criterion sampling. No debug observation/stdlib numbers are admitted. Private benchmark tests run as the measurement entry points; normal tests do not overlap samples.

| Archived executable | SHA-256 |
| --- | --- |
| `typst` | `9f16ed26379de7484ef1ab6226911537b4620526c106faf30656c0bbc972ec5c` |
| `pdf_builder_bench` | `9ec5fb13b1768cfa9e172df5fde702ae3588bcd8d0ec28e65fe6e8518f0dca27` |
| `stdlib_bench` | `a033b00851759883019e41534c70e675b53d3ec2e6073d9029da5a6f1caf7cd3` |
| `pipeline_bench` | `fc431fc9395ad2ccebf22899584ad9a4bfd07e9ae023736f4321888c34b76f85` |
| `content_bench` | `7c6a96c9a79a7622924e2836ca047e9dd0277403c4fe51583ca53f4dc1b2ca8d` |
| `observation_bench` | `95a2e62f29aa0c35a16aa10fdd19e9e2c372c86374b659877cb3376e14f1a83c` |
| `layout_bench` | `d476989cdebd24b110f0a595eb5062b8beb04b87a146a47c0c63edda9abee15f` |
| `stream_bench` | `cb42244b919be816aa9233571d4b32a493cd04fb1a8fda200d110def22e9d87f` |
| `scanner_bench` | `7020c720862dac8ed006f45edd29d2dd7dd89e1330679b3e6a13dc410bde20f1` |
| `core-benchmark-tests` | `3f4c68b0bde4f0ed2f2f5cd1a2950c80fe5c55f74f1d7592804cb652a4db7c55` |
| `language_probe` | `70169df319995114b45b51e26f164d78e3efdf67e0e36d30364f2c7886c96f59` |
| `infra-font-followup` | `67eda0a67b4d900761ebc7a583e48eaf6feac6886aa0162fe456984d0bdf5782` |

Demand control executable SHA-256: `92d69568624f05a7f7f5f0d37cbd366b8785be3b2c93ebde24707a7bd8657820`. Its independent lock, profile, sources, preparation/build logs and commands are retained under `demandprobe/`. Its release binary was built after the primary timing window and before its own accepted samples.

Three primary rounds comprise nine accepted cohorts. A 30-second quiet gate precedes each and process inspection runs every 100ms during collection. `micro-r1/a1` was rejected at the quiet gate for a competing Python unittest process; `a2` is admitted. No measurements from that rejected attempt enter results. Subsequent cohorts passed on their first attempt. Input/executable mutations or detected competing build/test/benchmark activity reject the entire cohort. This monitor can miss sub-100ms activity and does not eliminate unrelated background load.

## What the measured intervals contain

The initial audit follows `benches/README.md`, `benchmark-suite-review.md` and the sealed font protocol. These boundaries remain unchanged:

| Cohort | Timed operation / composition | Setup, caches, I/O and destruction |
| --- | --- | --- |
| Scanner | Lexer construction and complete token consumption | Corpus reading/creation outside; per-operation construction and token traversal inside. `scanner-summary.json` reports ns/byte. |
| Content | Public operations; representation equality/hash and traversal | Fixture creation outside; API allocations/clones/traversal inside according to each closure. Equality is mechanics, not language parity. Destruction follows Criterion batching. |
| Layout | Public measurement using FixedMetrics, plus complete styled layout | Fixtures/setup outside; temporary layouters/subframes and introspection/layout inside. Does not measure real font shaping in isolation. |
| PDF builder and stream | Complete PDF export grouped by fixture behavior | Resource discovery, builder/object construction, emission, serialization and output destruction inside. Stream cohorts use Pretty output. No attribution to isolated page/text/shape/draw emitters. |
| Pipeline | Warm-process public compilation | Source parsing/world/font setup outside; evaluation, stabilization, layout, resources/export and API temporary work inside. Distinct from CLI launch-to-exit. |
| Observation / stdlib | Private relation calls; registry creation, lookup and pre-resolved native-call groups | Guarded launcher preparation/startup outside Criterion. Each closure retains its intended registry/argument/setup boundary; no new public API. |
| Font constructors | Generic construction and separately clone+construction | Identical codepoint-vector clones outside generic timer, inside clone case. Result drop outside; input consumption/allocation/sort and input drop inside. Vanilla generic algorithm is not its actual FontInfo streaming cmap acquisition. |
| Font metadata+coverage | Public vanilla FontInfo versus explicit public Tekt diagnostic composition | Parsed metadata, coverage and intermediate work included. Tekt composition parses twice; vanilla includes additional metadata. This does not isolate either private production extractor. |
| Font first / retained | Actual public slot/store loading versus initialized access | Fresh slot/store setup/path/book clones outside first-access timer; I/O, lazy loading and returned handle clone inside; final handle/input destruction outside. Retained case preloads outside; no fresh read intended. TTC handling differs between engines. |
| CLI plain | New process launch-to-exit | Process startup, setup, source/catalog acquisition, filesystem I/O, compilation, export, write and teardown included. Identical inputs/options/project fonts; system font/config fingerprints recorded. |
| CLI traced | Same end-to-end invocation with internal timers | Separate population with instrumentation overhead; disjoint broad envelopes and a wall residual. Nested discovery/layout spans are not added again. |

Six standard Criterion targets and font cases use 30 samples, 200ms warmup, 600ms target measurement, 5000 resamples. Observation/stdlib retain fixed 30/150ms/300ms. There are 139 existing-suite cases and 96 font cases across four original font files/indices (see `fontbench/inputs.json` and `font-fixture-controls.json`). Font fixture/engine order reverses in round 2. OS page caches are warm after preflight/warmup; no host cache flush or cold-disk claim. First slots are fresh per batched iteration, retained slots initialized.

CLI fixtures: context, image, pipeline-single-font, shapes, text-dense, with the same tiny JPEG and project font directory. Options include identical root, `--font-path`, `--pdf-tagged=false`, `--pretty=false`. Two unrecorded warmups per engine/block. Three rounds of 12 plain pairs per fixture (360 measured invocations) and 6 traced pairs (180 invocations), alternating engine order. Math remains excluded because equivalent assets/fonts were not established. Selected output controls (one plain pair per fixture/round) match page count/size and normalized extracted text. This bounded check does not prove all geometric, font or language parity.

## Exploratory wall comparison

Milliseconds; brackets are pooled p05/p95. Each engine has 36 plain observations per fixture. Ratio and delta are medians of corresponding alternating pairs, not ratios/subtractions of marginal medians.

| Fixture | Tekt median [p05,p95] | Vanilla median [p05,p95] | Paired Tekt/vanilla | Paired delta ms |
| --- | --- | --- | --- | --- |
| context | 251.88 [213.47, 312.22] | 182.73 [157.47, 216.90] | 1.38 | 68.49 |
| image | 248.28 [208.59, 359.51] | 21.30 [13.32, 25.71] | 14.37 | 227.03 |
| pipeline-single-font | 250.34 [224.16, 362.50] | 181.76 [158.20, 277.92] | 1.36 | 70.23 |
| shapes | 275.44 [246.91, 404.39] | 22.36 [15.36, 27.01] | 11.94 | 252.80 |
| text-dense | 263.99 [251.25, 396.97] | 188.93 [182.26, 288.06] | 1.38 | 71.40 |

`e2e-summary.json` retains raw pair ratios/deltas, pooled and per-round mean/median/SD/min/max/p05/p95. `cli-stability.json` shows each population's drift; no absolute CLI population passes. Do not use these ratios as a stable machine-independent promise or subtract old runs to claim the correction commits improved performance.

## Compatible broad phase accounting

The following **signed mean deltas use only the 18 traced observations per engine/fixture**. Within that population the four columns sum to the measured mean wall delta. They do not decompose the plain medians above. Eval/layout envelopes include first lazy acquisition where demand happens; PDF includes complete resources/builder/serialization. Residual includes remaining startup/setup/I/O/teardown/instrumentation and timer-boundary differences.

| Fixture | Initial eval delta ms | Stabilization/layout delta ms | Complete PDF delta ms | Residual delta ms | Traced wall delta ms |
| --- | --- | --- | --- | --- | --- |
| context | 62.76 | 0.27 | -0.62 | -0.83 | 61.58 |
| image | 3.52 | 256.24 | -0.57 | 0.44 | 259.64 |
| pipeline-single-font | 3.96 | 96.99 | -0.09 | -0.28 | 100.59 |
| shapes | 5.64 | 262.44 | -0.55 | -1.36 | 266.17 |
| text-dense | 88.18 | 6.88 | 2.12 | -1.36 | 95.82 |

`phase-summary.json` preserves each engine's absolute distributions; `phase-samples.json` retains the underlying timings, traces and interval interpretation. `phase-envelopes.png`/`.svg` visualize the means. The large shapes/image layout envelopes warrant investigating acquisition before optimizing layout arithmetic. PDF deltas are small in these fixtures; none is evidence of an isolated stream bottleneck. Shifts relative to historical timers can reflect lazy acquisition moving between envelopes, not changed algorithm costs. No addition of microbenchmarks from different fixtures is used.

The absolute micro stability rule requires mean-versus-median of three round means <3% and range/median <=5%. The only passing cases are:

- `fonts/LibertinusSerif-Regular/retained_font/tekt_slot`
- `content_bench/content/traversal/map_content_noop_deep`
- `content_bench/content/traversal/map_text_identity_wide`
- `stdlib_bench/stdlib/registry/html0_a11y1_bundle0`
- `stdlib_bench/stdlib/inputs/0`
- `stdlib_bench/stdlib/inputs/16`

All other results remain exploratory in `micro-summary.json`, including every matched Tekt/vanilla font pair (none has both sides stable). `font-comparisons.json` provides descriptive per-font/per-round ratios only. Metadata+coverage ratios roughly 1.18–1.79 and CJK first-font ratios 2.22–4.50 motivate equivalent-work controls, not a decomposition of the full catalog or CLI gap. Tekt's CJK slot reconstructs a collection face while vanilla retains collection/index; other first-font cases favor Tekt but include unequal internal work. Constructor/contains/cmap controls cannot be summed into a catalog-acquisition estimate.

## Controlled first catalog demand

A diagnostic World decorator delegates to the real SystemWorld and the real public paged/PDF pipeline. A fresh-world mode begins with the catalog unrequested; the otherwise equivalent warm mode preloads it before the timer. Source parsing, world construction and PDF file writing are excluded from the pipeline timer; decorator overhead is included. Three rounds × four pairs × three fixtures yield 72 measured pipeline calls. All before/after consumer/input/helper fingerprints and executable hashes match; all quiet/continuous process gates pass. Separate untimed backtraces identify first demand. Sources, all raw calls, distributions, receipts and output checks are under `demandprobe/`.

| Fixture | Fresh pipeline mean ms | First book acquisition mean ms | Included fraction | Preloaded pipeline mean ms | Preload + pipeline mean ms | Book accessor calls |
| --- | --- | --- | --- | --- | --- | --- |
| shapes | 234.66 | 223.41 | 95.21% | 10.10 | 228.10 | 1026 |
| pipeline-single-font | 245.76 | 239.32 | 97.38% | 5.55 | 248.44 | 12 |
| context | 295.86 | 287.13 | 97.05% | 7.04 | 281.45 | 29 |

Each world acquires 1,130 catalog entries. Accessor counts are **not rebuild counts**: the OnceLock catalog is built once per world and reused afterward. The 95–97% fraction is direct included-interval accounting in this control. It cannot be transplanted numerically into the different CLI populations. Preload+pipeline totals show that warm timing moves work outside the interval; it does not remove fresh acquisition, prove a production speedup or establish persistent reuse across processes. The first call covers complete acquisition/composition: embedded/system/project metadata, coverage and book formation, not disk I/O alone. `demandprobe/distributions.json` exposes dispersion rather than hiding it in the means.

Backtrace controls show:

- Shapes: `World::book → FallbackFontMetrics::resolve_primary → vertical_metrics → Layouter::layout_sub_frame → layout_place` (read in caller order from right to left). `place` reaches font metrics despite no visible text.
- Single-font: `World::book → resolve_primary → text_edges → ensure_initial_baseline → layout_word`.
- Context: `World::book → eval_set_rule → eval_expr → eval_markup`.

These are actual call stacks, not inferred from microbenchmark correlation. Source anchors include `03_infra/src/font_metrics/resolution.rs:27`, `03_infra/src/font_metrics/basic.rs:164` and `:194`, and `01_core/src/compiler/eval/rules.rs:1577`. `03_infra/src/world.rs:70` owns the instance-local catalog OnceLock. Existing sealed vanilla mechanics explain laziness/reuse, and the current pinned CLI traces independently distinguish the shapes/image path. Absent discovery spans establish no recorded discovery in those traces, not a claim that vanilla performs no font-related instruction. Baseline-sensitive positioning can legitimately need metrics; “no visible text” alone is insufficient to change that contract.

## Additional demonstrated correctness defect

The untimed `language_probe` uses actual public Tekt shaping over NotoSansCJK face 0 and compares a direct rustybuzz control on the same face/text/direction with an explicit ja/zh/ko language. Across six discriminating characters, Tekt emits identical glyph IDs across languages while the configured control exposes language-specific substitutions. For example `骨`: configured ja/zh/ko = 45132/45133/45132; `漢` = 24227/58935/58935. `language-control.json` retains all results and fixture identity.

Tekt width and emission buffers at `03_infra/src/shaper/shaping.rs:85` and `:249` omit `set_language`; pinned vanilla sets it at `lab/typst-original/crates/typst-layout/src/inline/shaping.rs:980`. The direct rustybuzz control is **not a vanilla full-pipeline language oracle**; its discriminating result plus consumer inspection demonstrates ignored language in these Tekt paths. This is independent correctness evidence, not a demonstrated explanation for the measured performance gap.

A future L0-first correction must configure both buffers and include language in `ShapeCache::key` (`03_infra/src/shaper/cache.rs:34`), with fresh/shared AB/BA regressions. Fixing only buffer configuration would introduce incorrect cross-language cache reuse. Existing width-cache language identity does not make the buffer honor language. The separate forced-math-script cache-identity concern remains unproved by this probe. No repair was mixed into this frozen measurement dataset.

## Controlled experiments still needed

1. **Premature demand during absolute placement.** Compare matched shape/image fixtures varying only placement/baseline alignment. Count catalog/metrics requests and verify language/geometry before and after a narrowly scoped lazy-demand change in its proper owner. This can refute the explanation if metrics are required for these positions or equivalent vanilla also acquires the catalog. Keep L3 I/O and L4 composition contracts intact.
2. **Catalog acquisition work.** Drive actual public full-catalog loaders with the same enumerated files, indices and metadata/coverage requirements; record discoveries/parses, construction and retained lookup separately. The current public metadata composition is not a substitute for that control. This distinguishes inherent metadata cost, different work and repeated work without new production benchmark APIs.
3. **TTC reconstruction.** On the same collection/face, compare fresh/retained acquisition and controlled diagnostic normalization versus retained collection/index. Separate OS-cache and output-destruction boundaries; validate glyph/metric coverage. The current CJK ratios motivate the experiment, not causal proof for the system catalog.
4. **Trustworthy reuse.** Separate fresh process, fresh World and retained World lifetimes; mutate font/catalog input and source inputs independently. Measure amortization only after proving invalidation and equivalence. This study shows once-per-world acquisition, not constant reconstruction on every cached accessor call, and does not justify global stale state.
5. **Supported variable-axis costs.** Compare successful equivalent exports on a proven matching variable face, including instancing and downstream resources. The earlier unknown-axis failure versus corrected success is not a performance comparison.
6. **Language shaping.** Obtain a pinned full-language parity fixture and RED→GREEN fresh/shared-cache controls for width/emission. Treat language and script/math shaping identity independently; benchmark only after behavior is correct.

All measurements and derivatives live exclusively in this new diagnostic directory and report. Existing production code, L0s, ADRs, tests and benchmark source are preserved. Prior sealed evidence is historical; no global sanitization, performance improvement or completion of infra correctness is claimed.
