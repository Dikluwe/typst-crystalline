from pathlib import Path
import json,subprocess,sys,datetime,hashlib
out=Path('00_nucleo/diagnosticos/clippy-default-2026-10-16');records=[]
sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest()
hashes={str(p):sha(p) for top in ['01_core','02_shell','03_infra','04_wiring','benches','00_nucleo/prompts'] for p in Path(top).rglob('*') if p.is_file() and p.suffix in ['.rs','.md','.toml']}
for label,cmd in [('core',['cargo','test','--offline','--locked','-p','typst-core','--lib','default_contract','--','--test-threads=1']),('infra',['cargo','test','--offline','--locked','-p','typst-infra','--lib','default_contract','--','--test-threads=1']),('clippy',['cargo','clippy','--offline','--locked','--workspace','--all-targets','--','-A','warnings','-D','clippy::new_without_default','-D','clippy::field_reassign_with_default','-D','clippy::type_complexity','-D','unfulfilled_lint_expectations'])]:
 with (out/('green-'+label+'.log')).open('w') as f:r=subprocess.run(cmd,stdout=f,stderr=subprocess.STDOUT)
 records.append(dict(label=label,command=cmd,exit=r.returncode));(out/'green-details.json').write_text(json.dumps(dict(utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),commands=records,sources_unchanged=all(sha(p)==h for p,h in hashes.items())),indent=2)+'\n')
 assert all(sha(p)==h for p,h in hashes.items())
 if r.returncode:sys.exit(r.returncode)
print(json.dumps(records),flush=True)
