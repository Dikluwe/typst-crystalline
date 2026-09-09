"""Close P1331 only when frozen gates and independent reviews agree."""
import importlib.util
import json
from pathlib import Path
import re
import sys
sys.dont_write_bytecode = True
D = Path(__file__).resolve().parent
spec = importlib.util.spec_from_file_location('r',D/'p1331-record.py')
r = importlib.util.module_from_spec(spec); spec.loader.exec_module(r)
s = r.state(); r.verify(s)
m = json.loads((D/'p1331-manifest.json').read_text())
assert r.sha(D/'p1330-closure.json') == m['previous_closure_sha256']
assert r.sha(D/'p1331-baseline.json') == m['baseline_sha256']
baseline = json.loads((D/'p1331-baseline.json').read_text())
historical = baseline['historical_preserved']
assert all(r.sha(path) == h for path,h in historical.items())
gates = {}
for name in ['final-build','unit-green','workspace-tests','final-fmt','final-lint',
             'final-lineage','final-lineage-lint','final-diff-check',
             'cli-normal','cli-repeat','cli-reverse']:
    path = D/('p1331-'+name+'.json')
    receipt = json.loads(path.read_text())
    assert receipt['exit'] == 0, name
    assert receipt['before']['product_inventory'] == receipt['after']['product_inventory'] == s['product_inventory'],name
    assert receipt['manifest_sha256'] == r.sha(D/'p1331-manifest.json'),name
    gates[name] = dict(path=str(path),sha256=r.sha(path),exit=0)
red = json.loads((D/'p1331-unit-red.json').read_text())
assert red['exit'] == 101 and 'FAILED' in red['stdout']
assert any(int(n)>0 for n in re.findall(r'running (\d+) tests',red['stdout']))
integration = json.loads((D/'p1331-test-integration.json').read_text())
assert all(r.sha(path) == h for path,h in integration['frozen'].items())
history = json.loads((D/'p1331-historical-expectations.json').read_text())
assert history['pass_gate'] and len(history['changed']) == 8 and history['unchanged'] == 324
assert history['expected_sha256'] == r.sha(D/'p1331-ab-cli-expected.json')
frozen = json.loads((D/'p1331-ab-cli-expected.json').read_text())
expected = {(x['case'],x['profile']):x['expected'] for x in frozen['expectations']}
assert len(expected) == len(frozen['expectations']) and len(expected)>332
binary_hash = r.sha(m['target']+'/release/typst')
for order in ['normal','repeat','reverse']:
    cli = json.loads((D/('p1331-ab-cli-'+order+'.json')).read_text())
    assert cli['binaries']['CANDIDATE']['sha256'] == binary_hash
    assert cli['manifest_sha256'] == r.sha(D/'p1331-manifest.json')
    assert cli['l0_norm_sha256'] == m['prompt_norm_sha256']
    assert len(cli['cases']) == len(expected)
    outputs = {}
    for row in cli['cases']:
        assert row['candidate_matches_frozen_policy']
        output = row['results']['CANDIDATE']
        outputs[row['case'],row['profile']] = {k:output[k] for k in ('exit','stdout','stderr')}
    assert outputs == expected
assert r.sha(r.ROOT/m['step']['path']) == m['step']['sha256']
report,review,ab_review = [D/('p1331-'+name) for name in ['final-report.md','review-final.md','ab-receipt.md']]
assert report.is_file() and 'PASS_SCOPED' in review.read_text()
assert 'PASS' in ab_review.read_text()
artifacts = {str(p):r.sha(p) for p in sorted(D.glob('p1331-*')) if p.is_file()}
r.save('closure',dict(at=r.now(),state=s,manifest_sha256=r.sha(D/'p1331-manifest.json'),
    baseline_sha256=r.sha(D/'p1331-baseline.json'),previous_closure_sha256=r.sha(D/'p1330-closure.json'),
    historical_preserved=historical,artifacts=artifacts,gates=gates,step_sha256=m['step']['sha256'],
    report_sha256=r.sha(report),review_sha256=r.sha(review),ab_review_sha256=r.sha(ab_review),
    binary_sha256=binary_hash,limits=['Only calc.abs fallback diagnostic; no accepted types or general parity',
    'A/B without technical isolation attestation or refinement seal',
    'Parser, guards, traces and dimensional construction debts preserved',
    'No stage, commit or push; prior dirty files preserved']))
