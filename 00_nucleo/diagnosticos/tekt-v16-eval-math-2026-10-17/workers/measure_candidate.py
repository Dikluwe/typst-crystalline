from pathlib import Path
import json,subprocess,collections,hashlib,datetime,gzip
out=Path(__file__).resolve().parents[1];b=json.load(gzip.open(out/'before.json.gz'));h=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest();hashes={p:h(p) for p in b['hashes']};cmd=[b['linter']['binary'],'.','--format','sarif']
with (out/'candidate-architecture.log').open('w') as f:r=subprocess.run(cmd,stdout=f,stderr=subprocess.PIPE,text=True)
assert r.returncode==1,r.stderr;rows=json.load(open(out/'candidate-architecture.log'))['runs'][0]['results'];counts=dict(collections.Counter(x['ruleId']+':'+x['level'] for x in rows));record={'utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'command':cmd,'exit':r.returncode,'counts':counts,'source_hashes':hashes,'files':{p:Path(p).read_text() for p in ['01_core/src/compiler/eval/tests.rs','00_nucleo/prompts/compiler/eval/tests.md']},'source_unchanged':all(h(p)==v for p,v in hashes.items())}
with gzip.open(out/'candidate-state.json.gz','wt') as f:json.dump(record,f)
(out/'candidate-result.json').write_text(json.dumps({'counts':counts,'source_unchanged':record['source_unchanged']},indent=2)+'\n');print(json.dumps(counts),flush=True)
