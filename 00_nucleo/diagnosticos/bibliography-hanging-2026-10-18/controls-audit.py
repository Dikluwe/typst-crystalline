from pathlib import Path
import json,gzip,datetime
p=Path(__file__).resolve().parent;q=p.parent/'bibliography-margins-2026-10-18'
read=lambda f:json.loads(gzip.decompress(f.read_bytes()))
rows=[]
for old,new in [('after','controls-after'),('extra-after','controls-extra-after')]:
 b=read(q/(old+'.json.gz'));a=read(p/(new+'.json.gz'));assert [c['source_sha256'] for c in a['cases']]==[c['source_sha256'] for c in b['cases']]
 for prev,cur in zip(b['cases'],a['cases']):
  t=cur['results']['tekt'];v=cur['results']['vanilla'];prior=prev['results']['tekt'];assert t['exit']==v['exit']==0
  flat=lambda x:[(i,w) for i,page in enumerate(x['pages']) for w in page['words']]
  wa=flat(t);wv=flat(v);wo=flat(prior);labels=lambda x:[w['text'] for i,w in x]
  row=dict(label=cur['label'],words_equal=labels(wa)==labels(wv),unchanged=wa==wo and t['pages']==prior['pages'])
  if row['words_equal']:
   row['bbox_max_error']=max([abs(x-y) for (_,a),(_,b) in zip(wa,wv) for x,y in zip(a['bounds'],b['bounds'])],default=0)
   row['pages_equal']=[i for i,_ in wa]==[i for i,_ in wv]
   row['sizes_equal']=len(t['pages'])==len(v['pages']) and all(abs(a[k]-b[k])<=.001 for a,b in zip(t['pages'],v['pages']) for k in ['width','height'])
   row['full_geometry_equal']=row['bbox_max_error']<=.001 and row['pages_equal'] and row['sizes_equal']
  else:row['full_geometry_equal']=False
  # Closed generic margins must remain fully equal to the oracle.
  if row['label'].startswith(('blocks-','heading-block-','block-heading-')):assert row['full_geometry_equal'],row
  # Non-MLA extra controls must retain their prior results or improve to full parity.
  if old=='extra-after' and not row['label'].endswith('-mla'):assert row['unchanged'] or row['full_geometry_equal'],row
  rows.append(row)
result=dict(utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),pairs=len(rows),full_geometry_equal=sum(r['full_geometry_equal'] for r in rows),unchanged=sum(r['unchanged'] for r in rows),rows=rows)
(p/'controls-audit.json').write_text(json.dumps(result,indent=2)+'\n');print(json.dumps({k:v for k,v in result.items() if k!='rows'},indent=2))
for row in rows:
 if not row['full_geometry_equal']:print(row)
