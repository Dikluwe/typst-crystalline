"""P1332 immutable evidence, with P1331 inputs preserved."""
import importlib.util
import json
import os
from pathlib import Path
import subprocess
import sys
import time
sys.dont_write_bytecode = True
D = Path(__file__).resolve().parent
spec = importlib.util.spec_from_file_location('previous', D/'p1331-record-r2.py')
previous = importlib.util.module_from_spec(spec); spec.loader.exec_module(previous)
ROOT, sha, now = previous.ROOT, previous.sha, previous.now
OWNER, PROMPT = previous.OWNER, previous.PROMPT
BASE = '/tmp/p1331-target.rtY0la/release/typst'
VANILLA = '/usr/local/bin/typst'

def state():
    helpers=previous.previous.prior
    git=helpers.git
    return dict(utc=now(),head=git('rev-parse','HEAD').strip(),branch=git('branch','--show-current').strip(),
        status=git('status','--short','--','00_nucleo/prompts','00_nucleo/diagnosticos','01_core','02_shell','03_infra','04_wiring','Cargo.toml','Cargo.lock'),
        diff=git('diff','HEAD','--binary'),diff_stat=git('diff','HEAD','--stat'),
        staged=git('diff','--cached','--binary'),product_inventory=helpers.inventory())

def save(name, data):
    path = D/('p1332-'+name+'.json')
    assert not path.exists(), path
    payload = json.dumps(data, ensure_ascii=True, indent=2)+'\n'
    patch = '*** Begin Patch\n*** Add File: '+str(path)+'\n'+''.join('+'+x+'\n' for x in payload.splitlines())+'*** End Patch\n'
    subprocess.run(['apply_patch'],input=patch,text=True,cwd=ROOT,check=True,capture_output=True)
    print(path,sha(path),flush=True)

def verify(s):
    b=json.loads((D/'p1332-baseline.json').read_text())['state']
    assert s['head']==b['head'] and s['staged']==b['staged']
    assert {k:v for k,v in s['product_inventory'].items() if k not in (OWNER,PROMPT)}=={k:v for k,v in b['product_inventory'].items() if k not in (OWNER,PROMPT)}

def init():
    s=state(); prior=json.loads((D/'p1331-closure.json').read_text())
    assert all(s[k]==prior['state'][k] for k in ('head','diff','staged','product_inventory'))
    history={**prior['historical_preserved'],**prior['artifacts']}
    assert all(sha(p)==h for p,h in history.items())
    assert sha(BASE)==prior['binary_sha256']
    assert sha(VANILLA)=='7b4f40c56d6fa95082ebcfd893e275d418ebcaed1b97b62785f78284c63ff7b8'
    expressions=['calc.abs','repr(calc.abs)','repr(calc.abs.with(-1))','{let a=calc.abs; repr(a)}','{import calc: abs; repr(abs)}','calc.abs == calc.abs','calc.abs == calc.sqrt','{let a=calc.abs; a == calc.abs}', '{let a=calc.abs.with([x]); a()}', '{let a=calc.abs.with(2pt+3em); a()}', '{let a=calc.abs.with(-9223372036854775807-1); a()}', '{let a=calc.abs.with("x"); a()}', '{let a=arguments(true); calc.abs(..a)}','calc.abs([x])','calc.abs(-2pt)','calc.abs(-2)','calc.abs()','calc.abs(-1,bad:2)','calc.abs(-1,2)','repr(calc.sqrt)','repr(calc.pow)','{let a=calc.sqrt.with("x"); a()}', '$std.calc.abs(-1)$']
    rows=[]
    for expr in expressions:
        for role,binary in [('BASE',BASE),('VANILLA',VANILLA)]:
            argv=[binary,'--color','never','eval',expr];at=now()
            p=subprocess.run(argv,cwd=ROOT,text=True,capture_output=True,timeout=30)
            rows.append(dict(expression=expr,role=role,argv=argv,at=at,exit=p.returncode,stdout=p.stdout,stderr=p.stderr))
    assert state()['product_inventory']==s['product_inventory']
    public=dict(at=now(),head=s['head'],working_tree='uncommitted',diff_stat=s['diff_stat'],product_inventory=s['product_inventory'],rows=rows,binaries={role:dict(path=b,sha256=sha(b)) for role,b in [('BASE',BASE),('VANILLA',VANILLA)]})
    save('baseline',dict(**public,state=s,original_owner=(ROOT/OWNER).read_text(),original_prompt=(ROOT/PROMPT).read_text(),historical_preserved=history,previous_closure_sha256=sha(D/'p1331-closure.json')))
    save('baseline-public',dict(**public,baseline_sha256=sha(D/'p1332-baseline.json')))

def command(name, argv):
    before=state();verify(before);at=now();tick=time.monotonic()
    target=json.loads((D/'p1332-target.json').read_text())['target']
    p=subprocess.run(argv,cwd=ROOT,env={**os.environ,'CARGO_TARGET_DIR':target,'PYTHONDONTWRITEBYTECODE':'1'},text=True,capture_output=True,timeout=2400)
    after=state();verify(after);binary=Path(target)/'release/typst'
    save(name,dict(at=at,end=now(),seconds=time.monotonic()-tick,argv=argv,cwd=str(ROOT),target=target,before=before,after=after,exit=p.returncode,stdout=p.stdout,stderr=p.stderr,manifest_sha256=sha(D/'p1332-manifest.json'),binary=dict(path=str(binary),sha256=sha(binary)) if binary.exists() else None))
    if name.startswith('unit-red'):print('RED exit',p.returncode,'private output retained for reviewer')
    else:print(p.stdout[-1500:],p.stderr[-800:])
    sys.exit(p.returncode)

if __name__=='__main__':
    if sys.argv[1]=='init':init()
    else:command(sys.argv[1],sys.argv[2:])
