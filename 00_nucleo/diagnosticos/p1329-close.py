"""Aggregate P1329 gates without weakening frozen expectations."""
import importlib.util
import json
from pathlib import Path
import re
import sys
sys.dont_write_bytecode = True
D = Path(__file__).resolve().parent
spec = importlib.util.spec_from_file_location('r', D / 'p1329-record.py')
r = importlib.util.module_from_spec(spec)
spec.loader.exec_module(r)
s = r.state()
r.verify(s)
previous = json.loads((D / 'p1328-closure.json').read_text())
frozen_manifest = json.loads((D / 'p1329-manifest-r2.json').read_text())
assert r.sha(D / 'p1328-closure.json') == frozen_manifest['previous_closure_sha256']
assert r.sha(D / 'p1329-baseline.json') == frozen_manifest['baseline_sha256']
historical = {**previous['historical_preserved'], **previous['artifacts']}
assert all(r.sha(path) == h for path, h in historical.items())
gates = {}
for name in ['final-build', 'unit-green', 'workspace-tests', 'final-fmt', 'final-lint',
             'final-lineage', 'final-lineage-lint', 'final-diff-check',
             'cli-normal', 'cli-repeat', 'cli-reverse']:
    path = D / ('p1329-' + name + '.json')
    receipt = json.loads(path.read_text())
    assert receipt['exit'] == 0, name
    assert receipt['before']['product_inventory'] == receipt['after']['product_inventory'], name
    assert receipt['after']['product_inventory'] == s['product_inventory'], name
    assert receipt['manifest_sha256'] == r.sha(D / 'p1329-manifest-r2.json'), name
    gates[name] = dict(path=str(path), sha256=r.sha(path), exit=0)
red = json.loads((D / 'p1329-unit-red.json').read_text())
assert red['exit'] == 101 and 'FAILED' in red['stdout']
assert any(int(n) > 0 for n in re.findall(r'running (\d+) tests', red['stdout']))
integration = json.loads((D / 'p1329-test-integration.json').read_text())
assert all(r.sha(path) == h for path, h in integration['frozen'].items())
history = json.loads((D / 'p1329-historical-expectations.json').read_text())
assert history['pass_gate'] and len(history['changed']) == 16 and history['unchanged'] == 96
assert history['expected_sha256'] == r.sha(D / 'p1329-ab-cli-expected-r2.json')
frozen = json.loads((D / 'p1329-ab-cli-expected-r2.json').read_text())
expected = {(x['case'], x['profile']): x['expected'] for x in frozen['expectations']}
assert len(expected) == len(frozen['expectations']) and len(expected) > 112
binary_hash = r.sha(r.TARGET + '/release/typst')
domain = json.loads((D / 'p1329-domain-final.json').read_text())
assert domain['exit'] == 0
assert domain['before']['product_inventory'] == domain['after']['product_inventory'] == s['product_inventory']
assert domain['manifest_sha256'] == r.sha(D / 'p1329-manifest-r2.json')
assert domain['binary']['sha256'] == binary_hash
for order in ['normal', 'repeat', 'reverse']:
    cli = json.loads((D / ('p1329-ab-cli-' + order + '.json')).read_text())
    assert cli['binaries']['CANDIDATE']['sha256'] == binary_hash
    assert len(cli['cases']) == len(expected)
    observed = {}
    for row in cli['cases']:
        assert row['candidate_matches_frozen_policy']
        output = row['results']['CANDIDATE']
        observed[row['case'], row['profile']] = {k: output[k] for k in ('exit','stdout','stderr')}
    assert observed == expected
manifest = json.loads((D / 'p1329-manifest-r2.json').read_text())
step_hash = r.sha(r.ROOT / '00_nucleo/materialization/typst-passo-1329.md')
assert step_hash == manifest['step_sha256']
report = D / 'p1329-final-report.md'
review = D / 'p1329-review-final.md'
assert report.is_file() and review.is_file() and 'PASS_SCOPED' in review.read_text()
ab_review = D / 'p1329-ab-receipt.md'
assert ab_review.is_file() and 'PASS' in ab_review.read_text()
artifacts = {}
for path in sorted(D.glob('p1329-*')):
    if path.is_file():
        artifacts[str(path)] = r.sha(path)
    elif path.is_dir():
        artifacts.update({str(p):r.sha(p) for p in sorted(path.rglob('*')) if p.is_file()})
r.save('closure', dict(at=r.now(), state=s, manifest_sha256=r.sha(D/'p1329-manifest-r2.json'),
    baseline_sha256=r.sha(D/'p1329-baseline.json'), previous_closure_sha256=r.sha(D/'p1328-closure.json'),
    historical_preserved=historical, artifacts=artifacts, gates=gates,
    step_sha256=step_hash, report_sha256=r.sha(report), review_sha256=r.sha(review),
    ab_review_sha256=r.sha(ab_review),
    binary_sha256=binary_hash, limits=['A/B without technical isolation attestation or refinement seal',
    'Only dimensional calc.abs values and mixed length diagnostics; no general calc parity',
    'External trace names, upstream construction/operations, other types, Int overflow and guards retain explicit baseline debts',
    'Previous dirty files and evidence preserved; no stage, commit or push']))
