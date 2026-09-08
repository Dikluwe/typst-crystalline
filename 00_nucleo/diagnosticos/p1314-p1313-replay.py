"""Recount P1313 cohort executed within independent A/B, in original cwd."""
import importlib.util
import json
from pathlib import Path
import sys

sys.dont_write_bytecode = True
D = Path(__file__).resolve().parent
spec = importlib.util.spec_from_file_location('record', D/'p1314-record.py')
r = importlib.util.module_from_spec(spec); spec.loader.exec_module(r)
def read(n): return json.loads((D/n).read_text())
fields = ('exit', 'stdout', 'stderr')
def obs(x): return {k:x[k] for k in fields}
f = read('p1314-ab-freeze.json')
assert r.sha(D/'p1314-ab-freeze.json') == '41ac0730e01cd31cd9660b0e229e37d5e486a2c114927f3ac49c3475733756b9'
old_cases = {c['id']:c for c in read('p1313-ab-cases.json')['cases']}
old = {(x['id'],x['profile']):obs(x) for x in read('p1313-ab-candidate-runs.json')['runs'] if x['order']=='normal'}
expected = {(x['id'],x['profile']):x['expected'] for x in f['expected'] if x['historical_p1313']}
actual = [x for x in read('p1314-ab-candidate-runs.json')['runs'] if x['order']=='normal' and x['id'] in old_cases]
profiles = ('default','html','a11y','html+a11y')
required = {(i,p) for i in f['historical_deltas'] for p in profiles}
rows = []; changed = set()
for x in actual:
    key=x['id'],x['profile']; got=obs(x)
    assert x['cwd'] == '/tmp/p1313-ab-fixtures'
    assert x['argv'][2] == old_cases[x['id']]['expr']
    if got != old[key]: changed.add(key)
    rows.append({'case':key[0],'profile':key[1],'argv':x['argv'],'cwd':x['cwd'],'at':x['utc'],
        'observable':got,'previous':old[key],'expected':expected[key],
        'pass':got == (expected[key] if key in required else old[key]),'unknown':x['exit'] < 0})
candidate = read('p1314-build.json')['candidate']
checks = {'complete_unique': len(rows)==len(old)==len(expected)==len({(x['case'],x['profile']) for x in rows}),
    'exact_predeclared_deltas':changed==required,'all_expected':all(x['pass'] for x in rows),
    'no_unknown':not any(x['unknown'] for x in rows),'candidate_intact':r.sha(candidate['path'])==candidate['sha256']}
inputs=['p1313-ab-cases.json','p1313-ab-candidate-runs.json','p1314-ab-freeze.json','p1314-ab-candidate-runs.json','p1314-build.json']
r.save('p1313-replay',{'at':r.now(),'state':r.state(),'candidate':candidate,'runner_sha256':r.sha(__file__),
    'inputs':{n:r.sha(D/n) for n in inputs},'baseline_sha256':r.sha(D/'p1314-baseline.json'),
    'counts_against_p1313':{'Preserved':len(rows)-len(changed),'Changed':len(changed),'Unknown':sum(x['unknown'] for x in rows)},
    'rows':rows,'checks':checks,'pass':all(checks.values()),
    'policy':'Derived from independently executed normal-order A/B rows for all original P1313 expressions in original cwd. No extra execution and no rewrite of the historical oracle. Exactly the frozen option ids may differ.'})
print(checks)
assert all(checks.values())
