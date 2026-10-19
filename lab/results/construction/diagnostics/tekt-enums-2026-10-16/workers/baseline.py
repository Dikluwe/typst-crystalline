from pathlib import Path
import json,gzip,hashlib,subprocess,datetime
out=Path('00_nucleo/diagnosticos/tekt-enums-2026-10-16');before=json.load(gzip.open(out/'before.json.gz'));scope=json.loads((out/'scope.json').read_text());sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest();hashes={p:sha(p) for p in before['hashes']};now=lambda:datetime.datetime.now(datetime.timezone.utc).isoformat();record=dict(start=now(),head=before['head'],status=subprocess.check_output(['git','status','--porcelain=v1'],text=True),diffstat=subprocess.check_output(['git','diff','HEAD','--stat'],text=True),source_hashes=hashes,commands=[])
commands=[('entities',['cargo','test','--offline','--locked','-p','typst-core','--lib','entities::','--','--test-threads=1']),('red',['/tmp/tekt-linter-ci-target/debug/crystalline-lint','.','--checks','v16','--format','sarif','--fail-on','warning']),('probe',['python3',str(out/'workers/probe.py'),'baseline'])]
for label,cmd in commands:
 start=now()
 with (out/('baseline-'+label+'.log')).open('w') as f:r=subprocess.run(cmd,stdout=f,stderr=subprocess.STDOUT)
 record['commands'].append(dict(label=label,command=cmd,start=start,end=now(),exit=r.returncode));record['sources_unchanged']=all(sha(p)==h for p,h in hashes.items());record['end']=now();(out/'baseline-details.json').write_text(json.dumps(record,indent=2)+'\n');print(json.dumps(record['commands'][-1]),flush=True);assert record['sources_unchanged']
 if label=='red':
  results=json.loads((out/'baseline-red.log').read_text())['runs'][0]['results'];selected=[x for x in results if x['level']=='warning' and x['locations'][0]['physicalLocation']['artifactLocation']['uri'].removeprefix('./') in scope['sources']];assert len(selected)==11 and all(x['ruleId']=='V16' for x in selected) and r.returncode!=0
 else:assert r.returncode==0
