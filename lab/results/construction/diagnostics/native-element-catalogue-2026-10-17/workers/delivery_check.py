from pathlib import Path
import json,gzip,hashlib,datetime
out=Path(__file__).resolve().parents[1]
def sha(p):
 h=hashlib.sha256()
 with Path(p).open('rb') as f:
  for data in iter(lambda:f.read(1048576),b''):h.update(data)
 return h.hexdigest()
state=json.load(gzip.open(out/'final-state.json.gz'));manifest=json.load(open(out/'manifest.json'));assert sha(out/'SHA256SUMS')==manifest['manifest_sha256'];checked=0
for line in (out/'SHA256SUMS').read_text().splitlines():
 digest,path=line.split(maxsplit=1);assert sha(out/path)==digest,path;checked+=1
assert checked==manifest['files'];assert all(sha(p)==h for p,h in state['source_hashes'].items())
r=dict(utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),manifest_sha256=manifest['manifest_sha256'],files_verified=checked,protected_files_verified=len(state['source_hashes']),source_freeze_preserved=True)
(out/'delivery-check.json').write_text(json.dumps(r,indent=2)+'\n')
with gzip.open(out/'delivery-check.json.gz','wt') as f:json.dump(r,f)
print(json.dumps(r),flush=True)
