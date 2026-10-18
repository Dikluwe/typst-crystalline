from pathlib import Path
import json,gzip,hashlib,base64,datetime,subprocess
p=Path(__file__).resolve().parent;r=p.parents[2]
sha=lambda b:hashlib.sha256(b).hexdigest()
files=[]
for f in sorted(p.rglob('*')):
 if f.is_file() and (f.suffix in {'.log','.json','.typ','.html'} or f.name.endswith('.patch.gz')) and not f.name.startswith('package-run'):
  b=f.read_bytes();files.append(dict(path=str(f.relative_to(r)),sha256=sha(b),bytes=len(b),mode=f.stat().st_mode & 0o777,base64=base64.b64encode(b).decode()))
payload=dict(format='base64-files-v1',scope=str(p.relative_to(r)),utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),head=subprocess.check_output(['git','rev-parse','HEAD'],text=True).strip(),diffstat=subprocess.check_output(['git','diff','HEAD','--stat'],text=True),files=files)
# The existing restore-evidence helper uses a root files array with path/data/hash.
f=p/'evidencias-commit.json.gz';f.write_bytes(gzip.compress(json.dumps(payload).encode()))
check=json.loads(gzip.decompress(f.read_bytes()))
for item in check['files']:
 b=base64.b64decode(item['base64']);assert len(b)==item['bytes'] and sha(b)==item['sha256'];assert (r/item['path']).read_bytes()==b
manifest={str(f.relative_to(r)):sha(f.read_bytes()) for f in sorted(p.iterdir()) if f.is_file() and f.name!='manifest.json.gz' and not f.name.startswith('package-run')}
review=r/'00_nucleo/materialization/context-occurrences-2026-10-18/review.md';manifest[str(review.relative_to(r))]=sha(review.read_bytes())
receipt=review.parent/'approval-receipt.json.gz';manifest[str(receipt.relative_to(r))]=sha(receipt.read_bytes())
(p/'manifest.json.gz').write_bytes(gzip.compress(json.dumps(dict(utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),files=manifest)).encode()))
print(json.dumps(dict(files=len(files),package_bytes=(p/'evidencias-commit.json.gz').stat().st_size,manifest_files=len(manifest))))
