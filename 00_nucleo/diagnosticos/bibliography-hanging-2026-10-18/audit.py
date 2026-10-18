from pathlib import Path
import gzip,json,collections,datetime
p=Path(__file__).resolve().parent
read=lambda n:json.loads(gzip.decompress((p/(n+'.json.gz')).read_bytes()))
def audit(c):
 a=c['results']['tekt'];b=c['results']['vanilla']
 out=dict(label=c['label'],compile_ok=a['exit']==b['exit']==0,text_equal=a['text']==b['text'])
 if not out['compile_ok']:return out
 pa=a['pages'];pb=b['pages'];out['pages_equal']=len(pa)==len(pb);out['tekt_pages']=len(pa);out['vanilla_pages']=len(pb)
 wa=[(i,w) for i,page in enumerate(pa) for w in page['words']];wb=[(i,w) for i,page in enumerate(pb) for w in page['words']]
 out['words_equal']=[w['text'] for _,w in wa]==[w['text'] for _,w in wb]
 out['size_max_error']=max([abs(x[k]-y[k]) for x,y in zip(pa,pb) for k in ['width','height']],default=0)
 if out['words_equal']:
  out['word_pages_equal']=[i for i,_ in wa]==[i for i,_ in wb]
  out['bbox_max_error']=max([abs(x-y) for (_,a),(_,b) in zip(wa,wb) for x,y in zip(a['bounds'],b['bounds'])],default=0)
  out['bbox_x_max_error']=max([abs(a['bounds'][k]-b['bounds'][k]) for (_,a),(_,b) in zip(wa,wb) for k in [0,2]],default=0)
  out['first_difference']=next((dict(tekt=dict(page=i,**a),vanilla=dict(page=j,**b)) for (i,a),(j,b) in zip(wa,wb) if i!=j or max(abs(x-y) for x,y in zip(a['bounds'],b['bounds']))>.001),None)
 out['full_geometry_equal']=out['words_equal'] and out['pages_equal'] and out.get('word_pages_equal',False) and out.get('bbox_max_error',float('inf'))<=.001 and out['size_max_error']<=.001
 return out
rows=[]
for before,after in [('before','after'),('extra-before','extra-after'),('boundary-before','boundary-after'),('repeat-before','repeat-after')]:
 b=read(before);a=read(after);assert [c['source_sha256'] for c in b['cases']]==[c['source_sha256'] for c in a['cases']]
 for old,new in zip(b['cases'],a['cases']):rows.append(dict(label=new['label'],before=audit(old),after=audit(new)))
result=dict(utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),pairs=len(rows),full_before=sum(x['before'].get('full_geometry_equal',False) for x in rows),full_after=sum(x['after'].get('full_geometry_equal',False) for x in rows),regressed=[x['label'] for x in rows if x['before'].get('full_geometry_equal') and not x['after'].get('full_geometry_equal')],rows=rows)
(p/'audit.json.gz').write_bytes(gzip.compress(json.dumps(result,indent=2).encode()))
print(json.dumps({k:v for k,v in result.items() if k!='rows'},indent=2))
for x in rows:
 if not x['after'].get('full_geometry_equal'):print(x['label'],json.dumps(x['after']))
assert all(x['after']['compile_ok'] for x in rows)
