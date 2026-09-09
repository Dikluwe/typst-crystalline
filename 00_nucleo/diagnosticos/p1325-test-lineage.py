"""Audit the explicit test-only succession without relaxing runtime freezes."""
import hashlib
import importlib.util
import json
import re
import sys
from pathlib import Path
sys.dont_write_bytecode = True
spec = importlib.util.spec_from_file_location('r', Path(__file__).with_name('p1325-record.py'))
r = importlib.util.module_from_spec(spec)
spec.loader.exec_module(r)
r.verify(r.state())
manifest = json.loads((r.D / 'p1325-test-succession-manifest.json').read_text())
owner, prompt_path = manifest['test_only_allowlist']
source = (r.ROOT / owner).read_bytes()
prompt = (r.ROOT / prompt_path).read_bytes()
sm = rb'(?m)^//! @prompt-hash ([0-9a-f]{8})\n'
pm = rb'(?m)^Hash do C\xc3\xb3digo: ([0-9a-f]{8})\n'
assert len(re.findall(sm, source)) == len(re.findall(pm, prompt)) == 1
norm = re.sub(pm, b'', prompt)
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
print(json.dumps(dict(owner=owner, hash_a=a, hash_b=b, norm_sha256=hashlib.sha256(norm).hexdigest(), nucleus_pin=pin.hex(), succession_manifest_sha256=r.sha(r.D / 'p1325-test-succession-manifest.json'), pass_gate=True), indent=2))
