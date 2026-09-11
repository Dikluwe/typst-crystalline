"""Freeze the authorized angle correction; do not rewrite historical evidence."""
import datetime
import hashlib
import json
from pathlib import Path
import subprocess

ROOT = Path(__file__).resolve().parents[2]
D = ROOT/'00_nucleo/diagnosticos'
def sha(p):
    with Path(p).open('rb') as f:
        return hashlib.file_digest(f,'sha256').hexdigest()
def save(name, data):
    p=D/('p1339-'+name+'.json')
    assert not p.exists(),p
    body=json.dumps(data,ensure_ascii=True,indent=2)+'\n'
    subprocess.run(['apply_patch'],input='*** Begin Patch\n*** Add File: '+str(p)+'\n'+''.join('+'+l+'\n' for l in body.splitlines())+'*** End Patch\n',text=True,cwd=ROOT,check=True)

a0=json.loads((D/'p1339-a0.json').read_text())
assert all(sha(ROOT/p)==h for p,h in a0['product_inventory'].items())
assert sha(D/'p1339-step-before-angle-retification.md')==a0['step']['sha256']
assert all(sha(ROOT/p)==h for p,h in a0['antecedents'].items())
step=ROOT/a0['step']['path']
prior={str(p.relative_to(ROOT)):sha(p) for p in sorted(D.glob('p1339-*')) if p.is_file()}
save('resume-r1',dict(at=datetime.datetime.now(datetime.timezone.utc).isoformat(),head=subprocess.check_output(['git','rev-parse','HEAD'],cwd=ROOT,text=True).strip(),tracked_diff_stat=subprocess.check_output(['git','diff','HEAD','--stat'],cwd=ROOT,text=True),authorization='User: Autrorizo, in direct response to correcting Angle -> float while preserving ten routes.',authorized_change='Only angle obligation and causal note in execution step; no reduced scope, protocol downgrade or product change.',step_before_sha256=a0['step']['sha256'],step_after=dict(path=str(step.relative_to(ROOT)),sha256=sha(step)),baseline_product_sha256=a0['product_inventory'],preserved_inputs=prior,recorder_sha256=sha(__file__)))
save('authority-manifest-r1',dict(at=datetime.datetime.now(datetime.timezone.utc).isoformat(),predecessor=dict(path='p1339-resume-r1.json',sha256=sha(D/'p1339-resume-r1.json')),step=dict(path=str(step.relative_to(ROOT)),sha256=sha(step)),regime='protocolo completo solicitado; ensaio sem atestação de isolamento',capabilities_enforced=False,roles={
    'operator_and_implementer':dict(executor='/root',context='full conversation',read=['baseline','step','existing L0/product','measurements','sealed contract later'],write=['step correction','P1339 provenance','L0 after measurements','product only after seal'],forbidden=['edit sealed contract/oracles','write own independent verdict']),
    'measurement_author':dict(executor='/root/p1339_measure_r1',context='fresh task only',read=['explicit corrected P1339 step','A0/resume/manifests','old focal probes','pinned vanilla source and binaries'],write=['p1339-full-* diagnostics and dedicated temporary fixtures'],forbidden=['candidate product','L0 edits','contract/seal/final verdict']),
    'protocol_reviewer':dict(executor='/root/p1339_protocol_r1',context='fresh task only',read=['explicit corrected step','skill','ADRs','A0/resume/manifests','existing relevant L0/product and pinned vanilla source'],write=['p1339-protocol-review-*'],forbidden=['edit judged material','candidate','contract/oracles']),
    'contract_author':dict(executor=None,status='waiting for completed A and resealed L0'),
    'oracles_and_adversary':dict(executor=None,status='waiting for contract'),
    'final_verifier':dict(executor=None,status='waiting for sealed inputs; cannot correct judged material')},input_hashes=prior,outputs='each role must record hashes and predecessor in its receipt',unknown='Mandatory Unknown blocks seal; deliberately opaque controls remain Unknown.',budget=dict(contract_revisions=3,full_discrimination_runs=2,no_gain_revisions=2)))
