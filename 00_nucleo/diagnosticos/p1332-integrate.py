"""Blindly integrate frozen A/B snippets; runtime must still match baseline."""
import importlib.util
import json
from pathlib import Path
import re
import subprocess
import sys
sys.dont_write_bytecode=True
D=Path(__file__).resolve().parent
spec=importlib.util.spec_from_file_location('r',D/'p1332-record.py')
r=importlib.util.module_from_spec(spec);spec.loader.exec_module(r)
before=r.state();r.verify(before)
baseline=json.loads((D/'p1332-baseline.json').read_text())
owner=r.ROOT/r.OWNER;source=owner.read_text()
normalize=lambda s:re.sub(r'(?m)^//! @prompt-hash [0-9a-f]{8}\n','',s)
assert normalize(source)==normalize(baseline['original_owner'])
pairs=[('p1331-ab-p1328-successor.rs','p1332-ab-p1328-successor.rs'),
       ('p1329-ab-tests-r1.rs','p1332-ab-p1329-successor.rs'),
       ('p1330-ab-tests.rs','p1332-ab-p1330-successor.rs'),
       ('p1331-ab-tests-r2.rs','p1332-ab-p1331-successor.rs')]
freeze=json.loads((D/'p1332-ab-freeze.json').read_text())
assert all(r.sha(D/name)==h for name,h in {**freeze['artifacts'],**freeze['inputs']}.items())
frozen={str(p):r.sha(p) for p in sorted(D.glob('p1332-ab-*')) if p.is_file()}
for name in ['p1332-ab-tests.rs','p1332-ab-cli.py','p1332-ab-cli-baseline.json','p1332-ab-cli-expected.json']:
    assert str(D/name) in frozen,name
wanted=source
for old,new in pairs:
    previous=(D/old).read_text().strip();successor=(D/new).read_text().strip()
    assert wanted.count(previous)==1 and successor.startswith('#[cfg(test)]')
    wanted=wanted.replace(previous,successor)
snippet=(D/'p1332-ab-tests.rs').read_text().strip()
assert snippet.startswith('#[cfg(test)]')
wanted=wanted.rstrip()+'\n\n'+snippet+'\n'
patch='*** Begin Patch\n*** Update File: '+str(owner)+'\n@@\n'
patch+=''.join('-'+line+'\n' for line in source.splitlines())
patch+=''.join('+'+line+'\n' for line in wanted.splitlines())+'*** End Patch\n'
subprocess.run(['apply_patch'],input=patch,text=True,check=True,capture_output=True,cwd=r.ROOT)
assert owner.read_text()==wanted
after=r.state();r.verify(after)
r.save('test-integration',dict(at=r.now(),before=before,after=after,
    manifest_sha256=r.sha(D/'p1332-manifest.json'),frozen=frozen,
    integrated_snippets=[new for old,new in pairs]+['p1332-ab-tests.rs'],
    predecessors={str(D/old):r.sha(D/old) for old,new in pairs},
    delta='Exact frozen successors and new tests; runtime baseline preserved except lineage header.'))
