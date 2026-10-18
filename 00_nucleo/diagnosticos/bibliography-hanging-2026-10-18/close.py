from pathlib import Path
import json,gzip,hashlib,datetime,subprocess,collections,re
p=Path(__file__).resolve().parent;r=Path.cwd()
now=lambda:datetime.datetime.now(datetime.timezone.utc).isoformat()
def sha(f):
 with Path(f).open('rb') as s:return hashlib.file_digest(s,'sha256').hexdigest()
read=lambda n:json.loads(gzip.decompress((p/n).read_bytes()))
before=read('before-sources.json.gz');freeze=read('freeze.json.gz')
assert all(sha(f)==h for f,h in freeze['sources'].items())
changed=[f for f,s in before['files'].items() if hashlib.sha256(s.encode()).hexdigest()!=freeze['sources'][f]]
added=sorted(set(freeze['sources'])-set(before['files']))
resources={str(f.relative_to(p))[:-5]:json.loads(f.read_text()) for f in p.rglob('*-resources.json')}
assert resources['gates-resources']['exit']==1
assert resources['finish-resources']['exit']==resources['measure-resources']['exit']==resources['deco-empty-green-resources']['exit']==0
for key in ['gates-resources','measure-resources','finish-resources']:
 assert datetime.datetime.fromisoformat(resources[key]['start']) >= datetime.datetime.fromisoformat(freeze['utc']),key
for d in resources.values():
 assert d['memory_after']['memory.max']==str(12*2**30) and d['memory_after']['memory.swap.max']=='0'
 events=dict(line.split() for line in d['memory_after']['memory.events'].splitlines());assert events['oom']==events['oom_kill']=='0'
peak=max(int(d['memory_after']['memory.peak']) for d in resources.values())
suite=json.loads((p/'suite-summary.json').read_text());assert suite['failed']==0
binsha=sha('target/debug/typst')
for f in ['after.json.gz','extra-after.json.gz','boundary-after.json.gz','repeat-after.json.gz','controls-after.json.gz','controls-extra-after.json.gz']:
 assert read(f)['binaries']['tekt']['sha256']==binsha,f
for f in ['deco-after.json.gz','deco-empty-after.json.gz']:
 assert all(c['results']['tekt']['binary_sha256']==binsha for c in read(f)['cases']),f
assert sha('/tmp/tekt-linter-none-target/debug/crystalline-lint')=='e2e51f3074cee2d0179bfa89f5b12884728f3d2d8f3113589dd095ef1d9eadb2'
assert sha('lab/typst-original/target/release/typst')=='9b2373155e9c05bbb0daf5755bf4be88a11e64bd6b4f8b376648fa27303373b9'
proof=dict(utc=now(),head=subprocess.check_output(['git','rev-parse','HEAD'],text=True).strip(),working_tree='não commitado',status=subprocess.check_output(['git','status','--porcelain=v1','--untracked-files=all'],text=True),diffstat=subprocess.check_output(['git','diff','HEAD','--stat'],text=True),changed=changed,added=added,changed_hashes={f:sha(f) for f in changed+added},source_count=len(freeze['sources']),candidate_binary_sha256=binsha,memory_peak_bytes=peak,memory_peak_gib=peak/2**30,resources={k:dict(exit=v['exit'],peak_bytes=int(v['memory_after']['memory.peak']),events=v['memory_after']['memory.events']) for k,v in resources.items()},suite=suite,goal_complete=False,pending='proximos.md')
(p/'final-state.json.gz').write_bytes(gzip.compress(json.dumps(proof,indent=2,ensure_ascii=False).encode()))
(p/'after-sources.json.gz').write_bytes(gzip.compress(json.dumps(dict(utc=now(),head=proof['head'],status=proof['status'],diffstat=proof['diffstat'],files={f:Path(f).read_text() for f in freeze['sources']}),ensure_ascii=False).encode()))
print(json.dumps({k:proof[k] for k in ['head','source_count','candidate_binary_sha256','memory_peak_bytes','memory_peak_gib','goal_complete']},indent=2));print('tests',suite['passed'],suite['failed'],suite['ignored'])
