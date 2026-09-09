"""Reseal reciprocal metadata only; the frozen normative intention cannot change."""
import importlib.util, subprocess, sys
from pathlib import Path
sys.dont_write_bytecode = True
D = Path(__file__).resolve().parent
def load(name):
    spec = importlib.util.spec_from_file_location(name, D / (name + '.py'))
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module
r = load('p1336-record')
lineage = load('p1334-lineage-lib')
manifest = r.read('manifest')
before = lineage.hashes(r.L0, r.SOURCE)
assert before['norm_sha256'] == manifest['prompt']['normative_sha256']
assert before['recorded_a'] == before['effective_a'][:8]
old = 'Hash do Código: ' + before['recorded_b']
new = 'Hash do Código: ' + before['code_b'][:8]
assert old != new
subprocess.run(['apply_patch'], input='*** Begin Patch\n*** Update File: ' + r.L0 + '\n@@\n-' + old + '\n+' + new + '\n*** End Patch\n', text=True, check=True)
after = lineage.hashes(r.L0, r.SOURCE)
assert after['norm_sha256'] == before['norm_sha256']
assert after['recorded_a'] == after['effective_a'][:8]
assert after['recorded_b'] == after['code_b'][:8]
r.save('lineage-reciprocal', dict(at=r.now(), before=before, after=after,
    reused_hash_library=dict(path=str(D / 'p1334-lineage-lib.py'), sha256=r.sha(D / 'p1334-lineage-lib.py')),
    reason='Final linter dry-run reports Nothing to fix because A has no drift; B independently recomputed as SHA-256 of source excluding its sole @prompt-hash line. Only reciprocal metadata changed.',
    dry_run_sha256=r.sha(D / 'p1336-reseal-final-dry-run.json')))
