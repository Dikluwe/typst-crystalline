from pathlib import Path
from PIL import Image
import subprocess,gzip,json,datetime,hashlib,sys
out=Path('00_nucleo/diagnosticos/tiling-transformacao-2026-10-14')/sys.argv[1]
r=dict(utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),renders=[],comparisons=[])
for p in sorted(out.glob('*.svg')):
 png=p.with_name(p.stem+'-svg.png');cmd=['rsvg-convert','-w','240','-h','192','-b','white','-o',str(png),str(p)];x=subprocess.run(cmd,capture_output=True,text=True);assert x.returncode==0
 im=Image.open(png).convert('RGB');r['renders'].append(dict(command=cmd,exit=x.returncode,path=str(p),sha256=hashlib.sha256(p.read_bytes()).hexdigest(),painted=sum(max(255-v for v in px)>32 for px in im.getdata())))
for p in sorted(out.glob('*-tekt.svg')):
 name=p.stem.removesuffix('-tekt');pairs=[('bilateral-svg',name+'-tekt-svg',name+'-vanilla-svg'),('tekt-targets',name+'-tekt',name+'-tekt-svg'),('vanilla-targets',name+'-vanilla',name+'-vanilla-svg'),('bilateral-pdf',name+'-tekt',name+'-vanilla')]
 for kind,a,b in pairs:
  a=Image.open(out/(a+'.png')).convert('RGB');b=Image.open(out/(b+'.png')).convert('RGB');assert a.size==b.size
  d=[max(abs(x-y) for x,y in zip(px,py)) for px,py in zip(a.getdata(),b.getdata())];r['comparisons'].append(dict(name=name,kind=kind,above16=sum(x>16 for x in d),different=sum(x>0 for x in d),max=max(d)))
with gzip.open(out/'render-comparison.json.gz','wt') as f:json.dump(r,f)
for c in r['comparisons']:print(c)
