"""Replay immutable predecessors against P1313; no expected changes in these cohorts."""
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
r = module('record', 'p1314-record.py')
def read(name):
    return json.loads((D/name).read_text())
baseline = read('p1314-baseline.json')
for name, digest in baseline['prior_artifacts'].items():
    assert r.sha(r.ROOT/name) == digest, name
candidate = read('p1314-build.json')['candidate']
assert r.sha(candidate['path']) == candidate['sha256']
before = r.state(); tick = time.monotonic()
phase = sys.argv[1]
fields = ('exit', 'stdout', 'stderr')
if phase == 'p1310':
    ab = module('ab', 'p1310-ab-suite.py')
    f = read('p1310-ab-frozen-r2.json')
    assert r.sha(D/'p1310-ab-suite.py') == f['suite_sha256']
    for name, info in f['fixtures'].items():
        assert r.sha(Path(f['fixture_dir'])/name) == info['sha256']
    expected = {(x['case'], x['profile']): x['observable'] for x in read('p1313-p1310-replay.json')['rows']}
    delta_expected = {(x['case'], x['profile']): x['observable'] for x in read('p1310-ab-freeze-measurement.json')['rows'] if x['side'] == 'vanilla'}
    rows = ab.run(f['cases'], {'candidate': candidate}, Path(f['fixture_dir']))
    allowed = set()
    inputs = ['p1310-ab-frozen-r2.json', 'p1310-ab-suite.py', 'p1310-ab-freeze-measurement.json', 'p1313-p1310-replay.json']
elif phase == 'p1311':
    ab = module('ab', 'p1311-ab-runner.py')
    expected = {(x['case'], x['profile']): x['observable'] for x in read('p1313-p1311-replay.json')['rows']}
    env = dict(os.environ)
    for key in ('TYPST_FEATURES', 'TYPST_DIAGNOSTIC_FORMAT', 'TYPST_ROOT'):
        env.pop(key, None)
    env.update(NO_COLOR='1', TERM='dumb')
    rows = []
    for case in read('p1311-ab-cases.json')['cases']:
        for profile, flags in ab.PROFILES.items():
            argv = [candidate['path'], 'eval', case['expr'], *flags]
            at = r.now()
            proc = subprocess.run(argv, cwd=r.ROOT, env=env, capture_output=True, text=True, timeout=30)
            rows.append({'case': case['id'], 'profile': profile, 'argv': argv, 'cwd': str(r.ROOT), 'at': at,
                'observable': {'exit': proc.returncode, 'stdout': proc.stdout, 'stderr': proc.stderr}})
    allowed = set(); delta_expected = {}
    inputs = ['p1313-p1311-replay.json', 'p1311-ab-runner.py', 'p1311-ab-cases.json']
else:
    raise ValueError(phase)
changed = set(); unknown = 0
for row in rows:
    key = row['case'], row['profile']; observed = row['observable']
    if observed.get('kind') == 'Unknown' or observed.get('exit', 0) < 0: unknown += 1
    row['p1313_observable'] = expected[key]
    row['preserved'] = observed == expected[key]
    if not row['preserved']:
        changed.add(key)
        row['delta_expected'] = delta_expected.get(key)
        row['delta_matches'] = observed == delta_expected.get(key)
checks = {
    'unique_complete_keys': len(rows) == len(expected) == len({(x['case'], x['profile']) for x in rows}),
    'exact_predeclared_csv_deltas': changed == allowed,
    'all_deltas_equal_prepatch_expectations': all(x.get('delta_matches', True) for x in rows),
    'no_unknown': unknown == 0,
    'candidate_intact': r.sha(candidate['path']) == candidate['sha256'],
}
r.save(phase+'-replay', {'before': before, 'after': r.state(), 'candidate': candidate,
    'baseline_sha256': r.sha(D/'p1314-baseline.json'), 'runner_sha256': r.sha(__file__),
    'inputs': {n: r.sha(D/n) for n in inputs}, 'rows': rows,
    'counts_against_p1313': {'Preserved': len(rows)-len(changed), 'Changed': len(changed), 'Unknown': unknown},
    'checks': checks, 'pass': all(checks.values()), 'seconds': time.monotonic()-tick,
    'policy': 'Historical oracles immutable. Both cohorts must be wholly preserved against P1313. Historical L0 pins do not describe the new P1314 amendment.'})
print(checks)
assert all(checks.values())
