#!/usr/bin/env python3
"""Deduplicate immutable historical results; never touch active lab or editable source."""
from pathlib import Path
import os,json,gzip,hashlib,subprocess,datetime,collections
ROOT=Path(__file__).resolve().parents[3];OUT=ROOT/'lab/results/storage-dedup-2026-11-14';OUT.mkdir(exist_ok=False)
def sha(p):
 with open(p,'rb') as f:return hashlib.file_digest(f,'sha256').hexdigest()
def utc():return datetime.datetime.now(datetime.timezone.utc).isoformat()
tracked=set(subprocess.check_output(['git','ls-files','-z'],cwd=ROOT).decode().split('\0'));new=set(subprocess.check_output(['git','ls-files','--others','--exclude-standard','-z'],cwd=ROOT).decode().split('\0'));dirty={e[3:] for e in subprocess.check_output(['git','status','--porcelain=v1','-z'],cwd=ROOT).decode().split('\0') if e}
protected={p:sha(ROOT/p) for p in tracked|new if p and (ROOT/p).is_file()}
active={'box-launch-bias-2026-11-14','box-launch-control-2026-11-14','box-lifecycle-2026-11-14','vanilla-resync-2026-10-26'}
before={'utc':utc(),'head':subprocess.check_output(['git','rev-parse','HEAD'],cwd=ROOT,text=True).strip(),'diffstat':subprocess.check_output(['git','diff','HEAD','--stat'],cwd=ROOT,text=True),'protected_sha256':protected,'du_bytes':int(subprocess.check_output(['du','-sx','-B1',str(ROOT)],text=True).split()[0])}
with gzip.open(OUT/'before.json.gz','wt') as f:json.dump(before,f)
# Only result data/container formats, not .typ/.rs/.md/.toml/patches/scripts/assets.
allowed={'.gz','.json','.jsonl','.log','.txt','.xml','.html','.time','.stderr','.stdout','.sarif','.csv'}
seen={};changed=set();count=0;logical=0
with (OUT/'actions.jsonl').open('x') as log:
 for parent,dirs,files in os.walk(ROOT/'lab/results',followlinks=False):
  parts=Path(parent).relative_to(ROOT/'lab/results').parts
  if any(p in active or p.startswith('storage-') for p in parts):dirs[:]=[];continue
  for name in files:
   p=Path(parent)/name;rel=str(p.relative_to(ROOT))
   if p.suffix not in allowed or p.is_symlink() or rel in dirty or rel in new:continue
   st=p.stat()
   if not st.st_size:continue
   raw=sha(p)
   if p.suffix=='.gz':
    try:
     with gzip.open(p,'rb') as f:content=hashlib.file_digest(f,'sha256').hexdigest()
    except (OSError,EOFError):continue
    kind='gzip-decoded'
   else:content=raw;kind='exact-bytes'
   key=(kind,content,st.st_mode&0o777)
   if key not in seen:seen[key]=(p,raw,st.st_ino);continue
   canonical,canon_hash,inode=seen[key]
   if st.st_ino==inode:continue
   assert sha(canonical)==canon_hash
   assert p.stat().st_mtime_ns==st.st_mtime_ns
   tmp=p.with_name(p.name+'.dedup-temp');os.link(canonical,tmp)
   row={'utc':utc(),'path':rel,'canonical':str(canonical.relative_to(ROOT)),'kind':kind,'content_sha256':content,'before_sha256':raw,'after_sha256':canon_hash,'before_bytes':st.st_size,'after_bytes':canonical.stat().st_size,'mtime_ns':st.st_mtime_ns,'mode':st.st_mode&0o777}
   log.write(json.dumps(row)+'\n');log.flush();os.replace(tmp,p)
   changed.add(rel);count+=1;logical+=st.st_size
   if count%1000==0:print('linked',count,flush=True)
for p,h in protected.items():
 if p not in changed:assert sha(ROOT/p)==h,p
# Every changed path now resolves to precisely the verified canonical inode.
for row in map(json.loads,(OUT/'actions.jsonl').read_text().splitlines()):
 a=ROOT/row['path'];b=ROOT/row['canonical'];assert os.path.samefile(a,b);assert sha(a)==row['after_sha256']
after={'utc':utc(),'head':before['head'],'du_bytes':int(subprocess.check_output(['du','-sx','-B1',str(ROOT)],text=True).split()[0]),'deduplicated_paths':count,'logical_duplicate_bytes':logical,'protected_files_verified':True,'tracked_changed':len(changed&tracked)};after['reclaimed_gib']=(before['du_bytes']-after['du_bytes'])/1024**3;(OUT/'receipt.json').write_text(json.dumps(after,indent=2));print(json.dumps(after),flush=True)
