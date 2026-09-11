"""Freeze the partial integration L0 and the additional public-payload gate."""
import datetime
import hashlib
import json
from pathlib import Path
import subprocess

root = Path(__file__).resolve().parents[2]
diagnostics = root / '00_nucleo/diagnosticos'
def sha(path):
    with path.open('rb') as stream:
        return hashlib.file_digest(stream, 'sha256').hexdigest()
def now():
    return datetime.datetime.now(datetime.timezone.utc).isoformat()
def git(*args):
    return subprocess.check_output(['git', *args], cwd=root, text=True)

started = now()
owners = [
    'entities/selector', 'entities/show', 'entities/element_payload',
    'compiler/eval/bindings/value_methods', 'compiler/eval/bindings/field_access',
    'compiler/eval/selector_matching', 'compiler/eval/operators/equality',
    'compiler/eval/repr', 'compiler/eval/rules', 'compiler/eval/call_dispatch',
    'compiler/stdlib/foundations/selector',
]
prompts = ['00_nucleo/prompts/' + p + '.md' for p in owners]
sources = ['01_core/src/' + p + '.rs' for p in owners]
a0 = json.loads((diagnostics / 'p1339-a0.json').read_text())
changed = [p for p, h in a0['product_inventory'].items() if sha(root / p) != h]
assert sorted(changed) == sorted(prompts), changed
assert sorted(git('diff', 'HEAD', '--name-only').splitlines()) == sorted(prompts)
assert not git('ls-files', '--others', '--exclude-standard', '01_core', '02_shell', '03_infra', '04_wiring').strip()

old = json.loads((diagnostics / 'p1339-retification-check.json').read_text())
assert all(sha(root / p) == h for p, h in old['artifact_hashes'].items())
prior = json.loads((diagnostics / 'p1339-where-l0-gate-receipt.json').read_text())
assert all(sha(diagnostics / p) == h for p, h in prior['artifact_sha256'].items())
assert all(sha(root / p) == h for p, h in prior['unchanged_consumers_sha256'].items())

checks = []
for command, expected in [
    (['crystalline-lint', '--checks', 'v15,v26', '--fail-on', 'warning', '.'], 0),
    (['crystalline-lint', '--checks', 'v5', '--fail-on', 'warning', '.'], 1),
    (['git', 'diff', '--check'], 0),
]:
    begin = now()
    result = subprocess.run(command, cwd=root, text=True, capture_output=True, timeout=60)
    checks.append(dict(command=command, cwd=str(root), start=begin, end=now(),
                       exit_code=result.returncode, stdout=result.stdout, stderr=result.stderr))
    assert result.returncode == expected, checks[-1]
v5 = checks[1]['stdout'] + checks[1]['stderr']
assert v5.count('[V5]') == len(prompts), v5
assert all(path in v5 for path in sources), v5

artifacts = [
    'p1339-where-approval.json', 'p1339-where-payload-gate.md',
    'p1339-where-integration-probe.md', 'p1339-where-integration-probe.py',
    'p1339-where-integration-probe-runs.json',
    'p1339-where-integration-probe-supplement.py',
    'p1339-where-integration-probe-supplement-runs.json',
    'p1339-where-query-design-review.md', 'p1339-where-query-design-review-r2.md',
    'p1339-where-payload-draft-review.md', 'p1339-where-payload-draft-review-r2.md',
]
data = dict(
    start=started, end=now(), head=git('rev-parse', 'HEAD').strip(),
    branch=git('branch', '--show-current').strip(), status=git('status', '--short'),
    diff_stat=git('diff', 'HEAD', '--stat'),
    state='PARTIAL_L0_INTEGRATION; ADDITIONAL_ELEMENTPAYLOAD_GATE_PENDING',
    regime='executed without isolation attestation',
    l0_sha256={p: sha(root / p) for p in prompts},
    unchanged_consumers_sha256={p: sha(root / p) for p in sources},
    a0_inventory_differences=changed, historical_artifacts_preserved=True,
    product_unchanged=True, checks=checks,
    artifact_sha256={p: sha(diagnostics / p) for p in artifacts},
    recorder_sha256=sha(Path(__file__)),
    command='python3 00_nucleo/diagnosticos/p1339-where-integration-recorder.py',
    no_claims=['No sealed contract', 'No RED/GREEN candidate', 'No general parity',
               'No implementation', 'No rehash of source headers', 'No commit'],
)
target = diagnostics / 'p1339-where-integration-receipt.json'
assert not target.exists(), target
body = json.dumps(data, ensure_ascii=False, indent=2) + '\n'
patch = '*** Begin Patch\n*** Add File: ' + str(target) + '\n' + ''.join('+' + line + '\n' for line in body.splitlines()) + '*** End Patch\n'
subprocess.run(['apply_patch'], cwd=root, input=patch, text=True, check=True)
print(json.dumps(dict(receipt_sha256=sha(target), product_unchanged=True,
                     v15_v26='clean', pending_v5=len(prompts))))
