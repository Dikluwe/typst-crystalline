from pathlib import Path
import gzip,json
from PIL import Image,ImageChops
out=Path('00_nucleo/diagnosticos/raster-mascaras-2026-10-14')
def read(name):
 with gzip.open(out/name/'results.json.gz','rt') as f:return json.load(f)
def pixels(path):
 im=Image.open(path).convert('RGBA');bg=Image.new('RGBA',im.size,'white');bg.alpha_composite(im);return bg.convert('RGB')
def diff(a,b):
 assert a.size==b.size
 v=list(ImageChops.difference(a,b).getdata());return dict(max=max(max(p) for p in v),pixels_gt16=sum(max(p)>16 for p in v))
old={}
for folder in ['before-explicit-ppi','before-fill-rule','before-fill-rule-stroke']:
 r=read(folder)
 for row in r['rows']:old[(row['case'],row['binary'],row['format'])]=(folder,row)
a=read('after-final');current={(r['case'],r['binary'],r['format']):r for r in a['rows']};results=[];errors=[]
for row in a['rows']:
 if row['binary']!='tekt' or row['format']!='png':continue
 case=row['case'];base=old.get((case,'tekt','png'))
 if row['exit']:
  errors.append(dict(case=case,exit=row['exit'],stderr=row['stderr']));continue
 im=pixels(out/'after-final'/(case+'-tekt.png'));svg=pixels(out/'after-final'/(case+'-tekt-svg.png'));pin=pixels(out/'after-final'/(case+'-vanilla.png'))
 oldim=pixels(out/base[0]/(case+'-tekt.png')) if base and base[1]['exit']==0 else None
 result=dict(case=case,before_red=base[1].get('red_bbox') if base else None,after_red=row['red_bbox'],before_pixels=base[1].get('red_pixels') if base else None,after_pixels=row['red_pixels'],pin_red=current[(case,'vanilla','png')]['red_bbox'],pin_pixels=current[(case,'vanilla','png')]['red_pixels'],after_svg=diff(im,svg),after_pin=diff(im,pin),before_after=diff(oldim,im) if oldim else None,seconds=row['seconds'])
 results.append(result);print(result)
(out/'comparison.json').write_text(json.dumps(dict(after_start=a['start'],rows=results,errors=errors),indent=2))
