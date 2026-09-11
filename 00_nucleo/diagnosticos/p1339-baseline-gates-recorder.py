"""Record pre-candidate build/lint controls without changing frozen inputs."""
import datetime
import hashlib
import json
import os
from pathlib import Path
import subprocess
import time

ROOT=Path(__file__).resolve().parents[2]
D=ROOT/'00_nucleo/diagnosticos'
def sha(path):
    return hashlib.sha256(Path(path).read_bytes()).hexdigest()
def now():
    return datetime.datetime.now(datetime.timezone.utc).isoformat()
def git(*args):
    return subprocess.check_output(['git',*args],cwd=ROOT,text=True)
out=D/'p1339-baseline-gates.json'
assert not out.exists()
freeze=D/'p1339-l0-freeze.json'
assert sha(freeze)=='397c136fc8710d44b2f7537193fe5b9c44ab89d40a99bf6b994296e05e7c4d04'
f=json.loads(freeze.read_text())
def check_inputs():
    for group in ('l0_sha256','lineage_only_source_sha256'):
        assert all(sha(ROOT/p)==h for p,h in f[group].items())
check_inputs()
start=now()
before=dict(head=git('rev-parse','HEAD').strip(),status=git('status','--short'),diff_stat=git('diff','HEAD','--stat'))
checks=[]
env=dict(os.environ,CARGO_TARGET_DIR='/tmp/p1339-target.UD8gh7')
for argv in [
 ['cargo','build','--workspace','--release','--locked','--offline'],
 ['cargo','fmt','--all','--','--check'],
 ['git','diff','--check'],
 ['crystalline-lint','.'],
 ['crystalline-lint','--checks','v5,v15,v26','--fail-on','warning','.'],
]:
    at=now(); tick=time.monotonic()
    p=subprocess.run(argv,cwd=ROOT,env=env,text=True,capture_output=True,timeout=300)
    checks.append(dict(argv=argv,cwd=str(ROOT),start=at,end=now(),seconds=time.monotonic()-tick,exit=p.returncode,stdout=p.stdout,stderr=p.stderr))
    assert p.returncode==0, (argv,p.stderr[-1000:])
check_inputs()
binary=Path(env['CARGO_TARGET_DIR'])/'release/typst'
result=dict(state='PRE_CANDIDATE_CONTROLS_ONLY; NOT_GREEN_OR_CLOSURE',
 manifest_sha256=sha(D/'p1339-authority-manifest-r2.json'),freeze_sha256=sha(freeze),
 start=start,end=now(),before=before,after=dict(head=git('rev-parse','HEAD').strip(),status=git('status','--short'),diff_stat=git('diff','HEAD','--stat')),
 checks=checks,environment_override=dict(CARGO_TARGET_DIR=env['CARGO_TARGET_DIR']),
 rebuilt_baseline=dict(path=str(binary),sha256=sha(binary)),
 warmup='A preceding successful identical offline release build populated this isolated target. Recorded build below uses that cache; source still has only lineage changes.',
 full_lint_has_existing_warnings='warning:' in checks[3]['stdout']+checks[3]['stderr'],
 recorder_sha256=sha(__file__),command='python3 00_nucleo/diagnosticos/p1339-baseline-gates-recorder.py',
 limitations=['No candidate semantics','No independent phase-D RED','No seal','No workspace test claim','Full lint exit success does not mean zero warnings'])
body=json.dumps(result,ensure_ascii=False,indent=2)
patch='*** Begin Patch\n*** Add File: '+str(out)+'\n'+''.join('+'+line+'\n' for line in body.splitlines())+'*** End Patch\n'
subprocess.run(['apply_patch'],cwd=ROOT,input=patch,text=True,check=True)
print(json.dumps(dict(receipt_sha256=sha(out),rebuilt_baseline=result['rebuilt_baseline'])))
