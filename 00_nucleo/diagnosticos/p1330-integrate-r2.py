"""Recover only placement of an intact test module after ambiguous patch context."""
import importlib.util
import json
from pathlib import Path
import re
import subprocess
import sys
sys.dont_write_bytecode = True
D = Path(__file__).resolve().parent
spec = importlib.util.spec_from_file_location('r',D/'p1330-record.py')
r = importlib.util.module_from_spec(spec); spec.loader.exec_module(r)
before = r.state(); r.verify(before)
baseline = json.loads((D/'p1330-baseline.json').read_text())
source = re.sub(r'(?m)^//! @prompt-hash [0-9a-f]{8}', '//! @prompt-hash 9a5d734e', baseline['original_owner'])
old = (D/'p1329-ab-p1328-successor.rs').read_text().strip()
successor = (D/'p1330-ab-p1328-successor.rs').read_text().strip()
snippet = (D/'p1330-ab-tests.rs').read_text().strip()
names = ['p1330-ab-tests.rs','p1330-ab-p1328-successor.rs','p1330-ab-cli.py',
         'p1330-ab-cli-baseline.json','p1330-ab-cli-expected.json','p1330-ab-freeze.md']
frozen = {str(D/name):r.sha(D/name) for name in names}
assert frozen[str(D/'p1330-ab-tests.rs')] == '6ed8f3af58855a88c9ea15113b38c2dad6029639448b052671b4effed4f5ca59'
assert frozen[str(D/'p1330-ab-p1328-successor.rs')] == '56edc0c1b8c92d4e25bcfb73d1c3972526ea968f0647ea9462b659c2f517b47f'
intermediate = source.replace(old,successor)
wanted = intermediate.rstrip()+'\n\n'+snippet+'\n'
owner = r.ROOT/r.OWNER
actual = owner.read_text()
assert actual.count(snippet) == actual.count(successor) == 1
assert actual.replace('\n\n'+snippet,'') == intermediate
r.save('integration-placement-incident',dict(at=r.now(),state=before,
    manifest_sha256=r.sha(D/'p1330-manifest-r2.json'),frozen=frozen,
    cause='Unanchored closing-brace context inserted intact new P1330 module before P1329, not EOF. Postimage assertion failed before any RED/C. Removal in memory exactly reproduces baseline plus intended successor.',
    compiler_or_test_failure=False,oracle_changes=False))
patch = '*** Begin Patch\n*** Update File: '+str(owner)+'\n@@\n'
patch += ''.join('-'+line+'\n' for line in actual.splitlines())
patch += ''.join('+'+line+'\n' for line in wanted.splitlines())+'*** End Patch\n'
subprocess.run(['apply_patch'],input=patch,text=True,check=True,capture_output=True,cwd=r.ROOT)
assert owner.read_text() == wanted
after = r.state(); r.verify(after)
r.save('test-integration',dict(at=r.now(),before=before,after=after,
    manifest_sha256=r.sha(D/'p1330-manifest-r2.json'),frozen=frozen,
    incident_sha256=r.sha(D/'p1330-integration-placement-incident.json'),
    delta='Exact baseline with only lineage, frozen P1328 successor and appended P1330 snippet; runtime and P1329 tests unchanged.'))
