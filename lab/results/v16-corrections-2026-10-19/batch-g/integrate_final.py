"""Serialize scoped formatting/seals and the actual focused GREEN gates."""
from pathlib import Path
import json,subprocess,datetime
P=Path(__file__).resolve().parent;R=Path.cwd();out=P/'integration-final';out.mkdir(exist_ok=False)
r=[]
def run(name,args):
 start=datetime.datetime.now(datetime.timezone.utc).isoformat();print(start,'START',name,flush=True)
 with (out/(name+'.log')).open('w') as f: p=subprocess.run(args,stdout=f,stderr=subprocess.STDOUT)
 r.append({'name':name,'utc_start':start,'utc_end':datetime.datetime.now(datetime.timezone.utc).isoformat(),'argv':args,'exit':p.returncode});(out/'commands.json').write_text(json.dumps(r,indent=2)+'\n');print(name,p.returncode,flush=True)
 if p.returncode:print((out/(name+'.log')).read_text()[-7000:],flush=True);raise SystemExit(p.returncode)
run('fmt',['cargo','fmt','--all'])
run('reseal',['python3',str(P/'reseal.py'),'--output',str(P/'reseal-candidate-final')])
for pkg,total in [('typst-core',20),('typst-infra',32)]:run(pkg,['python3',str(P/'run_tests.py'),'--output','focused-green-final-'+pkg,'--package',pkg,'--expected-total',str(total)])
