from pathlib import Path
import json,gzip,hashlib,subprocess,datetime
p=Path(__file__).resolve().parent
now=lambda:datetime.datetime.now(datetime.timezone.utc).isoformat()
def sha(f):
 with Path(f).open('rb') as s:return hashlib.file_digest(s,'sha256').hexdigest()
freeze=json.loads(gzip.decompress((p/'freeze.json.gz').read_bytes()));assert all(sha(f)==h for f,h in freeze['sources'].items())
measured=json.loads(gzip.decompress((p/'after.json.gz').read_bytes()))['binaries']['tekt']['sha256'];before=sha('target/debug/typst');record={'start':now(),'binary_after_targeted_retry':before,'measured_workspace_binary_sha256':measured,'rebuilt_workspace':before!=measured}
if before!=measured:
 cmd=['cargo','build','--offline','--locked','--workspace'];record['command']=cmd
 with (p/'final-build-command.log').open('w') as f:z=subprocess.run(cmd,stdout=f,stderr=subprocess.STDOUT)
 record['exit']=z.returncode;assert z.returncode==0
record['final_binary_sha256']=sha('target/debug/typst');record['end']=now();record['measurement_identity_verified']=record['final_binary_sha256']==measured
(p/'final-build-result.json').write_text(json.dumps(record,indent=2)+'\n')
assert record['measurement_identity_verified'],record
assert all(sha(f)==h for f,h in freeze['sources'].items())
f=p/'summary.json';summary=json.loads(f.read_text());(p/'summary-before-final-build.json').write_text(json.dumps(summary,indent=2)+'\n');summary['binary_sha256']=record['final_binary_sha256'];summary['final_build_result']='final-build-result.json';f.write_text(json.dumps(summary,indent=2)+'\n')
print(json.dumps(record))
