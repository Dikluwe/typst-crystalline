"""Integrate the test author's formatting-only successor before candidate code."""
import importlib.util
import subprocess
from pathlib import Path
import sys
sys.dont_write_bytecode = True
D = Path(__file__).resolve().parent
spec = importlib.util.spec_from_file_location('r', D / 'p1328-record.py')
r = importlib.util.module_from_spec(spec)
spec.loader.exec_module(r)
before = r.state()
r.verify(before)
old = (D / 'p1328-ab-tests-r1.rs').read_text().strip()
new = (D / 'p1328-ab-tests-r2.rs').read_text().strip()
old_import = ('        EvalContext, EvalTarget, eval_expression_with_features,\n'
              '        eval_with_full_error_target_and_features,')
new_import = ('        eval_expression_with_features, eval_with_full_error_target_and_features,\n'
              '        EvalContext, EvalTarget,')
assert old.count(old_import) == 1
assert old.replace(old_import, new_import) == new
owner = r.ROOT / r.OWNER
source = owner.read_text()
assert source.count(old) == 1 and source.rstrip().endswith(old)
patch = ('*** Begin Patch\n*** Update File: ' + str(owner) + '\n@@\n'
         + ''.join('-' + line + '\n' for line in old_import.splitlines())
         + ''.join('+' + line + '\n' for line in new_import.splitlines())
         + '*** End Patch\n')
subprocess.run(['apply_patch'], input=patch, text=True, check=True, capture_output=True, cwd=r.ROOT)
assert owner.read_text() == source.replace(old, new)
after = r.state()
r.verify(after)
r.save('test-integration-r2', dict(at=r.now(), before=before, after=after,
    manifest_sha256=r.sha(D/'p1328-manifest.json'),
    frozen={str(D/name):r.sha(D/name) for name in (
        'p1328-ab-tests-r1.rs', 'p1328-ab-tests-r2.rs', 'p1328-ab-freeze-r2.md',
        'p1328-ab-cli.py', 'p1328-ab-cli-expected-r1.json')},
    delta='Only edition2021 import ordering; all assertions and runtime byte-identical'))
