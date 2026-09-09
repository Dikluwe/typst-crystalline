"""Audit the authorized successor of P1328 CLI expectations before C."""
import importlib.util
import json
from pathlib import Path
import sys
sys.dont_write_bytecode = True
D = Path(__file__).resolve().parent
spec = importlib.util.spec_from_file_location('r', D / 'p1329-record.py')
r = importlib.util.module_from_spec(spec)
spec.loader.exec_module(r)
old = json.loads((D / 'p1328-ab-cli-normal.json').read_text())
new = json.loads((D / 'p1329-ab-cli-expected-r2.json').read_text())
baseline = json.loads((D / 'p1329-ab-cli-baseline-r1.json').read_text())
key = lambda x:(x['case'], x['profile'])
expected = {key(x):x['expected'] for x in new['expectations']}
measured = {key(x):x for x in baseline['cases']}
assert len(expected) == len(new['expectations']) == len(measured)
assert set(expected) == set(measured)
changed_cases = {'length', 'angle', 'ratio', 'fraction'}
changed = []
for row in old['cases']:
    k = key(row)
    assert measured[k]['expression'] == row['expression'], k
    role = 'VANILLA' if row['case'] in changed_cases else 'CANDIDATE'
    target = {name:row['results'][role][name] for name in ('exit','stdout','stderr')}
    assert expected[k] == target, k
    if row['case'] in changed_cases:
        changed.append(k)
assert len(changed) == 16 and len(old['cases']) == 112
r.save('historical-expectations', dict(at=r.now(),
    manifest_sha256=r.sha(D / 'p1329-manifest-r2.json'),
    predecessor_sha256=r.sha(D / 'p1328-ab-cli-normal.json'),
    expected_sha256=r.sha(D / 'p1329-ab-cli-expected-r2.json'),
    baseline_sha256=r.sha(D / 'p1329-ab-cli-baseline-r1.json'),
    pass_gate=True, predecessor_observations=112, changed=changed,
    unchanged=96, new_observations=len(expected)-112,
    rule='Only length/angle/ratio/fraction migrate to pinned vanilla; all other P1328 complete outputs remain literal'))
