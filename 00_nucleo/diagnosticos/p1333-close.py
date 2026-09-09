"""Immutable scoped closure after independent verdicts and all final gates."""
import importlib.util
import json
from pathlib import Path
import re
import sys
sys.dont_write_bytecode=True
D=Path(__file__).resolve().parent
spec=importlib.util.spec_from_file_location('r',D/'p1333-record.py')
r=importlib.util.module_from_spec(spec);spec.loader.exec_module(r)
s=r.state();r.verify(s);m=json.loads((D/'p1333-manifest.json').read_text())
baseline=json.loads((D/'p1333-baseline.json').read_text())
assert r.sha(D/'p1332-closure.json')==m['previous_closure_sha256']
assert r.sha(D/'p1333-baseline.json')==m['baseline_sha256']
assert all(r.sha(p)==h for p,h in baseline['historical_preserved'].items())
assert r.sha(r.ROOT/m['step']['path'])==m['step']['sha256']
i=json.loads((D/'p1333-test-integration.json').read_text())
assert all(r.sha(p)==h for p,h in i['frozen'].items())
f=json.loads((D/'p1333-ab-freeze.json').read_text())
assert all(r.sha(D/p)==h for p,h in {**f['inputs'],**f['artifacts']}.items())
assert f['manifest_sha256']==r.sha(D/'p1333-manifest.json')
assert f['l0_norm_sha256']==m['prompt_norm_sha256']
gates={}
for name in ['unit-green-final','final-build','workspace-tests','final-fmt-r1','final-lint-r1',
             'final-lineage-r1','final-lineage-lint-r1','final-diff-check-r1',
             'cli-normal','cli-repeat','cli-reverse']:
    p=D/('p1333-'+name+'.json');g=json.loads(p.read_text())
    assert g['exit']==0,name
    assert g['before']['product_inventory']==g['after']['product_inventory']==s['product_inventory'],name
    assert g['manifest_sha256']==r.sha(D/'p1333-manifest.json'),name
    gates[name]=dict(path=str(p),sha256=r.sha(p),exit=0)
red=json.loads((D/'p1333-unit-red.json').read_text())
assert red['exit']==101 and 'FAILED' in red['stdout']
assert any(int(n)>0 for n in re.findall(r'running (\d+) tests',red['stdout']))
binary=r.sha(m['target']+'/release/typst')
expected=json.loads((D/'p1333-ab-cli-expected.json').read_text())
index={(x['case'],x['profile']):x['expected'] for x in expected['expectations']}
assert len(index)==len(expected['expectations']) and len(index)>=504
for order in ['normal','repeat','reverse']:
    cli=json.loads((D/('p1333-ab-cli-'+order+'.json')).read_text())
    assert cli['manifest_sha256']==r.sha(D/'p1333-manifest.json')
    assert cli['l0_norm_sha256']==m['prompt_norm_sha256']
    assert cli['binaries']['CANDIDATE']['sha256']==binary
    outputs={(x['case'],x['profile']):{k:x['results']['CANDIDATE'][k] for k in ('exit','stdout','stderr')} for x in cli['cases']}
    assert len(cli['cases'])==len(outputs)==len(index) and outputs==index
report=D/'p1333-final-report.md';review=D/'p1333-review-final.md';ab=D/'p1333-ab-receipt.md'
assert 'PASS_SCOPED' in review.read_text() and 'PASS' in ab.read_text()
assert 'este rascunho ainda não constitui fechamento' not in report.read_text()
artifacts={str(p):r.sha(p) for p in sorted(D.glob('p1333-*')) if p.is_file()}
r.save('closure',dict(at=r.now(),state=s,manifest_sha256=r.sha(D/'p1333-manifest.json'),
    baseline_sha256=r.sha(D/'p1333-baseline.json'),previous_closure_sha256=r.sha(D/'p1332-closure.json'),
    historical_preserved=baseline['historical_preserved'],artifacts=artifacts,gates=gates,
    binary_sha256=binary,report_sha256=r.sha(report),review_sha256=r.sha(review),ab_review_sha256=r.sha(ab),
    limits=['First-invalid positional precedence only; no-value/valid-first guards remain debt',
    'No whole-call span or dispatcher change; no full argument parity',
    'A/B without technical isolation attestation or refinement seal','No stage, commit or push']))
