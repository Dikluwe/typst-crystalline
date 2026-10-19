from pathlib import Path
import subprocess,json,hashlib,gzip,datetime
root=Path.cwd();out=root/'00_nucleo/diagnosticos/tekt-v16-introspect-walk-2026-10-16';before=json.load(gzip.open(out/'before.json.gz'));sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest();hashes={p:sha(p) for p in before['hashes']};libs={c:max((root/'target/debug/deps').glob('lib'+c+'-*.rlib'),key=lambda p:p.stat().st_mtime_ns) for c in ['typst_core','comemo']};src=out/'workers/trace_eval.rs';binary=out/'trace-eval';cmd=['rustc','--edition=2021',str(src),'-L','dependency='+str(root/'target/debug/deps'),'-o',str(binary)]
for c,p in libs.items():cmd+=['--extern',c+'='+str(p)]
record={'utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'head':before['head'],'source_hashes':hashes,'libs':{str(p):sha(p) for p in libs.values()},'source':sha(src),'command':cmd}
with (out/'trace-build.log').open('w') as f:r=subprocess.run(cmd,stdout=f,stderr=subprocess.STDOUT)
record['build_exit']=r.returncode;assert r.returncode==0
cmd=[str(binary),str(out/'fixtures/direct-fields.typ')];record['run_command']=cmd
with (out/'trace.log').open('w') as f:r=subprocess.run(cmd,stdout=f,stderr=subprocess.STDOUT)
record.update(exit=r.returncode,binary_sha256=sha(binary),sources_unchanged=all(sha(p)==h for p,h in hashes.items()));(out/'trace-result.json').write_text(json.dumps(record,indent=2)+'\n');assert r.returncode==0 and record['sources_unchanged'];print('Raw L1 eval trace saved without root code instrumentation',flush=True)
