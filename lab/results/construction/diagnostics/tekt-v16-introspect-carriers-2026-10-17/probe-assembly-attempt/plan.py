from pathlib import Path
import json,re,subprocess,itertools
out=Path(__file__).resolve().parents[1]
def variants(path,name):
 body=Path(path).read_text().split('pub enum '+name+' {',1)[1].split('\n}',1)[0];rows=[]
 for m in re.finditer(r'^    (\w+)\s*([({,])',body,re.M):
  n,k=m.groups();fields=[]
  if k=='{':
   part=body[m.end():];part=part[:part.index('}')];part='\n'.join(x for x in part.splitlines() if not x.lstrip().startswith('//'));fields=[dict(name=f,bool=t.strip()=='bool') for f,t in re.findall(r'\b(\w+):\s*([^,\n]+)',part)];assert fields,n
  rows.append(dict(name=n,kind=k,fields=fields))
 return rows
content=variants('01_core/src/entities/content.rs','Content');payload=variants('01_core/src/entities/element_payload.rs','ElementPayload');assert len(content)==120 and len(payload)==19
rows=json.load(open(out/'contexts-before.json'));targets=[]
for row in rows:
 if row['function']=='walk':continue
 en='Content' if row['function']=='compute_labelled' else 'ElementPayload';vs=content if en=='Content' else payload;asts=json.loads(subprocess.check_output(['/tmp/tekt-math-layout-latent',row['source'],'matches'],text=True));ix=[i for i,n in enumerate(asts) if n['line']==row['match_line'] and n['end_line']==row['match_end_line']];assert len(ix)==1
 if en=='Content':
  excluded={'Source','Heading','Figure'};complement=[v for v in vs if v['name'] not in excluded];patterns=['Content::'+v['name']+('(..)' if v['kind']=='(' else ' { .. }' if v['kind']=='{' else '') for v in complement]
 else:
  active={'NativeElement','Footnote','Citation','Heading','Figure','Table','Equation'};complement=[v for v in vs if v['name'] not in active];patterns=['ElementPayload::'+v['name']+(' { .. }' if v['kind']=='{' else '') for v in complement];patterns+=['ElementPayload::Heading { numbering_active: false, .. }','ElementPayload::Figure { is_counted: false, .. }','ElementPayload::Table { is_counted: false, .. }','ElementPayload::Equation { block: false, .. }','ElementPayload::Equation { block: true, numbering_active: false, .. }']
 targets.append(dict(id=len(targets),**row,enum=en,match_index=ix[0],replacement='\n            | '.join(patterns),alternatives=len(patterns)))
assert len(targets)==2 and [t['alternatives'] for t in targets]==[117,17];(out/'projection-plan.json').write_text(json.dumps(dict(content=content,payload=payload,targets=targets),indent=2)+'\n');print([(t['function'],t['alternatives']) for t in targets])
