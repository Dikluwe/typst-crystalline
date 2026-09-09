"""Check every historical CLI cell, not merely aggregate counts."""
import importlib.util
import json
from pathlib import Path
import sys
sys.dont_write_bytecode = True
D = Path(__file__).resolve().parent
spec = importlib.util.spec_from_file_location('r',D/'p1330-record.py')
r = importlib.util.module_from_spec(spec); spec.loader.exec_module(r)
old = json.loads((D/'p1329-ab-cli-expected-r2.json').read_text())
new = json.loads((D/'p1330-ab-cli-expected.json').read_text())
old_rows = json.loads((D/'p1329-ab-cli-normal.json').read_text())['cases']
new_rows = json.loads((D/'p1330-ab-cli-baseline.json').read_text())['cases']
key = lambda row: (row['case'],row['profile'])
old_expected = {key(x):x['expected'] for x in old['expectations']}
new_expected = {key(x):x['expected'] for x in new['expectations']}
old_map = {key(x):x for x in old_rows}
new_map = {key(x):x for x in new_rows}
assert len(old_expected) == len(old['expectations']) == 252
assert len(new_expected) == len(new['expectations']) == len(new_rows)
observed = lambda x: {k:x[k] for k in ('exit','stdout','stderr')}
changed = []
for k,e in old_expected.items():
    assert k in new_expected and k in new_map
    assert old_map[k]['expression'] == new_map[k]['expression']
    assert observed(new_map[k]['results']['BASE']) == e
    if e != new_expected[k]:
        assert k[0] == 'overflow'
        assert new_expected[k] == observed(new_map[k]['results']['VANILLA'])
        changed.append(k)
assert len(changed) == 4
r.save('historical-expectations',dict(at=r.now(),pass_gate=True,unchanged=248,
    changed=changed,new_observations=len(new_expected)-252,
    expected_sha256=r.sha(D/'p1330-ab-cli-expected.json'),
    previous_expected_sha256=r.sha(D/'p1329-ab-cli-expected-r2.json'),
    baseline_sha256=r.sha(D/'p1330-ab-cli-baseline.json'),
    manifest_sha256=r.sha(D/'p1330-manifest-r2.json')))
