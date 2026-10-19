from pathlib import Path
import json,subprocess,gzip,hashlib,datetime
out=Path('00_nucleo/diagnosticos/tekt-v16-introspect-walk-2026-10-16');scope=json.load(open(out/'scope.json'));commands=[('fmt',['cargo','fmt','--all'],0),('seal',['python3',str(out/'workers/seal_owned.py'),*scope['sources']],0),('red',['cargo','test','--offline','--locked','-p','typst-core','--lib','v16_walk_','--','--test-threads=1'],101),('architecture',[json.load(gzip.open(out/'before.json.gz'))['linter']['binary'],'.','--format','sarif'],1),('probe',['python3',str(out/'workers/probe.py'),'baseline'],0)]
record={'start':datetime.datetime.now(datetime.timezone.utc).isoformat(),'commands':[]}
for label,cmd,expected in commands:
 print(label,flush=True)
 with (out/('baseline-'+label+'.log')).open('w') as f:r=subprocess.run(cmd,stdout=f,stderr=subprocess.STDOUT)
 record['commands'].append({'label':label,'command':cmd,'exit':r.returncode});(out/'baseline-result.json').write_text(json.dumps(record,indent=2)+'\n');assert r.returncode==expected,(label,r.returncode)
 if label=='red':assert '1 passed; 7 failed;' in (out/'baseline-red.log').read_text()
 if label=='architecture':
  import collections
  c=collections.Counter(x['ruleId']+':'+x['level'] for run in json.load(open(out/'baseline-architecture.log'))['runs'] for x in run['results']);assert c==scope['initial_counts'],c
with gzip.open(out/'tests-and-l0.json.gz','wt') as f:json.dump({p:Path(p).read_text() for p in scope['sources']+scope['owners']},f)
print('BASELINE PASS: functional RED and sentinel measured',flush=True)
