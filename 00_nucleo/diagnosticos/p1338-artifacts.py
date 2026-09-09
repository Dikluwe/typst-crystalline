"""Candidate snapshot and metadata-only reciprocal seal; never adjusts tests."""
import difflib, re, sys
from pathlib import Path
sys.dont_write_bytecode = True
import importlib.util
D = Path(__file__).resolve().parent
spec = importlib.util.spec_from_file_location('r', D / 'p1338-record.py')
r = importlib.util.module_from_spec(spec); spec.loader.exec_module(r)
lineage = r.load('p1334-lineage-lib')
def candidate(review_path):
    manifest = r.read('manifest-r1'); state = r.state(); r.verify(state)
    source = (r.ROOT / r.SOURCE).read_text()
    hashes = lineage.hashes(r.L0, r.SOURCE)
    assert hashes['norm_sha256'] == manifest['prompt']['normative_sha256']
    assert hashes['recorded_a'] == hashes['effective_a'][:8]
    review = Path(review_path).resolve()
    tests = D / 'p1338-tests-freeze.json'; attacks = D / 'p1338-attacks-freeze.json'
    red = D / 'p1338-unit-red.json'
    assert r.read('unit-red')['exit'] == 101
    r.save('candidate', dict(at=r.now(), manifest_sha256=r.sha(D / 'p1338-manifest-r1.json'),
        red_path=str(red), red_sha256=r.sha(red), red_review=dict(path=str(review), sha256=r.sha(review)),
        tests_freeze_path=str(tests), tests_freeze_sha256=r.sha(tests),
        attacks_freeze_path=str(attacks), attacks_freeze_sha256=r.sha(attacks),
        attacks_tests_freeze=dict(path=str(D / 'p1338-attacks-freeze-tests.json'), sha256=r.sha(D / 'p1338-attacks-freeze-tests.json')),
        source=source, source_sha256=r.sha(r.ROOT / r.SOURCE), lineage=hashes, state=state,
        delta_from_pre_test_source=''.join(difflib.unified_diff(manifest['pre_candidate_source'].splitlines(True), source.splitlines(True), fromfile='pre-test baseline', tofile='candidate with independent tests'))))
def seal():
    before = lineage.hashes(r.L0, r.SOURCE)
    manifest = r.read('manifest-r1')
    assert before['norm_sha256'] == manifest['prompt']['normative_sha256']
    assert before['recorded_a'] == before['effective_a'][:8]
    old, new = before['recorded_b'], before['code_b'][:8]
    assert old != new
    import subprocess
    patch = f'*** Begin Patch\n*** Update File: {r.ROOT / r.L0}\n@@\n-Hash do Código: {old}\n+Hash do Código: {new}\n*** End Patch\n'
    subprocess.run(['apply_patch'], input=patch, text=True, check=True)
    after = lineage.hashes(r.L0, r.SOURCE)
    assert before['norm_sha256'] == after['norm_sha256'] and after['recorded_b'] == new
    r.save('lineage-reciprocal', dict(at=r.now(), before=before, after=after,
        manifest_sha256=r.sha(D / 'p1338-manifest-r1.json'),
        library=dict(path=str(D / 'p1334-lineage-lib.py'), sha256=r.sha(D / 'p1334-lineage-lib.py'))))
if __name__ == '__main__':
    if sys.argv[1] == 'candidate': candidate(sys.argv[2])
    elif sys.argv[1] == 'seal': seal()
    else: raise ValueError(sys.argv[1])
