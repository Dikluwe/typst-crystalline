from pathlib import Path
import json,gzip,hashlib,subprocess,datetime
out=Path(__file__).resolve().parents[1];b=json.load(gzip.open(out/'before.json.gz'));scope=json.load(open(out/'scope.json'));sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest();assert all(sha(p)==h for p,h in b['hashes'].items() if p not in scope['owners']);assert json.load(open(out/'l0-before-code.json'))['code_unchanged']
record={'utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'commands':[]}
for label,cmd in [('controls',['cargo','test','--offline','--locked','-p','typst-core','--lib','compiler::eval::tests::','--','--test-threads=1']),('probe',['python3',str(out/'workers/probe.py'),'baseline'])]:
 print(label,flush=True)
 with (out/('baseline-'+label+'.log')).open('w') as f:r=subprocess.run(cmd,stdout=f,stderr=subprocess.STDOUT)
 record['commands'].append({'label':label,'command':cmd,'exit':r.returncode});(out/'baseline-result.json').write_text(json.dumps(record,indent=2));assert r.returncode==0
record.update(structural_red=True,code_unchanged=all(sha(p)==h for p,h in b['hashes'].items() if p not in scope['owners']));assert record['code_unchanged'];(out/'baseline-result.json').write_text(json.dumps(record,indent=2)+'\n');print('BASELINE PASS',flush=True)
