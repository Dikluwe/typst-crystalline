"""Reconcile measured states without overwriting raw evidence or hiding gaps."""
from collections import Counter
import importlib.util
import json
from pathlib import Path
import sys

spec = importlib.util.spec_from_file_location('audit', Path(__file__).with_name('p1320-audit.py'))
a = importlib.util.module_from_spec(spec)
spec.loader.exec_module(a)
out = Path(sys.argv[1])
if out.exists():
    raise FileExistsError(out)
names = ['p1320-measurement.json', 'p1320-focal-r1.json', 'p1320-supplement.json',
         'p1320-harness-tests-r1.json']
raw, focal, supplement, tests = [json.loads((a.D / n).read_text()) for n in names]
before = a.snapshot()
assert all(x['unchanged'] for x in [raw, focal, supplement])
assert all(x['exit'] == 0 for x in tests['tests'])
for path, expected in raw['before']['inputs'].items():
    actual = a.D / 'p1320-audit-r0.py' if path.endswith('/p1320-audit.py') else a.ROOT / path
    assert a.sha(actual) == expected, path
for receipt in [focal, supplement]:
    for path, expected in receipt['before']['inputs'].items():
        assert a.sha(a.ROOT / path) == expected, path
rows = []
for row in focal['focal']:
    state = a.classify(row['vanilla'], row['crystalline'], row['case']['observation'])
    assert state == row['state']
    reason = 'direct measurement'
    if row['case']['id'] == 'json-roundtrip':
        fixed = next(r for r in supplement['rows'] if r['id'] == 'json-roundtrip-corrected' and r['profile'] == row['profile'])
        state = a.classify(fixed['vanilla'], fixed['crystalline'], 'value')
        assert state == fixed['state'] == 'MATCH_VALUE'
        reason = 'replaced invalid filename probe by explicit bytes roundtrip; raw retained'
    elif row['state'] == 'UNKNOWN_BASELINE' and row['case']['id'] in ['html-binding', 'a11y-binding']:
        assert row['vanilla']['exit'] == row['crystalline']['exit'] == 1
        assert row['vanilla']['stderr'] == row['crystalline']['stderr']
        assert 'feature is not enabled' in row['vanilla']['stderr']
        assert any(r['case']['id'] == row['case']['id'] and r['state'] == 'MATCH_VALUE' for r in focal['focal'])
        state = 'DISABLED_BY_PROFILE'
        reason = 'measured identical disabled diagnostic plus successful active peer; no success credit'
    rows.append(dict(id=row['case']['id'], profile=row['profile'], state=state, reason=reason))
assert len(rows) == len({(r['id'], r['profile']) for r in rows}) == 204
matrix = []
for item in raw['matrix']:
    assert item['result']['order_check']['identical']
    assert item['result']['total'] == len(item['result']['results']) == 20
    matrix.append(dict(profile=item['profile'], counts=item['result']['counts'],
                       exit=item['run']['exit'], exit_meaning='expected historical states met, not zero differences'))
gates = [a.invoke(['git', 'diff', '--check']), a.invoke(['crystalline-lint', '.'])]
after = a.snapshot()
assert before['inputs'] == after['inputs'] and before['binaries'] == after['binaries']
a.save(out, dict(schema='p1320-reconciliation/v1', before=before, after=after,
    input_receipts={n: a.sha(a.D / n) for n in names}, script_sha256=a.sha(__file__),
    focal_rows=rows, focal_counts=dict(Counter(r['state'] for r in rows)), matrix=matrix,
    supplement_rows=[dict(id=r['id'], profile=r['profile'], state=r['state']) for r in supplement['rows']],
    gates=gates, previous_inputs_intact=True,
    discarded_as_product_evidence='initial focal and combined compile in p1320-measurement.json: --color invocation defect',
    verdict='AUDIT_COMPLETE_PARITY_OPEN',
    limitation='same-author read-only diagnostic; no global denominator and no independent seal'))
print(out, a.sha(out))
print(dict(Counter(r['state'] for r in rows)))
sys.exit(0 if all(g['exit'] == 0 for g in gates) else 1)
