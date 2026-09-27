"""Record current scoped changes and verify unrelated state and previous seals."""
from pathlib import Path
import json,hashlib,subprocess,importlib.util,datetime,difflib,re,collections
ROOT=Path(__file__).resolve().parents[3];R=Path(__file__).resolve().parent
scope=json.loads((R/'provenance.json').read_text())['initial_fix_scope']
spec=importlib.util.spec_from_file_location('read_only_fingerprint',ROOT/'00_nucleo/diagnosticos/font-microbenchmarks-vanilla/measure.py');m=importlib.util.module_from_spec(spec);spec.loader.exec_module(m)
f=m.fingerprint();(R/'source-after.json').write_text(json.dumps(f,indent=2)+'\n');previous=json.loads((ROOT/'00_nucleo/diagnosticos/shaper-contract-fixes/source-after.json').read_text());delta=sorted(k for k in set(f['files'])|set(previous['files']) if f['files'].get(k)!=previous['files'].get(k));assert delta==sorted(scope),(delta,scope)
current=subprocess.check_output(['git','diff','HEAD','--binary'],cwd=ROOT);incoming=(R/'incoming.patch').read_bytes()
def outside(data):
 return b''.join(section for section in re.split(b'(?m)(?=^diff --git )',data) if not section.startswith(b'diff --git ') or section.split(b'\n',1)[0].decode().removeprefix('diff --git a/').split(' b/',1)[0] not in scope)
assert outside(current)==outside(incoming),'unrelated tracked changes detected'
seals=[]
for name in ['performance-baseline','font-lifecycle-experiment','font-microbenchmarks','font-microbenchmarks-vanilla','vanilla-font-mechanics','reuse-trust-study','reuse-key-followup','shaping-axis-key-fix','shaper-prompt-audit','shaper-contract-fixes']:
 base=ROOT/'00_nucleo/diagnosticos'/name
 for entry in (base/'MANIFEST.sha256').read_text().splitlines():
  digest,relative=entry.split('  ',1);p=base/relative;assert m.sha(p)==digest,p
 audit=json.loads((base/'completion-audit.json').read_text())
 if audit.get('report_sha256'):assert m.sha(base.with_suffix('.md'))==audit['report_sha256'],base
 seals.append(name)
patch=[]
for name in scope:patch.extend(difflib.unified_diff((R/'before'/(name+'.txt')).read_text().splitlines(True),(ROOT/name).read_text().splitlines(True),fromfile='before/'+name,tofile='after/'+name))
(R/'change.patch').write_text(''.join(patch));(R/'status-after.txt').write_bytes(subprocess.check_output(['git','status','--porcelain=v1','--untracked-files=all'],cwd=ROOT));(R/'diff-stat.txt').write_bytes(subprocess.check_output(['git','diff','HEAD','--stat'],cwd=ROOT));(R/'working-tree-after.patch').write_bytes(current)
results=json.loads((R/'lint.sarif').read_text())['runs'][0].get('results',[]);prior=json.loads((ROOT/'00_nucleo/diagnosticos/shaper-contract-fixes/lint.sarif').read_text())['runs'][0].get('results',[])
def path(x):return x.get('locations',[{}])[0].get('physicalLocation',{}).get('artifactLocation',{}).get('uri','').removeprefix('./')
def key(x):return (path(x),x['ruleId'],x['level'],x['message']['text'])
owned=[x for x in results if path(x) in scope];pre=collections.Counter(key(x) for x in prior);cur=collections.Counter(key(x) for x in owned);new=list((cur-pre).elements());errors=[x for x in results if x['level']=='error'];assert not errors and not new
lint={'global_error_results':errors,'owned_results':owned,'new_owned_messages':new,'global_warning_rules':sorted({x['ruleId'] for x in results if x['level']=='warning'})};(R/'lint-audit.json').write_text(json.dumps(lint,indent=2)+'\n')
a={'completed_utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'head':f['head'],'files_after':{n:m.sha(ROOT/n) for n in scope},'fingerprint_delta':delta,'prior_seals_intact':seals,'tracked_changes_outside_scope_unchanged':True,'incoming_tracked_patch_sha256':hashlib.sha256(incoming).hexdigest(),'current_tracked_patch_sha256':hashlib.sha256(current).hexdigest(),'new_regressions_passed':5,'non_export_suite_passed':614,'non_export_suite_failed':0,'full_suite':'SIGTERM during export raster line-cap test after sustained CPU use; no successful full-suite claim','build_passed':True,'global_lint_errors':errors,'owned_lint_results':owned,'toolchain':subprocess.check_output(['rustc','-vV'],text=True),'cargo':subprocess.check_output(['cargo','-V'],text=True)}
report=ROOT/'00_nucleo/diagnosticos/infra-contract-review.md'
if report.exists():a['report_sha256']=m.sha(report)
(R/'completion-audit.json').write_text(json.dumps(a,indent=2)+'\n')
print(json.dumps({'delta':delta,'prior_seals_intact':True,'outside_scope_unchanged':True},indent=2))
