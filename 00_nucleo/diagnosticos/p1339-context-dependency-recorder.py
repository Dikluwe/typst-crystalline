"""Freeze the approved on-demand L0 continuation and contextual scope blocker."""
import datetime
import hashlib
import json
from pathlib import Path
import subprocess

ROOT = Path(__file__).resolve().parents[2]
DIAG = ROOT / '00_nucleo/diagnosticos'

def sha(path):
    with path.open('rb') as stream:
        return hashlib.file_digest(stream, 'sha256').hexdigest()

def now():
    return datetime.datetime.now(datetime.timezone.utc).isoformat()

def git(*args):
    return subprocess.check_output(['git', *args], cwd=ROOT, text=True)

start = now()
target = DIAG / 'p1339-context-dependency-receipt.json'
assert not target.exists()
prior = DIAG / 'p1339-where-index-runtime-receipt.json'
assert sha(prior) == 'a5628e8ca314cc8d61900eec5a9dab2d1c661ec313bff220665908d74ee1f73d'
previous = json.loads(prior.read_text())
assert all(sha(DIAG / p) == h for p, h in previous['artifact_sha256'].items())
historical = json.loads((DIAG / 'p1339-retification-check.json').read_text())
assert all(sha(ROOT / p) == h for p, h in historical['artifact_hashes'].items())
prompts = git('diff', 'HEAD', '--name-only').splitlines()
assert len(prompts) == 22 and all(p.startswith('00_nucleo/prompts/') for p in prompts)
a0 = json.loads((DIAG / 'p1339-a0.json').read_text())
changes = [p for p, h in a0['product_inventory'].items() if sha(ROOT / p) != h]
assert sorted(changes) == sorted(prompts)
assert not git('ls-files', '--others', '--exclude-standard', '01_core', '02_shell', '03_infra', '04_wiring').strip()
checks = []
for argv, expected in [
    (['crystalline-lint', '--checks', 'v15,v26', '--fail-on', 'warning', '.'], 0),
    (['crystalline-lint', '--checks', 'v5', '--fail-on', 'warning', '.'], 1),
    (['git', 'diff', '--check'], 0),
]:
    begin = now()
    proc = subprocess.run(argv, cwd=ROOT, text=True, capture_output=True, timeout=60)
    checks.append(dict(argv=argv, cwd=str(ROOT), start=begin, end=now(),
                       exit=proc.returncode, stdout=proc.stdout, stderr=proc.stderr))
    assert proc.returncode == expected, checks[-1]
assert (checks[1]['stdout'] + checks[1]['stderr']).count('[V5]') == len(prompts)
artifacts = [
    'p1339-where-counter-phase-approval.json',
    'p1339-where-counter-runtime-probe.py',
    'p1339-where-counter-runtime-probe.md',
    'p1339-where-counter-runtime-probe-runs.json',
    'p1339-context-dependency-probe.py',
    'p1339-context-dependency-probe-runs.json',
    'p1339-context-dependency-review.md',
    'p1339-context-dependency-gate.md',
]
result = dict(start=start, end=now(), head=git('rev-parse', 'HEAD').strip(),
              branch=git('branch', '--show-current').strip(),
              status=git('status', '--short'), diff_stat=git('diff', 'HEAD', '--stat'),
              state='PARTIAL_L0; ON_DEMAND_PHASE_APPROVED; CONTEXT_STABILIZATION_SCOPE_DECISION_REQUIRED',
              regime='executed without isolation attestation',
              predecessor_sha256=sha(prior), checks=checks,
              artifact_sha256={p: sha(DIAG / p) for p in artifacts},
              l0_sha256={p: sha(ROOT / p) for p in prompts},
              l0_changed_since_predecessor=[p for p, h in previous['l0_sha256'].items() if sha(ROOT / p) != h],
              a0_inventory_differences=changes, product_source_unchanged=True,
              historical_artifacts_preserved=True, recorder_sha256=sha(Path(__file__)),
              command='python3 00_nucleo/diagnosticos/p1339-context-dependency-recorder.py',
              no_claims=['No implementation', 'No sealed contract', 'No phase-D independent RED',
                         'No functional GREEN', 'No general parity', 'No source rehash', 'No commit'])
body = json.dumps(result, ensure_ascii=False, indent=2)
patch = '*** Begin Patch\n*** Add File: ' + str(target) + '\n' + ''.join('+' + line + '\n' for line in body.splitlines()) + '*** End Patch\n'
subprocess.run(['apply_patch'], cwd=ROOT, input=patch, text=True, check=True)
print(json.dumps(dict(receipt_sha256=sha(target), v15_v26='clean', v5_pending=len(prompts),
                      l0_changed_since_predecessor=result['l0_changed_since_predecessor'],
                      source_unchanged=True)))
