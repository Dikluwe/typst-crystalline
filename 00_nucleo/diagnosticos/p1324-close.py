"""Pin final P1324 evidence, preserving previous work without commit or cleanup."""
import importlib.util
import json
from pathlib import Path
import sys
sys.dont_write_bytecode = True
D = Path(__file__).resolve().parent
spec = importlib.util.spec_from_file_location('record', D / 'p1324-record.py')
r = importlib.util.module_from_spec(spec)
spec.loader.exec_module(r)
s = r.state()
r.verify(s)
previous = json.loads((D / 'p1323-closure.json').read_text())
historical = {**previous['historical_preserved'], **previous['artifacts']}
assert all(r.sha(path) == value for path, value in historical.items())
gates = {}
for name in ['final-build','unit-green','workspace-tests','final-fmt','final-lint','final-lineage','final-diff-check']:
    path = D / ('p1324-' + name + '.json')
    receipt = json.loads(path.read_text())
    assert receipt['exit'] == 0
    gates[name] = dict(path=str(path), sha256=r.sha(path), exit=receipt['exit'])
red = json.loads((D / 'p1324-unit-red-r2.json').read_text())
assert red['exit'] == 101
assert red['before']['product_inventory'] == red['after']['product_inventory']
report = D / 'p1324-final-report.md'
assert 'aguardando gates' not in report.read_text()
review = D / 'p1324-review-final.md'
assert review.is_file()
ab = json.loads((D / 'p1324-ab-candidate-v3.json').read_text())
assert ab['counts']['obligation'] == {'Preserved':588, 'Violated':12}
assert ab['counts']['stability'] == {'Preserved':400}
changed = [row for row in ab['runs'] if row['vs_baseline']['status'] == 'Violated']
assert len(changed) == 204 and all(row['vs_vanilla']['status'] == 'Preserved' for row in changed)
remaining = [row for row in ab['runs'] if row['obligation']['status'] == 'Violated']
assert len(remaining) == 12 and all(row['case']=='json-callee-before-argument' and row['vs_baseline']['status']=='Preserved' for row in remaining)
artifacts = {}
for path in sorted(D.glob('p1324-*')):
    if path.is_file():
        artifacts[str(path)] = r.sha(path)
    elif path.is_dir():
        artifacts.update({str(child):r.sha(child) for child in sorted(path.rglob('*')) if child.is_file()})
# Exact new plan authored this turn; never scan historical step directories.
step = r.ROOT / '00_nucleo/materialization/typst-passo-1324.md'
r.save('closure', dict(at=r.now(), state=s,
    manifest_sha256=r.sha(D / 'p1324-manifest.json'), baseline_sha256=r.sha(D / 'p1324-baseline.json'),
    previous_closure_sha256=r.sha(D / 'p1323-closure.json'), historical_preserved=historical,
    step_sha256=r.sha(step), gates=gates, artifacts=artifacts,
    report_sha256=r.sha(report), review_sha256=r.sha(review),
    ab_raw_counts=ab['counts'], ab_changed_and_equal_to_vanilla=204,
    ab_order_supplement_sha256=r.sha(D / 'p1324-ab-order-preservation-supplement.json'),
    limits=['No technical isolation attestation or refinement seal',
            'Only namespace-native missing-field diagnostic parity',
            'A/B successor scope classification revised after C existed, before its author observed C',
            'First zero-test run is instrumentation failure, not RED',
            'Previous dirty product and evidence preserved; no stage, commit or push']))
