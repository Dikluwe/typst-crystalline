"""Distinguish new P1312 read cast from accepted P1310/P1311 deltas."""
import importlib.util
import json
from pathlib import Path
import sys

sys.dont_write_bytecode = True
D = Path(__file__).resolve().parent
spec = importlib.util.spec_from_file_location('record', D/'p1312-record.py')
r = importlib.util.module_from_spec(spec)
spec.loader.exec_module(r)

names = ['p1312-p1308-replay.json', 'p1311-p1308-replay.json',
         'p1309-sentinels-p1308-vanilla.json', 'p1311-p1308-delta.json']
def read(name):
    return json.loads((D/name).read_text())
current, previous, vanilla = [json.loads(read(n)['stdout']) for n in names[:3]]
def keyed(data):
    result = {(row['case'], row['profile']): row for row in data['rows']}
    assert len(result) == len(data['rows'])
    return result
c, p, v = map(keyed, [current, previous, vanilla])
assert c.keys() == p.keys() == v.keys()
profiles = ('default', 'html', 'a11y', 'html+a11y')
expected = {('p1307.decoder.read.wrong-type', profile) for profile in profiles}
changed = {key for key in c if c[key]['observable'] != p[key]['observable']}
old = {key for key in p if p[key]['verdict'] == 'Violated'}
violated = {key for key in c if c[key]['verdict'] == 'Violated'}
checks = {
    'exact_new_cohort': changed == expected,
    'all_deltas_equal_pinned_vanilla': all(c[k]['observable'] == v[k]['observable'] for k in expected | old),
    'only_current_and_preceding_deltas': violated == expected | old,
    'no_unknown': current['counts']['Unknown'] == 0,
    'same_frozen_oracle': current['oracle_sha256'] == previous['oracle_sha256'] == vanilla['oracle_sha256'],
    'candidate_matches_build': current['binary_sha256'] == read('p1312-build.json')['candidate']['sha256'],
}
r.save('p1308-delta', {
    'at': r.now(), 'state': r.state(), 'candidate': read('p1312-build.json')['candidate'],
    'inputs': {n: r.sha(D/n) for n in names}, 'runner_sha256': r.sha(__file__),
    'counts_against_frozen_predecessor': current['counts'],
    'preserved_against_p1311': len(c)-len(changed),
    'new_changes': [{'case': k[0], 'profile': k[1], 'before': p[k]['observable'],
                     'after': c[k]['observable'], 'vanilla': v[k]['observable']} for k in sorted(changed)],
    'preceding_changes': len(old), 'checks': checks, 'pass': all(checks.values()),
    'comparison': 'Exact frozen language envelopes; new deltas isolated against P1311. Old P1308 expectations remain immutable, not relabeled as a green replay.'})
print(json.dumps(checks, indent=2))
assert all(checks.values())
