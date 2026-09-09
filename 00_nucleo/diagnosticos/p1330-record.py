"""P1330 provenance and immutable command receipts (no product writes)."""
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
BASE = '/tmp/p1329-target.bg3p5A/release/typst'
VANILLA = '/usr/local/bin/typst'
TARGET_FILE = D / 'p1330-target.json'

def save(name, data):
    path = D / ('p1330-' + name + '.json')
    assert not path.exists(), path
    payload = json.dumps(data, ensure_ascii=True, indent=2) + '\n'
    patch = '*** Begin Patch\n*** Add File: ' + str(path) + '\n' + ''.join('+' + line + '\n' for line in payload.splitlines()) + '*** End Patch\n'
    subprocess.run(['apply_patch'], input=patch, text=True, cwd=ROOT, check=True, capture_output=True)
    print(path, sha(path), flush=True)

def verify(s):
    b = json.loads((D / 'p1330-baseline.json').read_text())['state']
    assert s['head'] == b['head'] and s['staged'] == b['staged']
    assert {k:v for k,v in s['product_inventory'].items() if k not in (OWNER,PROMPT)} == {k:v for k,v in b['product_inventory'].items() if k not in (OWNER,PROMPT)}

def init():
    s = state()
    prior = json.loads((D / 'p1329-closure.json').read_text())
    assert all(s[k] == prior['state'][k] for k in ('head','diff','staged','product_inventory'))
    historical = {**prior['historical_preserved'], **prior['artifacts']}
    assert all(sha(p) == h for p,h in historical.items())
    assert sha(BASE) == '9f347f742a5cdb5c4c36a4af985b4ff122ac1bb118a1e018b960e7f5d105c2ec'
    assert sha(VANILLA) == previous.prior.VANILLA
    expressions = [
        'calc.abs(-9223372036854775807 - 1)',
        'calc.abs(-9223372036854775808)',
        'calc.abs(-9223372036854775807)',
        'calc.abs(9223372036854775807)',
        '{let a=calc.abs; a(-9223372036854775807 - 1)}',
        '{let a=calc.abs.with(-9223372036854775807 - 1); a()}',
        'calc.abs(..(-9223372036854775807 - 1,))',
        '{let a=arguments(-9223372036854775807 - 1); calc.abs(..a)}',
        'calc.abs(-9223372036854775807 - 1, bad: 1)',
        'calc.abs(-9223372036854775807 - 1, 2)',
        'calc.abs(0)', 'calc.abs(-1)', 'calc.abs(-3.5)',
        'calc.abs(decimal("-9223372036854775808"))',
        'calc.abs(-2pt)', 'calc.abs(-2em)', 'calc.abs(-2deg)',
        'calc.abs(-2%)', 'calc.abs(-2fr)', 'calc.abs(2pt + 3em)',
        'calc.abs([x])', 'calc.abs("x")', 'calc.abs()',
        '$std.calc.abs(-1)$', 'calc.abs(-calc.inf)']
    rows = []
    for expr in expressions:
        for role,binary in [('baseline',BASE),('vanilla',VANILLA)]:
            argv = [binary,'--color','never','eval',expr]
            at = now()
            p = subprocess.run(argv,cwd=ROOT,text=True,capture_output=True,timeout=30)
            rows.append(dict(expression=expr,role=role,argv=argv,at=at,exit=p.returncode,stdout=p.stdout,stderr=p.stderr))
    assert state()['product_inventory'] == s['product_inventory']
    save('baseline',dict(at=now(),state=s,rows=rows,original_owner=(ROOT/OWNER).read_text(),original_prompt=(ROOT/PROMPT).read_text(),historical_preserved=historical,previous_closure_sha256=sha(D/'p1329-closure.json'),binaries={role:dict(path=b,sha256=sha(b)) for role,b in [('baseline',BASE),('vanilla',VANILLA)]}))
    save('baseline-public',dict(at=now(),head=s['head'],working_tree='uncommitted',product_inventory=s['product_inventory'],diff_stat=s['diff_stat'],rows=rows,binaries={role:dict(path=b,sha256=sha(b)) for role,b in [('baseline',BASE),('vanilla',VANILLA)]},baseline_sha256=sha(D/'p1330-baseline.json')))

def command(name, argv):
    before = state(); verify(before); at = now(); tick = time.monotonic()
    target = json.loads(TARGET_FILE.read_text())['target'] if TARGET_FILE.exists() else '/tmp/p1330-not-configured'
    p = subprocess.run(argv,cwd=ROOT,env={**os.environ,'CARGO_TARGET_DIR':target,'PYTHONDONTWRITEBYTECODE':'1'},text=True,capture_output=True,timeout=2400)
    after = state(); verify(after)
    binary = Path(target)/'release/typst'
    save(name,dict(at=at,end=now(),seconds=time.monotonic()-tick,argv=argv,cwd=str(ROOT),target=target,before=before,after=after,exit=p.returncode,stdout=p.stdout,stderr=p.stderr,manifest_sha256=sha(D/'p1330-manifest-r2.json') if (D/'p1330-manifest-r2.json').exists() else None,binary=dict(path=str(binary),sha256=sha(binary)) if binary.exists() else None))
    if name.startswith('unit-red'): print('RED exit',p.returncode,'private output retained for reviewer')
    else: print(p.stdout[-1500:],p.stderr[-800:])
    sys.exit(p.returncode)

if __name__ == '__main__':
    if sys.argv[1] == 'init': init()
    else: command(sys.argv[1],sys.argv[2:])
