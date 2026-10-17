from pathlib import Path
import re,json,gzip,hashlib,datetime,subprocess
out=Path('00_nucleo/diagnosticos/tiling-carrier-2026-10-14');sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest();rows=[]
for top in ['01_core','02_shell','03_infra','04_wiring','benches']:
 for p in sorted(Path(top).rglob('*.rs')):
  text=p.read_text();matches=[dict(line=i,text=line.strip()) for i,line in enumerate(text.splitlines(),1) if re.search(r'FrameItem::(?:Shape|Group)\b',line)]
  if not matches:continue
  owner=re.search(r'^//! @prompt (.+)$',text,re.M)
  rows.append(dict(path=str(p),sha256=sha(p),owner=owner.group(1) if owner else None,owner_sha256=sha(owner.group(1)) if owner else None,matches=matches))
r=dict(utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),head=subprocess.check_output(['git','rev-parse','HEAD'],text=True),diffstat=subprocess.check_output(['git','diff','HEAD','--stat'],text=True),note='Lexical inventory including tests, constructors and patterns; not a count of required edits or production failures.',rows=rows)
with gzip.open(out/'frame-consumers.json.gz','wt') as f:json.dump(r,f)
print('Inventory saved; no consumer source modified')
