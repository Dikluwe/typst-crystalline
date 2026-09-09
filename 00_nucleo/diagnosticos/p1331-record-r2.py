"""P1331 provenance and immutable command receipts (no product writes)."""
import importlib.util
import json
import os
from pathlib import Path
import subprocess
import sys
import time

sys.dont_write_bytecode = True
D = Path(__file__).resolve().parent
spec = importlib.util.spec_from_file_location('previous', D / 'p1329-record.py')
previous = importlib.util.module_from_spec(spec)
spec.loader.exec_module(previous)
ROOT, sha, now, state = previous.ROOT, previous.sha, previous.now, previous.state
OWNER, PROMPT = previous.OWNER, previous.PROMPT
BASE = '/tmp/p1330-target.f0lmDu/release/typst'
VANILLA = '/usr/local/bin/typst'
TARGET_FILE = D / 'p1331-target.json'

def save(name, data):
    path = D / ('p1331-' + name + '.json')
    assert not path.exists(), path
    payload = json.dumps(data, ensure_ascii=True, indent=2) + '\n'
    patch = '*** Begin Patch\n*** Add File: ' + str(path) + '\n' + ''.join('+' + line + '\n' for line in payload.splitlines()) + '*** End Patch\n'
    subprocess.run(['apply_patch'], input=patch, text=True, cwd=ROOT, check=True, capture_output=True)
    print(path, sha(path), flush=True)

def verify(s):
    b = json.loads((D / 'p1331-baseline.json').read_text())['state']
    assert s['head'] == b['head'] and s['staged'] == b['staged']
    assert {k:v for k,v in s['product_inventory'].items() if k not in (OWNER,PROMPT)} == {k:v for k,v in b['product_inventory'].items() if k not in (OWNER,PROMPT)}

def init():
    s = state()
    prior = json.loads((D / 'p1330-closure.json').read_text())
    assert all(s[k] == prior['state'][k] for k in ('head','diff','staged','product_inventory'))
    historical = {**prior['historical_preserved'], **prior['artifacts']}
    assert all(sha(p) == h for p,h in historical.items())
    assert sha(BASE) == '6f1db621bc0b2a7fe4fc9d05925fb96636f33232b8527b83c040b793970fbda0'
    assert sha(VANILLA) == previous.prior.VANILLA
    expressions = ['calc.abs("x")', 'calc.abs(sym.alpha)', 'calc.abs(true)', 'calc.abs(false)', 'calc.abs(none)', 'calc.abs(auto)', 'calc.abs(())', 'calc.abs((:))', 'calc.abs(calc)', 'calc.abs((x => x))', 'calc.abs(str)', 'calc.abs(regex("x"))', 'calc.abs(rgb("#123456"))', 'calc.abs(1pt + 2%)', 'calc.abs(1pt + red)', 'calc.abs(left)', 'calc.abs(ltr)', 'calc.abs(bytes((1,2)))', 'calc.abs(duration(seconds: 2))', 'calc.abs(datetime(year: 2024, month: 1, day: 1))', 'calc.abs(version(1,2,3))', 'calc.abs(selector(heading))', 'calc.abs(arguments(1))', 'calc.abs(state("x", 1))', 'calc.abs(counter("x"))', 'calc.abs(<x>)', 'calc.abs(gradient.linear(red,blue))', 'calc.abs(tiling(size: (1pt,1pt))[])', 'calc.abs(path())', '{let a=calc.abs; a("x")}', '{let a=calc.abs.with("x"); a()}', 'calc.abs(..("x",))', '{let a=arguments(sym.alpha); calc.abs(..a)}', 'calc.abs("x", bad: 1)', 'calc.abs("x", 1)', 'calc.abs([x])', 'calc.abs(-9223372036854775807 - 1)', 'calc.abs(-2pt)', 'calc.abs(2pt + 3em)', 'calc.abs(-2fr)', 'calc.abs(-3.5)', 'calc.abs(decimal("-3.5"))', 'calc.abs()', 'calc.sqrt("x")']
    rows = []
    for expr in expressions:
        for role,binary in [('baseline',BASE),('vanilla',VANILLA)]:
            argv = [binary,'--color','never','eval',expr]
            at = now()
            p = subprocess.run(argv,cwd=ROOT,text=True,capture_output=True,timeout=30)
            rows.append(dict(expression=expr,role=role,argv=argv,at=at,exit=p.returncode,stdout=p.stdout,stderr=p.stderr))
    assert state()['product_inventory'] == s['product_inventory']
    save('baseline',dict(at=now(),state=s,rows=rows,original_owner=(ROOT/OWNER).read_text(),original_prompt=(ROOT/PROMPT).read_text(),historical_preserved=historical,previous_closure_sha256=sha(D/'p1330-closure.json'),binaries={role:dict(path=b,sha256=sha(b)) for role,b in [('baseline',BASE),('vanilla',VANILLA)]}))
    save('baseline-public',dict(at=now(),head=s['head'],working_tree='uncommitted',product_inventory=s['product_inventory'],diff_stat=s['diff_stat'],rows=rows,binaries={role:dict(path=b,sha256=sha(b)) for role,b in [('baseline',BASE),('vanilla',VANILLA)]},baseline_sha256=sha(D/'p1331-baseline.json')))

def command(name, argv):
    before = state(); verify(before); at = now(); tick = time.monotonic()
    target = json.loads(TARGET_FILE.read_text())['target'] if TARGET_FILE.exists() else '/tmp/p1331-not-configured'
    p = subprocess.run(argv,cwd=ROOT,env={**os.environ,'CARGO_TARGET_DIR':target,'PYTHONDONTWRITEBYTECODE':'1'},text=True,capture_output=True,timeout=2400)
    after = state(); verify(after)
    binary = Path(target)/'release/typst'
    save(name,dict(at=at,end=now(),seconds=time.monotonic()-tick,argv=argv,cwd=str(ROOT),target=target,before=before,after=after,exit=p.returncode,stdout=p.stdout,stderr=p.stderr,manifest_sha256=sha(D/'p1331-manifest-r2.json') if (D/'p1331-manifest-r2.json').exists() else None,binary=dict(path=str(binary),sha256=sha(binary)) if binary.exists() else None))
    if name.startswith('unit-red'): print('RED exit',p.returncode,'private output retained for reviewer')
    else: print(p.stdout[-1500:],p.stderr[-800:])
    sys.exit(p.returncode)

if __name__ == '__main__':
    if sys.argv[1] == 'init': init()
    else: command(sys.argv[1],sys.argv[2:])
