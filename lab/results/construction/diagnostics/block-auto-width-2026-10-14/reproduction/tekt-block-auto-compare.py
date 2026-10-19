from pathlib import Path
import gzip,json,re,datetime,hashlib,xml.etree.ElementTree as ET
from PIL import Image,ImageChops
out=Path('00_nucleo/diagnosticos/block-auto-width-2026-10-14');rows=[]
def geometry(path):
 root=ET.fromstring(path.read_text())
 for e in root.iter():
  if e.attrib.get('fill')=='#0074d9':
   if 'width' in e.attrib:return float(e.attrib['width']),float(e.attrib['height'])
   m=re.search(r'v ([-.\d]+)h ([-.\d]+)',e.attrib.get('d',''))
   if m:return float(m[2]),float(m[1])
 return None
def diff(a,b):
 x=Image.open(a).convert('RGB');y=Image.open(b).convert('RGB');assert x.size==y.size
 d=ImageChops.difference(x,y);return dict(pixels_gt16=sum(max(p)>16 for p in d.getdata()),max=max(v[1] for v in d.getextrema()))
for f in sorted((out/'after-first').glob('*-tekt.svg')):
 name=f.name.removesuffix('-tekt.svg');v=f.with_name(name+'-vanilla.svg');before=out/'before'/f.name
 rows.append(dict(case=name,before=geometry(before) if before.exists() else None,after=geometry(f),vanilla=geometry(v),pixels=diff(f.with_suffix('.png'),v.with_suffix('.png'))))
parent=[];base=Path('00_nucleo/diagnosticos/paint-root-2026-10-14/after-first');after=out/'parent-after-first'
for f in sorted(after.glob('*-tekt*.png')):
 v=f.with_name(f.name.replace('-tekt','-vanilla'));b=base/f.name
 parent.append(dict(case=f.name,before_after=diff(b,f),after_pin=diff(f,v),before_pin=diff(b,base/v.name)))
r=dict(utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),geometry=rows,parent=parent,source_evidence=['before/results.json.gz','after-first/results.json.gz','parent-after-first/results.json.gz','../paint-root-2026-10-14/after-first/results.json.gz'])
(out/'comparison.json').write_text(json.dumps(r,indent=2))
for x in rows:
 if '-clip' not in x['case']:print(x['case'],x['before'],x['after'],x['vanilla'])
print('Changed parent renders:',[p for p in parent if p['before_after']['max']])
