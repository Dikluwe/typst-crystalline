#!/usr/bin/env python3
"""Restore selected historical JSON gzip snapshots as independent files."""
from pathlib import Path
import argparse,gzip,json,tarfile,subprocess,tempfile,hashlib,os,shutil
ROOT=Path(__file__).resolve().parents[3];OUT=ROOT/'lab/results/storage-snapshots-2026-11-14'
p=argparse.ArgumentParser(description=__doc__);p.add_argument('paths',nargs='+',help='exact repository-relative snapshot paths, or directory prefixes');args=p.parse_args()
from payload_store import available,restore as shared_restore
if available():
 shared_restore("snapshots",args.paths)
 raise SystemExit(0)
with gzip.open(OUT/'index.jsonl.gz','rt') as f:rows={r['path']:r for r in map(json.loads,f)}
wanted={k:v for k,v in rows.items() if any(k==s or k.startswith(s.rstrip('/')+'/') for s in args.paths)}
if not wanted:raise SystemExit('No snapshot matches selection')
needed={v['content_sha256'] for v in wanted.values()}
with tempfile.TemporaryDirectory(prefix='tekt-restore-snapshots-') as d:
 cache=Path(d);proc=subprocess.Popen(['zstd','-dc',str(OUT/'snapshots.tar.zst')],stdout=subprocess.PIPE)
 try:
  with tarfile.open(fileobj=proc.stdout,mode='r|') as tar:
   for item in tar:
    row=rows[item.name];h=row['content_sha256']
    if item.isfile() and h in needed:
     obj=cache/h
     with obj.open('wb') as f:shutil.copyfileobj(tar.extractfile(item),f)
     with obj.open('rb') as f:assert hashlib.file_digest(f,'sha256').hexdigest()==h
    if item.name not in wanted:continue
    rel=Path(item.name)
    if rel.is_absolute() or '..' in rel.parts:raise ValueError('unsafe archive path')
    dest=ROOT/rel
    if dest.exists():
     with gzip.open(dest,'rb') as f:assert hashlib.file_digest(f,'sha256').hexdigest()==h,'refusing overwrite'
     continue
    dest.parent.mkdir(parents=True,exist_ok=True)
    if dest.parent.resolve()!=dest.parent:raise ValueError('symlink parent')
    fd,name=tempfile.mkstemp(prefix='.restore-snapshot-',dir=dest.parent);os.close(fd);tmp=Path(name)
    try:
     with (cache/h).open('rb') as src,gzip.open(tmp,'wb',compresslevel=6) as out:shutil.copyfileobj(src,out)
     with gzip.open(tmp,'rb') as f:assert hashlib.file_digest(f,'sha256').hexdigest()==h
     tmp.chmod(row['mode']);os.utime(tmp,ns=(row['atime_ns'],row['mtime_ns']));os.replace(tmp,dest)
    finally:tmp.unlink(missing_ok=True)
  assert proc.wait()==0
 finally:
  if proc.poll() is None:proc.kill();proc.wait()
print('Restored/verified independent snapshots:',len(wanted))
