"""Compute report numbers from immutable receipts, without changing oracles."""
import collections
import importlib.util
import json
from pathlib import Path
import re
import sys
sys.dont_write_bytecode=True
D=Path(__file__).resolve().parent
spec=importlib.util.spec_from_file_location('r',D/'p1333-record.py')
r=importlib.util.module_from_spec(spec);spec.loader.exec_module(r)
read=lambda n:json.loads((D/('p1333-'+n+'.json')).read_text())
obs=lambda x:{k:x[k] for k in ('exit','stdout','stderr')}
baseline={(x['case'],x['profile']):x for x in read('ab-cli-baseline')['cases']}
candidate={(x['case'],x['profile']):x for x in read('ab-cli-normal')['cases']}
assert baseline.keys()==candidate.keys()
counts=collections.Counter();remaining={};changed=[]
for key,row in baseline.items():
    b,v=obs(row['results']['BASE']),obs(row['results']['VANILLA'])
    c=obs(candidate[key]['results']['CANDIDATE'])
    counts['observations']+=1
    counts['base_equals_vanilla']+=b==v
    counts['candidate_equals_vanilla']+=c==v
    counts['changed']+=c!=b
    counts['regressed_from_parity']+=b==v and c!=v
    if c!=b:changed.append(list(key))
    if c!=v:remaining.setdefault(key[0],[]).append(key[1])
tests={}
for name in ['unit-red','unit-green-final','workspace-tests']:
    g=read(name)
    rows=re.findall(r'test result: \w+\. (\d+) passed; (\d+) failed; (\d+) ignored;',g['stdout'])
    tests[name]=dict(zip(('passed','failed','ignored'),map(sum,zip(*(map(int,x) for x in rows)))))
    tests[name].update(at=g['at'],end=g['end'],sha256=r.sha(D/('p1333-'+name+'.json')))
r.save('metrics',dict(at=r.now(),state=r.state(),manifest_sha256=r.sha(D/'p1333-manifest.json'),
    counts=dict(counts),remaining_cases=remaining,changed=changed,tests=tests,
    inputs={str(D/('p1333-'+n+'.json')):r.sha(D/('p1333-'+n+'.json')) for n in ['ab-cli-baseline','ab-cli-normal','unit-red','unit-green-final','workspace-tests']}))
print(json.dumps(dict(counts=counts,tests=tests,remaining_cases=remaining),indent=2))
