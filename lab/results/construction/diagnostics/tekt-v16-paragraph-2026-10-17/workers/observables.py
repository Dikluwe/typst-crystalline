from pathlib import Path
import gzip,json,datetime
out=Path(__file__).resolve().parents[1];records=[];horizontal=[];total=0;repaired=0;retained_open=[]
key=lambda x:(x['name'],x['binary'])
def vertical(a,b):
 if len(a['pages'])!=len(b['pages']):return ['page-count']
 d=[]
 for p,q in zip(a['pages'],b['pages']):
  for prop in ['width','height']:
   if abs(p[prop]-q[prop])>0.002:d.append(prop)
  if [w['text'] for w in p['words']]!=[w['text'] for w in q['words']]:d.append('word-sequence');continue
  for w,z in zip(p['words'],q['words']):
   for prop in ['x0','y0','y1']:
    if abs(w[prop]-z[prop])>0.002:d.append(prop)
 return sorted(set(d))
for prefix in ['', 'supplement-','refuters-','edges-','empty-fonts-','descents-','fallback-']:
 before=json.load(gzip.open(out/(prefix+'before-measurement.json.gz')));after=json.load(gzip.open(out/(prefix+'after-measurement.json.gz')));old={key(x):x for x in before['cases']};new={key(x):x for x in after['cases']};assert old.keys()==new.keys()
 for name in dict.fromkeys(x['name'] for x in after['cases']):
  t=new[name,'tekt'];v=new[name,'vanilla'];total+=1
  if t['exit'] or v['exit']:
   assert prefix=='edges-' and name in ['strong-below-negative','strong-above-negative'] and t['exit']==1 and v['exit']==0;assert old[name,'tekt']['exit']==1;retained_open.append(dict(corpus=prefix,name=name,tekt_exit=1,vanilla_exit=0));continue
  assert not vertical(v,old[name,'vanilla']),name
  previous=vertical(old[name,'tekt'],old[name,'vanilla']);current=vertical(t,v);assert not current,(prefix,name,current)
  if not previous:assert not vertical(old[name,'tekt'],t),(prefix,name,'control regression')
  # Horizontal right-edge residue predates the change; no allowance for a new one.
  ta=[w for p in t['pages'] for w in p['words']];tb=[w for p in old[name,'tekt']['pages'] for w in p['words']];va=[w for p in v['pages'] for w in p['words']];assert [w['text'] for w in ta]==[w['text'] for w in tb]==[w['text'] for w in va]
  for a,b,c in zip(ta,tb,va):
   assert abs(a['x0']-b['x0'])<1e-7 and abs(a['x1']-b['x1'])<1e-7,(name,'new horizontal change')
   horizontal.append(dict(corpus=prefix,name=name,word=a['text'],before_residue=b['x1']-c['x1'],after_residue=a['x1']-c['x1']))
  records.append(dict(corpus=prefix,name=name,before_differences=previous,after_differences=current));repaired+=bool(previous)
assert total==70 and len(retained_open)==2
bt=json.load(gzip.open(out/'spacing-types-before.json.gz'));at=json.load(gzip.open(out/'spacing-types-after.json.gz'));old={key(dict(x,name=x['value'])):x for x in bt['cases']};new={key(dict(x,name=x['value'])):x for x in at['cases']};types=[]
for value in dict.fromkeys(x['value'] for x in at['cases']):
 t=new[value,'tekt'];v=new[value,'vanilla'];assert t['exit']==v['exit'],value
 if t['exit']:
  errors=lambda x:[line for line in x['stderr'].splitlines() if line.startswith('error:') or 'hint:' in line]
  assert errors(t)==errors(v),(value,errors(t),errors(v))
 types.append(dict(value=value,before_tekt_exit=old[value,'tekt']['exit'],after_tekt_exit=t['exit'],vanilla_exit=v['exit']))
record=dict(utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),tolerance_pt=0.002,method='Exact pages/word sequence; page height and xMin/yMin/yMax agree within fixed tolerance. xMax residual recorded separately and strictly unchanged within Tekt; no byte equality or whitespace normalization.',pairs=total,constructible_pairs=len(records),repaired=repaired,preserved=len(records)-repaired,retained_open=retained_open,cases=records,spacing_types=types,xmax_residue=horizontal,max_xmax_residue=max(abs(x['after_residue']) for x in horizontal));(out/'observables-result.json').write_text(json.dumps(record,indent=2)+'\n');print({k:record[k] for k in ['pairs','constructible_pairs','repaired','preserved','max_xmax_residue']})
