"""Check P1325 reciprocal lineage and immutable normative/test inputs."""
import hashlib
import importlib.util
import json
import re
import sys
from pathlib import Path
sys.dont_write_bytecode = True
spec = importlib.util.spec_from_file_location('record', Path(__file__).with_name('p1325-record.py'))
r = importlib.util.module_from_spec(spec)
spec.loader.exec_module(r)
s = r.state()
r.verify(s)
source = (r.ROOT / r.OWNER).read_bytes()
prompt = (r.ROOT / r.PROMPT).read_bytes()
sm = rb'(?m)^//! @prompt-hash ([0-9a-f]{8})\n'
pm = rb'(?m)^Hash do C\xc3\xb3digo: ([0-9a-f]{8})\n'
assert len(re.findall(sm, source)) == len(re.findall(pm, prompt)) == 1
norm = re.sub(pm, b'', prompt)
manifest = json.loads((r.D / 'p1325-manifest.json').read_text())
assert hashlib.sha256(norm).hexdigest() == manifest['prompt_norm_sha256']
pins = re.findall(rb'(?m)^- (00_nucleo/prompts/_nuclei/\S+) sha256:([0-9a-f]{64})$', prompt)
assert len(pins) == 1
path, expected = pins[0]
nucleus = (r.ROOT / path.decode()).read_bytes()
assert b'[[depends]]' not in nucleus
pin = hashlib.sha256(nucleus + b'\0TEKT-NUCLEUS-DEPS-V1\0').digest()
assert pin.hex() == expected.decode()
a = hashlib.sha256(norm + b'\0TEKT-PROMPT-NUCLEI-V1\0' + len(path).to_bytes(8, 'big') + path + bytes([32]) + pin).hexdigest()
b = hashlib.sha256(re.sub(sm, b'', source)).hexdigest()
assert a[:8] == re.search(sm, source).group(1).decode()
assert b[:8] == re.search(pm, prompt).group(1).decode()
# Author-frozen snippet is integrated literally; its receipt is checked by reviewer.
snippets = list(r.D.glob('p1325-ab-tests-r1.rs'))
assert len(snippets) == 1, snippets
snippet = snippets[0].read_bytes().strip()
assert source.count(snippet) == 1
after = r.state()
assert s['product_inventory'] == after['product_inventory']
print(json.dumps(dict(hash_a=a, hash_b=b, normative_sha256=hashlib.sha256(norm).hexdigest(), snippet_sha256=r.sha(snippets[0]), nucleus_pin=pin.hex(), manifest_sha256=r.sha(r.D / 'p1325-manifest.json'), pass_gate=True), indent=2))
