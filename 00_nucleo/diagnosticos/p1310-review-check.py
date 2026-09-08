#!/usr/bin/env python3
"""Independent read-only check of P1310 evidence; writes only reviewer receipt."""
import base64
from collections import Counter
import datetime
import hashlib
import json
from pathlib import Path
import re
import subprocess

ROOT = Path(__file__).resolve().parents[2]
D = ROOT / '00_nucleo/diagnosticos'
inputs = {}

def sha(path):
    return hashlib.sha256(Path(path).read_bytes()).hexdigest()

def read(name):
    path = D / name
    inputs[name] = sha(path)
    return json.loads(path.read_text())

def git(*args):
    return subprocess.check_output(['git', *args], cwd=ROOT, text=True)

b = read('p1310-baseline.json')
r1 = read('p1310-ab-frozen-r1.json')
r2 = read('p1310-ab-frozen-r2.json')
assert r1['baseline_receipt_sha256'] == inputs['p1310-baseline.json']
assert r2['preceding_frozen_sha256'] == inputs['p1310-ab-frozen-r1.json']
assert not r2['all_expectations_frozen_before_candidate']
assert r2['post_candidate_policy_correction']
assert r2['candidate_source_read'] is False
assert sha(D / 'p1310-ab-suite.py') == r2['suite_sha256']
assert sha(D / 'p1310-ab-r2.py') == r2['r2_runner_sha256']
for name, item in r2['fixtures'].items():
    assert sha(Path(r2['fixture_dir']) / name) == item['sha256']
l0 = ROOT / r1['L0_path']
restored = re.sub(r'(?m)^Hash do Código: .*$', 'Hash do Código: 4a491c50', l0.read_text())
assert hashlib.sha256(restored.encode()).hexdigest() == r1['L0_sha256']
measurement = read('p1310-ab-freeze-r1-measurement.json')
old = {c['id']: c for c in r1['cases']}
new = {c['id']: c for c in r2['cases']}
assert old.keys() == new.keys()
baseline_rows = {(r['case'], r['profile']): r['observable'] for r in measurement['rows'] if r['side'] == 'baseline'}
changed = []
for key in old:
    if old[key] == new[key]:
        continue
    assert key.split('.')[-1] in {'named-prefix', 'with-named-prefix', 'args-named-prefix'}
    assert {f for f in old[key].keys() | new[key].keys() if old[key].get(f) != new[key].get(f)} <= {'expected', 'policy', 'policy_reason', 'required_message'}
    assert new[key]['policy'] == 'baseline'
    for profile, expected in new[key]['expected'].items():
        assert expected == baseline_rows[key, profile]
    changed.append(key)
assert len(changed) == 15
expected_keys = {(c, p) for c in new for p in r2['profiles']}
orders = {}
verified_binaries = {}
for phase in ['normal', 'repeat', 'reverse']:
    result = read('p1310-ab-r2-' + phase + '.json')
    assert result['frozen_sha256'] == inputs['p1310-ab-frozen-r2.json']
    rows = {(r['case'], r['profile']): r for r in result['rows']}
    assert len(rows) == len(result['rows']) and rows.keys() == expected_keys
    assert result['counts'] == {'Preserved': len(rows)}
    for key, row in rows.items():
        assert row['observable']['kind'] != 'Unknown'
        assert row['observable'] == new[key[0]]['expected'][key[1]]
        assert row['verdict'] == 'Preserved'
        for channel in ['stdout', 'stderr']:
            raw = base64.b64decode(row[channel + '_base64'])
            assert hashlib.sha256(raw).hexdigest() == row[channel + '_sha256']
            assert raw.decode() == row['observable'][channel]
        binary_path = row['binary']['path']
        if binary_path not in verified_binaries:
            verified_binaries[binary_path] = sha(binary_path)
        assert verified_binaries[binary_path] == row['binary']['sha256']
    orders[phase] = rows
for key in expected_keys:
    for phase in ['repeat', 'reverse']:
        for field in ['observable', 'exit', 'stdout_base64', 'stderr_base64', 'binary', 'verdict']:
            assert orders[phase][key][field] == orders['normal'][key][field]
red = read('p1310-unit-red.json')
green = read('p1310-unit-green.json')
assert red['exit'] == 101 and '1 passed; 3 failed;' in red['stdout']
assert green['exit'] == 0 and '4 passed; 0 failed;' in green['stdout']
assert r1['at'] < green['before']['at']
for name in ['p1310-build.json', 'p1310-workspace-tests.json', 'p1310-lint.json', 'p1310-fmt.json', 'p1310-diff-check.json', 'p1310-lineage-preflight.json']:
    receipt = read(name)
    assert receipt['exit'] == 0
    assert receipt['baseline_sha256'] == inputs['p1310-baseline.json']
    assert receipt['before']['head'] == b['state']['head'] == receipt['after']['head']
workspace = read('p1310-workspace-tests.json')
totals = [tuple(map(int, m)) for m in re.findall(r'test result: ok\. (\d+) passed; (\d+) failed; (\d+) ignored;', workspace['stdout'])]
test_totals = [sum(row[i] for row in totals) for i in range(3)]
assert test_totals == [6624, 0, 3]
lint = read('p1310-lint.json')
lint_counts = {k: len(re.findall('(?m)^' + k + ':', lint['stdout'])) for k in ['error', 'warning', 'info']}
assert lint_counts == {'error': 0, 'warning': 240, 'info': 1136}
gates = read('p1310-gates.json')
assert gates['pass'] and not gates['p1309_changes'] and not gates['outside_allowlist']
for path, expected in b['p1309_artifacts'].items():
    assert sha(ROOT / path) == expected
for binary in [b['vanilla'], b['baseline_binary']]:
    assert sha(binary['path']) == binary['sha256']
product = ROOT / '01_core/src/compiler/stdlib/loading.rs'
source = product.read_text()
hash_b = hashlib.sha256(''.join(line for line in source.splitlines(keepends=True) if not line.startswith('//! @prompt-hash')).encode()).hexdigest()[:8]
assert hash_b == '1f56a80a' and 'Hash do Código: ' + hash_b in l0.read_text()
assert '//! @prompt-hash ec1dd115' in source
changed_tracked = set(git('diff', 'HEAD', '--name-only').splitlines())
assert changed_tracked == {'01_core/src/compiler/stdlib/loading.rs', '00_nucleo/prompts/compiler/stdlib/loading.md'}
assert not git('diff', '--cached')
assert git('rev-parse', 'HEAD').strip() == b['state']['head']
delta = read('p1310-p1308-delta.json')
assert delta['pass']
read('p1310-p1308-replay.json')
receipt = {
    'schema': 'p1310-review-evidence-v1',
    'issuer': '/root/p1310_review',
    'at': datetime.datetime.now(datetime.timezone.utc).isoformat(),
    'head': b['state']['head'],
    'diff_stat': git('diff', 'HEAD', '--stat'),
    'inputs': inputs,
    'script_sha256': sha(__file__),
    'source_sha256': sha(product),
    'L0_sha256': sha(l0),
    'candidate': next(iter(orders['normal'].values()))['binary'],
    'checks_pass': True,
    'cases': len(new),
    'cells_per_order': len(expected_keys),
    'orders': list(orders),
    'policies': dict(Counter(c['policy'] for c in new.values())),
    'R2_policy_changed_cases': changed,
    'R2_expected_bytes_recovered_from_pre_candidate_baseline': True,
    'R2_all_expectations_frozen_before_candidate': False,
    'L0_semantics_unchanged_since_R1': True,
    'workspace_tests': dict(zip(['passed', 'failed', 'ignored'], test_totals)),
    'lint': lint_counts,
    'p1309_artifacts_preserved': len(b['p1309_artifacts']),
    'tracked_changes': sorted(changed_tracked),
    'technical_isolation_attested': False,
}
target = D / 'p1310-review-evidence.json'
assert not target.exists()
data = json.dumps(receipt, ensure_ascii=False, indent=2) + '\n'
patch = '*** Begin Patch\n*** Add File: ' + str(target) + '\n' + ''.join('+' + line + '\n' for line in data.splitlines()) + '*** End Patch\n'
subprocess.run(['apply_patch'], input=patch, text=True, check=True, cwd=ROOT)
print(json.dumps({'checks_pass': True, 'receipt_sha256': sha(target), 'cases': len(new), 'cells_per_order': len(expected_keys), 'workspace': test_totals, 'lint': lint_counts}))
