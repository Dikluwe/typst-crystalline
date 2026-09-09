"""Compare complete lint messages, ignoring only source line relocation."""
import collections
import importlib.util
import json
from pathlib import Path
import re
import sys
sys.dont_write_bytecode=True
D=Path(__file__).resolve().parent
spec=importlib.util.spec_from_file_location('r',D/'p1334-record.py')
r=importlib.util.module_from_spec(spec);spec.loader.exec_module(r)
names=['preflight-all','final-lint']
receipts=[json.loads((D/('p1334-'+n+'.json')).read_text()) for n in names]
def messages(g):
    return collections.Counter(re.sub(r'(?m)(^\s*--> .*?):\d+(?::\d+)?$',r'\1:<line>',g['stdout']).split('\n\n'))
a,b=map(messages,receipts)
assert a==b,dict(added=list((b-a).elements()),removed=list((a-b).elements()))
assert all(g['exit']==0 for g in receipts)
r.save('lint-comparison',dict(at=r.now(),pass_gate=True,
    policy='Complete stdout messages equal as a multiset, ignoring source line coordinates only; no new warnings or infos.',
    counts=[dict(collections.Counter(re.findall(r'\[V\d+\]',g['stdout']))) for g in receipts],
    inputs={str(D/('p1334-'+n+'.json')):r.sha(D/('p1334-'+n+'.json')) for n in names}))
