from pathlib import Path
import json,re,subprocess,hashlib
out=Path(__file__).resolve().parents[1];s=Path('01_core/src/entities/content.rs').read_text().split('pub enum Content {',1)[1].split('\n}',1)[0];variants=[]
for m in re.finditer(r'^    (\w+)\s*([({,])',s,re.M):
 name,kind=m.groups();fields=[]
 if kind=='{':
  rest=s[m.end():];body=rest.split('\n    }',1)[0];fields=re.findall(r'^        (\w+):',body,re.M);assert fields,(name,body)
 variants.append(dict(name=name,kind=kind,fields=fields))
assert len(variants)==120
rows=json.load(open(out/'contexts-before.json'));matches={};planned=[]
for i,row in enumerate(rows):
 p=row['source'];matches.setdefault(p,json.loads(subprocess.check_output(['/tmp/tekt-math-layout-latent',p,'matches'],text=True)));idx=[j for j,x in enumerate(matches[p]) if x['line']==row['match_line'] and x['end_line']==row['match_end_line']];assert len(idx)==1
 covered=set()
 for arm in row['arms']:
  pat=arm['pattern'];guard=re.search(r'\s+if\s+',pat)
  if pat=='_' or guard:continue
  if row['function']=='get_field' and not re.search(r',\s*f\s*\)$',pat):continue
  covered.update(re.findall(r'(?:Content|Self)::(\w+)',pat))
 if row['function']=='styled_equation':covered.update(['Source','Located','Realized','ShowRecipe'])
 complement=[v for v in variants if v['name'] not in covered];prefix='Self' if row['scrutinee']=='self' else 'Content'
 def pattern(v):return prefix+'::'+v['name']+('(..)' if v['kind']=='(' else ' { .. }' if v['kind']=='{' else '')
 pat='\n            | '.join(map(pattern,complement))
 if row['function']=='get_field':pat='('+pat+', _)'
 planned.append(dict(id=i,**row,match_index=idx[0],covered=sorted(covered),complement=[v['name'] for v in complement],replacement=pat))
(out/'projection-plan.json').write_text(json.dumps(dict(variants=variants,targets=planned,ast_binary_sha256=hashlib.sha256(Path('/tmp/tekt-math-layout-latent').read_bytes()).hexdigest()),indent=2)+'\n');print([(x['function'],len(x['complement'])) for x in planned])
