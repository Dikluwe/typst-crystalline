from pathlib import Path
import json,gzip,hashlib,datetime,subprocess,collections
p=Path(__file__).resolve().parent;r=Path.cwd();q=p.parent/'bibliography-hanging-2026-10-18'
now=lambda:datetime.datetime.now(datetime.timezone.utc).isoformat()
def sha(f):
 with Path(f).open('rb') as s:return hashlib.file_digest(s,'sha256').hexdigest()
read=lambda n:json.loads(gzip.decompress((p/n).read_bytes()))
freeze=read('freeze.json.gz');before=read('before-sources.json.gz')
assert all(sha(f)==h for f,h in freeze['sources'].items())
resources={f.stem:json.loads(f.read_text()) for f in p.glob('*-resources.json')}
assert resources['final-measure-resources']['exit']==resources['final-finish-resources']['exit']==0
assert resources['final-gates-recheck-resources']['exit']!=0
assert resources['final-gates-resources']['exit']!=0
measure_freeze=read('measure-freeze.json.gz')
for key,frozen in [('final-gates-recheck-resources',freeze),('final-finish-resources',freeze)]:
 assert datetime.datetime.fromisoformat(resources[key]['start']) >= datetime.datetime.fromisoformat(frozen['utc'])
measure_commands=json.loads((p/'measure-results.json').read_text())
assert datetime.datetime.fromisoformat(resources['final-measure-resources']['start']) <= datetime.datetime.fromisoformat(measure_freeze['utc']) <= datetime.datetime.fromisoformat(measure_commands[0]['start'])
assert sorted(f for f,h in measure_freeze['sources'].items() if freeze['sources'][f]!=h)==['00_nucleo/prompts/infra/integration_tests.md','03_infra/src/integration_tests.rs']
for d in resources.values():
 assert d['memory_after']['memory.max']==str(12*2**30) and d['memory_after']['memory.swap.max']=='0'
 events=dict(line.split() for line in d['memory_after']['memory.events'].splitlines());assert events['oom']==events['oom_kill']=='0'
peak=max(int(d['memory_after']['memory.peak']) for d in resources.values())
binsha=sha('target/debug/typst')
for label in ['after','bib-after','regions-after','corpus-after','corpus-extra-after','corpus-boundary-after','corpus-repeat-after','controls-after','controls-extra-after']:
 assert read(label+'.json.gz')['binaries']['tekt']['sha256']==binsha,label
for label in ['deco-after','deco-empty-after']:
 assert all(c['results']['tekt']['binary_sha256']==binsha for c in read(label+'.json.gz')['cases'])
def lint(file):
 results=json.loads(file.read_text())['runs'][0]['results']
 keys=collections.Counter((v['ruleId'],v.get('level'),v['locations'][0]['physicalLocation']['artifactLocation']['uri'],v['message']['text']) for v in results)
 counts=collections.Counter((v['ruleId'],v.get('level')) for v in results)
 return keys,counts
a,ac=lint(q/'architecture.log');b,bc=lint(p/'architecture.log')
assert not [k for k in (b-a) if k[1] in ['error','warning']]
assert not [k for k in (a-b) if k[1] in ['error','warning']]
delta=dict(utc=now(),baseline=str(q/'architecture.log'),current=str(p/'architecture.log'),before=[dict(rule=k[0],level=k[1],count=v) for k,v in ac.items()],after=[dict(rule=k[0],level=k[1],count=v) for k,v in bc.items()],added=[dict(count=v,key=k) for k,v in (b-a).items()],removed=[dict(count=v,key=k) for k,v in (a-b).items()],no_new_errors_or_warnings=True)
(p/'lint-delta.json').write_text(json.dumps(delta,indent=2,ensure_ascii=False)+'\n')
suite=json.loads((p/'suite-summary.json').read_text());assert suite['failed']==0
assert suite['passed']==9041 and suite['ignored']==7 and len(suite['suite'])==29
assert sha('/tmp/tekt-linter-none-target/debug/crystalline-lint')==freeze['linter_sha256']=='e2e51f3074cee2d0179bfa89f5b12884728f3d2d8f3113589dd095ef1d9eadb2'
assert sha('lab/typst-original/target/release/typst')==freeze['oracle_sha256']=='9b2373155e9c05bbb0daf5755bf4be88a11e64bd6b4f8b376648fa27303373b9'
changed=list(freeze['changed_texts'])
indexes={str(Path('target/doc')/name/'index.html'):sha(Path('target/doc')/name/'index.html') for name in ['typst_core','typst_infra','typst_shell','typst']}
proof=dict(utc=now(),head=subprocess.check_output(['git','rev-parse','HEAD'],text=True).strip(),working_tree='não commitado',status=subprocess.check_output(['git','status','--porcelain=v1','--untracked-files=all'],text=True),diffstat=subprocess.check_output(['git','diff','HEAD','--stat'],text=True),changed=changed,added=[],changed_hashes={f:sha(f) for f in changed},source_count=len(freeze['sources']),candidate_binary_sha256=binsha,memory_peak_bytes=peak,memory_peak_gib=peak/2**30,resources={k:dict(exit=v['exit'],peak_bytes=int(v['memory_after']['memory.peak']),events=v['memory_after']['memory.events']) for k,v in resources.items()},suite=suite,goal_complete=False,pending='proximos.md')
(p/'final-state.json.gz').write_bytes(gzip.compress(json.dumps(proof,indent=2,ensure_ascii=False).encode()))
(p/'rustdoc-indexes.json').write_text(json.dumps(dict(utc=now(),indexes=indexes),indent=2)+'\n')
(p/'after-sources.json.gz').write_bytes(gzip.compress(json.dumps(dict(utc=now(),head=proof['head'],status=proof['status'],diffstat=proof['diffstat'],files={f:Path(f).read_text() for f in freeze['sources']}),ensure_ascii=False).encode()))
print(json.dumps({k:proof[k] for k in ['head','source_count','candidate_binary_sha256','memory_peak_bytes','memory_peak_gib','goal_complete']},indent=2));print('tests',suite['passed'],suite['failed'],suite['ignored'])
