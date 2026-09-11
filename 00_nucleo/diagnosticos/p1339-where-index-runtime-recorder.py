"""Freeze index L0 continuation and conditional filtered-counter phase design."""
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
prompts = git('diff', 'HEAD', '--name-only').splitlines()
assert len(prompts) == 22, prompts
assert all(p.startswith('00_nucleo/prompts/') and p.endswith('.md') for p in prompts)
a0 = json.loads((DIAG / 'p1339-a0.json').read_text())
changes = [p for p,h in a0['product_inventory'].items() if sha(ROOT / p) != h]
assert sorted(changes) == sorted(prompts), changes
assert not git('ls-files','--others','--exclude-standard','01_core','02_shell','03_infra','04_wiring').strip()
historical = json.loads((DIAG / 'p1339-retification-check.json').read_text())
assert all(sha(ROOT / p) == h for p,h in historical['artifact_hashes'].items())
prior = DIAG / 'p1339-where-integration-receipt.json'
assert sha(prior) == '8a0befbb221fb2745f3a18496923fa3f7ba955b59089b626e4a6b4b3a4202631'
prior_data = json.loads(prior.read_text())
assert all(sha(DIAG / p) == h for p,h in prior_data['artifact_sha256'].items())
assert all(sha(ROOT / p) == h for p,h in prior_data['unchanged_consumers_sha256'].items())
checks=[]
for command, expected in [
    (['crystalline-lint','--checks','v15,v26','--fail-on','warning','.'],0),
    (['crystalline-lint','--checks','v5','--fail-on','warning','.'],1),
    (['git','diff','--check'],0),
]:
    begin=now()
    p=subprocess.run(command,cwd=ROOT,text=True,capture_output=True,timeout=60)
    checks.append(dict(command=command,start=begin,end=now(),exit_code=p.returncode,stdout=p.stdout,stderr=p.stderr))
    assert p.returncode == expected,checks[-1]
v5=checks[1]['stdout']+checks[1]['stderr']
assert v5.count('[V5]') == len(prompts),v5
artifacts=[
    'p1339-where-payload-approval.json','p1339-where-occurrence-probe.py',
    'p1339-where-occurrence-probe.md','p1339-where-occurrence-probe-runs.json',
    'p1339-where-counter-integration-design.md','p1339-where-index-l0-review.md',
    'p1339-where-counter-phase-probe.py','p1339-where-counter-phase-probe-runs.json',
    'p1339-where-counter-phase-compile-runs.json','p1339-where-counter-phase-file-runs.json',
    'p1339-where-counter-phase-gate.md',
]
data=dict(start=start,end=now(),head=git('rev-parse','HEAD').strip(),
          branch=git('branch','--show-current').strip(),status=git('status','--short'),
          diff_stat=git('diff','HEAD','--stat'),
          state='PARTIAL_L0; PAYLOAD_APPROVED; FILTERED_COUNTER_ON_DEMAND_PHASE_GATE_PENDING',
          regime='executed without isolation attestation',
          l0_sha256={p:sha(ROOT/p) for p in prompts},
          product_source_unchanged=True,a0_inventory_differences=changes,
          historical_artifacts_preserved=True,checks=checks,
          predecessor_sha256=sha(prior),
          artifact_sha256={p:sha(DIAG/p) for p in artifacts},
          recorder_sha256=sha(Path(__file__)),
          command='python3 00_nucleo/diagnosticos/p1339-where-index-runtime-recorder.py',
          no_claims=['No implementation','No sealed contract','No independent phase-D RED',
                     'No general parity','No source rehash','No commit'])
target=DIAG/'p1339-where-index-runtime-receipt.json'
assert not target.exists(),target
body=json.dumps(data,ensure_ascii=False,indent=2)
patch='*** Begin Patch\n*** Add File: '+str(target)+'\n'+''.join('+'+line+'\n' for line in body.splitlines())+'*** End Patch\n'
subprocess.run(['apply_patch'],cwd=ROOT,input=patch,text=True,check=True)
print(json.dumps(dict(receipt_sha256=sha(target),v15_v26='clean',v5_pending=len(prompts),source_unchanged=True)))
