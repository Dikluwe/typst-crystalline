"""Integrate fixture successor only after exact restoration of pre-C runtime."""
import importlib.util
import json
from pathlib import Path
import subprocess
import sys
sys.dont_write_bytecode = True
D = Path(__file__).resolve().parent
spec = importlib.util.spec_from_file_location('r',D/'p1331-record-r2.py')
r = importlib.util.module_from_spec(spec); spec.loader.exec_module(r)
before = r.state(); r.verify(before)
restored = json.loads((D/'p1331-r2-restore-pre-c.json').read_text())['state']
assert before['product_inventory'] == restored['product_inventory']
m = json.loads((D/'p1331-manifest-r2.json').read_text())
assert all(r.sha(D/p) == h for p,h in m['frozen_r1'].items())
owner = r.ROOT/r.OWNER
source = owner.read_text()
old = (D/'p1331-ab-tests.rs').read_text().strip()
new = (D/'p1331-ab-tests-r2.rs').read_text().strip()
assert source.count(old) == 1 and source.rstrip().endswith(old)
names = ['p1331-ab-tests-r2.rs','p1331-ab-p1328-successor.rs',
         'p1331-ab-cli-r2.py','p1331-ab-cli-baseline.json',
         'p1331-ab-cli-expected-r2.json','p1331-ab-freeze-r2.md']
frozen = {str(D/p):r.sha(D/p) for p in names}
wanted = source.replace(old,new)
patch = '*** Begin Patch\n*** Update File: '+str(owner)+'\n@@\n'
patch += ''.join('-'+l+'\n' for l in old.splitlines())
patch += ''.join('+'+l+'\n' for l in new.splitlines())+'*** End Patch\n'
subprocess.run(['apply_patch'],input=patch,text=True,check=True,capture_output=True,cwd=r.ROOT)
assert owner.read_text() == wanted
after = r.state(); r.verify(after)
r.save('test-integration-r2',dict(at=r.now(),before=before,after=after,
    manifest_sha256=r.sha(D/'p1331-manifest-r2.json'),frozen=frozen,
    preserved_r1=m['frozen_r1'],delta='Only independent fixture successor R2; runtime remains pre-C and P1328 successor, P1329/P1330 unchanged.'))
