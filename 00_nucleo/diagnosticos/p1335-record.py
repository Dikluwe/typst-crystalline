"""Readonly product baseline and immutable P1335 audit receipts."""
import datetime, hashlib, importlib.util, json, os, subprocess, sys, time
from pathlib import Path
sys.dont_write_bytecode=True
ROOT=Path(__file__).resolve().parents[2];D=ROOT/'00_nucleo/diagnosticos'
STEP=ROOT/'00_nucleo/materialization/typst-passo-1335.md'
TARGET='/tmp/p1335-target.EkAvyv'
VANILLA='7b4f40c56d6fa95082ebcfd893e275d418ebcaed1b97b62785f78284c63ff7b8'
def sha(path):
    with open(path,'rb') as f:return hashlib.file_digest(f,'sha256').hexdigest()
def now():return datetime.datetime.now(datetime.timezone.utc).isoformat()
def git(*args):return subprocess.check_output(['git',*args],cwd=ROOT,text=True)
def inventory():
    paths=git('ls-files','-z','--','00_nucleo/prompts','01_core','02_shell','03_infra','04_wiring','benches','Cargo.toml','Cargo.lock','lab/surface-inventory','lab/typst-original').split('\0')
    return {p:sha(ROOT/p) for p in sorted(paths) if p and (ROOT/p).is_file()}
def state():
    return dict(utc=now(),head=git('rev-parse','HEAD').strip(),branch=git('branch','--show-current').strip(),
        status=git('status','--short','--','.',' :(exclude)00_nucleo/materialization'.strip(),':(exclude)00_nucleo/context'),
        diff=git('diff','HEAD','--binary'),diff_stat=git('diff','HEAD','--stat'),staged=git('diff','--cached','--binary'),product_inventory=inventory())
def save(name,data):
    p=D/('p1335-'+name+'.json');assert not p.exists(),p
    body=json.dumps(data,ensure_ascii=True,indent=2)+'\n'
    patch='*** Begin Patch\n*** Add File: '+str(p)+'\n'+''.join('+'+s+'\n' for s in body.splitlines())+'*** End Patch\n'
    subprocess.run(['apply_patch'],input=patch,text=True,cwd=ROOT,capture_output=True,check=True)
    print(p,sha(p),flush=True)
def baseline():return json.loads((D/'p1335-baseline.json').read_text())
def verify(s):
    b=baseline()['state'];assert all(s[k]==b[k] for k in ('head','diff','staged','product_inventory')),'protected state changed'
def init():
    s=state();prior=json.loads((D/'p1334-closure.json').read_text())
    assert all(s[k]==prior['state'][k] for k in ('head','diff','staged'))
    assert all(s['product_inventory'].get(p)==h for p,h in prior['state']['product_inventory'].items())
    protected={**prior['historical_preserved'],**prior['artifacts']}
    assert all(sha(p)==h for p,h in protected.items())
    assert sha('/usr/local/bin/typst')==VANILLA
    names=['p1322-o-que-falta-para-paridade.md','p1322-probe-catalog.json','p1322-classification-owner-ledger.tsv','p1322-classification-selection-r2.json','p1322-closure.json','p1334-final-report.md','p1334-metrics.json','p1334-ab-freeze-r1.json','p1334-closure.json']
    originals={str(p):sha(p) for p in D.rglob('*') if p.is_file() and not p.relative_to(D).parts[0].startswith('p1335-') and '__pycache__' not in p.parts}
    save('baseline',dict(at=now(),state=s,inputs={str(D/n):sha(D/n) for n in names},historical_preserved=originals,
        previous_closure_sha256=sha(D/'p1334-closure.json'),step_sha256=sha(STEP),recorder_sha256=sha(__file__),target=TARGET,
        target_capacity=dict(shm_available_bytes=3416522752,cache_bytes=3983365101,reason='cache exceeds free RAM filesystem; exclusive /tmp target, copied without hardlinks'),
        vanilla=dict(path='/usr/local/bin/typst',sha256=VANILLA,upstream='a51e02804')))
    save('manifest',dict(at=now(),baseline_sha256=sha(D/'p1335-baseline.json'),step=dict(path=str(STEP),sha256=sha(STEP)),
        regime='Auditoria segregada executada sem atestação técnica de isolamento; sem implementação/mutantes produtivos ou selo de refinamento',
        roles={
            'inventory':dict(executor='/root/p1335_inventory',context='fresh task only',read='step, skill, historical inventory methods/catalogs, current enumerator/product source, pinned binaries; no P1335 candidate matrix or selection',write='p1335-inventory-* and p1335-probe-catalog.json; dedicated tmp'),
            'operator':dict(executor='/root',context='conversation',read='frozen inputs, sources, outputs and historical methods',write='p1335 provenance, runners, matrices, supplements, gates, final report and closure'),
            'classifier':dict(executor='/root/p1335_classifier',context='fresh task only',read='step, history, source/full relevant L0, frozen catalog and public outputs',write='p1335-classification-*'),
            'reviewer':dict(executor='/root/p1335_review',context='fresh task only',read='protected inputs and judged outputs',write='p1335-review-* only; no edits to judged inputs')},
        policy=dict(unknown='mandatory Unknown or instability blocks closure/selection; report valid partial findings',product_readonly=True,commit=False,
            outputs='new p1335-* diagnostics and exclusive temporaries',orders=['normal','repeat','reverse'],max_workers=8,probe_timeout_seconds=30,
            observables='public lookup/kind/repr and full exit/stdout/stderr; functional and transversal projections separate from inventory denominator',
            budget='Two focal revisions without discriminatory gain require method review; run each full order once per validated runner; bounded command timeout 2700s; no unbounded retries or weakening Unknown')))
def command(name,argv):
    before=state();verify(before);start=now();tick=time.monotonic();env=dict(CARGO_TARGET_DIR=TARGET,PYTHONDONTWRITEBYTECODE='1')
    try:
        p=subprocess.run(argv,cwd=ROOT,env={**os.environ,**env},capture_output=True,text=True,timeout=2700)
        code,out,err=p.returncode,p.stdout,p.stderr
    except subprocess.TimeoutExpired as e:
        code=124;out=e.stdout or '';err=e.stderr or ''
        out=out.decode() if isinstance(out,bytes) else out;err=err.decode() if isinstance(err,bytes) else err
        err+='\nAUDIT_EXECUTION_UNKNOWN: timeout\n'
    after=state();data=dict(at=start,end=now(),seconds=time.monotonic()-tick,argv=argv,cwd=str(ROOT),env=env,before=before,after=after,exit=code,stdout=out,stderr=err,manifest_sha256=sha(D/'p1335-manifest.json'),recorder_sha256=sha(__file__))
    binary=Path(TARGET)/'release/typst'
    if binary.exists():data['candidate']=dict(path=str(binary),sha256=sha(binary))
    save(name,data);verify(after);print(out[-1200:],err[-1200:]);sys.exit(code)
if __name__=='__main__':
    if sys.argv[1]=='init':init()
    else:command(sys.argv[1],sys.argv[2:])
