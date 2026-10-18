from pathlib import Path
import json,gzip,hashlib,datetime
r=Path.cwd();p=Path(__file__).resolve().parent
def sha(f):
 h=hashlib.sha256()
 with f.open('rb') as stream:
  for chunk in iter(lambda:stream.read(1024*1024),b''):h.update(chunk)
 return h.hexdigest()
proof=json.loads(gzip.decompress((p/'final-state.json.gz').read_bytes()))
assert all(sha(Path(f))==h for f,h in proof['changed_hashes'].items())
freeze=json.loads(gzip.decompress((p/'complete-freeze.json.gz').read_bytes()));assert all(sha(Path(f))==h for f,h in freeze['sources'].items())
assert sha(r/'target/debug/typst')==proof['candidate_binary_sha256']
assert max(int(json.loads(f.read_text())['memory_after']['memory.peak']) for f in p.glob('*-resources.json'))==proof['memory_peak_bytes']
entries=[]
for f in sorted(p.rglob('*')):
 if f.is_file() and f.name not in ('manifest.json','manifest.sha256') and '__pycache__' not in f.parts:
  entries.append({'path':str(f.relative_to(r)),'sha256':sha(f),'bytes':f.stat().st_size})
for rel in proof['changed_from_before'] + proof['added']:
 f=r/rel;entries.append({'path':rel,'sha256':sha(f),'bytes':f.stat().st_size})
manifest={'utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'scope':'A2 block boundaries and type validation complete; indentation/prefix/entry structure/multi and full objective remain open','head':proof['head'],'files':entries}
(p/'manifest.json').write_text(json.dumps(manifest,ensure_ascii=False,indent=2)+'\n');digest=sha(p/'manifest.json');(p/'manifest.sha256').write_text(digest+'  manifest.json\n')
assert all(sha(r/x['path'])==x['sha256'] for x in entries)
print(json.dumps({'entries':len(entries),'manifest_sha256':digest,'sources_verified':len(freeze['sources']),'scope_complete':'A2 boundaries/types only','goal_complete':False}))
