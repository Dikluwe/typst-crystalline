from pathlib import Path
import subprocess,re,json,hashlib,datetime
out=Path(__file__).resolve().parents[1];s=Path('01_core/src/compiler/stdlib/calc/power.rs').read_text();value=Path('01_core/src/entities/value.rs').read_text().split('pub enum Value {',1)[1].split('\n}',1)[0]
variants=[]
for line in value.splitlines():
 m=re.match(r'^    (\w+)(\(.*\))?,\s*$',line)
 if m:
  name,args=m.groups();payload={'Int':'i64','Float':'f64','Decimal':'Decimal','LocatedContent':'(), ()'}.get(name,'()');variants.append(name+(f'({payload})' if args else ''))
assert len(variants)==39,len(variants)
# Compile the exact production conversion and enum (only dependency paths rewritten).
conversion=s[s.index('#[derive(Clone, Copy)]\nenum PowerBase'):s.index('\npub(crate) fn calc_sqrt')].replace('crate::entities::decimal::Decimal','Decimal')
preamble='#![allow(dead_code)]\n#[derive(Clone,Copy)] struct Decimal;\ntype SourceResult<T> = Result<T,Vec<SourceDiagnostic>>; struct SourceDiagnostic; impl SourceDiagnostic { fn error(_:Span,_:impl Into<String>) -> Self { Self } } struct Span; impl Span { fn detached() -> Self { Self } }\n\n'
results=[]
for future in [False,True]:
 program=preamble+'enum Value { '+','.join(variants+(['TektPowerFuture'] if future else []))+' }\nimpl Value { fn type_name(&self) -> &str { "stub" } }\n'+conversion
 path=out/('probe-future.rs' if future else 'probe-control.rs');path.write_text(program)
 cmd=['rustc','--edition=2021','--crate-type=lib','--emit=metadata','--error-format=json',str(path),'-o',str(out/'probe.rmeta')];r=subprocess.run(cmd,capture_output=True,text=True);errors=[json.loads(l) for l in r.stderr.splitlines() if l.startswith('{')];codes=[x.get('code',{}).get('code') for x in errors if x.get('code')];results.append(dict(future=future,command=cmd,exit=r.returncode,stderr=r.stderr,codes=codes))
 assert r.returncode==(1 if future else 0)
 if future:assert codes==['E0004'] and 'TektPowerFuture' in r.stderr
record=dict(utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),source_sha256=hashlib.sha256(s.encode()).hexdigest(),value_sha256=hashlib.sha256(Path('01_core/src/entities/value.rs').read_bytes()).hexdigest(),method='Isolated rustc: exact production boundary and private enums, Value variant shapes from live enum; unrelated payloads stubbed; compile-time coverage only, not a workspace mutation or runtime parity proof.',results=results)
(out/'probe-result.json').write_text(json.dumps(record,indent=2)+'\n');(out/'probe.rmeta').unlink(missing_ok=True);print('39-variant control PASS; future variant rejected E0004')
