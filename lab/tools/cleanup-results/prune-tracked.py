#!/usr/bin/env python3
"""User-authorized removal of ignored generated outputs; preserve source and active lab."""
from pathlib import Path
import os,json,hashlib,subprocess,datetime,gzip,re
ROOT=Path(__file__).resolve().parents[3];OUT=ROOT/'lab/results/storage-tracked-2026-11-14';OUT.mkdir(exist_ok=False)
def sha(p):
 with open(p,'rb') as f:return hashlib.file_digest(f,'sha256').hexdigest()
def utc():return datetime.datetime.now(datetime.timezone.utc).isoformat()
tracked=set(subprocess.check_output(['git','ls-files','-z'],cwd=ROOT).decode().split('\0'))
# Hash tracked work and all nonignored new work (including untracked directories).
protect=set(tracked)|set(subprocess.check_output(['git','ls-files','--others','--exclude-standard','-z'],cwd=ROOT).decode().split('\0'))
protected={p:sha(ROOT/p) for p in protect if p and (ROOT/p).is_file()}
changed=set()
for entry in subprocess.check_output(['git','status','--porcelain=v1','-z'],cwd=ROOT).decode().split('\0'):
 if entry:changed.add(entry[3:])
# Dirty tracked paths and nonignored new work never enter the deletion plan.
new_work=protect-tracked
active={'box-launch-bias-2026-11-14','box-launch-control-2026-11-14','box-lifecycle-2026-11-14','storage-2026-11-14','storage-tracked-2026-11-14','vanilla-resync-2026-10-26'}
# Source/assets in snapshots are protected even if ignored; never prune these subtrees.
source_dirs={'candidate','source','src','fixtures','inputs','input','assets','fonts','00_nucleo','01_core','02_shell','03_infra','04_wiring','history','crates','tests'}
before={'utc':utc(),'head':subprocess.check_output(['git','rev-parse','HEAD'],cwd=ROOT,text=True).strip(),'diffstat':subprocess.check_output(['git','diff','HEAD','--stat'],cwd=ROOT,text=True),'protected_sha256':protected,'du_bytes':int(subprocess.check_output(['du','-sx','-B1',str(ROOT)],text=True).split()[0])};(OUT/'before.json').write_text(json.dumps(before,indent=2))
candidates=[];counts={};approved=set()
for parent,dirs,files in os.walk(ROOT/'lab/results',followlinks=False):
 parts=Path(parent).relative_to(ROOT/'lab/results').parts
 if any(p in active or p in source_dirs for p in parts):dirs[:]=[];continue
 for name in files:
  p=Path(parent)/name;rel=p.relative_to(ROOT).as_posix()
  if rel in changed or rel in new_work or p.is_symlink():continue
  ext=p.suffix.lower();action=None
  if ext in {'.pdf','.png','.ppm','.pgm','.svg','.xml','.html'}:action='generated-render-or-extraction'
  elif ext in {'.json','.jsonl','.log','.txt','.sarif'} and p.stat().st_size>64*1024 and not Path(str(p)+'.gz').exists():action='gzip-raw-data'
  elif ext in {'','.so'} and p.stat().st_size>1024*1024:
   with p.open('rb') as f:
    if f.read(4)==b'\x7fELF':action='superseded-executable'
  if action:
   approved.add(rel)
   st=p.stat();candidates.append((p,action,st));counts[action]=counts.get(action,0)+st.st_size
(OUT/'plan.json').write_text(json.dumps({'utc':utc(),'files':len(candidates),'logical_bytes_by_action':counts},indent=2));print('PLAN',len(candidates),counts,flush=True)
with (OUT/'actions.jsonl').open('x') as log:
 for i,(p,action,st) in enumerate(candidates):
  h=sha(p);row={'utc':utc(),'path':str(p.relative_to(ROOT)),'sha256':h,'bytes':st.st_size,'action':action}
  if action=='gzip-raw-data':
   dest=Path(str(p)+'.gz');tmp=Path(str(dest)+'.partial')
   with p.open('rb') as src,gzip.open(tmp,'wb',compresslevel=3) as dst:
    import shutil;shutil.copyfileobj(src,dst)
   with gzip.open(tmp,'rb') as stream:assert hashlib.file_digest(stream,'sha256').hexdigest()==h
   os.replace(tmp,dest);row.update(compressed_path=str(dest.relative_to(ROOT)),compressed_bytes=dest.stat().st_size)
  assert p.stat().st_mtime_ns==st.st_mtime_ns,p
  log.write(json.dumps(row)+'\n');log.flush();p.unlink()
  if i%10000==0:print('processed',i+1,flush=True)
for p,h in protected.items():
 if p not in approved:assert sha(ROOT/p)==h,p
# Tracked removals are intentional; all other tracked bytes have been verified.
after={'utc':utc(),'head':before['head'],'du_bytes':int(subprocess.check_output(['du','-sx','-B1',str(ROOT)],text=True).split()[0]),'files':len(candidates),'protected_files_verified':len(protected)-len(approved.intersection(protected)), 'tracked_removed':len(approved.intersection(tracked)),'active_directories':sorted(active),'logical_bytes_by_action':counts};after['reclaimed_gib']=(before['du_bytes']-after['du_bytes'])/1024**3;(OUT/'receipt.json').write_text(json.dumps(after,indent=2));print(json.dumps(after),flush=True)
