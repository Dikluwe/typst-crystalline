"""Frozen P1312 corpus: only four explicitly approved CSV expressions change."""
import importlib.util
import json
import os
from pathlib import Path
import subprocess
import sys
import time

sys.dont_write_bytecode = True
D = Path(__file__).resolve().parent
spec = importlib.util.spec_from_file_location('record', D/'p1313-record.py')
r = importlib.util.module_from_spec(spec); spec.loader.exec_module(r)
def read(n): return json.loads((D/n).read_text())
fields = ('exit', 'stdout', 'stderr')
def obs(x): return {k: x[k] for k in fields}
f = read('p1313-ab-freeze.json')
assert r.sha(D/'p1313-ab-freeze.json') == '0f7ba52eb0ec820f53754274d51cb7ff4d20aab8d284d3c56161ff3d70b3e998'
new_cases = {x['id']: x for x in read('p1313-ab-cases.json')['cases']}
expectations = {(new_cases[x['id']]['expr'], x['profile']): x['expected'] for x in f['expected']}
changed_exprs = {'csv(42)', 'csv(bytes("a,b"))', 'csv(sym.alpha)', 'csv(delimiter: ";", false)'}
cases = read('p1312-ab-cases.json')['cases']
old = {(x['id'], x['profile']): obs(x) for x in read('p1312-ab-candidate-runs.json')['runs'] if x['order'] == 'normal'}
candidate = read('p1313-build.json')['candidate']
assert r.sha(candidate['path']) == candidate['sha256']
before = r.state(); tick = time.monotonic()
profiles = {'default': [], 'html': ['--features','html'], 'a11y': ['--features','a11y-extras'], 'html+a11y': ['--features','html,a11y-extras']}
env = dict(os.environ)
for key in ('TYPST_FEATURES','TYPST_DIAGNOSTIC_FORMAT','TYPST_ROOT'): env.pop(key, None)
env.update(NO_COLOR='1', TERM='dumb')
rows = []; changed = set(); required = set()
for c in cases:
    for profile, flags in profiles.items():
        key = c['id'], profile
        expected = old[key]
        if c['expr'] in changed_exprs:
            required.add(key)
            expected = expectations[c['expr'], profile]
        argv = [candidate['path'], 'eval', c['expr'], *flags]
        at = r.now()
        p = subprocess.run(argv, cwd='/tmp/p1312-ab-fixtures', env=env, capture_output=True, text=True, timeout=30)
        got = {'exit': p.returncode, 'stdout': p.stdout, 'stderr': p.stderr}
        if got != old[key]: changed.add(key)
        rows.append({'case': c['id'], 'profile': profile, 'argv': argv, 'cwd': '/tmp/p1312-ab-fixtures', 'at': at,
            'observable': got, 'previous': old[key], 'expected': expected, 'pass': got == expected, 'unknown': p.returncode < 0})
inputs = ['p1312-ab-cases.json', 'p1312-ab-candidate-runs.json', 'p1313-ab-freeze.json', 'p1313-ab-cases.json', 'p1313-build.json']
checks = {'complete_unique': len(rows) == len(old) == len({(x['case'], x['profile']) for x in rows}),
    'exact_declared_deltas': changed == required and len(required) == 16,
    'all_expected': all(x['pass'] for x in rows), 'no_unknown': not any(x['unknown'] for x in rows),
    'candidate_intact': r.sha(candidate['path']) == candidate['sha256']}
r.save('p1312-replay', {'before': before, 'after': r.state(), 'candidate': candidate, 'inputs': {n:r.sha(D/n) for n in inputs},
    'runner_sha256': r.sha(__file__), 'baseline_sha256': r.sha(D/'p1313-implementation-baseline.json'),
    'rows': rows, 'checks': checks, 'pass': all(checks.values()), 'seconds': time.monotonic()-tick,
    'counts_against_p1312': {'Preserved': len(rows)-len(changed), 'Changed': len(changed), 'Unknown': sum(x['unknown'] for x in rows)},
    'policy': 'Four CSV expressions use independently frozen P1313 expectations. All other P1312 outputs remain literal and immutable. Symbol is normative, not vanilla parity.'})
print(checks)
assert all(checks.values())
