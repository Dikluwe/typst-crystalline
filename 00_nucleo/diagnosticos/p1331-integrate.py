"""Blind integration of frozen tests using an unambiguous full-file preimage."""
import importlib.util
import json
from pathlib import Path
import re
import subprocess
import sys
sys.dont_write_bytecode = True
D = Path(__file__).resolve().parent
spec = importlib.util.spec_from_file_location('r', D/'p1331-record.py')
r = importlib.util.module_from_spec(spec); spec.loader.exec_module(r)
before = r.state(); r.verify(before)
baseline = json.loads((D/'p1331-baseline.json').read_text())
old = (D/'p1330-ab-p1328-successor.rs').read_text().strip()
successor = (D/'p1331-ab-p1328-successor.rs').read_text().strip()
snippet = (D/'p1331-ab-tests.rs').read_text().strip()
assert r.sha(D/'p1330-ab-p1328-successor.rs') == '56edc0c1b8c92d4e25bcfb73d1c3972526ea968f0647ea9462b659c2f517b47f'
owner = r.ROOT/r.OWNER; source = owner.read_text()
normalize = lambda s: re.sub(r'(?m)^//! @prompt-hash [0-9a-f]{8}\n','',s)
assert normalize(source) == normalize(baseline['original_owner'])
assert source.count(old) == 1
assert successor.startswith('#[cfg(test)]') and snippet.startswith('#[cfg(test)]')
names = ['p1331-ab-tests.rs','p1331-ab-p1328-successor.rs','p1331-ab-cli.py',
         'p1331-ab-cli-baseline.json','p1331-ab-cli-expected.json','p1331-ab-freeze.md']
frozen = {str(D/name):r.sha(D/name) for name in names}
wanted = source.replace(old,successor).rstrip()+'\n\n'+snippet+'\n'
for name in ['p1329-ab-tests-r1.rs','p1330-ab-tests.rs']:
    assert wanted.count((D/name).read_text().strip()) == 1
patch = '*** Begin Patch\n*** Update File: '+str(owner)+'\n@@\n'
patch += ''.join('-'+l+'\n' for l in source.splitlines())
patch += ''.join('+'+l+'\n' for l in wanted.splitlines())+'*** End Patch\n'
subprocess.run(['apply_patch'],input=patch,text=True,check=True,capture_output=True,cwd=r.ROOT)
assert owner.read_text() == wanted
after = r.state(); r.verify(after)
r.save('test-integration',dict(at=r.now(),before=before,after=after,
    manifest_sha256=r.sha(D/'p1331-manifest.json'),frozen=frozen,
    delta='Exact baseline plus lineage, frozen string/symbol P1328 successor and appended P1331 tests; runtime and P1329/P1330 tests preserved.'))
