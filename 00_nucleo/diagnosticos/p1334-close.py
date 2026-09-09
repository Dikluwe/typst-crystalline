"""Immutable scoped closure after independent verdicts and all final gates."""
import importlib.util
import json
from pathlib import Path
import re
import sys
sys.dont_write_bytecode=True
D=Path(__file__).resolve().parent
spec=importlib.util.spec_from_file_location('r',D/'p1334-record.py')
r=importlib.util.module_from_spec(spec);spec.loader.exec_module(r)
s=r.state();r.verify(s);m=json.loads((D/'p1334-manifest.json').read_text())
baseline=json.loads((D/'p1334-baseline.json').read_text())
assert r.sha(D/'p1333-closure.json')==m['previous_closure_sha256']
assert r.sha(D/'p1334-baseline.json')==m['baseline_sha256']
assert all(r.sha(p)==h for p,h in baseline['historical_preserved'].items())
assert r.sha(r.ROOT/m['step']['path'])==m['step']['sha256']
i=json.loads((D/'p1334-test-integration-r1.json').read_text())
assert all(r.sha(p)==h for p,h in i['frozen'].items())
f=json.loads((D/'p1334-ab-freeze-r1.json').read_text())
assert all(r.sha(D/p)==h for p,h in {**f['inputs'],**f['artifacts']}.items())
assert f['manifest_sha256']==r.sha(D/'p1334-manifest.json')
assert f['l0_norm_sha256']==m['prompt_norm_sha256']
gates={}
for name in ['unit-green','final-build','workspace-tests','final-fmt','final-lint',
             'final-lineage','final-lineage-lint','final-diff-check',
             'cli-normal','cli-repeat','cli-reverse',
             'cross-cli-normal','cross-cli-repeat','cross-cli-reverse']:
    p=D/('p1334-'+name+'.json');g=json.loads(p.read_text())
    assert g['exit']==0,name
    assert g['before']['product_inventory']==g['after']['product_inventory']==s['product_inventory'],name
    assert g['manifest_sha256']==r.sha(D/'p1334-manifest.json'),name
    gates[name]=dict(path=str(p),sha256=r.sha(p),exit=0)
red=json.loads((D/'p1334-unit-red-r1.json').read_text())
assert red['exit']==101 and 'FAILED' in red['stdout']
assert any(int(n)>0 for n in re.findall(r'running (\d+) tests',red['stdout']))
binary=r.sha(m['target']+'/release/typst')
expected=json.loads((D/'p1334-ab-cli-expected.json').read_text())
index={(x['case'],x['profile']):x['expected'] for x in expected['expectations']}
assert len(index)==len(expected['expectations']) and len(index)>=712
for order in ['normal','repeat','reverse']:
    cli=json.loads((D/('p1334-ab-cli-'+order+'.json')).read_text())
    assert cli['manifest_sha256']==r.sha(D/'p1334-manifest.json')
    assert cli['l0_norm_sha256']==m['prompt_norm_sha256']
    assert cli['candidate_binary']['sha256']==binary
    assert cli['expected_sha256']==r.sha(D/'p1334-ab-cli-expected.json')
    assert cli['cache_sha256']==r.sha(D/'p1334-ab-cli-baseline.json')
    outputs={(x['case'],x['profile']):{k:x['results']['CANDIDATE'][k] for k in ('exit','stdout','stderr')} for x in cli['cases']}
    assert len(cli['cases'])==len(outputs)==len(index) and outputs==index
report=D/'p1334-final-report.md';review=D/'p1334-review-final.md';ab=D/'p1334-ab-receipt.md'
cross=json.loads((D/'p1334-ab-cross-expected.json').read_text())
cross_index={(x['case'],x['profile']):x['expected'] for x in cross['expectations']}
assert len(cross_index)==len(cross['expectations'])==12
for order in ['normal','repeat','reverse']:
    cli=json.loads((D/('p1334-ab-cross-cli-'+order+'.json')).read_text())
    assert cli['manifest_sha256']==r.sha(D/'p1334-manifest.json')
    assert cli['candidate_binary']['sha256']==binary
    assert cli['expected_sha256']==r.sha(D/'p1334-ab-cross-expected.json')
    assert cli['cache_sha256']==r.sha(D/'p1334-ab-cross-baseline.json')
    assert cli['fixture']['sha256']==r.sha(D/'p1334-ab-fixtures/origins.typ')
    outputs={(x['case'],x['profile']):{k:x['results']['CANDIDATE'][k] for k in ('exit','stdout','stderr')} for x in cli['cases']}
    assert len(cli['cases'])==len(outputs)==len(cross_index) and outputs==cross_index
assert 'PASS_SCOPED' in review.read_text() and 'PASS' in ab.read_text()
assert 'este rascunho ainda não constitui fechamento' not in report.read_text()
artifacts={str(p):r.sha(p) for p in sorted(D.glob('p1334-*')) if p.is_file()}
artifacts.update({str(p):r.sha(p) for p in sorted((D/'p1334-ab-fixtures').rglob('*')) if p.is_file()})
r.save('closure',dict(at=r.now(),state=s,manifest_sha256=r.sha(D/'p1334-manifest.json'),
    baseline_sha256=r.sha(D/'p1334-baseline.json'),previous_closure_sha256=r.sha(D/'p1333-closure.json'),
    historical_preserved=baseline['historical_preserved'],artifacts=artifacts,gates=gates,
    binary_sha256=binary,report_sha256=r.sha(report),review_sha256=r.sha(review),ab_review_sha256=r.sha(ab),
    limits=['Native abs signature diagnostics completed in coherent argument domain; no general parity claim',
    'Dispatcher only resolved abs identity span; internal facade reexport; imported math resolution remains debt',
    'A/B without technical isolation attestation or refinement seal','No stage, commit or push']))
