from pathlib import Path
import gzip,json,datetime
out=Path(__file__).resolve().parents[1];rows=[];open_cases=[];max_residue=0.0
key=lambda r:(r['name'],r['binary'])
def differences(a,b,graphics=False):
 d=[]
 if len(a['pages'])!=len(b['pages']):d.append('page-count')
 for p,q in zip(a['pages'],b['pages']):
  for prop in ['width','height']:
   if abs(p[prop]-q[prop])>0.002:d.append(prop)
  if [w['text'] for w in p['words']]!=[w['text'] for w in q['words']]:d.append('word-sequence');continue
  for w,z in zip(p['words'],q['words']):
   for prop in ['x0','y0','y1']:
    if abs(w[prop]-z[prop])>0.002:d.append(prop)
 if graphics:
  if len(a['red_bounds'])!=len(b['red_bounds']):d.append('red-count')
  for p,q in zip(a['red_bounds'],b['red_bounds']):
   if p.get('page')!=q.get('page'):d.append('red-page')
   for prop in ['x0','x1','y0','y1']:
    if abs(p[prop]-q[prop])>0.002:d.append('red-'+prop)
 return sorted(set(d))
for prefix in ['', 'supplement-','finite-','regions-']:
 load=lambda phase:json.load(open(out/(prefix+phase+'-analysis.json'))) if prefix in ['', 'regions-'] else json.load(gzip.open(out/(prefix+phase+'-measurement.json.gz')))
 before=load('before');after=load('after');old={key(r):r for r in before['cases']};new={key(r):r for r in after['cases']};assert old.keys()==new.keys()
 for name in dict.fromkeys(x['name'] for x in after['cases']):
  t=new[name,'tekt'];v=new[name,'vanilla'];prior=differences(old[name,'tekt'],old[name,'vanilla'],prefix in ['', 'regions-']);current=differences(t,v,prefix in ['', 'regions-'])
  assert not differences(v,old[name,'vanilla'],prefix in ['', 'regions-']),(prefix,name,'oracle drift')
  if name=='empty-fr-box' and prefix in ['', 'regions-']:
   assert current and not differences(t,old[name,'tekt'],True);open_cases.append(dict(corpus=prefix,name=name,differences=current,reason='Existing incomplete fractional Box composition B; preserved'));continue
  assert not current,(prefix,name,current)
  if not prior:assert not differences(t,old[name,'tekt'],prefix in ['', 'regions-']),(prefix,name,'control regression')
  for p,q in zip(t['pages'],v['pages']):
   for w,z in zip(p['words'],q['words']):max_residue=max(max_residue,abs(w['x1']-z['x1']))
  rows.append(dict(corpus=prefix,name=name,before=prior,after=current))
r=dict(utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),tolerance_pt=0.002,method='Exact physical pages and ordered words; page width/height, xMin/yMin/yMax and main-corpus red SVG bounds within fixed tolerance. xMax recorded separately, no PDF-byte requirement.',pairs=len(rows)+len(open_cases),certified=len(rows),repaired=sum(bool(r['before']) for r in rows),preserved=sum(not r['before'] for r in rows),retained_open=open_cases,max_xmax_residue=max_residue,cases=rows)
(out/'observables-result.json').write_text(json.dumps(r,indent=2)+'\n');print({k:v for k,v in r.items() if k!='cases'})
