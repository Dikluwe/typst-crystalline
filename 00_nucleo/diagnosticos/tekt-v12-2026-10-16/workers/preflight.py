from pathlib import Path
import json,subprocess,hashlib,datetime
out=Path('00_nucleo/diagnosticos/tekt-v12-2026-10-16');scope=json.loads((out/'scope.json').read_text());paths=[p for top in ['01_core','02_shell','03_infra','04_wiring','benches','00_nucleo/prompts'] for p in Path(top).rglob('*') if p.is_file() and p.suffix in ['.rs','.md','.toml']]+[Path(p) for p in ['Cargo.toml','Cargo.lock','crystalline.toml','rustfmt.toml','clippy.toml','.cargo/config.toml','rust-toolchain.toml']];sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest();hashes={str(p):sha(p) for p in paths};now=lambda:datetime.datetime.now(datetime.timezone.utc).isoformat();record=dict(start=now(),source_hashes=hashes,commands=[])
commands=[('v12',['python3',str(out/'workers/gate.py'),'green']),('check',['cargo','check','--offline','--locked','--workspace','--all-targets']),('clippy',['python3',str(out/'workers/strict.py')])]
for name,cmd in commands:
 with (out/('preflight-'+name+'.log')).open('w') as f:r=subprocess.run(cmd,stdout=f,stderr=subprocess.STDOUT)
 record['commands'].append(dict(name=name,command=cmd,exit=r.returncode,utc=now()));record['sources_unchanged']=all(sha(p)==h for p,h in hashes.items());(out/'preflight-details.json').write_text(json.dumps(record,indent=2)+'\n');print(json.dumps(record['commands'][-1]),flush=True);assert record['sources_unchanged']
 if r.returncode:raise SystemExit(r.returncode)
