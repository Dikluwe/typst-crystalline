from pathlib import Path
import json,gzip,datetime
p=Path(__file__).resolve().parent
load=lambda n:json.loads(gzip.decompress((p/n).read_bytes()))
before=load('before.json.gz');after=load('after.json.gz')
def flat(r):return [(i,w['text'],w['bounds']) for i,page in enumerate(r['pages']) for w in page['words']]
def mismatches(c):
 t,v=[c['results'][k] for k in ['tekt','vanilla']]
 assert t['exit']==v['exit']==0,c['label']
 tr,vr=flat(t),flat(v)
 if [(i,s) for i,s,b in tr] != [(i,s) for i,s,b in vr]:return ['line/page/text or word segmentation']
 return [(s,[round(a-b,6) for a,b in zip(tb,vb)]) for (i,s,tb),(_,_,vb) in zip(tr,vr) if any(abs(a-b)>0.001 for a,b in zip(tb,vb))]
rows=[]
for b,c in zip(before['cases'],after['cases']):
 assert b['label']==c['label']
 t,v=c['results']['tekt'],c['results']['vanilla'];diff=mismatches(c)
 assert t['text']==b['results']['tekt']['text'],c['label']
 if c['label'].startswith('bib-'):
  tw,vw=flat(t),flat(v)
  title=next(b for _,s,b in tw if s=='Bibliography');vt=next(b for _,s,b in vw if s=='Bibliography')
  assert all(abs(a-b)<0.001 for a,b in zip(title,vt)),(c['label'],title,vt)
  word='A.' if c['label'].endswith('ieee') else 'Silva,'
  body=next(b for _,s,b in tw if s==word);vb=next(b for _,s,b in vw if s==word)
  assert all(abs(a-b)<0.001 for a,b in zip(body,vb)),(c['label'],body,vb)
 else:
  assert not diff,(c['label'],diff)
  assert all(abs(a['height']-b['height'])<0.001 for a,b in zip(t['pages'],v['pages'])),c['label']
 rows.append({'case':c['label'],'before_difference_count':len(mismatches(b)),'after_difference_count':len(diff),'remaining_differences':diff})
extra_before=load('extra-before.json.gz');extra_after=load('extra-after.json.gz');extra=[]
for b,c in zip(extra_before['cases'],extra_after['cases']):
 assert b['label']==c['label'];diff=mismatches(c);old=mismatches(b)
 # Compare each observable word against the same oracle, retaining old residuals.
 bt,at,vr=[flat(result) for result in [b['results']['tekt'],c['results']['tekt'],c['results']['vanilla']]]
 assert [w for _,w,_ in bt]==[w for _,w,_ in at]==[w for _,w,_ in vr],c['label']
 assert abs(len(c['results']['tekt']['pages'])-len(c['results']['vanilla']['pages'])) <= abs(len(b['results']['tekt']['pages'])-len(b['results']['vanilla']['pages'])),c['label']
 for (op,word,ob),(np,_,nb),(vp,_,vb) in zip(bt,at,vr):
  old_page_error,new_page_error=abs(op-vp),abs(np-vp)
  assert new_page_error<=old_page_error,(c['label'],word,'page regression')
  if new_page_error==old_page_error and not c['label'].endswith('-mla'):
   assert all(abs(n-v)<=abs(o-v)+0.001 for o,n,v in zip(ob,nb,vb)),(c['label'],word,ob,nb,vb)
 if c['label'].endswith('-mla'):
  # This remains a parity failure: only origin and preservation of the internal
  # layout are certified. The absolute wrong-line distance is recorded below.
  index=next(i for i,(_,word,_) in enumerate(at) if word=='Silva,')
  shift=at[index][2][1]-bt[index][2][1]
  assert all(abs(n-v)<0.001 for n,v in zip(at[index][2],vr[index][2])),c['label']
  assert bt[:index]==at[:index],c['label']
  for (op,word,ob),(np,_,nb) in zip(bt[index:],at[index:]):
   assert op==np and all(abs(n-o-(shift if j in (1,3) else 0))<0.001 for j,(o,n) in enumerate(zip(ob,nb))),(c['label'],word,'changed internal layout')
 assert c['results']['tekt']['text']==b['results']['tekt']['text'],c['label']
 extra.append({'case':c['label'],'before_difference_count':len(old),'after_difference_count':len(diff),'remaining_differences':diff})
result={'utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'source_state':'freeze.json.gz','cases':rows,'extras':extra,'generic_fully_matching':sum(not x['remaining_differences'] for x in rows if not x['case'].startswith('bib-')),'bibliography_scope':'Title and first body aligned. Mixed-run baselines, hanging and all later lines remain explicitly unaccepted.'}
(p/'audit.json.gz').write_bytes(gzip.compress(json.dumps(result,indent=2).encode()))
print(json.dumps({'cases':len(rows),'extras':len(extra),'generic_fully_matching':result['generic_fully_matching'],'remaining':[(x['case'],x['after_difference_count']) for x in rows+extra if x['after_difference_count']]}))
