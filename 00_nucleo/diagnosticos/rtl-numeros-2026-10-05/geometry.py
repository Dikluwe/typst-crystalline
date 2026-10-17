from pathlib import Path
import json,gzip,re
H=Path(__file__).resolve().parent;d=json.load(gzip.open(H/'verification.json.gz','rt'));out={}
for name,c in d['cases'].items():
 a=c['engines']['candidate']['words'];v=c['engines']['vanilla']['words'];records=[];missing=[]
 for w in a:
  matches=[z for z in v if z['text']==w['text'] and abs(float(z['yMin'])-float(w['yMin']))<.05]
  if not matches:
   for i,z in enumerate(v[:-1]):
    zz=v[i+1]
    if (z['text'] or '')+(zz['text'] or '')==w['text'] and abs(float(z['yMin'])-float(w['yMin']))<.05 and abs(float(zz['yMin'])-float(w['yMin']))<.05:
     matches.append({'text':w['text'],**{k:str(min(float(z[k]),float(zz[k])) if k in ['xMin','yMin'] else max(float(z[k]),float(zz[k]))) for k in ['xMin','xMax','yMin','yMax']}})
  if not matches:missing.append(w);continue
  near=min(matches,key=lambda z:abs(float(z['xMin'])-float(w['xMin'])))
  delta={k:abs(float(w[k])-float(near[k])) for k in ['xMin','xMax','yMin','yMax']};records.append({'candidate':w,'vanilla_group':near,'delta':delta})
 out[name]={'matched':len(records),'missing':missing,'max_delta_pt':max((x for r in records for x in r['delta'].values()),default=0),'records':records}
 if name=='mixed_ltr':
  baseline=c['engines']['baseline']['words'];latin=lambda ws:[w for w in ws if w['text'] in ['ABC','123']]
  assert latin(a)==latin(baseline), 'pre-existing LTR alignment must remain unchanged'
  out[name]['preexisting_ltr_geometry_preserved']=True
 print(name,'matched',len(records),'missing',len(missing),'maxdelta',out[name]['max_delta_pt'])
gzip.open(H/'geometry.json.gz','wt').write(json.dumps(out,ensure_ascii=False));assert all(not x['missing'] and (x['max_delta_pt']<.05 or (n=='mixed_ltr' and x['preexisting_ltr_geometry_preserved'] and all(r['delta']['xMin']<.05 for r in x['records'] if any(ch>'\u007f' for ch in r['candidate']['text'])))) for n,x in out.items())
