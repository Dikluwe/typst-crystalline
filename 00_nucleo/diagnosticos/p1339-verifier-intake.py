"""Independent intake of protected P1339 authorities, contract and L0."""
from collections import Counter
import datetime
import hashlib
import json
from pathlib import Path
import re
import subprocess

ROOT = Path(__file__).resolve().parents[2]
D = ROOT / '00_nucleo/diagnosticos'


def sha(path):
    return hashlib.sha256(Path(path).read_bytes()).hexdigest()


def git(*args):
    return subprocess.check_output(['git', *args], cwd=ROOT, text=True)


expected = {
    'p1339-authority-manifest-r2.json': '842d6526739014022c073800148a47b3c886831e2198ab65bbfdbe73ce59411b',
    'p1339-l0-freeze.json': '397c136fc8710d44b2f7537193fe5b9c44ab89d40a99bf6b994296e05e7c4d04',
    'p1339-implementation-authorities.json': 'cab15a1dce423a12fe198a640b541ee00b3f8743ec0f6a9e97519416d940f3c0',
    'p1339-contract.json': 'e062fb551fcffd78d21c1ae2a0a185aa376dd790c3e0825cec045a7f1e65b509',
    'p1339-contract-design.md': 'bf6da53d68024c41e1a76efbe6f82d4fee210c56a4e1aa80ac438d5dea8343b5',
}
for name, digest in expected.items():
    assert sha(D / name) == digest, name
frozen = json.loads((D / 'p1339-l0-freeze.json').read_text())
contract = json.loads((D / 'p1339-contract.json').read_text())
assert git('rev-parse', 'HEAD').strip() == frozen['head']
assert not git('diff', '--cached', '--name-only')
protected_checks = []
for path, digest in contract['input_sha256'].items():
    target = ROOT / path
    assert sha(target) == digest, path
    protected_checks.append(path)
for path, digest in frozen['l0_sha256'].items():
    assert sha(ROOT / path) == digest == contract['l0_raw_sha256'][path], path
    raw = (ROOT / path).read_bytes()
    preamble = raw.split(b'\n\n', 1)[0]
    assert b'```' not in preamble
    line, = re.findall(rb'(?m)^Hash do C\xc3\xb3digo: [0-9a-f]{8}\r?$', preamble)
    normalized = raw.replace(line + b'\n', b'', 1)
    assert hashlib.sha256(normalized).hexdigest() == contract['l0_normative_sha256'][path], path
for path, digest in frozen['lineage_only_source_sha256'].items():
    assert sha(ROOT / path) == digest, path
for path, digest in frozen['binary_sha256'].items():
    assert sha(path) == digest, path
rows = contract['historical_case_index']
keys = [(row['stage'], row['id']) for row in rows]
assert len(set(keys)) == len(keys)
result = {
    'schema': 'p1339-verifier-intake-v1',
    'utc': datetime.datetime.now(datetime.timezone.utc).isoformat(),
    'issuer': '/root/p1311_review',
    'authority_manifest_sha256': expected['p1339-authority-manifest-r2.json'],
    'context': 'Earlier completed P1311 independent review; no P1339 candidate implementation read. Current role verifier only.',
    'regime': 'protocolo completo executado sem atestação de isolamento',
    'capabilities': {
        'read': ['frozen L0, baseline and protected diagnostic inputs', 'immutable oracles and mutation artifacts', 'candidate outputs and source for final review'],
        'write': ['p1339-verifier-*', 'p1339-discrimination-runs.json', 'p1339-seal.json', 'p1339-verification.json', 'p1339-final-report.md', 'p1339-closure.json'],
        'forbidden': ['change implementation', 'change contract/oracles/mutants/baseline', 'self-correct verified input'],
        'technical_enforcement': False,
    },
    'received_input_sha256': expected,
    'additional_protected_files_verified': protected_checks,
    'frozen_l0_verified': len(frozen['l0_sha256']),
    'lineage_only_sources_verified': len(frozen['lineage_only_source_sha256']),
    'binary_sha256_verified': frozen['binary_sha256'],
    'historical_cases': len(rows),
    'historical_policy_counts': dict(Counter(row['target'] for row in rows)),
    'supplement_sets': [s['id'] for s in contract['supplement_sets']],
    'budget_state': {'maximum_contract_revisions': 3, 'maximum_preseal_complete_runs': 2, 'preseal_complete_runs_executed_by_verifier': 0},
    'head': frozen['head'], 'diff_stat': git('diff', 'HEAD', '--stat'), 'status': git('status', '--short'),
    'findings_pending': ['closed_state phase prerequisite needs canonical clarification before seal; no coverage or success credited'],
    'authority_supplement_status': 'Received and hash-verified; must be included in future seal, dormant until seal and independent RED',
    'checks_passed': True, 'seal_issued': False, 'checker_sha256': sha(__file__),
}
path = D / 'p1339-verifier-intake-r1.json'
assert not path.exists()
payload = json.dumps(result, ensure_ascii=False, indent=2) + '\n'
patch = '*** Begin Patch\n*** Add File: ' + str(path) + '\n' + ''.join('+' + line + '\n' for line in payload.splitlines()) + '*** End Patch\n'
subprocess.run(['apply_patch'], input=patch, cwd=ROOT, text=True, check=True, capture_output=True)
print(json.dumps({'path': str(path), 'sha256': sha(path), 'historical_cases': len(rows), 'policies': result['historical_policy_counts']}))
