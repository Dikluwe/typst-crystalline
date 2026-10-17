from pathlib import Path
import gzip,json,hashlib,subprocess,datetime
out=Path('00_nucleo/diagnosticos/place-origem-local-2026-10-14');sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest()
with gzip.open(out/'before-source.json.gz','rt') as f:old=json.load(f)['files']
with gzip.open(out/'native-before.json.gz','rt') as f:old.update(json.load(f))
current={n:dict(text=Path(n).read_text(),sha256=sha(n)) for n in old}
with gzip.open(out/'candidate-before-baseline-replay.json.gz','wt') as f:json.dump(current,f)
state=dict(start=datetime.datetime.now(datetime.timezone.utc).isoformat(),purpose='Replay exact pre-batch sources/owners to measure mixed Align controls, then restore candidate',old={n:v['sha256'] for n,v in old.items()})
try:
 for n,v in old.items():Path(n).write_text(v['text'])
 assert all(sha(n)==v['sha256'] for n,v in old.items())
 with (out/'baseline-replay-build.log').open('w') as f:subprocess.run(['cargo','build','--offline','--locked'],stdout=f,stderr=subprocess.STDOUT,check=True)
 with (out/'baseline-expanded.log').open('w') as f:subprocess.run(['python3','/tmp/tekt-place-expanded-controls.py','baseline-expanded'],stdout=f,stderr=subprocess.STDOUT,check=True)
 assert all(sha(n)==v['sha256'] for n,v in old.items())
finally:
 for n,v in current.items():
  assert sha(n)==old[n]['sha256'],n
  Path(n).write_text(v['text'])
 assert all(sha(n)==v['sha256'] for n,v in current.items())
 state.update(end=datetime.datetime.now(datetime.timezone.utc).isoformat(),candidate_restored=True)
 (out/'baseline-replay.json').write_text(json.dumps(state,indent=2))
with (out/'candidate-rebuild.log').open('w') as f:subprocess.run(['cargo','build','--offline','--locked'],stdout=f,stderr=subprocess.STDOUT,check=True)
with (out/'candidate-expanded.log').open('w') as f:subprocess.run(['python3','/tmp/tekt-place-expanded-controls.py','candidate-expanded'],stdout=f,stderr=subprocess.STDOUT,check=True)
print('Baseline replay and candidate controls complete; sources restored',flush=True)
