from pathlib import Path
import gzip,json,datetime
out=Path(__file__).resolve().parents[1];before=json.load(gzip.open(out/'before-measurement.json.gz'));after=json.load(gzip.open(out/'after-measurement.json.gz'));key=lambda x:(x['name'],x['binary']);old={key(x):x for x in before['cases']};new={key(x):x for x in after['cases']};assert old.keys()==new.keys()
def differences(a,b):
 d=[]
 if len(a['pages'])!=len(b['pages']):return ['page-count']
 for p,q in zip(a['pages'],b['pages']):
  for prop in ['width','height']:
   if abs(p[prop]-q[prop])>0.002:d.append(prop)
  if [w['text'] for w in p['words']]!=[w['text'] for w in q['words']]:d.append('words');continue
  for w,z in zip(p['words'],q['words']):
   for prop in ['x0','y0','x1','y1']:
    if abs(w[prop]-z[prop])>0.002:d.append(prop)
 return sorted(set(d))
rows=[]
for name in dict.fromkeys(x['name'] for x in after['cases']):
 # Fixed oracle itself must preserve geometry under the same font/input.
 assert not differences(new[name,'vanilla'],old[name,'vanilla']),name
 baseline=differences(old[name,'tekt'],old[name,'vanilla']);current=differences(new[name,'tekt'],new[name,'vanilla']);assert not current,(name,current)
 if not baseline:assert not differences(old[name,'tekt'],new[name,'tekt']),name
 rows.append(dict(name=name,before_differences=baseline,after_differences=current))
assert sum(bool(x['before_differences']) for x in rows)==5
record=dict(utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),tolerance_pt=0.002,method='Physical PDF pages, exact word sequence, per-word bounding coordinates; no byte equality or whitespace normalization.',cases=rows,repaired=5,preserved=13);(out/'observables-result.json').write_text(json.dumps(record,indent=2)+'\n');print('18 bilateral cases: 5 repaired; 13 coincident controls preserved.')
