from pathlib import Path
import subprocess,json,datetime,hashlib
out=Path('00_nucleo/diagnosticos/clippy-traits-formatacao-2026-10-16');records=[]
for mode,flags in [('debug',[]),('release',['-O'])]:
 binary=out/('duration-after-probe-'+mode)
 cmd=['rustc','--edition','2021','--crate-name','duration_probe',*flags,str(out/'duration_probe.rs'),'-o',str(binary)]
 r=subprocess.run(cmd,capture_output=True,text=True);assert r.returncode==0,r.stderr
 x=subprocess.run([str(binary)],capture_output=True,text=True)
 records.append(dict(utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),mode=mode,command=cmd,build_stderr=r.stderr,exit=x.returncode,stdout=x.stdout,stderr=x.stderr,binary_sha256=hashlib.sha256(binary.read_bytes()).hexdigest()))
(out/'duration-after-probe.json').write_text(json.dumps(records,indent=2)+'\n');print(json.dumps(records),flush=True)

expected='min\tOk("-1969226660422097589487121d13h41m55.884105728s")'
assert all(expected in r['stdout'] and 'Err(' not in r['stdout'] and not r['stderr'] for r in records),records
