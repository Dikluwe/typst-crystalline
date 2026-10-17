from pathlib import Path
from PIL import Image
import json,gzip,subprocess,hashlib,datetime
out=Path('00_nucleo/diagnosticos/tiling-cobertura-2026-10-14');results=[]
for p in sorted((out/'after').glob('*-tekt.png')):
 v=p.with_name(p.name.replace('-tekt','-vanilla'));a=Image.open(p).convert('RGB');b=Image.open(v).convert('RGB');assert a.size==b.size
 diff=[max(abs(x-y) for x,y in zip(px,py)) for px,py in zip(a.getdata(),b.getdata())]
 results.append(dict(case=p.stem.removesuffix('-tekt'),different=sum(x>0 for x in diff),above16=sum(x>16 for x in diff),max=max(diff),size=a.size))
checks=[]
for p in sorted(out.rglob('*.pdf')):
 r=subprocess.run(['qpdf','--check',str(p)],capture_output=True,text=True);checks.append(dict(path=str(p),sha256=hashlib.sha256(p.read_bytes()).hexdigest(),exit=r.returncode,stdout=r.stdout,stderr=r.stderr))
a=Image.open(out/'after/large-offset-tekt.png').convert('RGB');b=Image.open(out/'after/offset-control-tekt.png').convert('RGB');assert a.tobytes()==b.tobytes()
report=dict(utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),bilateral=results,qpdf=checks,tekt_large_offset_matches_zero=True)
with gzip.open(out/'render-comparison.json.gz','wt') as f:json.dump(report,f)
print(json.dumps(results,indent=2));print('qpdf checks',len(checks),'failures',sum(c['exit']!=0 for c in checks));assert all(c['exit']==0 for c in checks)
