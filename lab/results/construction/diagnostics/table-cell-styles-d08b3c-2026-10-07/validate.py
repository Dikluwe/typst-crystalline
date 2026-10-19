from pathlib import Path
import subprocess,datetime,json,hashlib,os
D=Path(__file__).parent;results=[]
for name,cmd in [('build-debug',['cargo','build']),('build-release',['cargo','build','--release']),('workspace',['cargo','test','--workspace','--lib']),('cli-query',['cargo','test','-p','typst-wiring','--test','cli','query'])]:
 start=datetime.datetime.now(datetime.timezone.utc).isoformat()
 with (D/(name+'.log')).open('w') as f:r=subprocess.run(cmd,stdout=f,stderr=subprocess.STDOUT)
 results.append({'name':name,'command':cmd,'status':r.returncode,'start_utc':start,'end_utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'snapshot':'tested-final.json.gz','cwd':str(Path.cwd()),'environment_overrides':{},'compiler_env':{k:os.environ.get(k) for k in ['RUSTFLAGS','CARGO_BUILD_TARGET','TYPST_COMMIT_SHA','CARGO_PROFILE_RELEASE_LTO','CARGO_PROFILE_RELEASE_CODEGEN_UNITS']}})
 (D/'validation.json').write_text(json.dumps(results,indent=2)+'\n');print(name,r.returncode,flush=True)
 if r.returncode:raise SystemExit(r.returncode)
