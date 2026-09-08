"""Replay previous immutable suites; isolate predeclared read-cast deltas."""
import importlib.util
import json
import os
from pathlib import Path
import subprocess
import sys
import time

sys.dont_write_bytecode = True
D = Path(__file__).resolve().parent
def module(name, filename):
    spec = importlib.util.spec_from_file_location(name, D/filename)
    obj = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(obj)
    return obj
r = module('record', 'p1312-record.py')
def read(filename):
    return json.loads((D/filename).read_text())
baseline = read('p1312-baseline.json')
for name, digest in baseline['prior_artifacts'].items():
    assert r.sha(r.ROOT/name) == digest, name
candidate = read('p1312-build.json')['candidate']
assert r.sha(candidate['path']) == candidate['sha256']
before = r.state(); tick = time.monotonic()
phase = sys.argv[1]
rows = []
if phase == 'p1310':
    ab = module('ab', 'p1310-ab-suite.py')
    f = read('p1310-ab-frozen-r2.json')
    assert r.sha(D/'p1310-ab-suite.py') == f['suite_sha256']
    for name, info in f['fixtures'].items():
        assert r.sha(Path(f['fixture_dir'])/name) == info['sha256']
    expected = {(c['id'], p): v for c in f['cases'] for p, v in c['expected'].items()}
    vanilla = {(x['case'], x['profile']): x['observable']
               for x in read('p1310-ab-freeze-measurement.json')['rows']
               if x['side'] == 'vanilla'}
    rows = ab.run(f['cases'], {'candidate': candidate}, Path(f['fixture_dir']))
    allowed = {('read.wrong', p) for p in ab.PROFILES}
    inputs = ['p1310-ab-frozen-r2.json', 'p1310-ab-suite.py', 'p1310-ab-freeze-measurement.json']
elif phase == 'p1311':
    ab = module('ab', 'p1311-ab-runner.py')
    f = read('p1311-ab-freeze.json')
    expected = {(x['id'], x['profile']): x['expected'] for x in f['expected']}
    fields = ('exit', 'stdout', 'stderr')
    vanilla = {(x['id'], x['profile']): {k: x[k] for k in fields}
               for x in read('p1311-ab-baseline-final.json')['runs'] if x['product'] == 'vanilla'}
    env = dict(os.environ)
    for key in ('TYPST_FEATURES', 'TYPST_DIAGNOSTIC_FORMAT', 'TYPST_ROOT'):
        env.pop(key, None)
    env.update(NO_COLOR='1', TERM='dumb')
    for case in read('p1311-ab-cases.json')['cases']:
        for profile, flags in ab.PROFILES.items():
            argv = [candidate['path'], 'eval', case['expr'], *flags]
            at = r.now()
            proc = subprocess.run(argv, cwd=r.ROOT, env=env, capture_output=True, text=True, timeout=30)
            rows.append({'case': case['id'], 'profile': profile, 'argv': argv,
                'cwd': str(r.ROOT), 'at': at,
                'observable': {'exit': proc.returncode, 'stdout': proc.stdout, 'stderr': proc.stderr}})
    allowed = {('read-loader-error', p) for p in ab.PROFILES}
    inputs = ['p1311-ab-freeze.json', 'p1311-ab-runner.py', 'p1311-ab-cases.json', 'p1311-ab-baseline-final.json']
else:
    raise ValueError(phase)
changed = set(); unknown = 0
for row in rows:
    key = row['case'], row['profile']
    observed = row['observable']
    if observed.get('kind') == 'Unknown' or observed.get('exit', 0) < 0:
        unknown += 1
    row['historical_expected'] = expected[key]
    row['historical_preserved'] = observed == expected[key]
    if not row['historical_preserved']:
        changed.add(key)
        row['pinned_vanilla'] = vanilla[key]
        row['equals_vanilla'] = observed == vanilla[key]
checks = {
    'unique_complete_keys': len(rows) == len(expected) == len({(x['case'], x['profile']) for x in rows}),
    'only_predeclared_read_deltas': changed == allowed,
    'all_deltas_equal_prepatch_vanilla': all(x.get('equals_vanilla', True) for x in rows),
    'no_unknown': unknown == 0,
    'candidate_intact': r.sha(candidate['path']) == candidate['sha256'],
}
r.save(phase+'-replay', {'before': before, 'after': r.state(), 'candidate': candidate,
    'baseline_sha256': r.sha(D/'p1312-baseline.json'), 'runner_sha256': r.sha(__file__),
    'inputs': {n: r.sha(D/n) for n in inputs}, 'rows': rows,
    'historical_counts': {'Preserved': len(rows)-len(changed), 'Violated': len(changed), 'Unknown': unknown},
    'checks': checks, 'pass': all(checks.values()), 'seconds': time.monotonic()-tick,
    'policy': 'Historical oracles unchanged. Only the predeclared read case across four profiles may change, to literal prepatch vanilla observations. Prior L0 pin is historical: current L0 is the authorized P1312 amendment, not claimed identical to P1310.'})
print(checks)
assert all(checks.values())
