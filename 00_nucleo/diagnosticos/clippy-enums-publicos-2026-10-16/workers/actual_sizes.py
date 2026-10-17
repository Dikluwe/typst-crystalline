from pathlib import Path
import json,subprocess,hashlib,datetime
out=Path('00_nucleo/diagnosticos/clippy-enums-publicos-2026-10-16');assert json.loads((out/'validation-result.json').read_text())['exit']==0
sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest();rlib=Path('target/debug/libtypst_core.rlib');start=datetime.datetime.now(datetime.timezone.utc).isoformat();cmd=['rustc','--edition','2021',str(out/'actual_sizes.rs'),'-L','dependency=target/debug/deps','--extern','typst_core='+str(rlib),'-o',str(out/'actual-sizes')]
r=subprocess.run(cmd,capture_output=True,text=True);assert r.returncode==0,r.stderr;r=subprocess.run([str(out/'actual-sizes')],capture_output=True,text=True);assert r.returncode==0,r.stderr;sizes=json.loads(r.stdout);assert sizes==dict(tag=48,element_info=400,transformation=120,show_rule=168,styles=328),sizes
record=dict(start=start,end=datetime.datetime.now(datetime.timezone.utc).isoformat(),command=cmd,source_sha256=sha(out/'actual_sizes.rs'),rlib_sha256=sha(rlib),binary_sha256=sha(out/'actual-sizes'),sizes=sizes)
(out/'actual-sizes.json').write_text(json.dumps(record,indent=2)+'\n');print(json.dumps(record),flush=True)
