from pathlib import Path
import json,gzip,datetime
out=Path(__file__).resolve().parents[1];counts={};changes=[];residual=[]
for name in ['probe','math-probe']:
 a=json.load(gzip.open(out/('before-'+name+'.json.gz')));z=json.load(gzip.open(out/('after-'+name+'.json.gz')));assert a['binaries']['vanilla']['sha256']==z['binaries']['vanilla']['sha256'];assert all(z.get('fixtures',{}).get(p)==h for p,h in a.get('fixtures',{}).items());assert len(a['cases'])==len(z['cases'])
 for old,new in zip(a['cases'],z['cases']):
  assert old['expression']==new['expression'];t=new['results']['tekt'];v=new['results']['vanilla'];o=old['results']['tekt'];assert old['results']['vanilla']==v
  norm=lambda r:(r['exit'],r['stdout'],r['stderr'].removesuffix('warning: ficheiro vazio: sem conteúdo\n\n'))
  if norm(t)!=norm(v):
   assert name=='math-probe' and norm(t)==norm(o),(new,t,v)
   residual.append(dict(expression=new['expression'],tekt=t,vanilla=v));category='unchanged-negative-diagnostic-residual'
  else:category='language-parity'
  counts[category]=counts.get(category,0)+1
  if norm(o)!=norm(t):changes.append(dict(corpus=name,expression=new['expression']))
result=dict(utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),counts=counts,changes=changes,residual=residual);(out/'observables-result.json').write_text(json.dumps(result,indent=2)+'\n');print(counts,'changes',len(changes),'residual',len(residual))
