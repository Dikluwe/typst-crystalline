"""P1328 plain-prompt lineage and independent oracle integrity."""
import hashlib
import importlib.util
import json
from pathlib import Path
import re
import sys
sys.dont_write_bytecode = True
spec = importlib.util.spec_from_file_location('r', Path(__file__).with_name('p1328-record.py'))
r = importlib.util.module_from_spec(spec)
spec.loader.exec_module(r)
before = r.state()
r.verify(before)
source = (r.ROOT / r.OWNER).read_bytes()
prompt = (r.ROOT / r.PROMPT).read_bytes()
sm = rb'(?m)^//! @prompt-hash ([0-9a-f]{8})\n'
pm = rb'(?m)^Hash do C\xc3\xb3digo: ([0-9a-f]{8})\n'
assert len(re.findall(sm, source)) == len(re.findall(pm, prompt)) == 1
assert b'sha256:' not in prompt
norm = re.sub(pm, b'', prompt)
a = hashlib.sha256(norm).hexdigest()
b = hashlib.sha256(re.sub(sm, b'', source)).hexdigest()
manifest = json.loads((r.D / 'p1328-manifest.json').read_text())
assert a == manifest['prompt_norm_sha256']
assert a[:8] == re.search(sm, source).group(1).decode()
assert b[:8] == re.search(pm, prompt).group(1).decode()
snippet = (r.D / 'p1328-ab-tests-r2.rs').read_bytes().strip()
assert source.count(snippet) == 1
assert before['product_inventory'] == r.state()['product_inventory']
print(json.dumps(dict(pass_gate=True, hash_a=a, hash_b=b,
    snippet_sha256=r.sha(r.D / 'p1328-ab-tests-r2.rs'), manifest_sha256=r.sha(r.D / 'p1328-manifest.json')), indent=2))
