"""Replay frozen G language and prior F policies against one exact G build."""
from pathlib import Path
import datetime,json,subprocess
P=Path(__file__).resolve().parent;F=P.parent/'figure-kind-implementation';product=Path('lab/target/product/debug/typst');build=P/'candidate-build/receipt.json'
out=P/'acceptance';out.mkdir(exist_ok=False);records=[]
def run(name,argv,expected_exit=0):
 start=datetime.datetime.now(datetime.timezone.utc).isoformat();print(start,'START',name,flush=True)
 with (out/(name+'.log')).open('w') as f:r=subprocess.run(argv,stdout=f,stderr=subprocess.STDOUT)
 records.append({'name':name,'utc_start':start,'utc_end':datetime.datetime.now(datetime.timezone.utc).isoformat(),'argv':argv,'exit':r.returncode,'expected_exit':expected_exit});(out/'commands.json').write_text(json.dumps(records,indent=2)+'\n');print(name,r.returncode,flush=True)
 if r.returncode!=expected_exit:print((out/(name+'.log')).read_text()[-7000:],flush=True);raise SystemExit(1)
run('g-equality',['python3',str(P/'agents/core/replay_equality.py'),'--product',str(product),'--build-receipt',str(build),'--out',str(P/'equality-candidate')])
r=json.loads((P/'equality-candidate/receipt.json').read_text());assert not r['required_failures']
run('g-html',['python3',str(P/'agents/html/replay_cli.py'),'--product',str(product),'--build-receipt',str(build),'--output',str(P/'html-candidate')])
run('f-policies',['python3',str(F/'replay.py'),'--product',str(product),'--build-receipt',str(build),'--output',str(P/'figure-policy-replay'),'--phase','candidate'])
run('f-references',['python3',str(F/'run_reference_acceptance.py'),'--build',str(build),'--output',str(P/'reference-corpora')])
run('f-targets',['python3',str(F/'target_controls.py'),'--product',str(product),'--build-receipt',str(build),'--output',str(P/'figure-target-controls')],expected_exit=1)
r=json.loads((P/'figure-target-controls/receipt.json').read_text());assert len(r['cases'])==11 and [c['name'] for c in r['cases'] if not c['passed']]==['header-figure-boundary']
print('PASS required G/F acceptance; known header-figure-boundary remains a strict failure.',flush=True)
