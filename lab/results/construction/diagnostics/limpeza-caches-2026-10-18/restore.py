"""Restore one exact retained artifact; never overwrite a different file."""
from pathlib import Path
import argparse,gzip,json,hashlib,subprocess,os
p=Path(__file__).resolve().parent;root=p.parents[2]
def sha(path):
 h=hashlib.sha256()
 with path.open('rb') as fp:
  for b in iter(lambda:fp.read(1024*1024),b''):h.update(b)
 return h.hexdigest()
a=argparse.ArgumentParser();a.add_argument('--list',action='store_true');a.add_argument('--original');args=a.parse_args()
r=json.loads(gzip.decompress((p/'retencao.json.gz').read_bytes()))
entries={row['original']:row for row in r['restore_mapping']}
if args.list or not args.original:
 for path in sorted(entries):print(path)
 raise SystemExit(0)
if args.original not in entries:raise SystemExit('Path is not in the retention manifest')
row=entries[args.original];dest=root/args.original
if dest.is_absolute() and not dest.resolve().is_relative_to(root.resolve()):raise SystemExit('Path escapes repository')
for parent in [dest,*dest.parents]:
 if parent==root:break
 if parent.is_symlink():raise SystemExit('Symlink path is not accepted')
if dest.exists():
 if sha(dest)==row['sha256']:print('Already present with the recorded SHA-256');raise SystemExit(0)
 raise SystemExit('Different file already exists; refusing overwrite')
archive=root/r['path']
if sha(archive)!=r['sha256']:raise SystemExit('Archive SHA-256 mismatch')
dest.parent.mkdir(parents=True,exist_ok=True)
temp=dest.with_name(dest.name+'.restore-partial')
with temp.open('xb') as output:
 subprocess.run(['tar','--zstd','-xOf',str(archive),'objetos/'+row['sha256']],stdout=output,check=True)
if sha(temp)!=row['sha256']:raise SystemExit('Restored object SHA-256 mismatch')
os.chmod(temp,row['mode']);temp.rename(dest);print('Restored:',dest)
