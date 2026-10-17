from pathlib import Path
import json,subprocess,hashlib,datetime,sys
out=Path('00_nucleo/diagnosticos/clippy-enums-publicos-2026-10-16');phase=sys.argv[1];sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest();rlib=Path('target/debug/libtypst_core.rlib');start=datetime.datetime.now(datetime.timezone.utc).isoformat()
cmd=['rustc','--edition','2021',str(out/'budget.rs'),'-L','dependency=target/debug/deps','--extern','typst_core='+str(rlib),'-o',str(out/'budget')];r=subprocess.run(cmd,capture_output=True,text=True);assert r.returncode==0,r.stderr
samples=[]
for label in ['tag','show']:
 r=subprocess.run([str(out/'budget'),label],capture_output=True,text=True);samples.append(dict(label=label,exit=r.returncode,stdout=r.stdout,stderr=r.stderr));assert r.returncode==(101 if phase=='red' else 0)
record=dict(start=start,end=datetime.datetime.now(datetime.timezone.utc).isoformat(),command=cmd,rlib_sha256=sha(rlib),source_sha256=sha(out/'budget.rs'),samples=samples)
if phase=='red':assert record['rlib_sha256']==json.loads((out/'probe-details.json').read_text())['inputs'][str(rlib)]
(out/('budget-'+phase+'.json')).write_text(json.dumps(record,indent=2)+'\n');print(json.dumps(record),flush=True)
