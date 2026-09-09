"""Scoped P1336 provenance. Existing product changes and old receipts are protected."""
import base64,datetime,hashlib,importlib.util,json,os,subprocess,sys,time
from pathlib import Path
sys.dont_write_bytecode=True
ROOT=Path(__file__).resolve().parents[2];D=ROOT/'00_nucleo/diagnosticos'
L0='00_nucleo/prompts/compiler/eval/bindings/field_access.md'
SOURCE='01_core/src/compiler/eval/bindings/field_access.rs'
TARGET='/tmp/p1336-target.QiOMGq'
def now():return datetime.datetime.now(datetime.timezone.utc).isoformat()
def sha(p):
    with open(p,'rb') as f:return hashlib.file_digest(f,'sha256').hexdigest()
def git(*a):return subprocess.check_output(['git',*a],cwd=ROOT,text=True)
def state():
    names=git('ls-files','-z','--','00_nucleo/prompts','01_core','02_shell','03_infra','04_wiring','Cargo.toml','Cargo.lock','benches','lab/surface-inventory','lab/typst-original').split('\0')
    return dict(utc=now(),head=git('rev-parse','HEAD').strip(),branch=git('branch','--show-current').strip(),
        status=git('status','--short','--','.',':(exclude)00_nucleo/materialization',':(exclude)00_nucleo/context'),
        diff=git('diff','HEAD','--binary'),diff_stat=git('diff','HEAD','--stat'),staged=git('diff','--cached','--binary'),
        product_inventory={p:sha(ROOT/p) for p in sorted(names) if p and (ROOT/p).is_file()})
def save(name,data):
    p=D/('p1336-'+name+'.json');assert not p.exists(),p
    body=json.dumps(data,ensure_ascii=True,indent=2)+'\n'
    patch='*** Begin Patch\n*** Add File: '+str(p)+'\n'+''.join('+'+s+'\n' for s in body.splitlines())+'*** End Patch\n'
    subprocess.run(['apply_patch'],input=patch,text=True,capture_output=True,cwd=ROOT,check=True)
    print(p,sha(p),flush=True)
def read(name):return json.loads((D/('p1336-'+name+'.json')).read_text())
def verify(s,strict=False):
    b=read('baseline')['state']
    assert s['head']==b['head'] and s['branch']==b['branch'] and s['staged']==b['staged']
    changed={p for p in set(s['product_inventory'])|set(b['product_inventory']) if s['product_inventory'].get(p)!=b['product_inventory'].get(p)}
    assert changed <= (set() if strict else {L0,SOURCE}),changed
def command(name,argv):
    before=state();verify(before);at=now();t=time.monotonic()
    env={**os.environ,'CARGO_TARGET_DIR':TARGET,'PYTHONDONTWRITEBYTECODE':'1'}
    try:
        p=subprocess.run(argv,cwd=ROOT,env=env,capture_output=True,text=True,timeout=2700);code,out,err=p.returncode,p.stdout,p.stderr
    except subprocess.TimeoutExpired as e:
        code=124;out=e.stdout or b'';err=e.stderr or b''
        out=out.decode() if isinstance(out,bytes) else out;err=err.decode() if isinstance(err,bytes) else err
        err+='\nEXECUTION_UNKNOWN timeout\n'
    after=state();verify(after)
    data=dict(at=at,end=now(),seconds=time.monotonic()-t,argv=argv,cwd=str(ROOT),env={'CARGO_TARGET_DIR':TARGET},before=before,after=after,exit=code,stdout=out,stderr=err)
    if (D/'p1336-manifest.json').exists():data['manifest_sha256']=sha(D/'p1336-manifest.json')
    binary=Path(TARGET)/'release/typst'
    if binary.exists():data['binary']=dict(path=str(binary),sha256=sha(binary))
    save(name,data)
    if name=='unit-red':print('Private test output retained for independent reviewer; exit:',code)
    else:print(out[-900:],err[-900:])
    return code
def init():
    s=state();old=json.loads((D/'p1335-closure.json').read_text())
    assert all(s[k]==old['state'][k] for k in ['head','diff','staged','product_inventory'])
    historical={str(p):sha(p) for p in D.rglob('*') if p.is_file() and not p.relative_to(D).parts[0].startswith('p1336-') and '__pycache__' not in p.parts}
    assert all(sha(p)==h for p,h in {**old['historical_preserved'],**old['artifacts']}.items())
    save('baseline',dict(at=now(),state=s,prior_closure_sha256=sha(D/'p1335-closure.json'),historical_preserved=historical,
        snapshots={p:(ROOT/p).read_text() for p in [L0,SOURCE]},target=TARGET,
        temp_policy='Exclusive /tmp target: /dev/shm observed ro,nosuid,nodev,inode64; no attempt to change mount. Cache copy without mutable hardlinks.',
        vanilla=dict(path='/usr/local/bin/typst',sha256=sha('/usr/local/bin/typst'),upstream='a51e02804'),
        predecessor=dict(path='/tmp/p1335-target.EkAvyv/release/typst',sha256=sha('/tmp/p1335-target.EkAvyv/release/typst'))))
def measure():
    b=read('baseline');before=state();verify(before,strict=True);rows=[]
    for expr in ['(1).nope','"abc".nope','{let x=7;\n x.campo-longo}','{let s="é";\n s.ausência}','(3).abs()','"abc".len()','true.nope','int.nope']:
        row=dict(expression=expr,source_sha256=hashlib.sha256(expr.encode()).hexdigest(),observations={})
        for side,bin in [('vanilla',b['vanilla']),('crystalline',b['predecessor'])]:
            argv=[bin['path'],'--color=never','eval',expr,'--format','json'];at=now()
            p=subprocess.run(argv,cwd=ROOT,capture_output=True,timeout=30)
            row['observations'][side]=dict(at=at,end=now(),argv=argv,cwd=str(ROOT),binary_sha256=sha(bin['path']),exit=p.returncode,
                stdout=p.stdout.decode(),stderr=p.stderr.decode(),stdout_base64=base64.b64encode(p.stdout).decode(),stderr_base64=base64.b64encode(p.stderr).decode())
        rows.append(row)
    after=state();verify(after,strict=True)
    save('measurement',dict(at=now(),baseline_sha256=sha(D/'p1336-baseline.json'),before=before,after=after,rows=rows))
if __name__=='__main__':
    if sys.argv[1]=='init':init()
    elif sys.argv[1]=='measure':measure()
    else:sys.exit(command(sys.argv[1],sys.argv[2:]))
