"""Check reverse hash explicitly; V5-only fix plan misses code-only drift."""
import hashlib
import importlib.util
import json
from pathlib import Path
import re
import subprocess
import sys

D = Path(__file__).resolve().parent
spec = importlib.util.spec_from_file_location('record', D/'p1319-record.py')
r = importlib.util.module_from_spec(spec)
spec.loader.exec_module(r)
before = r.state()
source = (r.ROOT/r.OWNER).read_bytes()
prompt = (r.ROOT/r.L0).read_bytes()
source_marker = rb'(?m)^//! @prompt-hash ([0-9a-f]{8})\r?\n'
prompt_marker = rb'(?m)^Hash do C\xc3\xb3digo: ([0-9a-f]{8})\r?\n'
assert len(re.findall(source_marker, source)) == 1
assert len(re.findall(prompt_marker, prompt)) == 1
computed = hashlib.sha256(re.sub(source_marker, b'', source)).hexdigest()
recorded = re.search(prompt_marker, prompt).group(1).decode()
normative = hashlib.sha256(re.sub(prompt_marker, b'', prompt)).hexdigest()
freeze = json.loads((D/'p1319-ab-freeze.json').read_text())
argv = ['crystalline-lint','--checks','v5,v15,v26','--fail-on','warning','.']
p = subprocess.run(argv, cwd=r.ROOT, capture_output=True, text=True, timeout=120)
toolsources = [Path('/repos/Antigravity/tekt-linter')/name for name in
              ['03_infra/hash_writer.rs','02_shell/fix_hashes.rs','04_wiring/main.rs']]
valid = recorded == computed[:8] and normative == freeze['l0']['normative_sha256'] and p.returncode == 0
r.save('lineage-'+sys.argv[1], dict(before=before, after=r.state(),
    source_without_header_sha256=computed, recorded_hash_b=recorded,
    hash_b_matches=recorded==computed[:8], normative_sha256=normative,
    frozen_normative_unchanged=normative==freeze['l0']['normative_sha256'],
    header_hash_a=re.search(source_marker,source).group(1).decode(),
    argv=argv,cwd=str(r.ROOT),exit=p.returncode,stdout=p.stdout,stderr=p.stderr,
    bidirectional_pass=valid, checker_sha256=r.sha(__file__),
    linter_sha256=r.sha('/home/dikluwe/.cargo/bin/crystalline-lint'),
    linter_source_head=subprocess.check_output(['git','-C','/repos/Antigravity/tekt-linter','rev-parse','HEAD'],text=True).strip(),
    linter_source_diff_stat=subprocess.check_output(['git','-C','/repos/Antigravity/tekt-linter','diff','HEAD','--stat'],text=True),
    linter_sources={str(path):r.sha(path) for path in toolsources}))
sys.exit(0 if valid else 1)
