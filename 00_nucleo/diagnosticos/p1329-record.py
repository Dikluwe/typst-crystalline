"""P1329 immutable execution evidence; preserve all unrelated product inputs."""
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
TARGET = '/tmp/p1329-target.bg3p5A'
BASE = '/tmp/p1328-target.T7Tg57/release/typst'
OWNER = '01_core/src/compiler/stdlib/calc.rs'
PROMPT = '00_nucleo/prompts/compiler/stdlib/calc.md'

def save(name, data):
    path = D / ('p1329-' + name + '.json')
    assert not path.exists(), path
    payload = json.dumps(data, ensure_ascii=True, indent=2) + '\n'
    patch = '*** Begin Patch\n*** Add File: ' + str(path) + '\n' + ''.join('+' + x + '\n' for x in payload.splitlines()) + '*** End Patch\n'
    subprocess.run(['apply_patch'], input=patch, text=True, check=True, capture_output=True, cwd=ROOT)
    print(path, sha(path), flush=True)

def verify(s):
    b = json.loads((D / 'p1329-baseline.json').read_text())['state']
    assert s['head'] == b['head'] and s['staged'] == b['staged']
    assert {k:v for k,v in s['product_inventory'].items() if k not in (OWNER,PROMPT)} == {k:v for k,v in b['product_inventory'].items() if k not in (OWNER,PROMPT)}

def init():
    s = state()
    old = json.loads((D / 'p1328-closure.json').read_text())
    assert all(s[k] == old['state'][k] for k in ('head','diff','staged','product_inventory'))
    assert all(sha(path) == h for path,h in {**old['historical_preserved'], **old['artifacts']}.items())
    assert sha(BASE) == '94c3d8cec16dc98784757227f554b2851fad186a9e76605272bdaab0e6f925f9'
    assert sha('/usr/local/bin/typst') == prior.VANILLA
    expressions = ["calc.abs(-2pt)","calc.abs(-2em)","calc.abs(-2cm)","calc.abs(-2mm)","calc.abs(-2in)","calc.abs(-2deg)","calc.abs(-2rad)","calc.abs(-2%)","calc.abs(-2fr)","calc.abs(0pt)","calc.abs(-0em)","calc.abs(2pt + 3em)","calc.abs(-2pt - 3em)","calc.abs(2pt - 3em)","calc.abs(0pt - 3em)","calc.abs(-2pt + 0em)","{let ab=calc.abs; ab(-2em)}","{let ab=calc.abs.with(-2em); ab()}","calc.abs(..(-2%,))","{let aa=arguments(2pt + 3em); calc.abs(..aa)}","{let ab=calc.abs.with(2pt + 3em); ab()}","calc.abs(-calc.inf * 1deg)","calc.abs(-calc.inf * 1fr)","calc.abs(-calc.inf * 1%)","calc.abs((0.0/0.0) * 1deg)","calc.abs((0.0/0.0) * 1fr)","calc.abs((0.0/0.0) * 1%)","calc.abs(-calc.inf * 1pt)","calc.abs((0.0/0.0) * 1pt)","calc.abs(-3)","calc.abs(-3.5)","calc.abs(decimal(\"-3.5\"))","calc.abs(-9223372036854775807 - 1)","calc.abs([x])","$std.calc.abs(-1)$","calc.abs(\"x\")","calc.abs(sym.alpha)","calc.abs()","calc.abs(-2pt, 1)","calc.abs(-2pt, bad: 1)","calc.sqrt(-2pt)"]
    rows=[]
    for expression in expressions:
        for role,binary in [('baseline',BASE),('vanilla','/usr/local/bin/typst')]:
            argv=[binary,'--color','never','eval',expression]
            at=now(); p=subprocess.run(argv,cwd=ROOT,capture_output=True,text=True,timeout=30)
            rows.append(dict(expression=expression,role=role,argv=argv,at=at,exit=p.returncode,stdout=p.stdout,stderr=p.stderr))
    assert state()['product_inventory'] == s['product_inventory']
    save('baseline',dict(at=now(),state=s,rows=rows,binaries={role:dict(path=b,sha256=sha(b)) for role,b in [('baseline',BASE),('vanilla','/usr/local/bin/typst')]}, original_owner=(ROOT/OWNER).read_text(),original_prompt=(ROOT/PROMPT).read_text(),target=TARGET,target_reason='Dedicated /tmp copy without hardlinks: host /dev/shm has 3.3G free versus 3.8G baseline target; prior targets preserved.',previous_closure_sha256=sha(D/'p1328-closure.json')))

def command(name, argv):
    before=state(); verify(before); at=now(); tick=time.monotonic()
    p=subprocess.run(argv,cwd=ROOT,env={**os.environ,'CARGO_TARGET_DIR':TARGET,'PYTHONDONTWRITEBYTECODE':'1'},capture_output=True,text=True,timeout=2400)
    after=state(); verify(after)
    binary=Path(TARGET)/'release/typst'
    save(name,dict(at=at,end=now(),seconds=time.monotonic()-tick,argv=argv,cwd=str(ROOT),target=TARGET,before=before,after=after,exit=p.returncode,stdout=p.stdout,stderr=p.stderr,manifest_sha256=sha(D/'p1329-manifest-r2.json') if (D/'p1329-manifest-r2.json').exists() else None,binary=dict(path=str(binary),sha256=sha(binary)) if binary.exists() else None))
    if name.startswith('unit-red'): print('RED exit',p.returncode,'private assertion output retained for reviewer')
    else: print(p.stdout[-1800:],p.stderr[-800:])
    sys.exit(p.returncode)

if __name__ == '__main__':
    if sys.argv[1]=='init': init()
    else: command(sys.argv[1],sys.argv[2:])
