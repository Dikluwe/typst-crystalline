"""Close only the P1325 scope; preserve prior work and receipts without commit."""
import importlib.util
import json
from pathlib import Path
import sys
sys.dont_write_bytecode = True
D = Path(__file__).resolve().parent
spec = importlib.util.spec_from_file_location('record', D / 'p1325-record.py')
r = importlib.util.module_from_spec(spec)
spec.loader.exec_module(r)
s = r.state()
r.verify(s)
previous = json.loads((D / 'p1324-closure.json').read_text())
historical = {**previous['historical_preserved'], **previous['artifacts']}
assert all(r.sha(path) == value for path, value in historical.items())
gates = {}
for name in ['final-build-r1', 'unit-green', 'test-succession-focal', 'workspace-tests-r1', 'final-fmt-r1', 'final-lint-r1', 'final-lineage-r1', 'test-succession-lineage', 'final-lineage-lint-r1', 'final-diff-check-r1']:
    path = D / ('p1325-' + name + '.json')
    receipt = json.loads(path.read_text())
    assert receipt['exit'] == 0, name
    assert receipt['before']['product_inventory'] == receipt['after']['product_inventory'], name
    gates[name] = dict(path=str(path), sha256=r.sha(path), exit=receipt['exit'])
red = json.loads((D / 'p1325-unit-red-r1.json').read_text())
assert red['exit'] == 101
assert red['before']['product_inventory'] == red['after']['product_inventory']
assert 'FAILED' in red['stdout'] and 'running 0 tests' not in red['stdout']
report = D / 'p1325-final-report.md'
review = D / 'p1325-review-final.md'
assert report.is_file() and review.is_file()
assert 'PASS' in review.read_text()
ab = json.loads((D / 'p1325-ab-verdict.json').read_text())
assert ab['status'] == 'PASS' and ab['compared_rows'] == 1008 and not ab['failures'] and not ab['changed_protected']
freeze = json.loads((D / 'p1325-ab-freeze.json').read_text())
assert all(r.sha(r.ROOT / path) == h for path,h in freeze['protected'].items())
assert r.sha(r.TARGET + '/release/typst') == '3511b08aa89d088e908dd239d8942f1eeea1978d31140ef9510e81de06023dab'
artifacts = {}
for path in sorted(D.glob('p1325-*')):
    if path.is_file():
        artifacts[str(path)] = r.sha(path)
    elif path.is_dir():
        artifacts.update({str(child): r.sha(child) for child in sorted(path.rglob('*')) if child.is_file()})
r.save('closure', dict(at=r.now(), state=s,
    manifest_sha256=r.sha(D / 'p1325-manifest.json'), baseline_sha256=r.sha(D / 'p1325-baseline.json'),
    previous_closure_sha256=r.sha(D / 'p1324-closure.json'), historical_preserved=historical,
    test_succession_manifest_sha256=r.sha(D / 'p1325-test-succession-manifest.json'), ab_verdict_sha256=r.sha(D / 'p1325-ab-verdict.json'),
    step_sha256=r.sha(r.ROOT / '00_nucleo/materialization/typst-passo-1325.md'),
    gates=gates, artifacts=artifacts, report_sha256=r.sha(report), review_sha256=r.sha(review),
    limits=['A/B without technical isolation attestation or refinement seal',
            'Direct Dict/raw Content/Float diagnostic anchor only',
            'math native identity and raw text lookup remain open',
            'Previous dirty product and evidence preserved; no stage, commit or push']))
