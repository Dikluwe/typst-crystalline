"""Only the historical csv-delimiter diagnostic may change in P1312."""
import importlib.util
import json
import os
from pathlib import Path
import subprocess
import sys
import time

sys.dont_write_bytecode = True
D = Path(__file__).resolve().parent
spec = importlib.util.spec_from_file_location('record', D/'p1314-record.py')
r = importlib.util.module_from_spec(spec); spec.loader.exec_module(r)
def read(n): return json.loads((D/n).read_text())
fields = ('exit', 'stdout', 'stderr')
def obs(x): return {k: x[k] for k in fields}
old = {(x['case'], x['profile']): x['observable'] for x in read('p1313-p1312-replay.json')['rows']}
vanilla = {(x['id'], x['profile']): obs(x) for x in read('p1312-ab-baseline-sealed.json')['runs'] if x['product'] == 'vanilla'}
candidate = read('p1314-build.json')['candidate']
assert r.sha(candidate['path']) == candidate['sha256']
before = r.state(); tick = time.monotonic()
profiles = {'default': [], 'html': ['--features','html'], 'a11y': ['--features','a11y-extras'], 'html+a11y': ['--features','html,a11y-extras']}
env = dict(os.environ)
for key in ('TYPST_FEATURES','TYPST_DIAGNOSTIC_FORMAT','TYPST_ROOT'): env.pop(key, None)
env.update(NO_COLOR='1', TERM='dumb')
rows = []; changed = set()
required = {('csv-delimiter', p) for p in profiles}
for c in read('p1312-ab-cases.json')['cases']:
    for profile, flags in profiles.items():
        key = c['id'], profile
        expected = vanilla[key] if key in required else old[key]
        argv = [candidate['path'], 'eval', c['expr'], *flags]
        at = r.now()
        p = subprocess.run(argv, cwd='/tmp/p1312-ab-fixtures', env=env, capture_output=True, text=True, timeout=30)
        got = {'exit': p.returncode, 'stdout': p.stdout, 'stderr': p.stderr}
        if got != old[key]: changed.add(key)
        rows.append({'case': c['id'], 'profile': profile, 'argv': argv, 'cwd': '/tmp/p1312-ab-fixtures', 'at': at,
            'observable': got, 'previous': old[key], 'expected': expected, 'pass': got == expected, 'unknown': p.returncode < 0})
inputs = ['p1312-ab-cases.json', 'p1312-ab-baseline-sealed.json', 'p1313-p1312-replay.json', 'p1314-build.json']
checks = {'complete_unique': len(rows) == len(old) == len({(x['case'], x['profile']) for x in rows}),
    'exact_declared_deltas': changed == required, 'all_expected': all(x['pass'] for x in rows),
    'no_unknown': not any(x['unknown'] for x in rows), 'candidate_intact': r.sha(candidate['path']) == candidate['sha256']}
r.save('p1312-replay', {'before': before, 'after': r.state(), 'candidate': candidate, 'inputs': {n:r.sha(D/n) for n in inputs},
    'runner_sha256': r.sha(__file__), 'baseline_sha256': r.sha(D/'p1314-baseline.json'),
    'rows': rows, 'checks': checks, 'pass': all(checks.values()), 'seconds': time.monotonic()-tick,
    'counts_against_p1313': {'Preserved': len(rows)-len(changed), 'Changed': len(changed), 'Unknown': sum(x['unknown'] for x in rows)},
    'policy': 'Only csv-delimiter across four profiles adopts prepatch vanilla; all other P1313 observations of P1312 remain literal.'})
print(checks)
assert all(checks.values())
