"""Replay immutable P1310 language expectations against P1311; no oracle writes."""
import importlib.util
import json
from pathlib import Path
import sys
import time

sys.dont_write_bytecode = True
D = Path(__file__).resolve().parent

def load(name, path):
    spec = importlib.util.spec_from_file_location(name, path)
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module

r = load('record', D/'p1311-record.py')
old = load('old', D/'p1310-ab-r2.py')
f = json.loads((D/'p1310-ab-frozen-r2.json').read_text())
old.verify_pins(f)
assert r.sha(D/'p1310-ab-r2.py') == f['r2_runner_sha256']
assert r.sha(D/'p1310-ab-frozen-r2.json') == '50641e69dd555f5599ff94ec82c23df947bff237dca9168dbe71a6e3776d1b68'
build = json.loads((D/'p1311-build.json').read_text())
candidate = build['candidate']
assert r.sha(candidate['path']) == candidate['sha256']
before = r.state(); tick = time.monotonic()
rows = old.ab.run(f['cases'], {'candidate': candidate}, Path(f['fixture_dir']))
expected = {(c['id'], p): v for c in f['cases'] for p, v in c['expected'].items()}
counts = {'Preserved': 0, 'Violated': 0, 'Unknown': 0}
for row in rows:
    v = row['observable']
    key = row['case'], row['profile']
    verdict = 'Unknown' if v['kind'] == 'Unknown' else 'Preserved' if v == expected[key] else 'Violated'
    row['verdict'] = verdict
    counts[verdict] += 1
assert r.sha(candidate['path']) == candidate['sha256']
r.save('p1310-replay', {'before': before, 'after': r.state(), 'candidate': candidate,
    'baseline_sha256': r.sha(D/'p1311-baseline.json'), 'script_sha256': r.sha(__file__),
    'inputs': {n: r.sha(D/n) for n in ('p1310-ab-r2.py','p1310-ab-suite.py','p1310-ab-frozen-r2.json','p1311-build.json')},
    'counts': counts, 'rows': rows, 'seconds': time.monotonic()-tick,
    'policy': 'Exact frozen P1310 R2 envelopes, with declared vanilla/Symbol/baseline policies; no global parity claim.'})
print(counts)
assert counts == {'Preserved': len(expected), 'Violated': 0, 'Unknown': 0}
