from pathlib import Path
import xml.etree.ElementTree as E
import json,re,hashlib,datetime,difflib,subprocess
R=Path(__file__).resolve().parent;ROOT=R.parents[2]
def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()
def geometry(p,kind):
 root=E.parse(p).getroot()
 for x in root.iter():
  if (kind.startswith('image') and x.tag.rsplit('}',1)[-1]=='image') or (not kind.startswith('image') and x.attrib.get('fill')=='#0074d9'):
   a=x.attrib
   if 'transform' in a:
    m=re.fullmatch(r'translate\(([-\d.]+) ([-\d.]+)\)',a['transform']);assert m,a
    xy=tuple(float(v) for v in m.groups())
   else:xy=(float(a['x']),float(a['y']))
   return xy
 raise RuntimeError(str(p))
rows=[]
for stage,folder in [('before',R),('after',R/'after')]:
 for fixture in ['image','shape','placed_shape','image_then_text']:
  for edge in ['default','0pt','20pt']:
   xy={engine:geometry(folder/f'{fixture}-{edge}-{engine}.svg',fixture) for engine in ['tekt','vanilla']};rows.append({'stage':stage,'fixture':fixture,'edge':edge,'object_translation_xy':xy,'equal':xy['tekt']==xy['vanilla']})
assert all(x['equal'] for x in rows if x['stage']=='after')
(R/'geometry-summary.json').write_text(json.dumps(rows,indent=2)+'\n')
original=json.loads((R/'provenance.json').read_text());final=json.loads((R/'after/provenance.json').read_text());changed={p for p,h in original['fingerprint_before'].items() if final['fingerprint_before'][p]!=h};scope={'01_core/src/compiler/layout/'+x+'.rs' for x in ['image','shape','line_commit']}|{'00_nucleo/prompts/compiler/layout-image.md','00_nucleo/prompts/compiler/layout/shape_block_behaviour.md','00_nucleo/prompts/compiler/layout/line_commit.md'};assert changed==scope
patch=[]
for name,p in [('image','00_nucleo/prompts/compiler/layout-image.md'),('shape','00_nucleo/prompts/compiler/layout/shape_block_behaviour.md'),('line_commit','00_nucleo/prompts/compiler/layout/line_commit.md')]:
 for old,new in [(R/'before'/(name+'.rs'),ROOT/'01_core/src/compiler/layout'/(name+'.rs')),(R/'before'/(name+'.md'),ROOT/p)]:
  patch+=list(difflib.unified_diff(old.read_text().splitlines(keepends=True),new.read_text().splitlines(keepends=True),fromfile='before/'+str(new.relative_to(ROOT)),tofile=str(new.relative_to(ROOT))))
(R/'change.patch').write_text(''.join(patch))
# Raw prior diagnostics stay immutable despite the intentional consumer correction.
prior={}
for name in ['infra-performance-followup','shaper-language-fix']:
 p=R.parent/name
 for line in (p/'MANIFEST.sha256').read_text().splitlines():
  digest,q=line.split(maxsplit=1);assert sha(p/q)==digest,q
 a=json.loads((p/'completion-audit.json').read_text());assert sha(p.with_suffix('.md'))==a['report_sha256'];prior[name]=sha(p/'MANIFEST.sha256')
report='''# Font-demand geometry study and first-block correction

Equivalent-input controls against pinned vanilla demonstrate that a first image or shape in normal flow must not move merely because text top-edge changes. Tekt's previous baseline preparation followed by cap-height subtraction incorrectly displaced those blocks. The correction uses the physical region origin for a fresh main-flow block with initial baseline pending and no pending line. It avoids acquiring font metrics to synthesize that reference. Pending-text and subframe paths remain distinct.

The empty-line flush independently requested text edges even when its material classification was false. The correction supplies neutral local edge values in that non-material case while retaining fraction/repeat expansion, drain/reset, marker transport and existing state protocol. Material lines still measure current/next text edges. There is no global cache, new public API, new default mode or pipeline phase change; this is an L0-first internal geometry parity repair under ADR-0127.

## Evidence and controls

`run.py` compiled the same four inputs (image, shape, explicit top-left Place, image then text) with default top-edge, 0pt and 20pt. Vanilla pin is `9f2b6e8715237cb086899a42873660fe744622e8`; CLI SHA-256 is `7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea`. Both engines receive identical JPEG/project-font bytes, root and dimensions. Original consumers stayed byte-identical through this initial 24-compilation geometry run. The pilot's invalid bare ascender identifier and failed attempts remain separate under `pilot/`; no failed pilot output is used as a geometry result.

Before correction, the 80×40pt shape at margin 8pt had Tekt y positions 8, 0.762 and 20.762 for default/0pt/20pt. Vanilla stayed at y=8 in all three. The cover-fit image object's y values were -12, -19.238 and 0.762 in Tekt versus -12 throughout vanilla (the 80×80pt image is centered/clipped into an 80×40pt target whose top is y=8). The explicit placed shape stayed at (11,13) in both engines for every edge, distinguishing the general subframe reference from the first-flow-block defect. Image-then-text controls retain the broader emitted output for investigation.

`run-after.py` preserves a second independent set of all 24 compilations under `after/`. Geometry extraction in `geometry-summary.json` confirms the corrected object translations equal vanilla for all twelve fixture/edge pairs. This is bounded geometric evidence, not universal page/text/font parity or an exported-byte equality requirement. Both Tekt CLI runs use debug artifacts solely for correctness; their exact hashes and all commands, timestamps, inputs and consumer fingerprints are in the respective provenance/results JSON. **No debug latency or performance ratio is reported.**

Mandatory pure unit controls were added within the image/shape consumers. They test default and explicit 0pt/20pt edges at the physical origin, and inject a FontMetrics provider that panics on any typography request. All four initially failed. An intermediate correction fixed geometry but still failed both no-typography controls, exposing the empty flush. After that shared owner was corrected, the complete layout suite passed: 936 tests, zero failures, including all four new controls. Existing subframe, alignment, block, following-text, wrapping, pagination and layout controls were retained. `cargo build` passed. Global lint exited successfully with zero errors; existing warnings remain. No owned V5/V6/V15/V26 findings appear. The two owned V16 warnings are existing domain-match sites, not new ownership violations. No global formatting or hash rewrite ran; only appended test modules were formatted.

## Actual pipeline demand count

The diagnostic-only `catalog-counts` World decorator delegates actual public APIs to SystemWorld and invokes the real PDF pipeline. It records accessor calls only, with no clocks or latency fields in its result. Its debug profile is explicitly marked functional, not performance evidence. New World/process per invocation, identical root/source/project fonts plus system discovery policy; three repetitions of each fixture:

| Fixture | World::book calls in each repetition |
| --- | --- |
| Isolated image | 0, 0, 0 |
| Isolated shape | 0, 0, 0 |
| Explicit placed shape | 6, 6, 6 |
| Image then text | 14, 14, 14 |

This shows the isolated paths do not demand the font catalog through the accessor and that real text still does. Counts are not catalog rebuilds: OnceLock continues to acquire once per World and reuse it. Delegated internal fallback paths are not universally intercepted, so this is not a global filesystem-I/O count. The rejecting metrics tests independently establish no font-metric request in the isolated corrected path. Counts do not estimate saved milliseconds and are not combined with the earlier release timings.

## Scope and provenance

Working tree at HEAD `0fea906e176acf93ae6301e4cd17ace6aa4fe349`, with earlier changes preserved. Before/after provenance files retain exact dirty/untracked status, diff-stat, binary identities, clocks and consumer/prompt/benchmark fingerprints. The only additional productive/owner changes in this study are:

- `01_core/src/compiler/layout/image.rs` ↔ `00_nucleo/prompts/compiler/layout-image.md`
- `01_core/src/compiler/layout/shape.rs` ↔ `00_nucleo/prompts/compiler/layout/shape_block_behaviour.md`
- `01_core/src/compiler/layout/line_commit.rs` ↔ `00_nucleo/prompts/compiler/layout/line_commit.md`

`before/`, `change.patch` and `scoped-seals.json` retain the scoped transition. L0s were edited first; hashes were updated only for these owners/consumers, retaining the line-commit coordinates nucleus pin. Source inspection established the preexisting owners and contracts before any implementation. The two prior sealed studies (performance and language repair) remain byte-identical. Final audit verifies all inventoried sources outside these six files are unchanged and the final consumer state still matches the post-correction geometry run.

Commands/logs: `cargo test -p typst-core --lib initial_nontext_block_ -- --test-threads=1` (RED/intermediate); `cargo test -p typst-core --lib compiler::layout:: -- --test-threads=1` (final); `cargo build`; `crystalline-lint . --format sarif`; `PYTHONDONTWRITEBYTECODE=1 python3 run.py` and `run-after.py`; diagnostic count build with its retained Cargo.lock using `cargo build --manifest-path .../catalog-counts/Cargo.toml --offline --locked --target-dir target --message-format=json`, followed by `count.py`. Build preparation failures are retained separately from behavioral RED evidence. Helpers and results stay exclusively under this diagnostic directory.

## Remaining investigation

The expensive benchmark fixture contains many explicit Place operations, whose subframes still initialize a typographic origin, whose callers then compensate it. That catalog demand is **not fixed here**. Removing it safely requires matched text/non-text/nested/float/deferred-alignment controls for the existing shared coordinate contract. A blanket zero origin or early return from line closure would risk geometry/state drift. The six placed-shape calls confirm the remaining route; they do not mean six scans. The earlier 95–97% first-acquisition interval remains evidence for its own release control population, not a post-correction speedup claim.

A fresh release measurement under stable conditions is still needed to quantify the isolated-image effect and compare whole catalog acquisition fairly. Fonts after text, later block chains, retained World invalidation, TTC normalization and unproved script/math cache identity remain separate topics. No optimization claim, global sanitization or complete resolution of font demand is made.
'''
(R.parent/'font-demand-geometry-study.md').write_text(report)
lint=json.loads((R/'lint-audit.json').read_text());assert not lint['global_errors'];assert not any(x['ruleId'] in ['V5','V6','V15','V26'] for x in lint['owned_results'])
assert '936 passed; 0 failed' in (R/'layout.stdout').read_text()
# Current consumers must match the output-producing post-correction state.
for p,h in final['fingerprint_after'].items():assert sha(ROOT/p)==h,p
(R/'completion-audit.json').write_text(json.dumps({'utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'head':final['head'],'changed_files':sorted(changed),'report_sha256':sha(R.parent/'font-demand-geometry-study.md'),'before_after_geometry_pairs':24,'after_geometry_pairs_passed':12,'layout_passed':936,'layout_failed':0,'global_lint_errors':0,'owned_lint_results':lint['owned_results'],'source_changes_outside_scope':False,'current_sources_match_after_geometry':True,'prior_seals':prior,'no_performance_claim':True,'final_status':subprocess.check_output(['git','status','--porcelain=v1','--untracked-files=all'],cwd=ROOT,text=True),'final_diff_stat':subprocess.check_output(['git','diff','HEAD','--stat'],cwd=ROOT,text=True)},indent=2)+'\n')
(R/'MANIFEST.sha256').write_text(''.join(sha(p)+'  '+str(p.relative_to(R))+'\n' for p in sorted(R.rglob('*')) if p.is_file() and p.name!='MANIFEST.sha256'))
print('Scoped geometry correction and functional catalog-demand evidence sealed.')
