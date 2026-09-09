"""Plain-prompt bidirectional lineage and frozen A/B integrity."""
import hashlib
import importlib.util
import json
from pathlib import Path
import re
import sys
sys.dont_write_bytecode = True
D = Path(__file__).resolve().parent
spec = importlib.util.spec_from_file_location('r',D/'p1331-record-r2.py')
r = importlib.util.module_from_spec(spec)
spec.loader.exec_module(r)
s = r.state(); r.verify(s)
source = (r.ROOT/r.OWNER).read_bytes()
prompt = (r.ROOT/r.PROMPT).read_bytes()
sm = rb'(?m)^//! @prompt-hash ([0-9a-f]{8})\n'
pm = rb'(?m)^Hash do C\xc3\xb3digo: ([0-9a-f]{8})\n'
assert len(re.findall(sm,source)) == len(re.findall(pm,prompt)) == 1
assert b'sha256:' not in prompt
a = hashlib.sha256(re.sub(pm,b'',prompt)).hexdigest()
b = hashlib.sha256(re.sub(sm,b'',source)).hexdigest()
m = json.loads((D/'p1331-manifest-r2.json').read_text())
assert a == m['prompt_norm_sha256']
assert a[:8] == re.search(sm,source).group(1).decode()
assert b[:8] == re.search(pm,prompt).group(1).decode()
for name in ['p1331-ab-tests-r2.rs','p1331-ab-p1328-successor.rs','p1329-ab-tests-r1.rs','p1330-ab-tests.rs']:
    assert source.count((D/name).read_bytes().strip()) == 1, name
integration = json.loads((D/'p1331-test-integration-r2.json').read_text())
assert all(r.sha(path) == h for path,h in integration['frozen'].items())
assert s['product_inventory'] == r.state()['product_inventory']
print(json.dumps(dict(pass_gate=True,hash_a=a,hash_b=b,manifest_sha256=r.sha(D/'p1331-manifest-r2.json')),indent=2))
