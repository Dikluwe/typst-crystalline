from pathlib import Path
import datetime,hashlib,json,re,subprocess,sys
out=Path(__file__).resolve().parents[1];label=sys.argv[1] if len(sys.argv)>1 else 'green';s=Path('01_core/src/compiler/stdlib/calc/abs.rs').read_text();v=Path('01_core/src/entities/value.rs').read_text();body=v.split('pub enum Value {',1)[1].split('\n}',1)[0];variants=[]
for line in body.splitlines():
 m=re.match(r'^    (\w+)(\(.*\))?,\s*$',line)
 if m:
  n,args=m.groups();variants.append(n+('('+('(), ()' if n=='LocatedContent' else '()')+')' if args else ''))
assert len(variants)==39
start=s.index('match other {',s.index('let found ='));end=s.index('\n            };',start)+len('\n            }');match=s[start:end];results=[]
for future in [False,True]:
 program='#![allow(dead_code)]\nenum Value {'+','.join(variants+(['TektAbsFuture'] if future else []))+'}\nimpl Value {fn type_name(&self)->&str {"stub"}}\nfn label(other:&Value)->&str{'+match+'}\n'
 path=out/(label+('-future.rs' if future else '-control.rs'));path.write_text(program);meta=out/(label+'-probe.rmeta');cmd=['rustc','--edition=2021','--crate-type=lib','--emit=metadata','--error-format=json',str(path),'-o',str(meta)];r=subprocess.run(cmd,capture_output=True,text=True);errors=[json.loads(x) for x in r.stderr.splitlines() if x.startswith('{')];codes=[x['code']['code'] for x in errors if x.get('code')];results.append(dict(future=future,command=cmd,exit=r.returncode,stderr=r.stderr,codes=codes));meta.unlink(missing_ok=True)
record=dict(utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),source_sha256=hashlib.sha256(s.encode()).hexdigest(),value_sha256=hashlib.sha256(v.encode()).hexdigest(),method='Exact productive diagnostic-name match extracted; live Value variant names/arities, unrelated payloads stubbed. Structural compiler guard only, not runtime parity or workspace mutation.',results=results)
(out/(label+'-probe-result.json')).write_text(json.dumps(record,indent=2)+'\n');assert results[0]['exit']==0 and not results[0]['codes'],results[0];assert results[1]['exit']==1 and results[1]['codes']==['E0004'] and 'TektAbsFuture' in results[1]['stderr'],results[1];print('Current39 compile; future variant rejected E0004')
