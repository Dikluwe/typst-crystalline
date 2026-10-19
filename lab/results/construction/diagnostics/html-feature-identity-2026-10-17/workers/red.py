from pathlib import Path
import subprocess,json,datetime,sys
out=Path(__file__).resolve().parents[1];cmd=['cargo','test','--offline','--locked','--workspace','builtin_module_identity_','--','--test-threads=1','--nocapture']
with (out/'red-test-output.log').open('w') as f:r=subprocess.run(cmd,stdout=f,stderr=subprocess.STDOUT)
s=(out/'red-test-output.log').read_text();result=dict(utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),command=cmd,exit=r.returncode,functional_red='test result: FAILED.' in s,compiler_error='error[E' in s);(out/'red-result.json').write_text(json.dumps(result,indent=2)+'\n');print(s[-8500:]);assert result['functional_red'] and not result['compiler_error'];sys.exit(r.returncode)
