from pathlib import Path
import json,subprocess,datetime,gzip,hashlib
out=Path(__file__).resolve().parents[1];b=json.load(gzip.open(out/'before.json.gz'));t=json.load(open(out/'tests-before-code.json'));sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest();assert all(Path(p).read_text()==v['expected'] for p,v in t['files'].items());record={'utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'source_hashes':{p:sha(p) for p in b['hashes']},'commands':[]}
for label,cmd,expected in [('guard-control',['cargo','test','--offline','--locked','-p','typst-core','--lib','p1285_math_grapheme_e_numero_preservam_variantes_distintas','--','--test-threads=1'],0),('functional-red',['cargo','test','--offline','--locked','-p','typst-core','--lib','v16_eval_math_','--','--test-threads=1'],101),('probe',['python3',str(out/'workers/probe.py'),'baseline'],0)]:
 print(label,flush=True)
 with (out/('baseline-'+label+'.log')).open('w') as f:r=subprocess.run(cmd,stdout=f,stderr=subprocess.STDOUT)
 record['commands'].append({'label':label,'command':cmd,'exit':r.returncode});(out/'baseline-result.json').write_text(json.dumps(record,indent=2)+'\n');assert r.returncode==expected,(label,r.returncode)
 if label=='guard-control':assert '1 passed; 0 failed;' in (out/('baseline-'+label+'.log')).read_text()
 if label=='functional-red':assert '3 passed; 2 failed;' in (out/('baseline-'+label+'.log')).read_text()
record.update(functional_red=True,structural_red=True,sources_unchanged=all(sha(p)==h for p,h in record['source_hashes'].items()));assert record['sources_unchanged'];(out/'baseline-result.json').write_text(json.dumps(record,indent=2)+'\n');print('BASELINE PASS',flush=True)
