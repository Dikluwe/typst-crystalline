#!/usr/bin/env python3
"""Delete historical ignored ELF outputs explicitly authorized by the user."""
from pathlib import Path
import datetime,hashlib,json,os,re,subprocess,sys
ROOT=Path(__file__).resolve().parents[3]
OUT=ROOT/'lab/results/storage-2026-11-14'
STORE=OUT/'objects'
def sha(p):
 with p.open('rb') as f:return hashlib.file_digest(f,'sha256').hexdigest()
def utc():return datetime.datetime.now(datetime.timezone.utc).isoformat()
OUT.mkdir(exist_ok=True);STORE.mkdir(exist_ok=True)
tracked=set(subprocess.check_output(['git','ls-files','-z'],cwd=ROOT).decode().split('\0'))
status=subprocess.check_output(['git','status','--porcelain=v1','-z'],cwd=ROOT)
protected={}
for entry in status.decode().split('\0'):
 if entry and (ROOT/entry[3:]).is_file():protected[entry[3:]]=sha(ROOT/entry[3:])
before={'utc':utc(),'head':subprocess.check_output(['git','rev-parse','HEAD'],cwd=ROOT,text=True).strip(),'status':status.decode(),'diffstat':subprocess.check_output(['git','diff','HEAD','--stat'],cwd=ROOT,text=True),'protected_sha256':protected,'du_bytes':int(subprocess.check_output(['du','-sx','-B1',str(ROOT)],text=True).split()[0])}
(OUT/'before.json').write_text(json.dumps(before,indent=2))
candidates=[]
for parent,dirs,files in os.walk(ROOT/'lab/results',followlinks=False):
 if Path(parent)==OUT:
  dirs[:]=[];continue
 for name in files:
  # Historical compiler/probe ELFs have no extension; reject generated fixtures
  # by name before doing expensive metadata lookups across the result corpus.
  if '.' in name and not name.endswith('.so'):continue
  p=Path(parent)/name;rel=p.relative_to(ROOT).as_posix()
  if p.is_symlink() or rel in tracked:continue
  dates=re.findall(r'2026-\d\d-\d\d',str(p.parent.relative_to(ROOT/'lab/results')))
  if not dates or max(dates)>='2026-11-12':continue
  st=p.stat()
  if st.st_size<1024*1024:continue
  with p.open('rb') as f:magic=f.read(4)
  if magic==b'\x7fELF':candidates.append((p,st))
(OUT/'plan.json').write_text(json.dumps({'utc':utc(),'files':len(candidates),'logical_bytes':sum(st.st_size for p,st in candidates),'paths':[str(p.relative_to(ROOT)) for p,st in candidates]},indent=2))
print('PLAN',len(candidates),sum(st.st_size for p,st in candidates),flush=True)
with (OUT/'removed.jsonl').open('x') as index:
 for i,(p,st) in enumerate(candidates):
  h=sha(p)
  # Reverify source and metadata immediately before removal.
  now=p.stat()
  if (now.st_ino,now.st_size,now.st_mtime_ns)!=(st.st_ino,st.st_size,st.st_mtime_ns) or sha(p)!=h:raise RuntimeError('source changed '+str(p))
  row={'utc':utc(),'path':str(p.relative_to(ROOT)),'action':'deleted-historical-ELF','sha256':h,'bytes':st.st_size,'mode':st.st_mode&0o777,'atime_ns':st.st_atime_ns,'mtime_ns':st.st_mtime_ns}
  index.write(json.dumps(row)+'\n');index.flush();p.unlink()
  if i%20==0:print('archived',i+1,'/',len(candidates),flush=True)
for p,h in protected.items():assert sha(ROOT/p)==h,p
assert subprocess.check_output(['git','diff','HEAD','--stat'],cwd=ROOT,text=True)==before['diffstat']
after={'utc':utc(),'head':before['head'],'du_bytes':int(subprocess.check_output(['du','-sx','-B1',str(ROOT)],text=True).split()[0]),'archived_files':len(candidates),'objects':len(list(STORE.glob('*.zst'))),'protected_files_unchanged':True}
after['reclaimed_bytes']=before['du_bytes']-after['du_bytes'];(OUT/'receipt.json').write_text(json.dumps(after,indent=2));print(json.dumps(after),flush=True)
