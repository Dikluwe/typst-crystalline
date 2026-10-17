from pathlib import Path
import datetime,json,re,subprocess,sys,hashlib
from rust_matches import block,arms,variants
out=Path(__file__).resolve().parents[1];phase=sys.argv[1];assert phase in ['red','green'];value=Path('01_core/src/entities/value.rs').read_text();sources=['01_core/src/compiler/eval/call_dispatch/type_fields.rs','01_core/src/compiler/stdlib/collections.rs'];targets=[]
if phase=='red':
 targets=[(sources[0],'constants','Type','let constant = match (t, field)', '(subject, field)'),(sources[0],'functions','Type','let function = match (t, field)','(subject, field)'),(sources[1],'instance','Value','match (target, method)','(subject, field)'),(sources[1],'collection-static','Type','let function = match (t, field)','(subject, field)')]
else:targets=[(sources[0],'type-fields','Type','match t {','subject'),(sources[1],'instance','Value','match target {','subject'),(sources[1],'collection-static','Type','match t {','subject')]
results=[]
for file,name,enum,marker,expr in targets:
 source=Path(file).read_text();patterns=[p for p,r in arms(block(source,marker)[2])];definition=variants(value,enum);assert len(definition)==(38 if enum=='Type' else 39)
 for future in [False,True]:
  entries=[n+('('+','.join('()' for _ in range(a))+')' if a else '') for n,a in definition];entries+=['TektCatalogueFuture'] if future else []
  program='#![allow(dead_code,unused_variables)]\nenum '+enum+'{'+','.join(entries)+'}\nfn check(subject:'+enum+',field:&str){match '+expr+'{'+''.join(p+'=>(),\n' for p in patterns)+'}}\n'
  path=out/f'{phase}-{name}-{future}.rs';path.write_text(program);meta=path.with_suffix('.rmeta');cmd=['rustc','--edition=2021','--crate-type=lib','--emit=metadata','--error-format=json',str(path),'-o',str(meta)];r=subprocess.run(cmd,capture_output=True,text=True);codes=[x['code']['code'] for line in r.stderr.splitlines() if line.startswith('{') and (x:=json.loads(line)).get('code')];row=dict(source=file,source_sha256=hashlib.sha256(source.encode()).hexdigest(),name=name,enum=enum,current_variants=len(definition),future=future,argv=cmd,exit=r.returncode,codes=codes,stderr=r.stderr);results.append(row);meta.unlink(missing_ok=True)
  assert r.returncode==(1 if future and phase=='green' else 0),(row,r.stderr)
  if future and phase=='green':assert codes==['E0004'] and 'TektCatalogueFuture' in r.stderr
record=dict(utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),method='Exact outer productive arm patterns; RHS erased to unit solely to test exhaustiveness. Variant names/arities read from live enum, payloads stubbed. This is a compiler guard, not runtime equivalence. No Cargo workspace/target mutation.',value_sha256=hashlib.sha256(value.encode()).hexdigest(),results=results)
(out/(phase+'-probe-result.json')).write_text(json.dumps(record,indent=2)+'\n');print(phase,len(targets),'matches checked')
