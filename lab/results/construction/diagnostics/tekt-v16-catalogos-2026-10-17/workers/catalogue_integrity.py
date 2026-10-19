from pathlib import Path
import gzip,json,re,datetime
from rust_matches import block,arms
out=Path(__file__).resolve().parents[1];b=json.load(gzip.open(out/'before.json.gz'));scope=json.load(open(out/'scope.json'))
def tokens(s):
 ts=re.findall(r'"(?:\\.|[^"\\])*"|\w+|[^\s]',s)
 return [x for i,x in enumerate(ts) if not (x==',' and i+1<len(ts) and ts[i+1] in [')',']','}'])]
def unwrap(s):return s[1:-1].strip() if s.startswith('{') and s.endswith('}') else s
rows=[]
for file,markers,outer,enum,func in [(scope['sources'][0],['let constant = match (t, field)','let function = match (t, field)'],'match t {','Type',True),(scope['sources'][1],['let function = match (t, field)'],'match t {','Type',True),(scope['sources'][1],['match (target, method)'],'match target {','Value',False)]:
 old={}
 for marker in markers:
  for p,r in arms(block(b['files'][file],marker)[2]):
   m=re.search(r'\('+enum+r'::(\w+)(?:\((\w+)\))?, ("[^"]+"|method)\)',p)
   if not m:continue
   kind,binding,field=m.groups()
   if 'function' in marker:r='Some(Value::Func('+unwrap(r)+'))'
   old[kind,field]=tokens(unwrap(r))
 new={}
 for p,r in arms(block(Path(file).read_text(),outer)[2]):
  if '|' in p:continue
  m=re.fullmatch(enum+r'::(\w+)(?:\((\w+)\))?',p.strip());assert m,p
  kind,binding=m.groups()
  if kind=='Bytes' and enum=='Value':new[kind,'method']=tokens(unwrap(r));continue
  for field,rhs in arms(block(r,'match ')[2]):
   if field=='_':assert rhs=='None';continue
   new[kind,field]=tokens(unwrap(rhs))
 assert old==new,(file,[(k,old.get(k),new.get(k)) for k in old.keys()|new.keys() if old.get(k)!=new.get(k)])
 rows.append(dict(source=file,enum=enum,preserved_arms=len(old)))
r=dict(utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),method='Token comparison preserving string literals and all RHS operators/identities; removes only wrapping braces, whitespace and syntactic trailing commas. Exhaustive negative families verified separately by compiler probes.',results=rows)
(out/'catalogue-integrity-result.json').write_text(json.dumps(r,indent=2)+'\n');print(rows)
