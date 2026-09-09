"""Audit both R2 owners and freeze RED tests/norms independently of lint repair."""
import hashlib
import importlib.util
import json
from pathlib import Path
import re
import subprocess
import sys

spec = importlib.util.spec_from_file_location('record', Path(__file__).with_name('p1321-r2-record.py'))
r = importlib.util.module_from_spec(spec)
spec.loader.exec_module(r)
before = r.state()
red_path = Path(__file__).with_name('p1321-r2-unit-red.json')
red = json.loads(red_path.read_text())
source_marker = rb'(?m)^//! @prompt-hash ([0-9a-f]{8})\n'
prompt_marker = rb'(?m)^Hash do C\xc3\xb3digo: ([0-9a-f]{8})\n'
results = []
digest = lambda value: hashlib.sha256(value).hexdigest()
for owner, l0 in r.PAIRS:
    source = (r.ROOT/owner).read_bytes()
    prompt = (r.ROOT/l0).read_bytes()
    assert len(re.findall(source_marker, source)) == len(re.findall(prompt_marker, prompt)) == 1
    b = digest(re.sub(source_marker, b'', source))
    norm = re.sub(prompt_marker, b'', prompt)
    pins = re.findall(rb'(?m)^- (00_nucleo/prompts/_nuclei/\S+) sha256:([0-9a-f]{64})$', prompt)
    assert len(pins) == 1
    path, expected = pins[0]
    nucleus = (r.ROOT/path.decode()).read_bytes()
    assert b'[[depends]]' not in nucleus
    pin = hashlib.sha256(nucleus + b'\0TEKT-NUCLEUS-DEPS-V1\0').digest()
    assert pin.hex() == expected.decode()
    a = digest(norm + b'\0TEKT-PROMPT-NUCLEI-V1\0' + len(path).to_bytes(8,'big') + path + bytes([32]) + pin)
    recorded_a = re.search(source_marker,source).group(1).decode()
    recorded_b = re.search(prompt_marker,prompt).group(1).decode()
    red_files = red['before']['files']
    frozen_norm = re.sub(prompt_marker, b'', red_files[l0]['text'].encode())
    test_marker = b'#[cfg(test)]\nmod tests {'
    assert source.count(test_marker) == 1
    tests = source.split(test_marker, 1)[1]
    frozen_tests = red_files[owner]['text'].encode().split(test_marker, 1)[1]
    valid = a[:8] == recorded_a and b[:8] == recorded_b and norm == frozen_norm and tests == frozen_tests
    results.append(dict(owner=owner,l0=l0,hash_a=a,hash_b=b,recorded_a=recorded_a,
        recorded_b=recorded_b,norm_sha256=digest(norm),red_norm_sha256=digest(frozen_norm),
        tests_sha256=digest(tests),red_tests_sha256=digest(frozen_tests),nucleus_pin=pin.hex(),pass_gate=valid))
argv = ['crystalline-lint','--checks','v5,v15,v26','--fail-on','warning','.']
p = subprocess.run(argv,cwd=r.ROOT,capture_output=True,text=True,timeout=120)
passed = all(item['pass_gate'] for item in results) and p.returncode == 0
r.save('lineage-'+sys.argv[1], dict(before=before,after=r.state(),owners=results,
    argv=argv,exit=p.returncode,stdout=p.stdout,stderr=p.stderr,pass_gate=passed,
    checker_sha256=r.sha(__file__),red_receipt_sha256=r.sha(red_path),
    linter_sha256=r.sha('/home/dikluwe/.cargo/bin/crystalline-lint')))
print(json.dumps(results,indent=2))
sys.exit(0 if passed else 1)
