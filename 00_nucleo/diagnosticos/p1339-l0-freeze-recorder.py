"""Freeze phase B with mechanical checks; never overwrite an existing receipt."""
import datetime
import hashlib
import json
from pathlib import Path
import subprocess

ROOT = Path(__file__).resolve().parents[2]
DIAG = ROOT / '00_nucleo/diagnosticos'
def sha(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()
def now():
    return datetime.datetime.now(datetime.timezone.utc).isoformat()
def git(*args):
    return subprocess.check_output(['git', *args], cwd=ROOT, text=True)
target = DIAG / 'p1339-l0-freeze.json'
assert not target.exists(), 'Immutable receipt: create a successor'
started = now()
prompts = git('diff', 'HEAD', '--name-only').splitlines()
assert len(prompts) == 31 and all(p.startswith('00_nucleo/prompts/') for p in prompts)
a0 = json.loads((DIAG / 'p1339-a0.json').read_text())
assert sorted(p for p,h in a0['product_inventory'].items() if sha(ROOT/p) != h) == sorted(prompts)
historical = json.loads((DIAG/'p1339-retification-check.json').read_text())
assert all(sha(ROOT/p) == h for p,h in historical['artifact_hashes'].items())
pre_hashes = {p:sha(ROOT/p) for p in prompts}
checks = []
for argv in [
    ['crystalline-lint','--checks','v15,v26','--fail-on','warning','.'],
    ['crystalline-lint','--fix-hashes','.'],
    ['crystalline-lint','--checks','v5,v15,v26','--fail-on','warning','.'],
    ['git','diff','--check'],
]:
    start=now()
    result=subprocess.run(argv,cwd=ROOT,text=True,capture_output=True,timeout=120)
    checks.append(dict(argv=argv,start=start,end=now(),exit=result.returncode,stdout=result.stdout,stderr=result.stderr))
    assert result.returncode == 0, checks[-1]
layers=['01_core','02_shell','03_infra','04_wiring']
assert not git('ls-files','--others','--exclude-standard',*layers).strip()
sources=git('diff','HEAD','--name-only','--',*layers).splitlines()
assert len(sources)==len(prompts), sources
def without_lineage(data):
    return b'\n'.join(line for line in data.split(b'\n') if not line.startswith((b'//! @prompt-hash ',b'//! @updated ')))
for path in sources:
    baseline=subprocess.check_output(['git','show','HEAD:'+path],cwd=ROOT)
    assert without_lineage(baseline)==without_lineage((ROOT/path).read_bytes()), path
artifacts=['p1339-completion-authorization.md','p1339-observation-integration-receipt.json',
 'p1339-remaining-l0-design.md','p1339-observation-design-resolution.md',
 'p1339-observation-design-boundaries.md','p1339-show-witness-receipt.md',
 'p1339-show-witness-bridge.json','p1339-show-witness-final-manifest.json',
 'p1339-show-witness-final-observation.json','p1339-show-witness-final-vanilla-runs.json',
 'p1339-show-witness-final-baseline-runs.json','p1339-full-receipt.json',
 'p1339-nan-resolution.md','p1339-nan-review-r2.md','p1339-authority-manifest-r1.json']
result=dict(start=started,end=now(),head=git('rev-parse','HEAD').strip(),
 branch=git('branch','--show-current').strip(),status=git('status','--short'),
 diff_stat=git('diff','HEAD','--stat'),regime='executado sem atestação de isolamento',
 state='PHASE_B_FROZEN; LINEAGE_ONLY; NO_CANDIDATE; CONTRACT_AND_RED_PENDING',
 step_path='00_nucleo/materialization/typst-passo-1339.md',
 step_sha256=sha(ROOT/'00_nucleo/materialization/typst-passo-1339.md'),
 pre_reseal_l0_sha256=pre_hashes,l0_sha256={p:sha(ROOT/p) for p in prompts},
 lineage_only_source_sha256={p:sha(ROOT/p) for p in sources},
 artifact_sha256={p:sha(DIAG/p) for p in artifacts},checks=checks,
 binary_sha256={p:sha(Path(p)) for p in ['/usr/local/bin/typst','/tmp/p1338-target.vlNAmp/release/typst']},
 historical_artifacts_preserved=True,product_semantics_unchanged=True,
 initial_v26_failure='Before this recorder, V26 rejected nucleus blocks displaced by new headings in field_access and call_dispatch. Existing nucleus blocks were moved back immediately below Hash do Código; recheck passed before rehash.',
 recorder_sha256=sha(Path(__file__)),
 command='python3 00_nucleo/diagnosticos/p1339-l0-freeze-recorder.py',
 no_claims=['No seal','No phase-D RED','No implementation','No GREEN','No commit'])
body=json.dumps(result,ensure_ascii=False,indent=2)
patch='*** Begin Patch\n*** Add File: '+str(target)+'\n'+''.join('+'+line+'\n' for line in body.splitlines())+'*** End Patch\n'
subprocess.run(['apply_patch'],cwd=ROOT,input=patch,text=True,check=True)
print(json.dumps(dict(receipt_sha256=sha(target),prompts=len(prompts),lineage_only_sources=len(sources))))
