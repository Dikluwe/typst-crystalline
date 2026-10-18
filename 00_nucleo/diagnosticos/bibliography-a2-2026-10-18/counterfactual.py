from pathlib import Path
import json,gzip,hashlib,subprocess,shutil,datetime,os,sys
r=Path.cwd();p=Path(__file__).resolve().parent;now=lambda:datetime.datetime.now(datetime.timezone.utc).isoformat();sha=lambda f:hashlib.sha256(Path(f).read_bytes()).hexdigest()
files=['01_core/src/compiler/layout/convergence.rs','00_nucleo/prompts/compiler/layout/convergence.md']
freeze=json.loads(gzip.decompress((p/'complete-freeze.json.gz').read_bytes()));before=json.loads(gzip.decompress((p/'before-sources.json.gz').read_bytes()))
assert all(sha(f)==h for f,h in freeze['sources'].items())
saved={f:Path(f).read_text() for f in files};(p/'candidate-state.json.gz').write_bytes(gzip.compress(json.dumps({'utc':now(),'files':saved}).encode()))
bins=p/'bin';bins.mkdir(exist_ok=True);shutil.copy2(r/'target/debug/typst',bins/'typst-candidate');candidate_sha=sha(bins/'typst-candidate')
# Preserve the first final-check attempt, including the new RED and its exact sources.
(p/'final-checks-attempt1.json.gz').write_bytes(gzip.compress(json.dumps({'utc':now(),'results':json.loads((p/'complete-results.json').read_text()),'driver':(p/'final-checks.py').read_text(),'custom_driver':(p/'custom-csl.py').read_text(),'log':(p/'complete-custom-csl.log').read_text()}).encode()))
records=[]
def run(label,cmd):
 start=now()
 with (p/(label+'.log')).open('w') as log:z=subprocess.run(cmd,stdout=log,stderr=subprocess.STDOUT)
 records.append({'label':label,'start':start,'end':now(),'command':cmd,'exit':z.returncode});(p/'counterfactual-results.json').write_text(json.dumps(records,indent=2)+'\n');print(label,z.returncode,flush=True)
 if z.returncode:raise RuntimeError(label)
try:
 for f in files:Path(f).write_text(before['files'][f])
 assert all(Path(f).read_text()==text for f,text in before['files'].items())
 run('counterfactual-build-before',['cargo','build','--offline','--locked','--workspace'])
 shutil.copy2(r/'target/debug/typst',bins/'typst-before')
 # Same probes and XML fixture, with only the Tekt executable changed and no failing aggregate assertion.
 s=(p/'custom-csl.py').read_text();s=s[:s.index("\nfor row in meta['cases']:")];s=s.replace("r/'target/debug/typst'","p/'bin/typst-before'")
 (p/'custom-before.py').write_text(s)
 run('counterfactual-probe-before',['python3',str(p/'custom-before.py'),'custom-csl-before'])
finally:
 for f,text in saved.items():Path(f).write_text(text)
 assert all(sha(f)==h for f,h in freeze['sources'].items())
 run('counterfactual-build-restored',['cargo','build','--offline','--locked','--workspace'])
 assert sha(r/'target/debug/typst')==candidate_sha,'restored binary differs from the validated candidate'
(p/'counterfactual-proof.json').write_text(json.dumps({'utc':now(),'before_code_matches_full_before_freeze':True,'restored_code_matches_full_candidate_freeze':True,'candidate_binary_sha256':candidate_sha,'before_binary_sha256':sha(bins/'typst-before'),'restored_binary_sha256':sha(r/'target/debug/typst'),'results':records},indent=2)+'\n')
