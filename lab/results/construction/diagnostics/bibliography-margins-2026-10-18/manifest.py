from pathlib import Path
import json,gzip,hashlib,datetime
r=Path.cwd();p=Path(__file__).resolve().parent
def sha(f):
 with Path(f).open('rb') as s:return hashlib.file_digest(s,'sha256').hexdigest()
proof=json.loads(gzip.decompress((p/'final-state.json.gz').read_bytes()));freeze=json.loads(gzip.decompress((p/'freeze.json.gz').read_bytes()))
assert all(sha(f)==h for f,h in freeze['sources'].items())
assert all(sha(f)==h for f,h in proof['changed_hashes'].items())
assert sha('target/debug/typst')==proof['candidate_binary_sha256']
assert max(int(json.loads(f.read_text())['memory_after']['memory.peak']) for f in p.glob('*-resources.json'))==proof['memory_peak_bytes']
entries=[]
for f in sorted(p.rglob('*')):
 if f.is_file() and f.name not in ['manifest.json','manifest.sha256'] and '__pycache__' not in f.parts:
  entries.append({'path':str(f.relative_to(r)),'sha256':sha(f),'bytes':f.stat().st_size})
for name in proof['changed']+proof['added']:
 f=r/name;entries.append({'path':name,'sha256':sha(f),'bytes':f.stat().st_size})
m={'utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'head':proof['head'],'scope':'A2 relative margins only. Hanging/multiple bibliography and full goal remain open.','files':entries}
(p/'manifest.json').write_text(json.dumps(m,indent=2)+'\n');h=sha(p/'manifest.json');(p/'manifest.sha256').write_text(h+'  manifest.json\n')
assert all(sha(r/x['path'])==x['sha256'] for x in entries)
print(json.dumps({'entries':len(entries),'manifest_sha256':h,'sources_verified':len(freeze['sources']),'goal_complete':False}))
