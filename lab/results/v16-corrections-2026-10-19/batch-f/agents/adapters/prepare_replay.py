"""Freeze case-specific acceptance from baseline evidence before implementation."""
import datetime,hashlib,importlib.util,json,pathlib
H=pathlib.Path(__file__).resolve().parent;R=H.parents[5]
sha=lambda p:hashlib.sha256(pathlib.Path(p).read_bytes()).hexdigest()
spec=importlib.util.spec_from_file_location('adapter_replay',H/'replay.py');m=importlib.util.module_from_spec(spec);spec.loader.exec_module(m)
files=['baseline.json','baseline-supplement.json'];receipts={n:json.loads((H/n).read_text()) for n in files};controls={c['name']:c for c in json.loads((H/'native-controls-baseline.json').read_text())['cases']};by={n:{c['name']:c for c in d['cases']} for n,d in receipts.items()}
manifest=dict(utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),baseline_receipts=[dict(path=n,sha256=sha(H/n),case_count=len(receipts[n]['cases'])) for n in files],oracle_pin=json.loads((H.parents[1]/'baseline-build-receipt.json').read_text())['oracle'],supporting_evidence=[],fixtures=[dict(path=str((H/'works.yml').relative_to(R)),sha256=sha(H/'works.yml')),dict(path=str((H/'missing.yml').relative_to(R)),absent=True)],cases={})
for p in [H.parents[1]/'baseline-build-receipt.json',H/'native-controls-baseline.json',R/'01_core/src/compiler/stdlib/primitives_constructors/array.rs',R/'01_core/src/compiler/eval/call_dispatch/apply.rs']:
 manifest['supporting_evidence'].append(dict(path=str(p.relative_to(R)),sha256=sha(p)))
def observed(file,name,label='product'):return m.observe(by[file][name]['results'][label])
def control(name):return m.observe(controls[name]['results']['product'])
def require(file,name,expected,reason,policy='native-constructor residual retained'):
 key=file+'::'+name;manifest['cases'][key]['expected_observation']=expected;manifest['cases'][key]['reason']=reason;manifest['cases'][key]['policy']=policy
for file,receipt in receipts.items():
 for c in receipt['cases']:
  old=m.observe(c['results']['product']);manifest['cases'][file+'::'+c['name']]=dict(expression_sha256=hashlib.sha256(c['expression'].encode()).hexdigest(),baseline_observation=old,expected_observation=old,policy='baseline preservation',reason='Unchanged contract; retain the exact parsed result and complete diagnostics, including preexisting parity debt.')
repairs=['decimal-map','regex-map','selector-map','stroke-map','version-map','label-map','array-map','arguments-map','decimal-key','version-key','empty-decimal-map','empty-duration-map','empty-arguments-map']
for n in repairs:require('baseline.json',n,observed('baseline.json',n,'oracle'),'Existing constructor admission must now match the frozen pinned oracle exactly.','oracle repair')
require('baseline.json','duration-map',observed('baseline.json','duration-direct'),'Forward existing duration(1) error exactly; source constructor has preexisting positional-language/diagnostic debt.')
require('baseline.json','color.spot-map',observed('baseline.json','color.spot-direct'),'Forward existing name-only SpotColorant missing-fallback error exactly; oracle trace differences remain explicit.')
require('baseline.json','tiling-map',dict(returncode=0,stderr='',value=[observed('baseline.json','tiling-direct')['value']]),'Wrap the measured existing native tiling representation once in map result; representation debt belongs to native tiling/repr owners.')
x=observed('baseline.json','str-filter').copy()
expr=by['baseline.json']['label-filter']['expression'];old_expr=by['baseline.json']['str-filter']['expression']
start=expr.index('(',expr.index('.filter'));old_start=old_expr.index('(',old_expr.index('.filter'))
assert x['stderr'].startswith('error: expected boolean, found string\n')
x['stderr']=x['stderr'].replace('found string','found label',1).replace(old_expr,expr).replace(f':1:{old_start}',f':1:{start}').replace('  │ '+' '*old_start+'^'*(len(old_expr)-old_start),'  │ '+' '*start+'^'*(len(expr)-start))
require('baseline.json','label-filter',x,'Same frozen predicate-result validator as str-filter; Label result requires the exact type word label. Retain full Args source diagnostic for the original label-filter expression using its measured parser span rule; no coordinate or trace stripping.')
require('baseline.json','decimal-by',control('decimal-arity'),'sorted forwards both arguments to existing Decimal; use independently measured decimal("2", "1") complete product error.')
for n in ['decimal-semantic','regex-semantic','selector-semantic','stroke-semantic','array-semantic','arguments-semantic','tiling-type','version-semantic','label-semantic','duration-lazy']:
 require('baseline-supplement.json',n,observed('baseline-supplement.json',n,'oracle'),'Require frozen oracle language value from newly admitted existing native constructor.','oracle repair')
require('baseline-supplement.json','spot-positive',control('spot-shortcut'),'Preserve independently measured existing native SpotColorant single-color shortcut; oracle rejects this already-established direct constructor behavior, so no parity claim.')
require('baseline-supplement.json','duration-native-error',observed('baseline.json','duration-direct'),'Forward the measured existing duration(1) error; native positional/error debt retained.')
require('baseline-supplement.json','decimal-native-error',control('decimal-invalid'),'Forward measured decimal("bad") diagnostic without rewriting native behavior.')
require('baseline-supplement.json','selector-native-error',control('selector-invalid'),'Forward measured selector(1) diagnostic without rewriting native behavior.')
require('baseline-supplement.json','spot-native-error',observed('baseline.json','color.spot-direct'),'Forward measured name-only SpotColorant missing-fallback error.')
require('baseline-supplement.json','array-native-error',dict(returncode=1,stdout='',stderr='error: type array does not have a constructor\n\n'),'Existing native_array_bytes rejects non-Bytes with this exact message at passed Args span (array.rs). apply_func directly forwards Args::positional with detached span; no source-call span or trace is invented. This differs from the measured direct-dispatch callee span and from vanilla general Array casts.')
for n,typ in [('by-error','version'),('arguments-by-error','arguments')]:
 x=observed('baseline-supplement.json',n).copy();old='error: expected function, found '+typ;new='error: expected boolean, found '+typ
 assert x['stderr'].startswith(old+'\n');x['stderr']=x['stderr'].replace(old,new,1)
 assert control('by-'+typ)['stderr'].startswith(new+'\n')
 require('baseline-supplement.json',n,x,'The independently measured by callback returns '+typ+' and fails the existing Bool validator. Preserve the original corpus Args source/span verbatim; replace only the prior admission diagnostic with the measured result diagnostic, without stripping coordinates or traces.')
p=H/'replay-manifest.json';assert not p.exists();p.write_text(json.dumps(manifest,indent=2));print(len(manifest['cases']),sha(p))
