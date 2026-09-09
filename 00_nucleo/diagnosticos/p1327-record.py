"""P1326 immutable execution evidence; preserve all unrelated product inputs."""
import importlib.util
import json
import os
from pathlib import Path
import subprocess
import sys
import time
sys.dont_write_bytecode = True
D = Path(__file__).resolve().parent
spec = importlib.util.spec_from_file_location('prior', D / 'p1322-record.py')
prior = importlib.util.module_from_spec(spec)
spec.loader.exec_module(prior)
ROOT, sha, now, state = prior.ROOT, prior.sha, prior.now, prior.state
TARGET = '/tmp/p1327-target.k9Mq0s'
BASE = '/tmp/p1326-target.6Vi4Km/release/typst'
OWNER = '01_core/src/compiler/eval/modules.rs'
PROMPT = '00_nucleo/prompts/compiler/eval/modules.md'
PAIRS = (OWNER, PROMPT, '01_core/src/compiler/eval/tests.rs', '00_nucleo/prompts/compiler/eval/tests.md')

def save(name, data):
    path = D / ('p1327-' + name + '.json')
    assert not path.exists(), path
    payload = json.dumps(data, ensure_ascii=True, indent=2) + '\n'
    patch = '*** Begin Patch\n*** Add File: ' + str(path) + '\n' + ''.join('+' + x + '\n' for x in payload.splitlines()) + '*** End Patch\n'
    subprocess.run(['apply_patch'], input=patch, text=True, check=True, capture_output=True, cwd=ROOT)
    print(path, sha(path), flush=True)

def verify(s):
    b = json.loads((D / 'p1327-baseline.json').read_text())['state']
    assert s['head'] == b['head'] and s['staged'] == b['staged']
    assert {k:v for k,v in s['product_inventory'].items() if k not in PAIRS} == {k:v for k,v in b['product_inventory'].items() if k not in PAIRS}

def init():
    s = state()
    old = json.loads((D / 'p1326-closure.json').read_text())
    assert all(s[k] == old['state'][k] for k in ('head','diff','staged','product_inventory'))
    assert sha(BASE) == 'bd86b34602323a390a60a8f41b97b180f93077affcd30b6b1e85aa5926dee811'
    assert sha('/usr/local/bin/typst') == prior.VANILLA
    expressions = ['{ import std; (repr(std), std.calc.abs(-7)) }', '{ let named = std; import named; (repr(named), named.calc.abs(-8)) }', '{ import calc; calc.abs(-9) }', '{ import std as s; s.calc.abs(-7) }', '{ import std as std; std.calc.abs(-7) }', '{ import std: calc; calc.abs(-7) }', '{ import calc: *; abs(-7) }', '{ import std.calc; calc.abs(-7) }', '{ import (std); 1 }', '{ import (std) as s; s.calc.abs(-7) }', '{ let x=1; import x; 1 }', '{ import missing; 1 }', '{ import std; std.nope }', '{ import std; import std; repr(std) }', '{ let f() = { import calc; calc.abs(-4) }; f() }']
    rows=[]
    for expression in expressions:
        for role,binary in [('baseline',BASE),('vanilla','/usr/local/bin/typst')]:
            argv=[binary,'--color','never','eval',expression]
            at=now(); p=subprocess.run(argv,cwd=ROOT,capture_output=True,text=True,timeout=30)
            rows.append(dict(expression=expression,role=role,argv=argv,at=at,exit=p.returncode,stdout=p.stdout,stderr=p.stderr))
    assert state()['product_inventory'] == s['product_inventory']
    save('baseline',dict(at=now(),state=s,rows=rows,binaries={role:dict(path=b,sha256=sha(b)) for role,b in [('baseline',BASE),('vanilla','/usr/local/bin/typst')]}, original_owner=(ROOT/OWNER).read_text(),original_prompt=(ROOT/PROMPT).read_text(),originals={p:(ROOT/p).read_text() for p in PAIRS},target=TARGET,target_reason='Dedicated /tmp copy without hardlinks; prior RAM host/sandbox free-space discrepancy, no new RAM copy attempted.',previous_closure_sha256=sha(D/'p1326-closure.json')))

def command(name, argv):
    before=state(); verify(before); at=now(); tick=time.monotonic()
    p=subprocess.run(argv,cwd=ROOT,env={**os.environ,'CARGO_TARGET_DIR':TARGET,'PYTHONDONTWRITEBYTECODE':'1'},capture_output=True,text=True,timeout=2400)
    after=state(); verify(after)
    binary=Path(TARGET)/'release/typst'
    save(name,dict(at=at,end=now(),seconds=time.monotonic()-tick,argv=argv,cwd=str(ROOT),target=TARGET,before=before,after=after,exit=p.returncode,stdout=p.stdout,stderr=p.stderr,manifest_sha256=sha(D/'p1327-manifest.json') if (D/'p1327-manifest.json').exists() else None,binary=dict(path=str(binary),sha256=sha(binary)) if binary.exists() else None))
    if name.startswith('unit-red'): print('RED exit',p.returncode,'private assertion output retained for reviewer')
    else: print(p.stdout[-1800:],p.stderr[-800:])
    sys.exit(p.returncode)

if __name__ == '__main__':
    if sys.argv[1]=='init': init()
    else: command(sys.argv[1],sys.argv[2:])
