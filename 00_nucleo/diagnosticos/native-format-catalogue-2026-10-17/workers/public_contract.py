from pathlib import Path
import re,json,hashlib,sys
out=Path(__file__).resolve().parents[1];scope=json.load(open(out/'scope.json'))
def contract(source):
 text=Path(source).read_text();headers=[]
 for m in re.finditer(r'^\s*pub fn \w+',text,re.M):
  end=text.index('{',m.start());headers.append(re.sub(r'\s+','',text[m.start():end]))
 exports=[re.sub(r'\s+','',m[0]) for m in re.finditer(r'^pub use [\s\S]*?;',text,re.M)]
 structs={}
 if source.endswith('/entities/func.rs'):
  for name in ['Func','NativeFunc','NativeFuncWithEngine','ElementFunc','ClosureRepr','ClosureParam']:
   m=re.search(r'^pub struct '+name+r'\b',text,re.M);start=m.start();kind=text[m.end():].lstrip()[0];end=text.index(';',m.end())+1 if kind=='(' else text.index('\n}',m.end())+2
   body=re.sub(r'^\s*///.*$','',text[start:end],flags=re.M);structs[name]=re.sub(r'\s+','',body)
 return dict(function_signatures=headers,public_reexports=exports,public_structs=structs)
values={p:contract(p) for p in scope['sources']};phase=sys.argv[1]
if phase=='after':assert values==json.load(open(out/'public-contract-before.json')),'External public Rust interface changed'
(out/('public-contract-'+phase+'.json')).write_text(json.dumps(values,indent=2)+'\n');print('public contract',phase,len(values),'consumers')
