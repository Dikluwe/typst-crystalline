"""Explicit A+nucleus and reverse B audit; tool success alone is insufficient."""
import hashlib
import importlib.util
from pathlib import Path
import re
import subprocess
import sys

spec = importlib.util.spec_from_file_location('record', Path(__file__).with_name('p1321-record.py'))
r = importlib.util.module_from_spec(spec)
spec.loader.exec_module(r)
before = r.state()
source = (r.ROOT/r.OWNER).read_bytes()
prompt = (r.ROOT/r.L0).read_bytes()
source_marker = rb'(?m)^//! @prompt-hash ([0-9a-f]{8})\n'
prompt_marker = rb'(?m)^Hash do C\xc3\xb3digo: ([0-9a-f]{8})\n'
assert len(re.findall(source_marker, source)) == len(re.findall(prompt_marker, prompt)) == 1
b = hashlib.sha256(re.sub(source_marker, b'', source)).hexdigest()
norm = re.sub(prompt_marker, b'', prompt)
pins = re.findall(rb'(?m)^- (00_nucleo/prompts/_nuclei/\S+) sha256:([0-9a-f]{64})$', prompt)
assert len(pins) == 1
path, expected = pins[0]
nucleus = (r.ROOT/path.decode()).read_bytes()
assert b'[[depends]]' not in nucleus
pin = hashlib.sha256(nucleus + b'\0TEKT-NUCLEUS-DEPS-V1\0').digest()
assert pin.hex() == expected.decode()
a = hashlib.sha256(norm + b'\0TEKT-PROMPT-NUCLEI-V1\0' + len(path).to_bytes(8,'big') + path + bytes([32]) + pin).hexdigest()
recorded_a = re.search(source_marker,source).group(1).decode()
recorded_b = re.search(prompt_marker,prompt).group(1).decode()
normative = hashlib.sha256(norm).hexdigest()
valid = (a[:8] == recorded_a and b[:8] == recorded_b and
    normative == '2f6dcab26aefce4a7db207bd347b46998dd845e7205954846be4512b5c2f2cc3')
argv = ['crystalline-lint','--checks','v5,v15,v26','--fail-on','warning','.']
p = subprocess.run(argv,cwd=r.ROOT,capture_output=True,text=True,timeout=120)
r.save('lineage-'+sys.argv[1], dict(before=before,after=r.state(),
    hash_a=a,hash_b=b,recorded_a=recorded_a,recorded_b=recorded_b,
    normative_sha256=normative,nucleus_pin=pin.hex(),bidirectional_pass=valid,
    argv=argv,exit=p.returncode,stdout=p.stdout,stderr=p.stderr,
    checker_sha256=r.sha(__file__),linter_sha256=r.sha('/home/dikluwe/.cargo/bin/crystalline-lint')))
print('A',a,'B',b,'valid',valid)
sys.exit(0 if valid and p.returncode==0 else 1)
