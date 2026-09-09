"""Freeze C after independent RED, or reseal only reciprocal lineage metadata."""
import difflib, importlib.util, subprocess, sys
from pathlib import Path
sys.dont_write_bytecode = True
D = Path(__file__).resolve().parent
spec = importlib.util.spec_from_file_location('r', D / 'p1337-record.py')
r = importlib.util.module_from_spec(spec); spec.loader.exec_module(r)
manifest = r.read('manifest-r1')
lineage = r.load('p1334-lineage-lib')
hashes = lineage.hashes(r.L0, r.SOURCE)
assert hashes['norm_sha256'] == manifest['prompt']['normative_sha256']
assert hashes['recorded_a'] == hashes['effective_a'][:8]
if sys.argv[1] == 'candidate':
    reviewer = Path(sys.argv[2]).resolve()
    assert reviewer.parent == D and reviewer.name.startswith('p1337-review-')
    assert r.read('unit-red-r1')['exit'] == 101 and reviewer.is_file()
    current = r.state(); r.verify(current)
    source = (r.ROOT / r.SOURCE).read_text()
    r.save('candidate', dict(at=r.now(), manifest_sha256=r.sha(D / 'p1337-manifest-r1.json'),
        red_path=str(D / 'p1337-unit-red-r1.json'), red_sha256=r.sha(D / 'p1337-unit-red-r1.json'), red_review=dict(path=str(reviewer), sha256=r.sha(reviewer)),
        tests_freeze_path=str(D / 'p1337-tests-freeze-r2.json'), tests_freeze_sha256=r.sha(D / 'p1337-tests-freeze-r2.json'),
        attacks_freeze_path=str(D / 'p1337-attacks-freeze-r2.json'), attacks_freeze_sha256=r.sha(D / 'p1337-attacks-freeze-r2.json'),
        red_mechanical_transport=dict(path=str(D / 'p1337-review-fmt-transition.md'), sha256=r.sha(D / 'p1337-review-fmt-transition.md')),
        source_sha256=r.sha(r.ROOT / r.SOURCE), source=source, lineage=hashes, state=current,
        delta_from_pre_test_source=''.join(difflib.unified_diff(manifest['pre_candidate_source'].splitlines(True), source.splitlines(True), fromfile='pre-test baseline', tofile='candidate with independent tests'))))
elif sys.argv[1] == 'seal':
    old = 'Hash do Código: ' + hashes['recorded_b']
    new = 'Hash do Código: ' + hashes['code_b'][:8]
    assert old != new
    subprocess.run(['apply_patch'], input='*** Begin Patch\n*** Update File: ' + r.L0 + '\n@@\n-' + old + '\n+' + new + '\n*** End Patch\n', text=True, check=True)
    after = lineage.hashes(r.L0, r.SOURCE)
    assert after['norm_sha256'] == hashes['norm_sha256'] and after['recorded_a'] == after['effective_a'][:8] and after['recorded_b'] == after['code_b'][:8]
    r.save('lineage-reciprocal', dict(at=r.now(), before=hashes, after=after,
        manifest_sha256=r.sha(D / 'p1337-manifest-r1.json'), library=dict(path=str(D / 'p1334-lineage-lib.py'), sha256=r.sha(D / 'p1334-lineage-lib.py'))))
else:
    raise SystemExit('candidate REVIEW_RECEIPT or seal')
