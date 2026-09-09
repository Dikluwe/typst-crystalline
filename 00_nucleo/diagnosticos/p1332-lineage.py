"""Verify final bidirectional lineage, A/B inputs, and product boundaries."""
import hashlib
import importlib.util
import json
from pathlib import Path
import re
import sys
sys.dont_write_bytecode=True
D=Path(__file__).resolve().parent
spec=importlib.util.spec_from_file_location('r',D/'p1332-record.py')
r=importlib.util.module_from_spec(spec);spec.loader.exec_module(r)
s=r.state();r.verify(s)
source=(r.ROOT/r.OWNER).read_bytes();prompt=(r.ROOT/r.PROMPT).read_bytes()
sm=rb'(?m)^//! @prompt-hash ([0-9a-f]{8})\n'
pm=rb'(?m)^Hash do C\xc3\xb3digo: ([0-9a-f]{8})\n'
assert len(re.findall(sm,source))==len(re.findall(pm,prompt))==1
a=hashlib.sha256(re.sub(pm,b'',prompt)).hexdigest()
b=hashlib.sha256(re.sub(sm,b'',source)).hexdigest()
m=json.loads((D/'p1332-manifest.json').read_text())
assert a==m['prompt_norm_sha256'] and a[:8]==re.search(sm,source).group(1).decode()
assert b[:8]==re.search(pm,prompt).group(1).decode()
i=json.loads((D/'p1332-test-integration.json').read_text())
assert all(r.sha(p)==h for p,h in i['frozen'].items())
for name in i['integrated_snippets']:
    assert source.count((D/name).read_bytes().strip())==1,name
assert s['product_inventory']==r.state()['product_inventory']
print(json.dumps(dict(pass_gate=True,hash_a=a,hash_b=b,manifest_sha256=r.sha(D/'p1332-manifest.json')),indent=2))
