"""Blind integration of frozen A/B test bytes before implementation."""
import difflib
import importlib.util
import json
from pathlib import Path
import re
import subprocess
import sys
sys.dont_write_bytecode = True
D = Path(__file__).resolve().parent
spec = importlib.util.spec_from_file_location('r', D / 'p1330-record.py')
r = importlib.util.module_from_spec(spec)
spec.loader.exec_module(r)
before = r.state(); r.verify(before)
old = (D / 'p1329-ab-p1328-successor.rs').read_text().strip()
successor = (D / 'p1330-ab-p1328-successor.rs').read_text().strip()
snippet = (D / 'p1330-ab-tests.rs').read_text().strip()
assert r.sha(D/'p1329-ab-p1328-successor.rs') == '1fb3d1c0b6ddfd486aa1cfd4cbdaffbdf2ecd774021b5512d217b769e92c3a7c'
assert successor.startswith('#[cfg(test)]') and snippet.startswith('#[cfg(test)]')
owner = r.ROOT/r.OWNER
source = owner.read_text()
baseline = json.loads((D/'p1330-baseline.json').read_text())
normalize = lambda s: re.sub(r'(?m)^//! @prompt-hash [0-9a-f]{8}\n','',s)
assert normalize(source) == normalize(baseline['original_owner'])
assert source.count(old) == 1
names = ['p1330-ab-tests.rs','p1330-ab-p1328-successor.rs','p1330-ab-cli.py',
         'p1330-ab-cli-baseline.json','p1330-ab-cli-expected.json','p1330-ab-freeze.md']
frozen = {str(D/name):r.sha(D/name) for name in names}
replacement = source.replace(old,successor).rstrip() + '\n\n' + snippet + '\n'
diff = ['@@\n' if line.startswith('@@ ') else line for line in
        list(difflib.unified_diff(source.splitlines(True),replacement.splitlines(True),n=3))[2:]]
patch = '*** Begin Patch\n*** Update File: '+str(owner)+'\n'+''.join(diff)+'*** End Patch\n'
subprocess.run(['apply_patch'],input=patch,text=True,check=True,capture_output=True,cwd=r.ROOT)
assert owner.read_text() == replacement
after = r.state(); r.verify(after)
r.save('test-integration',dict(at=r.now(),before=before,after=after,
    manifest_sha256=r.sha(D/'p1330-manifest-r2.json'),frozen=frozen,
    delta='Blind P1328 successor and new P1330 test module. Runtime and P1329 test module unchanged.'))
