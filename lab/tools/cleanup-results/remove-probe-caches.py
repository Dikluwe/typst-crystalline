#!/usr/bin/env python3
"""Delete old generated probe targets; keep product CLI, linter and all oracles."""
from pathlib import Path
import os,json,gzip,hashlib,subprocess,datetime,shutil
ROOT=Path(__file__).resolve().parents[3];OUT=ROOT/'lab/results/storage-probe-caches-2026-11-14';OUT.mkdir(exist_ok=False)
def sha(p):
 with open(p,'rb') as f:return hashlib.file_digest(f,'sha256').hexdigest()
def utc():return datetime.datetime.now(datetime.timezone.utc).isoformat()
tracked=set(subprocess.check_output(['git','ls-files','-z'],cwd=ROOT).decode().split('\0'));new=set(subprocess.check_output(['git','ls-files','--others','--exclude-standard','-z'],cwd=ROOT).decode().split('\0'));dirty={e[3:] for e in subprocess.check_output(['git','status','--porcelain=v1','-z'],cwd=ROOT).decode().split('\0') if e};known=tracked|new
protected={p:sha(ROOT/p) for p in known if p and (ROOT/p).is_file()}
before={'utc':utc(),'head':subprocess.check_output(['git','rev-parse','HEAD'],cwd=ROOT,text=True).strip(),'diffstat':subprocess.check_output(['git','diff','HEAD','--stat'],cwd=ROOT,text=True),'protected_sha256':protected,'du_bytes':int(subprocess.check_output(['du','-sx','-B1',str(ROOT)],text=True).split()[0])}
with gzip.open(OUT/'before.json.gz','wt') as f:json.dump(before,f)
active={'box-launch-bias-2026-11-14','box-launch-control-2026-11-14','box-lifecycle-2026-11-14','vanilla-resync-2026-10-26'};targets=[]
for parent,dirs,files in os.walk(ROOT/'lab/results'):
 parts=Path(parent).parts
 if any(p in active or p.startswith('storage-') for p in parts):dirs[:]=[];continue
 if Path(parent).name in {'target','__pycache__','node_modules'}:targets.append(Path(parent));dirs[:]=[]
probes=list((ROOT/'lab/target/product/release').glob('*-probe'));rows=[]
for target in targets+probes:
 paths=list(target.rglob('*')) if target.is_dir() else [target]
 for p in paths:
  if p.is_symlink() or not p.is_file():continue
  rel=str(p.relative_to(ROOT))
  if rel in known or rel in dirty:raise ValueError('protected file in generated cache: '+rel)
  rows.append({'path':rel,'sha256':sha(p),'bytes':p.stat().st_size})
with gzip.open(OUT/'removed-index.jsonl.gz','wt') as f:
 for row in rows:f.write(json.dumps(row)+'\n')
for target in targets+probes:
 if target.is_dir():shutil.rmtree(target)
 else:target.unlink()
for p,h in protected.items():assert sha(ROOT/p)==h,p
assert subprocess.check_output(['git','diff','HEAD','--stat'],cwd=ROOT,text=True)==before['diffstat']
after={'utc':utc(),'head':before['head'],'du_bytes':int(subprocess.check_output(['du','-sx','-B1',str(ROOT)],text=True).split()[0]),'removed_roots':[str(p.relative_to(ROOT)) for p in targets+probes],'removed_files':len(rows),'protected_files_verified':True};after['reclaimed_gib']=(before['du_bytes']-after['du_bytes'])/1024**3;(OUT/'receipt.json').write_text(json.dumps(after,indent=2));print(json.dumps(after),flush=True)
