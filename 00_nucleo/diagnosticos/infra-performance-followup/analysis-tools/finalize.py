"""Seal frozen measurements and derived evidence without changing consumers."""
from pathlib import Path
import datetime,gzip,hashlib,json,statistics,subprocess,tarfile,importlib.util
ROOT=Path(__file__).resolve().parents[4]
R=Path(__file__).resolve().parents[1]
def sha(p):
 h=hashlib.sha256()
 with p.open('rb') as f:
  for b in iter(lambda:f.read(1048576),b''):h.update(b)
 return h.hexdigest()
def save(p,d):p.write_text(json.dumps(d,indent=2)+'\n')
def dist(v):
 v=sorted(v)
 def q(f):
  x=(len(v)-1)*f;i=int(x);return v[i]+(v[min(i+1,len(v)-1)]-v[i])*(x-i)
 return dict(n=len(v),mean=statistics.mean(v),median=statistics.median(v),sd=statistics.stdev(v) if len(v)>1 else 0,min=v[0],p05=q(.05),p95=q(.95),max=v[-1])
rows=[]
for n in [1,2,3]:
 receipt=json.loads((R/f'demandprobe/r{n}/receipt.json').read_text());assert receipt['valid'] and receipt['before']==receipt['after'] and receipt['probe_sources_before']==receipt['probe_sources_after']
 rows+=json.loads((R/f'demandprobe/r{n}/samples.json').read_text())
supp={}
for fixture in sorted({x['fixture'] for x in rows}):
 supp[fixture]={}
 for mode in ['fresh','warm']:
  selected=[x for x in rows if x['fixture']==fixture and x['warm']==(mode=='warm')]
  supp[fixture][mode]={'pooled':{k:dist([x[k] for x in selected]) for k in ['pipeline_ms','preload_ms','book_first_ms','book_total_ms']},'per_round':{str(n):{k:dist([x[k] for x in selected if x['round']==n]) for k in ['pipeline_ms','preload_ms','book_first_ms']} for n in [1,2,3]}}
save(R/'demandprobe/distributions.json',supp)
(R/'demandprobe/protocol.md').write_text('''# Font-catalog first-demand control

This post-window diagnostic composes existing public APIs. It does not alter production code or expose a benchmarking API. The decorator delegates the actual World contract to SystemWorld and times calls to its book accessor. Cargo JSON in artifact.json verifies release optimization level 3 with debug assertions and overflow checks disabled. The independent Cargo.lock and build command/output are retained. Source-before.json and each receipt record the exact helper sources; the original frozen consumer/input fingerprint is checked again.

Both modes create a fresh SystemWorld with identical root, input and project font directory plus system fonts. Source parsing and world construction occur before timing. Fresh mode starts without requesting the catalog. Warm mode calls inner.book before the pipeline timer and records that preload separately. Each mode invokes compile_to_pdf_bytes_with_timings with Verbose output and disabled tags. PDF writing occurs after timing; temporary work/output ownership inside the compilation API remains included, while final PDF/world destruction occurs afterward. The book timer includes acquisition of the entire actual catalog, not an isolated metadata or filesystem primitive. Atomic/mutex/timer decorator overhead is included. A separate backtrace mode captures the first call path and is excluded from latency summaries.

Three rounds, four alternating-order pairs per fixture (shapes, pipeline-single-font, context): 72 measured pipeline calls in new processes. CPU 4, Rayon one, warm filesystem caches and unchanged governor/boost. Each round has a 30-second quiet gate and continuous 100ms process checking. Discard a round for concurrent builds/tests/benchmarks or consumer/input/helper/executable mutation. All three accepted receipts are retained. Preparation was restarted before accepted rounds; partial preparation evidence is excluded. Do not compare this helper's absolute durations directly with the CLI or combine them into a synthetic CLI total.

summary.json retains means, medians and ranges; distributions.json additionally retains SD/p05/p95 pooled and per round. All raw samples remain in r1/r2/r3. Selected output controls compare page fields and normalized text for fresh/warm pairs, not universal parity. Preload+warm is essential: moving acquisition outside the pipeline timer is not elimination of work, a production speedup or evidence of a persistent catalog across processes. book_calls counts cached accessor requests, not catalog rebuilds. The first-call share is direct interval accounting within this instrumented population and must not be transplanted into different CLI populations.
''')
e=json.loads((R/'e2e-summary.json').read_text());p=json.loads((R/'phase-summary.json').read_text());m=json.loads((R/'micro-summary.json').read_text());d=json.loads((R/'demandprobe/summary.json').read_text());f=json.loads((R/'frozen-state.json').read_text());b=json.loads((R/'build-provenance.json').read_text())
wall='\n'.join('| '+name+' | '+ ' | '.join(f"{x['median']:.2f} [{x['p05']:.2f}, {x['p95']:.2f}]" for x in [r['engines']['tekt'],r['engines']['vanilla']])+f" | {r['paired_ratio']['median']:.2f} | {r['paired_delta_ms']['median']:.2f} |" for name,r in e.items())
phase='\n'.join('| '+name+' | '+ ' | '.join(f'{v:.2f}' for v in r['mean_delta_ms'].values())+f" | {r['mean_wall_delta_ms']:.2f} |" for name,r in p.items())
demand='\n'.join(f"| {name} | {r['fresh']['pipeline_ms']['mean']:.2f} | {r['fresh']['book_first_ms']['mean']:.2f} | {100*r['fresh_first_book_fraction_of_pipeline_mean']:.2f}% | {r['warm']['pipeline_ms']['mean']:.2f} | {r['warm_preload_plus_pipeline_mean_ms']:.2f} | {r['fresh']['book_calls']['mean']} |" for name,r in d.items())
stable='\n'.join('- `'+k+'`' for k,v in m.items() if v['stable_absolute'])
artifacts='\n'.join('| `'+k+'` | `'+v['sha256']+'` |' for k,v in b['artifacts'].items())
report=f'''# Post-correction infra measurements and remaining defects

The measured first acquisition of the full font catalog dominates the instrumented Tekt control: 95.21–97.38% of fresh pipeline time for three fixtures. The shapes fixture reaches this acquisition through vertical metrics requested by placement layout. Vanilla records no font-discovery span for the shapes/image fixtures in this run. This identifies a concrete demand-path difference worth investigating; it does not establish that all layout time is layout-algorithm work or that complete PDF export is a stream cost.

The performance battery is **exploratory, not an accepted stable absolute baseline**. Only 6 of 235 microbenchmark cases pass the predeclared three-round rule; 229 fail. All ten plain CLI engine/fixture populations fail. Accepted receipts prove source stability and absence of detected concurrent builds/tests/benchmarks, not quiet hardware, thermal stability or stable timing. Raw results are retained rather than replaced by a favorable subset. No production optimization or correction was made during this study.

## Provenance and reproduction

HEAD: `{f['head']}`. This is a dirty working tree, not HEAD alone. `frozen-status.json` preserves the exact status including untracked files and `git diff HEAD --stat`; `incoming.patch.gz` preserves the full tracked binary patch. `frozen-state.json` records actual bytes of consumers, prompts, nuclei, existing benchmarks, manifests/locks, producer scripts, inputs, upstream and font/config files. Aggregate SHA-256: `{f['sha256']}`. Fingerprints match before/after every accepted primary cohort and all three demand-control rounds. Source archive `consumers-inputs.tar.gz` preserves the repository-local frozen files; absolute external dependencies/fonts remain identified by paths/hashes and reconstruction provenance. The final audit checks existing changes are preserved and prior evidence seals remain intact.

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
{artifacts}

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
{wall}

`e2e-summary.json` retains raw pair ratios/deltas, pooled and per-round mean/median/SD/min/max/p05/p95. `cli-stability.json` shows each population's drift; no absolute CLI population passes. Do not use these ratios as a stable machine-independent promise or subtract old runs to claim the correction commits improved performance.

## Compatible broad phase accounting

The following **signed mean deltas use only the 18 traced observations per engine/fixture**. Within that population the four columns sum to the measured mean wall delta. They do not decompose the plain medians above. Eval/layout envelopes include first lazy acquisition where demand happens; PDF includes complete resources/builder/serialization. Residual includes remaining startup/setup/I/O/teardown/instrumentation and timer-boundary differences.

| Fixture | Initial eval delta ms | Stabilization/layout delta ms | Complete PDF delta ms | Residual delta ms | Traced wall delta ms |
| --- | --- | --- | --- | --- | --- |
{phase}

`phase-summary.json` preserves each engine's absolute distributions; `phase-samples.json` retains the underlying timings, traces and interval interpretation. `phase-envelopes.png`/`.svg` visualize the means. The large shapes/image layout envelopes warrant investigating acquisition before optimizing layout arithmetic. PDF deltas are small in these fixtures; none is evidence of an isolated stream bottleneck. Shifts relative to historical timers can reflect lazy acquisition moving between envelopes, not changed algorithm costs. No addition of microbenchmarks from different fixtures is used.

The absolute micro stability rule requires mean-versus-median of three round means <3% and range/median <=5%. The only passing cases are:

{stable}

All other results remain exploratory in `micro-summary.json`, including every matched Tekt/vanilla font pair (none has both sides stable). `font-comparisons.json` provides descriptive per-font/per-round ratios only. Metadata+coverage ratios roughly 1.18–1.79 and CJK first-font ratios 2.22–4.50 motivate equivalent-work controls, not a decomposition of the full catalog or CLI gap. Tekt's CJK slot reconstructs a collection face while vanilla retains collection/index; other first-font cases favor Tekt but include unequal internal work. Constructor/contains/cmap controls cannot be summed into a catalog-acquisition estimate.

## Controlled first catalog demand

A diagnostic World decorator delegates to the real SystemWorld and the real public paged/PDF pipeline. A fresh-world mode begins with the catalog unrequested; the otherwise equivalent warm mode preloads it before the timer. Source parsing, world construction and PDF file writing are excluded from the pipeline timer; decorator overhead is included. Three rounds × four pairs × three fixtures yield 72 measured pipeline calls. All before/after consumer/input/helper fingerprints and executable hashes match; all quiet/continuous process gates pass. Separate untimed backtraces identify first demand. Sources, all raw calls, distributions, receipts and output checks are under `demandprobe/`.

| Fixture | Fresh pipeline mean ms | First book acquisition mean ms | Included fraction | Preloaded pipeline mean ms | Preload + pipeline mean ms | Book accessor calls |
| --- | --- | --- | --- | --- | --- | --- |
{demand}

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
'''
(R.parent/'infra-performance-followup.md').write_text(report)
# Validate exact frozen bytes and preserve local source/input snapshot.
for path,digest in f['files'].items():
 q=Path(path) if Path(path).is_absolute() else ROOT/path
 assert sha(q)==digest,path
with tarfile.open(R/'consumers-inputs.tar.gz','w:gz') as archive:
 for path in sorted(f['files']):
  if not Path(path).is_absolute():archive.add(ROOT/path,arcname=path,recursive=False)
prior={}
for name in ['performance-baseline','font-lifecycle-experiment','font-microbenchmarks','font-microbenchmarks-vanilla','vanilla-font-mechanics','reuse-trust-study','reuse-key-followup','shaping-axis-key-fix','shaper-prompt-audit','shaper-contract-fixes','infra-contract-review','infra-axis-gate-review']:
 base=R.parent/name;entries=0
 for line in (base/'MANIFEST.sha256').read_text().splitlines():
  digest,path=line.split(maxsplit=1);path=path.lstrip('*');assert sha(base/path)==digest,(name,path);entries+=1
 audit=base/'completion-audit.json';a=json.loads(audit.read_text()) if audit.exists() else {};report_hash=sha(R.parent/(name+'.md'))
 if 'report_sha256' in a:assert a['report_sha256']==report_hash,name
 prior[name]={'manifest_sha256':sha(base/'MANIFEST.sha256'),'entries_verified':entries,'report_sha256':report_hash,'report_checked_against_prior_audit':'report_sha256' in a}
spec=importlib.util.spec_from_file_location('frozen_run',R/'run.py');module=importlib.util.module_from_spec(spec);spec.loader.exec_module(module);now=module.fp();assert now['head']==f['head'] and now['sha256']==f['sha256']
assert subprocess.check_output(['git','diff','HEAD','--binary'],cwd=ROOT)==gzip.decompress((R/'incoming.patch.gz').read_bytes())
for path,digest in json.loads((R/'demandprobe/source-before.json').read_text()).items():assert sha(R/'demandprobe'/path)==digest
save(R/'completion-audit.json',{'completed_utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'head':now['head'],'frozen_fingerprint':now['sha256'],'report_sha256':sha(R.parent/'infra-performance-followup.md'),'source_archive_sha256':sha(R/'consumers-inputs.tar.gz'),'prior_sealed_evidence':prior,'tracked_diff_matches_incoming_patch':True,'production_L0_ADR_test_existing_benchmark_edits':False,'scope':['00_nucleo/diagnosticos/infra-performance-followup/','00_nucleo/diagnosticos/infra-performance-followup.md'],'admitted_primary_cohorts':json.loads((R/'accepted-blocks.json').read_text()),'micro_cases':len(m),'stable_micro_cases':sum(v['stable_absolute'] for v in m.values()),'stable_absolute_baseline_claimed':False,'plain_cli_invocations':360,'traced_cli_invocations':180,'demand_control_invocations':len(rows),'demand_rounds_valid':3,'all_executable_profiles_release_optimized':True,'all_current_fingerprints_match':True,'status_after':subprocess.check_output(['git','status','--porcelain=v1','--untracked-files=all'],cwd=ROOT,text=True),'diff_stat_after':subprocess.check_output(['git','diff','HEAD','--stat'],cwd=ROOT,text=True)})
entries=[]
for q in sorted(R.rglob('*')):
 if q.is_file() and q.name!='MANIFEST.sha256':entries.append(sha(q)+'  '+str(q.relative_to(R)))
(R/'MANIFEST.sha256').write_text('\n'.join(entries)+'\n')
print('Report, archive, audit and manifest completed; frozen state and 12 prior datasets verified.')
