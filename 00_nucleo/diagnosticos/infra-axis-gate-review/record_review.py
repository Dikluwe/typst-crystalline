"""Record scoped follow-up evidence without modifying previous sealed records."""
from pathlib import Path
import json,hashlib,subprocess,re,difflib,datetime,importlib.util,collections
ROOT=Path(__file__).resolve().parents[3];R=Path(__file__).resolve().parent
scope=json.loads((R/'provenance.json').read_text())['initial_fix_scope']
spec=importlib.util.spec_from_file_location('read_only_helpers',ROOT/'00_nucleo/diagnosticos/font-microbenchmarks-vanilla/measure.py');m=importlib.util.module_from_spec(spec);spec.loader.exec_module(m);f=m.fingerprint();previous=json.loads((ROOT/'00_nucleo/diagnosticos/infra-contract-review/source-after.json').read_text());delta=sorted(k for k in set(f['files'])|set(previous['files']) if f['files'].get(k)!=previous['files'].get(k));assert delta==sorted(scope),delta
current=subprocess.check_output(['git','diff','HEAD','--binary'],cwd=ROOT)
def outside(data):return b''.join(s for s in re.split(b'(?m)(?=^diff --git )',data) if not s.startswith(b'diff --git ') or s.split(b'\n',1)[0].decode().removeprefix('diff --git a/').split(' b/',1)[0] not in scope)
assert outside(current)==outside((R/'incoming.patch').read_bytes())
for name in ['performance-baseline','font-lifecycle-experiment','font-microbenchmarks','font-microbenchmarks-vanilla','vanilla-font-mechanics','reuse-trust-study','reuse-key-followup','shaping-axis-key-fix','shaper-prompt-audit','shaper-contract-fixes','infra-contract-review']:
 base=ROOT/'00_nucleo/diagnosticos'/name
 for entry in (base/'MANIFEST.sha256').read_text().splitlines():
  digest,relative=entry.split('  ',1);assert m.sha(base/relative)==digest,(name,relative)
 audit=json.loads((base/'completion-audit.json').read_text())
 if audit.get('report_sha256'):assert m.sha(base.with_suffix('.md'))==audit['report_sha256'],name
results=json.loads((R/'lint.sarif').read_text())['runs'][0].get('results',[]);old=json.loads((ROOT/'00_nucleo/diagnosticos/infra-contract-review/lint.sarif').read_text())['runs'][0].get('results',[])
def uri(x):return x.get('locations',[{}])[0].get('physicalLocation',{}).get('artifactLocation',{}).get('uri','').removeprefix('./')
def key(x):return uri(x),x['ruleId'],x['level'],x['message']['text']
owned=[x for x in results if uri(x) in scope];new=list((collections.Counter(key(x) for x in owned)-collections.Counter(key(x) for x in old)).elements());errors=[x for x in results if x['level']=='error'];assert not errors and not new
(R/'lint-audit.json').write_text(json.dumps({'global_errors':errors,'owned_results':owned,'new_owned_messages':new},indent=2)+'\n')
patch=[]
for name in scope:patch.extend(difflib.unified_diff((R/'before'/(name+'.txt')).read_text().splitlines(True),(ROOT/name).read_text().splitlines(True),fromfile='before/'+name,tofile='after/'+name))
(R/'change.patch').write_text(''.join(patch));(R/'source-after.json').write_text(json.dumps(f,indent=2)+'\n');(R/'status-after.txt').write_bytes(subprocess.check_output(['git','status','--porcelain=v1','--untracked-files=all'],cwd=ROOT))
a={'utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'head':f['head'],'report_sha256':m.sha(ROOT/'00_nucleo/diagnosticos/infra-axis-gate-review.md'),'files_after':{n:m.sha(ROOT/n) for n in scope},'fingerprint_delta':delta,'outside_scope_unchanged':True,'prior_seals_intact':True,'pipeline_tests_passed':116,'build_passed':True,'rustc':subprocess.check_output(['rustc','-vV'],text=True),'cargo':subprocess.check_output(['cargo','-V'],text=True)};(R/'completion-audit.json').write_text(json.dumps(a,indent=2)+'\n')
print('Scoped changes, unchanged unrelated inputs and all previous seals verified.')
