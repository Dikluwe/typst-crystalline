"""P1322 readonly product snapshots and immutable audit receipts."""
import datetime
import hashlib
import json
import os
from pathlib import Path
import subprocess
import sys
import time

sys.dont_write_bytecode = True
ROOT = Path(__file__).resolve().parents[2]
D = ROOT / '00_nucleo/diagnosticos'
STEP = ROOT / '00_nucleo/materialization/typst-passo-1322.md'
TARGET = '/tmp/p1322-target.Ir19xI'
HEAD = 'd31047d7b8af7837c84adae4ded3d2ff50c62093'
VANILLA = '7b4f40c56d6fa95082ebcfd893e275d418ebcaed1b97b62785f78284c63ff7b8'

def sha(path):
    with open(path, 'rb') as stream:
        return hashlib.file_digest(stream, 'sha256').hexdigest()

def now():
    return datetime.datetime.now(datetime.timezone.utc).isoformat()

def git(*args):
    return subprocess.check_output(['git', *args], cwd=ROOT, text=True)

def inventory():
    paths = git('ls-files','-z','--','00_nucleo/prompts','01_core','02_shell',
        '03_infra','04_wiring','benches','Cargo.toml','Cargo.lock',
        'lab/surface-inventory','lab/typst-original').split('\0')
    return {p:sha(ROOT/p) for p in sorted(paths) if p and (ROOT/p).is_file()}

def state():
    return dict(utc=now(),head=git('rev-parse','HEAD').strip(),branch=git('branch','--show-current').strip(),
        status=git('status','--short'),diff=git('diff','HEAD','--binary'),
        diff_stat=git('diff','HEAD','--stat'),staged=git('diff','--cached','--binary'),product_inventory=inventory())

def save(name,data):
    path=D/('p1322-'+name+'.json')
    if path.exists(): raise FileExistsError(path)
    payload=json.dumps(data,ensure_ascii=True,indent=2)+'\n'
    patch='*** Begin Patch\n*** Add File: '+str(path)+'\n'+''.join('+'+line+'\n' for line in payload.rstrip('\n').split('\n'))+'*** End Patch\n'
    subprocess.run(['apply_patch'],input=patch,text=True,cwd=ROOT,capture_output=True,check=True)
    print(str(path),sha(path),flush=True)

def baseline():
    return json.loads((D/'p1322-baseline.json').read_text())

def verify(s):
    b=baseline()['state']
    assert all(s[k]==b[k] for k in ('head','diff','staged','product_inventory')), 'Protected baseline changed'

def init():
    s=state()
    assert s['head']==HEAD and sha('/usr/local/bin/typst')==VANILLA
    names=['p1309-final-report.md','p1309-probe-catalog.json','p1309-owner-ledger.tsv',
        'p1309-verification.json','p1320-o-que-falta-para-paridade.md','p1321-final-report.md']
    untracked=git('ls-files','--others','--exclude-standard','-z','--','00_nucleo/diagnosticos').split('\0')
    save('baseline',dict(state=s,local=datetime.datetime.now().astimezone().isoformat(),
        inputs={str(D/n):sha(D/n) for n in names},untracked_at_start={p:sha(ROOT/p) for p in untracked if p and (ROOT/p).is_file()},
        step_sha256=sha(STEP),recorder_sha256=sha(__file__),target=TARGET,
        target_reason='/dev/shm free 3.3G is less than 3.8G baseline cache; /tmp dedicated copy without hardlinks',
        vanilla=dict(path='/usr/local/bin/typst',sha256=VANILLA,upstream='a51e02804')))
    save('manifest',dict(at=now(),baseline_sha256=sha(D/'p1322-baseline.json'),step=dict(path=str(STEP),sha256=sha(STEP)),
        regime='auditoria bilateral com autores separados, sem atestação técnica de isolamento; sem mutação produtiva',
        roles={
            'inventory':dict(executor='/root/p1322_inventory',context='fresh task only',read='step, skill, historical inventory methods/catalogs, current source, pinned binaries; no candidate matrix',write='p1322-inventory-* and p1322-probe-catalog.json'),
            'operator':dict(executor='/root',context='conversation',read='step, frozen catalog, binaries, prior methods and receipts',write='p1322 provenance/runners/matrices/sentinels/gates/final report and own step'),
            'classifier':dict(executor='/root/p1322_classifier',context='fresh task only',read='catalog, outputs, history, sources and full relevant L0',write='p1322-classification-*'),
            'reviewer':dict(executor='/root/p1322_review',context='fresh task only',read='protected inputs and judged outputs',write='p1322-review-*; not judged inputs')},
        policy=dict(unknown='mandatory Unknown blocks closure/selection',product_readonly=True,commit=False,
            orders=['normal','repeat','reverse'],max_workers=8,probe_timeout_seconds=30,
            calibration='maximum two focal revisions without gain before method review; full rerun only after focal validation',
            outputs='new p1322-* diagnostics and dedicated temporary directories only')))

def command(name,argv):
    before=state();verify(before);start=now();tick=time.monotonic()
    env=dict(CARGO_TARGET_DIR=TARGET,PYTHONDONTWRITEBYTECODE='1')
    p=subprocess.run(argv,cwd=ROOT,env={**os.environ,**env},capture_output=True,text=True,timeout=2700)
    after=state()
    data=dict(at=start,end=now(),seconds=time.monotonic()-tick,argv=argv,cwd=str(ROOT),env=env,
        before=before,after=after,exit=p.returncode,stdout=p.stdout,stderr=p.stderr,
        manifest_sha256=sha(D/'p1322-manifest.json'),recorder_sha256=sha(__file__))
    binary=Path(TARGET)/'release/typst'
    if binary.exists(): data['candidate']=dict(path=str(binary),sha256=sha(binary))
    save(name,data);verify(after)
    print(p.stdout[-1400:],p.stderr[-1400:]);sys.exit(p.returncode)

if __name__=='__main__':
    if sys.argv[1]=='init':init()
    else:command(sys.argv[1],sys.argv[2:])
