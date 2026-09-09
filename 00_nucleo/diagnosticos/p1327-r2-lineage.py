"""Independently compute both P1327 owner lineage pairs against frozen L0."""
import hashlib
import importlib.util
import json
from pathlib import Path
import re
import sys
sys.dont_write_bytecode = True
spec = importlib.util.spec_from_file_location('r', Path(__file__).with_name('p1327-r2-record.py'))
r = importlib.util.module_from_spec(spec)
spec.loader.exec_module(r)
s = r.state()
r.verify(s)
m = json.loads((r.D / 'p1327-r2-manifest.json').read_text())
rows = []
for owner, prompt_path in [(r.OWNER, r.PROMPT), (r.PAIRS[2], r.PAIRS[3]), (r.PAIRS[4], r.PAIRS[5])]:
    source = (r.ROOT / owner).read_bytes()
    prompt = (r.ROOT / prompt_path).read_bytes()
    sm = rb'(?m)^//! @prompt-hash ([0-9a-f]{8})\n'
    pm = rb'(?m)^Hash do C\xc3\xb3digo: ([0-9a-f]{8})\n'
    assert len(re.findall(sm, source)) == len(re.findall(pm, prompt)) == 1
    norm = re.sub(pm, b'', prompt)
    assert hashlib.sha256(norm).hexdigest() == m['prompts'][prompt_path]
    pins = re.findall(rb'(?m)^- (00_nucleo/prompts/_nuclei/\S+) sha256:([0-9a-f]{64})$', prompt)
    assert pins
    payload = norm + b'\0TEKT-PROMPT-NUCLEI-V1\0'
    verified_pins = {}
    for path, expected in sorted(pins):
        nucleus = (r.ROOT / path.decode()).read_bytes()
        assert b'[[depends]]' not in nucleus
        pin = hashlib.sha256(nucleus + b'\0TEKT-NUCLEUS-DEPS-V1\0').digest()
        assert pin.hex() == expected.decode()
        verified_pins[path.decode()] = pin.hex()
        payload += len(path).to_bytes(8, 'big') + path + bytes([32]) + pin
    a = hashlib.sha256(payload).hexdigest()
    b = hashlib.sha256(re.sub(sm, b'', source)).hexdigest()
    assert a[:8] == re.search(sm, source).group(1).decode()
    assert b[:8] == re.search(pm, prompt).group(1).decode()
    rows.append(dict(owner=owner, prompt=prompt_path, hash_a=a, hash_b=b, nucleus_pins=verified_pins))
baseline = json.loads((r.D / 'p1327-baseline.json').read_text())
test_source = baseline['originals'][r.PAIRS[2]]
legacy = json.loads((r.D / 'p1327-ab-legacy-replacements.json').read_text())
for item in legacy['replacements']:
    assert test_source.count(item['old']) == 1
    test_source = test_source.replace(item['old'], item['new'], 1)
anchor = legacy['append_before_unique_anchor']
assert test_source.count(anchor) == 1
snippet = (r.ROOT / legacy['snippet']).read_text()
test_source = test_source.replace(anchor, snippet + '\n' + anchor, 1)
format_r1 = json.loads((r.D / 'p1327-ab-r1-format.json').read_text())
for item in format_r1['replacements']:
    assert test_source.count(item['old']) == 1
    test_source = test_source.replace(item['old'], item['new'], 1)
assert re.sub(sm, b'', test_source.encode()) == re.sub(sm, b'', (r.ROOT / r.PAIRS[2]).read_bytes())
freeze = json.loads((r.D / 'p1327-ab-freeze.json').read_text())
for path, h in freeze['hashes'].items():
    if path not in (r.PROMPT, r.PAIRS[3]):
        assert r.sha(r.ROOT / path) == h
assert s['product_inventory'] == r.state()['product_inventory']
print(json.dumps(dict(pass_gate=True, rows=rows, manifest_sha256=r.sha(r.D / 'p1327-r2-manifest.json')), indent=2))
