from pathlib import Path
import json,gzip,yaml,hashlib
from PIL import Image
out=Path('00_nucleo/diagnosticos/raw-presenca-2026-10-14')
def read(n):
 with gzip.open(out/n,'rt') as f:return json.load(f)
report={}
for stage in ['before','after']:
 data=read(stage+'/results.json.gz');groups={}
 for row in data['rows']:groups.setdefault((row['case'],row['mode']),{})[row['binary']]=row
 rows=[]
 for (case,mode),d in groups.items():
  a,b=d['tekt'],d['vanilla']
  def decode(r):
   if r['exit']:return r['stderr'].splitlines()[0]
   return yaml.safe_load(r['stdout']) if 'yaml' in mode else json.loads(r['stdout'])
  va,vb=decode(a),decode(b)
  rows.append(dict(case=case,mode=mode,same=a['exit']==b['exit'] and va==vb,tekt=va,vanilla=vb))
 report[stage]=dict(comparisons=len(rows),matches=sum(x['same'] for x in rows),differences=[x for x in rows if not x['same']])
report['render']=[]
for row in read('render-before/results.json.gz')['rows']:
 name=row['name'];a=Image.open(out/'render-before'/(name+'.png')).convert('RGB');b=Image.open(out/'render-after'/(name+'.png')).convert('RGB');report['render'].append(dict(case=name,same_size=a.size==b.size,identical_pixels=a.tobytes()==b.tobytes(),before_pixels_sha256=hashlib.sha256(a.tobytes()).hexdigest(),after_pixels_sha256=hashlib.sha256(b.tobytes()).hexdigest()))
(out/'comparison.json').write_text(json.dumps(report,ensure_ascii=False,indent=2));print(json.dumps(report,ensure_ascii=False,indent=2))
