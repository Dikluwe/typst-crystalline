#!/usr/bin/env python3
"""Lossless archive for ignored historical Typst copies; independent-copy restore."""
from pathlib import Path
import argparse,os,json,gzip,hashlib,subprocess,datetime,tarfile,io,tempfile,shutil
ROOT=Path(__file__).resolve().parents[3];OUT=ROOT/'lab/results/storage-fixtures-2026-11-14'
def sha(p):
 with open(p,'rb') as f:return hashlib.file_digest(f,'sha256').hexdigest()
def utc():return datetime.datetime.now(datetime.timezone.utc).isoformat()
def restore(archive,index,selected):
 with gzip.open(index,'rt') as f:rows={r['path']:r for r in map(json.loads,f)}
 wanted={p for p in rows if not selected or any(p==s or p.startswith(s.rstrip('/')+'/') for s in selected)}
 if selected and not wanted:raise ValueError('no indexed fixture matches selection')
 with tempfile.TemporaryDirectory(prefix='tekt-fixture-restore-') as d:
  cache=Path(d);proc=subprocess.Popen(['zstd','-dc',str(archive)],stdout=subprocess.PIPE)
  try:
   with tarfile.open(fileobj=proc.stdout,mode='r|') as tar:
    for item in tar:
     row=rows[item.name]
     if item.isfile():
      obj=cache/row['sha256']
      with obj.open('wb') as dst:shutil.copyfileobj(tar.extractfile(item),dst)
      assert sha(obj)==row['sha256']
     if item.name not in wanted:continue
     rel=Path(item.name)
     if rel.is_absolute() or '..' in rel.parts:raise ValueError('unsafe archive path')
     dest=ROOT/rel
     if dest.exists():
      if sha(dest)!=row['sha256']:raise ValueError('refusing overwrite '+str(dest))
      continue
     dest.parent.mkdir(parents=True,exist_ok=True)
     if dest.parent.resolve()!=dest.parent:raise ValueError('symlink parent')
     fd,name=tempfile.mkstemp(prefix='.fixture-restore-',dir=dest.parent);os.close(fd);tmp=Path(name)
     try:
      shutil.copyfile(cache/row['sha256'],tmp);assert sha(tmp)==row['sha256']
      tmp.chmod(row['mode']);os.utime(tmp,ns=(row['atime_ns'],row['mtime_ns']));os.replace(tmp,dest)
     finally:tmp.unlink(missing_ok=True)
   assert proc.wait()==0
  finally:
   if proc.poll() is None:proc.kill();proc.wait()
 print('Restored/verified independent fixtures:',len(wanted))
def pack():
 OUT.mkdir(exist_ok=False)
 tracked=set(subprocess.check_output(['git','ls-files','-z'],cwd=ROOT).decode().split('\0'));new=set(subprocess.check_output(['git','ls-files','--others','--exclude-standard','-z'],cwd=ROOT).decode().split('\0'));dirty={e[3:] for e in subprocess.check_output(['git','status','--porcelain=v1','-z'],cwd=ROOT).decode().split('\0') if e}
 known=tracked|new
 protected={p:sha(ROOT/p) for p in known if p and (ROOT/p).is_file()}
 before={'utc':utc(),'head':subprocess.check_output(['git','rev-parse','HEAD'],cwd=ROOT,text=True).strip(),'diffstat':subprocess.check_output(['git','diff','HEAD','--stat'],cwd=ROOT,text=True),'protected_sha256':protected,'du_bytes':int(subprocess.check_output(['du','-sx','-B1',str(ROOT)],text=True).split()[0])}
 with gzip.open(OUT/'before.json.gz','wt') as f:json.dump(before,f)
 active={'box-launch-bias-2026-11-14','box-launch-control-2026-11-14','box-lifecycle-2026-11-14','vanilla-resync-2026-10-26'}
 rows=[]
 for parent,dirs,files in os.walk(ROOT/'lab/results',followlinks=False):
  parts=Path(parent).relative_to(ROOT/'lab/results').parts
  if any(p in active or p.startswith('storage-') for p in parts):dirs[:]=[];continue
  for name in files:
   if not name.endswith('.typ'):continue
   p=Path(parent)/name;rel=str(p.relative_to(ROOT))
   if p.is_symlink() or rel in known or rel in dirty:continue
   st=p.stat();rows.append({'path':rel,'sha256':sha(p),'bytes':st.st_size,'mode':st.st_mode&0o777,'atime_ns':st.st_atime_ns,'mtime_ns':st.st_mtime_ns})
 with gzip.open(OUT/'index.jsonl.gz','wt') as f:
  for row in rows:f.write(json.dumps(row)+'\n')
 print('PLAN',len(rows),sum(r['bytes'] for r in rows),flush=True)
 archive=OUT/'fixtures.tar.zst';seen={};tmp=OUT/'fixtures.partial'
 with tmp.open('xb') as raw:
  proc=subprocess.Popen(['zstd','-q','-3','-T1'],stdin=subprocess.PIPE,stdout=raw)
  with tarfile.open(fileobj=proc.stdin,mode='w|') as tar:
   for row in rows:
    p=ROOT/row['path'];data=p.read_bytes();assert hashlib.sha256(data).hexdigest()==row['sha256']
    item=tarfile.TarInfo(row['path']);item.mode=row['mode'];item.mtime=row['mtime_ns']/1e9;key=(row['sha256'],row['mode'])
    if key in seen:
     item.type=tarfile.LNKTYPE;item.linkname=seen[key];tar.addfile(item)
    else:seen[key]=row['path'];item.size=len(data);tar.addfile(item,io.BytesIO(data))
  proc.stdin.close();assert proc.wait()==0
 expected={r['path']:r['sha256'] for r in rows};verified={};proc=subprocess.Popen(['zstd','-dc',str(tmp)],stdout=subprocess.PIPE)
 with tarfile.open(fileobj=proc.stdout,mode='r|') as tar:
  for item in tar:
   verified[item.name]=verified[item.linkname] if item.islnk() else hashlib.file_digest(tar.extractfile(item),'sha256').hexdigest()
 assert proc.wait()==0;assert verified==expected;os.replace(tmp,archive)
 print('ARCHIVE VERIFIED',len(rows),'unique payloads',len(seen),flush=True)
 with (OUT/'removed.jsonl').open('x') as log:
  for i,row in enumerate(rows):
   p=ROOT/row['path'];assert p.stat().st_mtime_ns==row['mtime_ns'] and sha(p)==row['sha256']
   log.write(json.dumps({'utc':utc(),'path':row['path']})+'\n');log.flush();p.unlink()
   if i%50000==0:print('removed loose copies',i+1,flush=True)
 for p,h in protected.items():assert sha(ROOT/p)==h,p
 assert subprocess.check_output(['git','diff','HEAD','--stat'],cwd=ROOT,text=True)==before['diffstat']
 after={'utc':utc(),'head':before['head'],'du_bytes':int(subprocess.check_output(['du','-sx','-B1',str(ROOT)],text=True).split()[0]),'archived_files':len(rows),'unique_payloads':len(seen),'archive_sha256':sha(archive),'archive_bytes':archive.stat().st_size,'protected_files_verified':True};after['reclaimed_gib']=(before['du_bytes']-after['du_bytes'])/1024**3;(OUT/'receipt.json').write_text(json.dumps(after,indent=2));print(json.dumps(after),flush=True)
if __name__=='__main__':
 p=argparse.ArgumentParser(description=__doc__);p.add_argument('--restore',action='store_true');p.add_argument('paths',nargs='*');args=p.parse_args()
 if args.restore:restore(OUT/'fixtures.tar.zst',OUT/'index.jsonl.gz',args.paths)
 else:pack()
