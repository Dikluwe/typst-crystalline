from pathlib import Path
from collections import OrderedDict
import gzip,json,re,datetime,hashlib
from rust_matches import block,arms,variants
out=Path(__file__).resolve().parents[1];b=json.load(gzip.open(out/'before.json.gz'));scope=json.load(open(out/'scope.json'));value=Path('01_core/src/entities/value.rs').read_text()
def negative(enum,positive):
 return '\n        | '.join(enum+'::'+n+('(..)' if a else '') for n,a in variants(value,enum) if n not in positive)
def unwrap(rhs):
 return rhs[1:-1].strip() if rhs.startswith('{') and rhs.endswith('}') else rhs
# Constant and function lookup share one exhaustive Type classification.
p=Path(scope['sources'][0]);text=p.read_text();original=b['files'][str(p)];groups=OrderedDict()
for marker,is_func in [('let constant = match (t, field)',False),('let function = match (t, field)',True)]:
 for pattern,rhs in arms(block(original,marker)[2]):
  if pattern=='_':continue
  m=re.fullmatch(r'\(Type::(\w+), ("[^"]+")\)',pattern);assert m,pattern
  kind,field=m.groups();groups.setdefault(kind,[]).append((field,'Some(Value::Func('+unwrap(rhs)+'))' if is_func else rhs))
body='\n    use crate::entities::dir::Dir;\n    use crate::entities::layout_types::{Align2D, HAlign, VAlign};\n\n    match t {\n'
for kind,entries in groups.items():
 body+='        Type::'+kind+' => match field {\n'+''.join('            '+field+' => '+rhs+',\n' for field,rhs in entries)+'            _ => None,\n        },\n'
body+='        // These types have no members owned by this catalogue.\n        '+negative('Type',groups)+' => None,\n    }\n'
a,z,_=block(text,'pub(crate) fn p1284_type_field');p.write_text(text[:a+1]+body+text[z-1:])
# Receiver classification precedes open method lookup. Preserve every original RHS.
p=Path(scope['sources'][1]);text=p.read_text();original=b['files'][str(p)];groups=OrderedDict()
for pattern,rhs in arms(block(original,'match (target, method)')[2]):
 if pattern.startswith('_'):continue
 m=re.search(r'\(Value::(\w+)\((\w+)\), ("[^"]+"|method)\)',pattern);assert m,pattern
 kind,binding,method=m.groups();groups.setdefault((kind,binding),[]).append((method,rhs))
body='\n'
for (kind,binding),entries in groups.items():
 body+='        Value::'+kind+'('+binding+') => '
 if kind=='Bytes':
  assert len(entries)==1 and entries[0][0]=='method';body+=entries[0][1]+',\n'
 else:body+='match method {\n'+''.join('            '+method+' => '+rhs+',\n' for method,rhs in entries)+'            _ => None,\n        },\n'
body+='        // Values outside the collection receiver families continue in the caller.\n        '+negative('Value',{k for k,_ in groups})+' => None,\n    '
a,z,_=block(text,'match (target, method)');start=text.rfind('match (target, method)',0,a);text=text[:start]+'match target {'+body+text[z-1:]
groups=OrderedDict()
for pattern,rhs in arms(block(original,'let function = match (t, field)')[2]):
 if pattern=='_':continue
 m=re.fullmatch(r'\(Type::(\w+), ("[^"]+")\)',pattern);assert m,pattern
 kind,field=m.groups();groups.setdefault(kind,[]).append((field,'Some(Value::Func('+unwrap(rhs)+'))'))
body='\n    match t {\n'
for kind,entries in groups.items():body+='        Type::'+kind+' => match field {\n'+''.join('            '+field+' => '+rhs+',\n' for field,rhs in entries)+'            _ => None,\n        },\n'
body+='        // These types have no collection catalogue in this owner.\n        '+negative('Type',groups)+' => None,\n    }\n'
a,z,_=block(text,'pub(crate) fn collection_type_field');text=text[:a+1]+body+text[z-1:];p.write_text(text)
(out/'implementation.json').write_text(json.dumps(dict(utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),sources={p:hashlib.sha256(Path(p).read_bytes()).hexdigest() for p in scope['sources']},method='Regroup existing arm bodies by enum family; preserve all runtime RHS and function identities.'),indent=2)+'\n')
