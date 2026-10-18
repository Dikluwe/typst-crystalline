from pathlib import Path
import json,gzip,hashlib,datetime
r=Path.cwd();p=Path(__file__).resolve().parent
def sha(f):
 with Path(f).open('rb') as s:return hashlib.file_digest(s,'sha256').hexdigest()
proof=json.loads(gzip.decompress((p/'final-state.json.gz').read_bytes()));freeze=json.loads(gzip.decompress((p/'freeze.json.gz').read_bytes()))
assert all(sha(f)==h for f,h in freeze['sources'].items())
assert sha('target/debug/typst')==proof['candidate_binary_sha256']
entries=[]
for f in sorted(p.rglob('*')):
 if not f.is_file() or f.name in ['manifest.json','manifest.sha256'] or any(part in ['outputs','outputs-candidate1','bin','__pycache__'] for part in f.relative_to(p).parts):continue
 entries.append(dict(path=str(f.relative_to(r)),sha256=sha(f),bytes=f.stat().st_size))
for name in proof['changed']:
 f=r/name;entries.append(dict(path=name,sha256=sha(f),bytes=f.stat().st_size))
manifest=dict(utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),head=proof['head'],scope='Mixed face metrics and certified text admission. The entire correction goal remains open; remaining parity cases and global V16 are listed in proximos.md.',raw_artifacts='Archived in artifacts-final.tar.gz and artifacts-candidate1.tar.gz. Preserved local binaries are identified by SHA in measurement records; bin/ is not bundled in this manifest.',files=entries)
(p/'manifest.json').write_text(json.dumps(manifest,indent=2)+'\n');h=sha(p/'manifest.json');(p/'manifest.sha256').write_text(h+'  manifest.json\n')
assert all(sha(r/e['path'])==e['sha256'] for e in entries)
print(json.dumps(dict(entries=len(entries),manifest_sha256=h,sources_verified=len(freeze['sources']),goal_complete=False)))
