#!/usr/bin/env python3
"""Pack old gzip result data jointly, retaining exact decoded payloads."""
from pathlib import Path
import os,json,gzip,hashlib,subprocess,datetime,tarfile,io,time
ROOT=Path(__file__).resolve().parents[3];OUT=ROOT/'lab/results/storage-historical-data-2026-11-14'
def sha(p):
 with open(p,'rb') as f:return hashlib.file_digest(f,'sha256').hexdigest()
def utc():return datetime.datetime.now(datetime.timezone.utc).isoformat()
OUT.mkdir(exist_ok=False)
tracked=set(subprocess.check_output(['git','ls-files','-z'],cwd=ROOT).decode().split('\0'));new=set(subprocess.check_output(['git','ls-files','--others','--exclude-standard','-z'],cwd=ROOT).decode().split('\0'));dirty={e[3:] for e in subprocess.check_output(['git','status','--porcelain=v1','-z'],cwd=ROOT).decode().split('\0') if e}
protected={p:sha(ROOT/p) for p in tracked|new if p and (ROOT/p).is_file()}
before={'utc':utc(),'head':subprocess.check_output(['git','rev-parse','HEAD'],cwd=ROOT,text=True).strip(),'diffstat':subprocess.check_output(['git','diff','HEAD','--stat'],cwd=ROOT,text=True),'protected_sha256':protected,'du_bytes':int(subprocess.check_output(['du','-sx','-B1',str(ROOT)],text=True).split()[0])}
with gzip.open(OUT/'before.json.gz','wt') as f:json.dump(before,f)
active={'box-launch-bias-2026-11-14','box-launch-control-2026-11-14','box-lifecycle-2026-11-14','vanilla-resync-2026-10-26'}
paths=[]
for parent,dirs,files in os.walk(ROOT/'lab/results'):
 if any(p in active or p.startswith('storage-') for p in Path(parent).parts):dirs[:]=[];continue
 for n in files:
  if not n.endswith('.gz') or n.endswith('.tar.gz'):continue
  p=Path(parent)/n;rel=str(p.relative_to(ROOT))
  if rel in dirty or rel in new or p.is_symlink():continue
  paths.append(p)
print('PLAN historical gzip files',len(paths),flush=True)
rows=[];seen={};tmp=OUT/'data.partial';archive=OUT/'data.tar.zst'
with tmp.open('xb') as raw:
 proc=subprocess.Popen(['zstd','-q','-6','--long=27','-T1'],stdin=subprocess.PIPE,stdout=raw)
 with tarfile.open(fileobj=proc.stdin,mode='w|') as tar,gzip.open(OUT/'index.jsonl.gz','wt') as log:
  for i,p in enumerate(paths):
   st=p.stat();rawsha=sha(p)
   with gzip.open(p,'rb') as f:data=f.read()
   h=hashlib.sha256(data).hexdigest();rel=str(p.relative_to(ROOT))
   row={'path':rel,'container_sha256':rawsha,'content_sha256':h,'content_bytes':len(data),'container_bytes':st.st_size,'mode':st.st_mode&0o777,'mtime_ns':st.st_mtime_ns,'atime_ns':st.st_atime_ns};rows.append(row);log.write(json.dumps(row)+'\n')
   item=tarfile.TarInfo(rel);item.mode=row['mode'];item.mtime=st.st_mtime_ns/1e9
   if h in seen:item.type=tarfile.LNKTYPE;item.linkname=seen[h];tar.addfile(item)
   else:seen[h]=rel;item.size=len(data);tar.addfile(item,io.BytesIO(data))
   if i%100==0:print('packed gzip files',i+1,flush=True)
 proc.stdin.close();assert proc.wait()==0
expected={r['path']:r['content_sha256'] for r in rows};verified={};proc=subprocess.Popen(['zstd','-dc',str(tmp)],stdout=subprocess.PIPE)
with tarfile.open(fileobj=proc.stdout,mode='r|') as tar:
 for item in tar:verified[item.name]=verified[item.linkname] if item.islnk() else hashlib.file_digest(tar.extractfile(item),'sha256').hexdigest()
assert proc.wait()==0;assert expected==verified;os.replace(tmp,archive)
# Compare actual allocated blocks of distinct original inodes, not sizes by alias.
original_blocks={}
for p in paths:
 st=p.stat();original_blocks[(st.st_dev,st.st_ino)]=st.st_blocks*512
if archive.stat().st_size>=sum(original_blocks.values()):
 (OUT/'skipped.json').write_text(json.dumps({'utc':utc(),'reason':'archive not smaller than original allocated blocks','original_blocks':sum(original_blocks.values()),'archive_bytes':archive.stat().st_size}));archive.unlink();raise SystemExit('No gain; originals retained')
with (OUT/'removed.jsonl').open('x') as log:
 for row in rows:
  p=ROOT/row['path'];assert p.stat().st_mtime_ns==row['mtime_ns'] and sha(p)==row['container_sha256']
  log.write(json.dumps({'utc':utc(),'path':row['path']})+'\n');log.flush();p.unlink()
changed={r['path'] for r in rows}
for p,h in protected.items():
 if p not in changed:assert sha(ROOT/p)==h,p
after={'utc':utc(),'head':before['head'],'du_bytes':int(subprocess.check_output(['du','-sx','-B1',str(ROOT)],text=True).split()[0]),'snapshots':len(rows),'unique_payloads':len(seen),'archive_bytes':archive.stat().st_size,'archive_sha256':sha(archive),'all_decoded_source_contents_verified':True,'protected_files_verified':True};after['reclaimed_gib']=(before['du_bytes']-after['du_bytes'])/1024**3;(OUT/'receipt.json').write_text(json.dumps(after,indent=2));print(json.dumps(after),flush=True)
