from pathlib import Path
import json,gzip,hashlib,subprocess,datetime,sys
out=Path(__file__).resolve().parents[1];phase=sys.argv[1];b=json.load(gzip.open(out/'before.json.gz'));scope=json.load(open(out/'scope.json'));sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest();now=lambda:datetime.datetime.now(datetime.timezone.utc).isoformat();hashes={p:sha(p) for p in set(b['hashes'])|set(scope['nuclei']) if Path(p).exists()}
def function(p,name):
 s=Path(p).read_text();i=s.index('fn '+name+'(');start=s.rfind('\n',0,i)+1;a=s.index('{',i);depth=1;z=a+1
 while depth:depth+=(s[z]=='{')-(s[z]=='}');z+=1
 return s[start:z]
types=Path('01_core/src/entities/layout_types.rs').read_text();i=types.index('pub enum MathSize {');a=types.rfind('#[derive(',0,i);z=types.index('\n}',i)+2;enum=types[a:z];assert [x for x in ['Display','Text','Script','ScriptScript'] if x in enum]==['Display','Text','Script','ScriptScript'];metric=function(scope['sources'][0],'ssty_level_of');shape=function(scope['sources'][1],'ssty_level_for_subrun');eligible=function(scope['sources'][0],'ssty_eligible_text');results=[];begin=now()
main=r'''
fn main() {
 let mut count=0;
 for (size,label,level) in [(MathSize::Display,"display",None),(MathSize::Text,"text",None),(MathSize::Script,"script",Some(1u8)),(MathSize::ScriptScript,"scriptscript",Some(2))] {
  for math in [false,true] { for math_text_item in [false,true] {
   let style=TextStyle{math,math_text_item,math_size:size};
   for eligible in [false,true] {
    let got=metrics::ssty_level_of(&style,eligible);
    assert_eq!(got, if math && !math_text_item && eligible { level } else { None });
    println!("M {label} {math} {math_text_item} {eligible} {got:?}");count+=1;
   }
   for (text,eligible) in [("",false),("x",true),("𝑥",true),(".",true),(" ",true),("12",true),("3.14",true),("..",false),("xy",false),("1.2.3",false),("١٢",false),("e\u{301}",false)] {
    assert_eq!(ssty_eligible_text(text),eligible);
    let got=shaping::ssty_level_for_subrun(&style,text);
    assert_eq!(got,if math && !math_text_item && eligible { level } else { None });
    println!("S {label} {math} {math_text_item} {text:?} {got:?}");count+=1;
   }
  }}
 }
 assert_eq!(count,224);
}
'''
for future in [False,True]:
 e=enum if not future else enum[:-1]+'    TektSstyFuture,\n}'
 program='#![allow(dead_code)]\nmod typst_core {pub mod entities {pub mod layout_types {\n'+e+'\n}}}\nuse typst_core::entities::layout_types::MathSize;\nstruct TextStyle { math:bool,math_text_item:bool,math_size:MathSize }\n'+eligible+'\nmod metrics {use super::*;\n'+metric+'\n}\nmod shaping {use super::*;\n'+shape+'\n}\n'+main
 tag=phase+('-future' if future else '-control');p=out/(tag+'.rs');p.write_text(program);binary=out/(tag+'.bin');cmd=['rustc','--edition=2021','--error-format=json','--crate-name','ssty_probe',str(p),'-o',str(binary)];r=subprocess.run(cmd,capture_output=True,text=True);diagnostics=[]
 for line in r.stderr.splitlines():
  try:d=json.loads(line)
  except ValueError:continue
  if d.get('code'):diagnostics.append(d)
 item=dict(future=future,command=cmd,exit=r.returncode,stderr=r.stderr,codes=[d['code']['code'] for d in diagnostics],diagnostics=diagnostics)
 if not future:
  assert r.returncode==0 and not diagnostics,item;run=subprocess.run([str(binary)],capture_output=True,text=True);assert run.returncode==0 and len(run.stdout.splitlines())==224;item['runtime']=dict(exit=run.returncode,stdout=run.stdout,stderr=run.stderr,cases=224)
 binary.unlink(missing_ok=True);results.append(item)
assert all(sha(p)==h for p,h in hashes.items());record=dict(start=begin,end=now(),head=b['head'],source_hashes=hashes,enum_source_sha256=sha('01_core/src/entities/layout_types.rs'),rustc=subprocess.check_output(['rustc','-Vv'],text=True),method='Exact live functions and MathSize declaration extracted verbatim; TextStyle fixture models only the three read fields. Structural future-enum guard and value controls, not full shaping/parity proof. Product enum and tree never mutated.',results=results,structural_guarded_sites=len(results[1]['codes']),sources_unchanged=True)
(out/(phase+'-probe-result.json')).write_text(json.dumps(record,indent=2)+'\n')
if phase=='before':assert results[1]['exit']==0 and not results[1]['codes'];print('Structural RED: both existing wildcard matches accept a future MathSize; 224 current controls pass')
else:
 assert results[1]['exit']==1 and results[1]['codes']==['E0004','E0004'],results[1];before=json.load(open(out/'before-probe-result.json'));assert before['results'][0]['runtime']==results[0]['runtime'];assert all('TektSstyFuture' in d['message'] for d in results[1]['diagnostics']);print('GREEN: future variant rejected at both owners; 224 controls identical')
