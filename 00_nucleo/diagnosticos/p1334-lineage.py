"""Audit all three final owners, effective nucleus pins and frozen inputs."""
import importlib.util
import json
from pathlib import Path
import sys
sys.dont_write_bytecode=True
D=Path(__file__).resolve().parent
def load(n):
    spec=importlib.util.spec_from_file_location(n,D/(n+'.py'))
    m=importlib.util.module_from_spec(spec);spec.loader.exec_module(m);return m
r=load('p1334-record');l=load('p1334-lineage-lib')
s=r.state();r.verify(s);m=json.loads((D/'p1334-manifest.json').read_text());results=[]
for owner in m['owners']:
    h=l.hashes(owner['prompt'],owner['source'])
    assert h['norm_sha256']==owner['norm_sha256'] and h['effective_a']==owner['effective_a']
    assert h['effective_a'][:8]==h['recorded_a'] and h['code_b'][:8]==h['recorded_b']
    results.append(h)
i=json.loads((D/'p1334-test-integration-r1.json').read_text())
for source,names in i['owners'].items():
    body=(r.ROOT/source).read_bytes()
    for name in names:assert body.count((D/name).read_bytes().strip())==1,name
f=json.loads((D/'p1334-ab-freeze-r1.json').read_text())
assert all(r.sha(D/p)==h for p,h in {**f['inputs'],**f['artifacts']}.items())
assert f['manifest_sha256']==r.sha(D/'p1334-manifest.json')
assert s['product_inventory']==r.state()['product_inventory']
print(json.dumps(dict(pass_gate=True,owners=results,manifest_sha256=r.sha(D/'p1334-manifest.json')),indent=2))
