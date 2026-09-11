"""Freeze the L0-only submission and verify no product was edited."""
import datetime
import hashlib
import json
from pathlib import Path
import subprocess

ROOT = Path(__file__).resolve().parents[2]
D = ROOT / '00_nucleo/diagnosticos'
def sha(path):
    with Path(path).open('rb') as stream:
        return hashlib.file_digest(stream, 'sha256').hexdigest()
def now():
    return datetime.datetime.now(datetime.timezone.utc).isoformat()
def git(*args):
    return subprocess.check_output(['git', *args], cwd=ROOT, text=True)

owners = [
    'entities/selector', 'entities/show',
    'compiler/eval/bindings/value_methods',
    'compiler/eval/selector_matching', 'compiler/eval/operators/equality',
]
prompts = ['00_nucleo/prompts/' + owner + '.md' for owner in owners]
sources = ['01_core/src/' + owner + '.rs' for owner in owners]
start = now()
a0 = json.loads((D / 'p1339-a0.json').read_text())
changed = [p for p,h in a0['product_inventory'].items() if sha(ROOT/p) != h]
assert sorted(changed) == sorted(prompts), changed
assert sorted(git('diff','HEAD','--name-only').splitlines()) == sorted(prompts)
previous = json.loads((D / 'p1339-retification-check.json').read_text())
assert all(sha(ROOT/p) == h for p,h in previous['artifact_hashes'].items())
checks = []
for args, expected in [(['crystalline-lint','--checks','v15,v26','--fail-on','warning','.'],0), (['crystalline-lint','--checks','v5','--fail-on','warning','.'],1), (['git','diff','--check'],0)]:
    at = now()
    proc = subprocess.run(args, cwd=ROOT, text=True, capture_output=True, timeout=60)
    checks.append(dict(argv=args,cwd=str(ROOT),start=at,end=now(),exit=proc.returncode,stdout=proc.stdout,stderr=proc.stderr))
    assert proc.returncode == expected, checks[-1]
v5 = checks[1]['stdout'] + checks[1]['stderr']
assert v5.count('[V5]') == len(prompts), v5
assert all(p in v5 for p in sources), v5
artifacts = [
    'p1339-where-l0-gate.md', 'p1339-where-l0-manifest.json',
    'p1339-where-l0-vanilla.json', 'p1339-where-l0-crystalline.json',
    'p1339-where-l0-review.md', 'p1339-where-l0-review-draft-r2.md',
    'p1339-nan-resolution.md',
]
data = dict(
    start=start,end=now(),head=git('rev-parse','HEAD').strip(),branch=git('branch','--show-current').strip(),
    status_short=git('status','--short'),diff_stat=git('diff','HEAD','--stat'),
    status='DRAFT_L0_AWAITING_ADR0127; not a seal or implementation verdict',
    authority='User Faça: prepare L0 and present public Selector extension, no product authorization yet',
    regime='executed without isolation attestation',
    l0_sha256={p:sha(ROOT/p) for p in prompts},
    unchanged_consumers_sha256={p:sha(ROOT/p) for p in sources},
    inventory_changed_only=changed,
    historical_artifacts_preserved=True,
    checks=checks,artifact_sha256={p:sha(D/p) for p in artifacts},
    script_sha256=sha(__file__),command='python3 00_nucleo/diagnosticos/p1339-where-l0-gate-receipt.py',
    hashes_resealed=False,implementation_written=False,commit_created=False,
)
target = D / 'p1339-where-l0-gate-receipt.json'
assert not target.exists(), target
body = json.dumps(data,ensure_ascii=False,indent=2)+'\n'
patch = '*** Begin Patch\n*** Add File: '+str(target)+'\n'+''.join('+'+line+'\n' for line in body.splitlines())+'*** End Patch\n'
subprocess.run(['apply_patch'],input=patch,cwd=ROOT,text=True,capture_output=True,check=True)
print(json.dumps(dict(l0_sha256=data['l0_sha256'],product_unchanged=True,v15_v26='clean',v5='five expected pending draft warnings',receipt_sha256=sha(target)),ensure_ascii=False))
