"""Full serial candidate gates after focused GREEN; no source repairs."""
from pathlib import Path
import datetime,json,subprocess
P=Path(__file__).resolve().parent;R=Path.cwd();out=P/'candidate-gates';out.mkdir(exist_ok=False);records=[]
def run(name,args):
 start=datetime.datetime.now(datetime.timezone.utc).isoformat();print(start,'START',name,flush=True)
 with (out/(name+'.log')).open('w') as f:r=subprocess.run(args,stdout=f,stderr=subprocess.STDOUT)
 records.append({'name':name,'utc_start':start,'utc_end':datetime.datetime.now(datetime.timezone.utc).isoformat(),'argv':args,'exit':r.returncode});(out/'commands.json').write_text(json.dumps(records,indent=2)+'\n');print(name,r.returncode,flush=True)
 if r.returncode:print((out/(name+'.log')).read_text()[-7000:],flush=True);raise SystemExit(r.returncode)
for pkg in ('typst-core','typst-infra'):
 r=json.loads((P/('focused-green-2-'+pkg)/'receipt.json').read_text());assert not r['failed'] and r['exit']==0 and not r['changed_inputs']
run('complete-lint',['python3',str(P/'lint.py'),'--output',str(P/'lint-final'),'--expected-v16','0'])
run('complete-lint-delta',['python3',str(P/'lint_delta.py')])
run('structural-evolution',['python3',str(P/'structural.py'),'green','--name','green'])
run('tuple-axes',['python3',str(P/'tuple_axes.py'),'--base','structural-green','--name','green'])
run('clean-real-products',['cargo','clean','-p','typst-core','-p','typst-shell','-p','typst-infra','-p','typst-wiring'])
run('full-validation',['python3','lab/tools/ci/validate_layout_migration.py','--output',str(P/'validation-final')])
run('fresh-build',['python3',str(P/'build.py'),'--output',str(P/'candidate-build')])
