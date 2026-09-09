"""Blind integration of frozen independent tests, before candidate code."""
import importlib.util
import json
from pathlib import Path
import re
import subprocess
import sys
sys.dont_write_bytecode = True
D = Path(__file__).resolve().parent
spec = importlib.util.spec_from_file_location('r', D / 'p1329-record.py')
r = importlib.util.module_from_spec(spec)
spec.loader.exec_module(r)
before = r.state()
r.verify(before)
old = (D / 'p1328-ab-tests-r2.rs').read_text().strip()
successor = (D / 'p1329-ab-p1328-successor.rs').read_text().strip()
snippet = (D / 'p1329-ab-tests-r1.rs').read_text().strip()
assert r.sha(D / 'p1328-ab-tests-r2.rs') == 'b689de73f3bbe572ed4cc9a0fea0fc29a792f91a8125cc8e1a657a19e676315e'
assert successor.startswith('#[cfg(test)]') and snippet.startswith('#[cfg(test)]')
owner = r.ROOT / r.OWNER
source = owner.read_text()
baseline = json.loads((D / 'p1329-baseline.json').read_text())
without_lineage = lambda text: re.sub(r'(?m)^//! @prompt-hash [0-9a-f]{8}\n', '', text)
assert without_lineage(source) == without_lineage(baseline['original_owner'])
assert source.count(old) == 1 and source.rstrip().endswith(old)
files = ['p1329-ab-tests-r1.rs', 'p1329-ab-p1328-successor.rs',
         'p1329-ab-cli-r2.py', 'p1329-ab-cli-baseline-r1.json',
         'p1329-ab-cli-expected-r2.json', 'p1329-ab-freeze-r2.md']
frozen = {str(D / name):r.sha(D / name) for name in files}
replacement = successor + '\n\n' + snippet
patch = ('*** Begin Patch\n*** Update File: ' + str(owner) + '\n@@\n'
         + ''.join('-' + line + '\n' for line in old.splitlines())
         + ''.join('+' + line + '\n' for line in replacement.splitlines())
         + '*** End Patch\n')
subprocess.run(['apply_patch'], input=patch, text=True, check=True, capture_output=True, cwd=r.ROOT)
assert owner.read_text() == source.replace(old, replacement)
after = r.state()
r.verify(after)
r.save('test-integration', dict(at=r.now(), before=before, after=after,
    manifest_sha256=r.sha(D / 'p1329-manifest-r2.json'), frozen=frozen,
    delta='Only independent successor P1328 tests and appended P1329 tests; runtime unchanged'))
