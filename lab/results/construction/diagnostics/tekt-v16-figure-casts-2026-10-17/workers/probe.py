from pathlib import Path
import datetime,json,subprocess,sys,hashlib
from rust_matches import block,arms,variants
out=Path(__file__).resolve().parents[1];phase=sys.argv[1];assert phase in ['red','green'];file=Path('01_core/src/compiler/stdlib/layout/casting.rs');source=file.read_text();value=Path('01_core/src/entities/value.rs').read_text();definition=variants(value,'Value');assert len(definition)==39;rows=[]
for name in ['extract_length','extract_block_width']:
 function=block(source,'pub(crate) fn '+name+'(')[2];patterns=[p for p,r in arms(block(function,'match val')[2])]
 for future in [False,True]:
  entries=[n+('('+','.join('()' for _ in range(a))+')' if a else '') for n,a in definition];entries+=['TektMeasureFuture'] if future else [];program='#![allow(dead_code,unused_variables)]\nenum Value{'+','.join(entries)+'}\nfn check(val:&Value){match val{'+''.join(p+'=>(),\n' for p in patterns)+'}}\n';p=out/f'{phase}-{name}-{future}.rs';p.write_text(program);meta=p.with_suffix('.rmeta');cmd=['rustc','--edition=2021','--crate-type=lib','--emit=metadata','--error-format=json',str(p),'-o',str(meta)];r=subprocess.run(cmd,capture_output=True,text=True);codes=[x['code']['code'] for line in r.stderr.splitlines() if line.startswith('{') and (x:=json.loads(line)).get('code')];rows.append(dict(name=name,future=future,exit=r.returncode,codes=codes,argv=cmd,stderr=r.stderr));meta.unlink(missing_ok=True);assert r.returncode==(1 if future and phase=='green' else 0)
  if future and phase=='green':assert codes==['E0004']
record=dict(utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),source_sha256=hashlib.sha256(source.encode()).hexdigest(),value_sha256=hashlib.sha256(value.encode()).hexdigest(),method='Exact productive arm patterns, RHS erased; enum names/arities real, payloads stub. No Cargo mutation or runtime equivalence claim.',results=rows);(out/(phase+'-probe-result.json')).write_text(json.dumps(record,indent=2)+'\n');print(phase,'two matches checked')
