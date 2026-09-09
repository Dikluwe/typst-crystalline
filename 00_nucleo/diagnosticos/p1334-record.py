"""Immutable P1333 provenance; no reads of historical execution-step folders."""
import importlib.util
import json
import os
from pathlib import Path
import subprocess
import sys
import time
sys.dont_write_bytecode=True
D=Path(__file__).resolve().parent
spec=importlib.util.spec_from_file_location('previous',D/'p1333-record.py')
previous=importlib.util.module_from_spec(spec);spec.loader.exec_module(previous)
ROOT,sha,now,state=previous.ROOT,previous.sha,previous.now,previous.state
OWNER,PROMPT=previous.OWNER,previous.PROMPT
PAIRS=[(PROMPT,OWNER),('00_nucleo/prompts/compiler/eval/call_dispatch.md','01_core/src/compiler/eval/call_dispatch.rs'),('00_nucleo/prompts/compiler/stdlib/_comum.md','01_core/src/compiler/stdlib/mod.rs')]
ALLOWED={p for pair in PAIRS for p in pair}
BASE='/tmp/p1333-target.MDDkou/release/typst'
VANILLA='/usr/local/bin/typst'

def save(name,data):
    path=D/('p1334-'+name+'.json');assert not path.exists(),path
    payload=json.dumps(data,ensure_ascii=True,indent=2)+'\n'
    patch='*** Begin Patch\n*** Add File: '+str(path)+'\n'+''.join('+'+line+'\n' for line in payload.splitlines())+'*** End Patch\n'
    subprocess.run(['apply_patch'],input=patch,text=True,cwd=ROOT,check=True,capture_output=True)
    print(path,sha(path),flush=True)

def verify(s):
    b=json.loads((D/'p1334-baseline.json').read_text())['state']
    assert s['head']==b['head'] and s['staged']==b['staged']
    assert {k:v for k,v in s['product_inventory'].items() if k not in ALLOWED}=={k:v for k,v in b['product_inventory'].items() if k not in ALLOWED}

def init():
    s=state();prior=json.loads((D/'p1333-closure.json').read_text())
    assert all(s[k]==prior['state'][k] for k in ('head','diff','staged','product_inventory'))
    history={**prior['historical_preserved'],**prior['artifacts']}
    assert all(sha(p)==h for p,h in history.items())
    assert sha(BASE)==prior['binary_sha256']
    assert sha(VANILLA)=='7b4f40c56d6fa95082ebcfd893e275d418ebcaed1b97b62785f78284c63ff7b8'
    expressions=['calc.abs()','calc.abs(bad: 1)','calc.abs(value: -1)','calc.abs(bad: 1,value: -1)',
      'calc.abs(-1,2)','calc.abs(bad: 1,-1,2)','calc.abs(-1,2,bad: 1)','calc.abs(-1,bad: 1,2)',
      '{let f=calc.abs.with(value: -1); f(value: -2)}','{let f=calc.abs.with(bad: 1); f()}',
      '{let f=calc.abs.with(); f()}','{let f=calc.abs.with(-1,bad: 1); f(2)}',
      '{let f=calc.abs.with(-1,2); f(bad: 1)}','{let f=calc.abs.with(value: -1).with(bad: 2); f()}',
      '{let a=arguments(bad: 1,value: -1); calc.abs(..a)}','calc.abs(..(value: -1))',
      'calc.abs(..())','calc.abs(..none)','calc.abs(..(-1,2))',
      '{import calc: abs; abs()}','{let café=calc.abs;\n café(value: -1)}',
      '$std.calc.abs()$','{let f=calc.abs; $f()$}','{let f=calc.abs.with(); $f()$}',
      '{import calc: abs; $abs()$}','calc.abs(false,2)','calc.abs(value: 2,false)',
      'calc.abs(-1,bad:panic("later"))','{let abs(..a)=a; abs()}','calc.sqrt()',
      'json.encode()','panic()','csv()','calc.abs(-1)','calc.abs(-2pt)']
    rows=[]
    for expr in expressions:
        for role,b in [('BASE',BASE),('VANILLA',VANILLA)]:
            argv=[b,'--color','never','eval',expr];at=now()
            p=subprocess.run(argv,cwd=ROOT,text=True,capture_output=True,timeout=30)
            rows.append(dict(expression=expr,role=role,argv=argv,at=at,exit=p.returncode,stdout=p.stdout,stderr=p.stderr))
    assert state()['product_inventory']==s['product_inventory']
    public=dict(at=now(),head=s['head'],working_tree='uncommitted',diff_stat=s['diff_stat'],product_inventory=s['product_inventory'],rows=rows,binaries={role:dict(path=b,sha256=sha(b)) for role,b in [('BASE',BASE),('VANILLA',VANILLA)]})
    save('baseline',dict(**public,state=s,original_owner=(ROOT/OWNER).read_text(),original_prompt=(ROOT/PROMPT).read_text(),original_files={p:(ROOT/p).read_text() for p in ALLOWED},historical_preserved=history,previous_closure_sha256=sha(D/'p1333-closure.json')))
    save('baseline-public',dict(**public,baseline_sha256=sha(D/'p1334-baseline.json')))

def command(name,argv):
    before=state();verify(before);at=now();tick=time.monotonic()
    target=json.loads((D/'p1334-target.json').read_text())['target']
    p=subprocess.run(argv,cwd=ROOT,env={**os.environ,'CARGO_TARGET_DIR':target,'PYTHONDONTWRITEBYTECODE':'1'},text=True,capture_output=True,timeout=2400)
    after=state();verify(after);binary=Path(target)/'release/typst'
    save(name,dict(at=at,end=now(),seconds=time.monotonic()-tick,argv=argv,cwd=str(ROOT),target=target,before=before,after=after,exit=p.returncode,stdout=p.stdout,stderr=p.stderr,manifest_sha256=sha(D/'p1334-manifest.json'),binary=dict(path=str(binary),sha256=sha(binary)) if binary.exists() else None))
    if name.startswith('unit-red'):print('RED exit',p.returncode,'private output retained for reviewer')
    else:print(p.stdout[-1500:],p.stderr[-800:])
    sys.exit(p.returncode)

if __name__=='__main__':
    if sys.argv[1]=='init':init()
    else:command(sys.argv[1],sys.argv[2:])
