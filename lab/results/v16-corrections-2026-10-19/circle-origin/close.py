"""Close only against complete, immutable, successful validation evidence."""
from pathlib import Path
import datetime
import gzip
import hashlib
import json
import re
import subprocess

ROOT = Path.cwd()
OUT = Path(__file__).resolve().parent
sha = lambda p: hashlib.sha256(p.read_bytes()).hexdigest()


def read(path):
    p = OUT / path
    with (gzip.open(p, 'rt') if p.suffix == '.gz' else p.open()) as f:
        return json.load(f)


scope = read('scope.json')
proposal = ROOT / 'lab/results/v16-corrections-2026-10-19/batch-f/agents/geometry/circle-deferred/proposal/review.md'
assert sha(proposal) == '5b7136bc18f419004113d8d83c9c54adb816e3b70389039aeeed35e1c6f772e8'
state = read('validation/validation-state.json.gz')
freeze = read('validation/validation-freeze.json')
commands = read('validation/validation-commands.json')
assert len(commands) == 12 and all(r['exit'] == 0 for r in commands)
assert freeze['changed_inputs'] == [] and freeze['completed_jobs'] == 12
assert all(sha(ROOT / p) == digest for p, digest in state['inputs'].items())
lint = read('lint-state.json.gz')
assert lint['inputs'] == state['inputs'] and lint['changed_inputs'] == []
assert lint['counts'] == {'V16:error': 20, 'V19:note': 1262, 'V20:note': 877}
lint_review = read('agents/audit/lint-review.json')
source_review = read('agents/audit/source-review.json')
structural = read('agents/tests/structural-review.json')
assert lint_review['passed'] and structural['passed']
assert source_review['actionable_code_findings'] == [] and source_review['reviewed_scope_pairs'] == 14
assert all(sha(ROOT / p) == digest for p, digest in source_review['source_and_owner_sha256'].items())
assert source_review['lint_review_sha256'] == sha(OUT / 'agents/audit/lint-review.json')
assert lint_review['outside_scope_identical'] and not lint_review['new_errors']
assert lint_review['after_sarif_sha256'] == sha(OUT / 'after.sarif.gz')
assert all(sha(ROOT / p) == digest for p, digest in lint_review['source_sha256'].items())
assert sha(OUT / 'agents/audit/cases.json') == '8d95ef237e4569c5f86f01705cff76a06afec9df5ae210a710b99885bcb42b77'
assert sha(OUT / 'agents/audit/paint-cases.json') == 'b000a4403c7f8c4ac757dfa8d2549eacee7ad355f2805936dad42dcea04cf2b8'
assert sha(OUT / 'agents/audit/label-controls-policy.json') == 'f80b0c594adf929d64b5a521520f83f48bb3c61890459fba3d38c132bcdb2ef4'
build = read('accepted-build-receipt.json')
assert sha(Path(build['product']['path'])) == build['product']['sha256']
for name in ('state', 'commands'):
    assert sha(Path(build[name]['path'])) == build[name]['sha256']
replays = {}
for directory, expected_count in [('accepted-replay', 224), ('accepted-paint', 6), ('accepted-label-controls', 18)]:
    receipt = read(directory + '/receipt.json')
    assert receipt['passed'] and len(receipt['cases']) == expected_count
    assert receipt['changed_inputs'] == []
    assert receipt['binaries']['product']['sha256'] == build['product']['sha256']
    assert all(case['passed'] for case in receipt['cases'])
    replays[directory] = {'count': expected_count, 'sha256': sha(OUT / directory / 'receipt.json'), 'utc_end': receipt['utc_end']}
tests = read('new-tests.json')['tests']
full_log = (OUT / 'validation/tests-domain-infra.log').read_text()
assert len(tests) == 23
assert all('::' + test['name'] + ' ... ok' in full_log for test in tests)
assert 'FAILED' not in full_log
productive = scope['sources'] + scope['owners']
assert len(scope['pairs']) == 14
original = read('structural-green-final/original-sources.json.gz')
assert all((ROOT / p).read_text() == text for p, text in original.items())
assert read('structural-green-final/results.json')[0]['selected_e0004'] == 5
changed = subprocess.check_output(['git', 'diff', 'HEAD', '--name-only'], text=True).splitlines()
assert not any(p.startswith('lab/results/v16-corrections-2026-10-19/batch-f/') for p in changed)
assert {p for p in changed if p.startswith(('01_core/', '02_shell/', '03_infra/', '04_wiring/'))} == set(scope['sources'])
assert {p for p in changed if p.startswith('00_nucleo/prompts/')} == set(scope['owners'])
pairs = []
for pair in scope['pairs']:
    source = ROOT / pair['source']
    owner = ROOT / pair['owner']
    header = re.search(r'^//! @prompt (.+)$', source.read_text(), re.M).group(1)
    assert header == pair['owner']
    pairs.append({**pair, 'source_sha256': sha(source), 'owner_sha256': sha(owner),
                  'prompt_seal': re.search(r'^//! @prompt-hash (.+)$', source.read_text(), re.M).group(1),
                  'code_seal': re.search(r'^Hash do Código: (.+)$', owner.read_text(), re.M).group(1)})
record = {
    'utc': datetime.datetime.now(datetime.timezone.utc).isoformat(),
    'head': subprocess.check_output(['git', 'rev-parse', 'HEAD'], text=True).strip(),
    'state': 'uncommitted working tree; validated inputs are unchanged',
    'diff_head_stat': subprocess.check_output(['git', 'diff', 'HEAD', '--stat'], text=True),
    'closed_ids': ['V16-065'], 'remaining_v16': 20,
    'batch_f_repaired': 17, 'batch_f_pending': ['V16-055'],
    'source_owner_pairs': pairs, 'full_gates': 12, 'new_tests_observed_in_full_suite': tests,
    'replays': replays, 'product_sha256': build['product']['sha256'],
    'validation_end': commands[-1]['utc_end'], 'input_count': len(state['inputs']),
    'audit_sha256': {p: sha(OUT / p) for p in ('agents/audit/source-review.json', 'agents/audit/lint-review.json', 'agents/tests/structural-review.json')},
    'limits': 'Scoped Circle and Label corrections; Auto/default sizing and broader Circle surfaces remain debt. Full linter still exits 1 for 20 open V16 findings.',
}
target = OUT / 'closure-ledger.json'
assert not target.exists(), 'preserve closure attempts'
target.write_text(json.dumps(record, indent=2) + '\n')
print('PASS: 14 source/owner pairs, 23 new tests, 12 gates, 248 scoped replay cases; V16 21 -> 20')
