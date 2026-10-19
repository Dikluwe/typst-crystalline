"""Materialize versioned textual evidence before replaying historical scripts."""
from pathlib import Path
import argparse,json,gzip,hashlib,base64,os
root=Path(__file__).resolve().parents[3]
a=argparse.ArgumentParser();a.add_argument('package');a.add_argument('--restore',action='store_true');args=a.parse_args()
path=Path(args.package).resolve();assert path.is_relative_to(root)
d=json.loads(gzip.decompress(path.read_bytes()));assert d['format']=='base64-files-v1'
for entry in d['files']:
 dest=root/entry['path'];assert dest.resolve().is_relative_to(root) and dest.is_relative_to(root/d['scope'])
 for q in [dest,*dest.parents]:
  if q==root:break
  assert not q.is_symlink(),q
 data=base64.b64decode(entry['base64'],validate=True);assert len(data)==entry['bytes'] and hashlib.sha256(data).hexdigest()==entry['sha256']
 if not args.restore:continue
 if dest.exists():
  assert hashlib.sha256(dest.read_bytes()).hexdigest()==entry['sha256'],('different file; refusing overwrite',entry['path'])
 else:
  dest.parent.mkdir(parents=True,exist_ok=True)
  with dest.open('xb') as f:f.write(data)
  os.chmod(dest,entry['mode'])
print(('Restored/verified' if args.restore else 'Verified'),len(d['files']),'evidence files')
