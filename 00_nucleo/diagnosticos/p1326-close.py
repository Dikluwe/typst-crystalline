"""Pin P1326 closure while preserving all prior evidence and unrelated product."""
import importlib.util
import json
from pathlib import Path
import sys
sys.dont_write_bytecode = True
D = Path(__file__).resolve().parent
spec = importlib.util.spec_from_file_location('record', D / 'p1326-record.py')
r = importlib.util.module_from_spec(spec)
spec.loader.exec_module(r)
s = r.state()
r.verify(s)
previous = json.loads((D / 'p1325-closure.json').read_text())
historical = {**previous['historical_preserved'], **previous['artifacts']}
assert all(r.sha(path) == value for path, value in historical.items())
gates = {}
for name in ['final-build', 'unit-green', 'workspace-tests', 'final-fmt', 'final-lint', 'final-lineage', 'final-lineage-lint', 'final-diff-check']:
    path = D / ('p1326-' + name + '.json')
    receipt = json.loads(path.read_text())
    assert receipt['exit'] == 0, name
    assert receipt['before']['product_inventory'] == receipt['after']['product_inventory'], name
    gates[name] = dict(path=str(path), sha256=r.sha(path), exit=receipt['exit'])
red = json.loads((D / 'p1326-unit-red-r1.json').read_text())
assert red['exit'] == 101 and 'FAILED' in red['stdout'] and 'running 0 tests' not in red['stdout']
assert red['before']['product_inventory'] == red['after']['product_inventory']
report = D / 'p1326-final-report.md'
review = D / 'p1326-review-final.md'
assert report.is_file() and review.is_file() and 'PASS' in review.read_text()
assert 'ainda pendentes' not in report.read_text()
freeze = json.loads((D / 'p1326-ab-freeze.json').read_text())
assert all(r.sha(r.ROOT / path) == h for path,h in freeze['artifacts'].items())
ab = json.loads((D / 'p1326-ab-verdict.json').read_text())
assert ab['verdict'] == 'PASS_SCOPED' and ab['full']['counts'] == {'Preserved': 276}
assert ab['unknown'] == 0 and ab['candidate_repeat_reverse_stable_keys'] == 92
assert all(ab['integrity_checks'].values())
assert all(r.sha(r.ROOT / path) == h for path,h in ab['artifacts'].items())
assert r.sha(r.TARGET + '/release/typst') == ab['candidate']['sha256']
artifacts = {}
for path in sorted(D.glob('p1326-*')):
    if path.is_file():
        artifacts[str(path)] = r.sha(path)
    elif path.is_dir():
        artifacts.update({str(child): r.sha(child) for child in sorted(path.rglob('*')) if child.is_file()})
r.save('closure', dict(at=r.now(), state=s,
    manifest_sha256=r.sha(D / 'p1326-manifest.json'), baseline_sha256=r.sha(D / 'p1326-baseline.json'),
    previous_closure_sha256=r.sha(D / 'p1325-closure.json'), historical_preserved=historical,
    step_sha256=r.sha(r.ROOT / '00_nucleo/materialization/typst-passo-1326.md'),
    gates=gates, artifacts=artifacts, report_sha256=r.sha(report), review_sha256=r.sha(review),
    ab_verdict_sha256=r.sha(D / 'p1326-ab-verdict.json'),
    binary_sha256=r.sha(r.TARGET + '/release/typst'),
    limits=['A/B without technical isolation attestation or refinement seal',
            'Closure/With missing-field diagnostic only; no general parity claim',
            'Argument-before-field order, math native identity and raw text lookup remain open',
            'Previous dirty product and evidence preserved; no stage, commit or push']))
