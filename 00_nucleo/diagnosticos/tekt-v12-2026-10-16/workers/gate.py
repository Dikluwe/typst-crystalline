from pathlib import Path
import subprocess,json,sys,hashlib,datetime
out=Path('00_nucleo/diagnosticos/tekt-v12-2026-10-16');phase=sys.argv[1];assert phase in ['baseline','green'];cmd=['/tmp/tekt-linter-ci-target/debug/crystalline-lint','.','--checks','v12','--fail-on','warning','--format','sarif']
assert hashlib.sha256(Path(cmd[0]).read_bytes()).hexdigest()=='26fe38b564a0aa92e7cc49f3b09a4324d1127e3586862afa5af4480e6c394dde'
with (out/f'gate-{phase}.json').open('w') as f:r=subprocess.run(cmd,stdout=f,stderr=subprocess.STDOUT)
x=json.loads((out/f'gate-{phase}.json').read_text())['runs'][0]['results'];warnings=[t for t in x if t['level']=='warning'];assert not any(t['level']=='error' for t in x);expected=19 if phase=='baseline' else 0;assert len(warnings)==expected
record=dict(utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),phase=phase,command=cmd,exit=r.returncode,warnings=len(warnings));(out/f'gate-{phase}-result.json').write_text(json.dumps(record,indent=2)+'\n');print(json.dumps(record));assert r.returncode==(1 if phase=='baseline' else 0)
