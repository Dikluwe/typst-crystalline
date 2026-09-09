"""Close only after functional, architectural and independent review gates."""
import importlib.util
import json
from pathlib import Path
import sys
sys.dont_write_bytecode = True
D = Path(__file__).resolve().parent
spec = importlib.util.spec_from_file_location('r', D / 'p1327-r2-record.py')
r = importlib.util.module_from_spec(spec)
spec.loader.exec_module(r)
s = r.state()
r.verify(s)
previous = json.loads((D / 'p1326-closure.json').read_text())
historical = {**previous['historical_preserved'], **previous['artifacts']}
assert all(r.sha(path) == value for path, value in historical.items())
gates = {}
for name in ['final-build', 'unit-green', 'workspace-tests', 'final-fmt', 'final-lint', 'final-lineage', 'final-lineage-lint', 'final-diff-check', 'cli-candidate', 'cli-supplement']:
    path = D / ('p1327-r2-' + name + '.json')
    receipt = json.loads(path.read_text())
    assert receipt['exit'] == 0, name
    assert receipt['before']['product_inventory'] == receipt['after']['product_inventory'], name
    gates[name] = dict(path=str(path), sha256=r.sha(path), exit=0)
red = json.loads((D / 'p1327-unit-red-r1.json').read_text())
assert red['exit'] == 101 and 'FAILED' in red['stdout'] and 'running 0 tests' not in red['stdout']
assert red['before']['product_inventory'] == red['after']['product_inventory']
freeze = json.loads((D / 'p1327-ab-freeze.json').read_text())
for path, h in freeze['hashes'].items():
    if path not in (r.PROMPT, r.PAIRS[3]):
        assert r.sha(r.ROOT / path) == h, path
# Normative L0 is checked separately by final-lineage; only reciprocal metadata changes.
report = D / 'p1327-final-report.md'
review = D / 'p1327-review-final.md'
assert report.is_file() and review.is_file() and 'PASS_SCOPED' in review.read_text()
ab = json.loads((D / 'p1327-ab-c2-verdict.json').read_text())
assert ab['verdict'] == 'PASS_SCOPED' and not ab['integrity_failures'] and not ab['violations']
assert ab['candidate_sha256'] == r.sha(r.TARGET + '/release/typst')
cli = json.loads((D / 'p1327-ab-cli-candidate-c2.json').read_text())
oracle = json.loads((D / 'p1327-ab-cli-oracle.json').read_text())
assert cli['oracle_sha256'] == r.sha(D / 'p1327-ab-cli-oracle.json')
assert cli['candidate_sha256'] == r.sha(r.TARGET + '/release/typst')
assert cli['fixtures'] == oracle['fixtures']
assert len(cli['runs']) == 336 and all(row['verdict'] == 'Preserved' for row in cli['runs'])
expected = {(x['profile'], x['id']): [x['exit'], x['stdout'], x['stderr']] for x in oracle['expected']}
assert len(expected) == 112
for order in ['normal', 'repeat', 'reverse']:
    rows = [x for x in cli['runs'] if x['order'] == order]
    observed = {(x['profile'], x['id']): [x['exit'], x['stdout'], x['stderr']] for x in rows}
    assert len(rows) == 112 and observed == expected
supplement = json.loads((D / 'p1327-ab-r2-candidate.json').read_text())
supplement_oracle = json.loads((D / 'p1327-ab-r2-oracle.json').read_text())
assert supplement['oracle_sha256'] == r.sha(D / 'p1327-ab-r2-oracle.json')
assert supplement['candidate_sha256'] == cli['candidate_sha256']
assert len(supplement['runs']) == 96 and all(row['verdict'] == 'Preserved' for row in supplement['runs'])
supplement_expected = {(x['profile'], x['id']): [x['exit'], x['stdout'], x['stderr']] for x in supplement_oracle['expected']}
assert len(supplement_expected) == 32
for order in ['normal', 'repeat', 'reverse']:
    rows = [x for x in supplement['runs'] if x['order'] == order]
    observed = {(x['profile'], x['id']): [x['exit'], x['stdout'], x['stderr']] for x in rows}
    assert len(rows) == 32 and observed == supplement_expected
artifacts = {}
for path in sorted(D.glob('p1327-*')):
    if path.is_file():
        artifacts[str(path)] = r.sha(path)
    elif path.is_dir():
        artifacts.update({str(child): r.sha(child) for child in sorted(path.rglob('*')) if child.is_file()})
r.original_save('closure', dict(at=r.now(), state=s, manifest_sha256=r.sha(D / 'p1327-r2-manifest.json'),
    baseline_sha256=r.sha(D / 'p1327-r2-baseline.json'), previous_closure_sha256=r.sha(D / 'p1326-closure.json'),
    historical_preserved=historical, step_sha256=r.sha(r.ROOT / '00_nucleo/materialization/typst-passo-1327.md'),
    gates=gates, artifacts=artifacts, report_sha256=r.sha(report), review_sha256=r.sha(review),
    binary_sha256=r.sha(r.TARGET + '/release/typst'),
    limits=['A/B without technical isolation attestation or refinement seal',
        'Bare Ident Module warning and run_eval Err presentation only; no general parity claim',
        'Redundant rename, unsupported import type, import trace and native math identity remain outside',
        'Previous dirty product and evidence preserved; no stage, commit or push']))
