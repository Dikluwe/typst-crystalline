"""P1323 immutable receipts; preserve unrelated product and historical evidence."""
import importlib.util
import json
import os
from pathlib import Path
import subprocess
import sys
import time
sys.dont_write_bytecode=True
D=Path(__file__).resolve().parent
spec=importlib.util.spec_from_file_location('previous_record',D/'p1322-record.py')
old=importlib.util.module_from_spec(spec);spec.loader.exec_module(old)
ROOT=old.ROOT
TARGET='/tmp/p1323-target.9NrOxY'
ALLOWED={'00_nucleo/prompts/wiring.md','04_wiring/src/main.rs'}
sha=old.sha
now=old.now
state=old.state

def save(name,data):
    path=D/('p1323-'+name+'.json')
    if path.exists():raise FileExistsError(path)
    payload=json.dumps(data,ensure_ascii=True,indent=2)+'\n'
    patch='*** Begin Patch\n*** Add File: '+str(path)+'\n'+''.join('+'+s+'\n' for s in payload.splitlines())+'*** End Patch\n'
    subprocess.run(['apply_patch'],input=patch,text=True,capture_output=True,check=True,cwd=ROOT)
    print(path,sha(path),flush=True)

def verify(s):
    b=json.loads((D/'p1323-baseline.json').read_text())['state']
    assert s['head']==b['head'] and s['staged']==b['staged']
    assert {k:v for k,v in s['product_inventory'].items() if k not in ALLOWED}=={k:v for k,v in b['product_inventory'].items() if k not in ALLOWED}

def init():
    s=state();old.verify(s)
    fixture=D/'p1323-fixtures/hello.typ'
    bins={'vanilla':'/usr/local/bin/typst','crystalline':'/tmp/p1322-target.Ir19xI/release/typst'}
    assert sha(bins['vanilla'])==old.VANILLA
    assert sha(bins['crystalline'])==json.loads((D/'p1322-build.json').read_text())['candidate']['sha256']
    rows=[]
    for profile,features in {'default':[],'html':['html'],'a11y':['a11y-extras'],'html+a11y':['html','a11y-extras']}.items():
        for side,binary in bins.items():
            output=Path(TARGET)/(profile+'-'+side+'.html')
            argv=[binary,'--color=never','compile',str(fixture),str(output),'--format','html']
            if features:argv+=['--features',','.join(features)]
            at=now();p=subprocess.run(argv,cwd=ROOT,capture_output=True,text=True,timeout=30)
            rows.append(dict(profile=profile,side=side,argv=argv,cwd=str(ROOT),at=at,exit=p.returncode,stdout=p.stdout,stderr=p.stderr,
                artifact_sha256=sha(output) if output.exists() else None))
    save('baseline',dict(at=now(),state=s,binaries={k:dict(path=v,sha256=sha(v)) for k,v in bins.items()},rows=rows,
        source=dict(path=str(fixture),sha256=sha(fixture),text=fixture.read_text()),target=TARGET,
        target_reason='RAM target has executable rw host mount; sandbox read-only view requires scoped escalation; copied cache without hardlinks.',
        prior_inputs={str(D/n):sha(D/n) for n in ['p1322-closure.json','p1322-review-final.json','p1322-classification-selection-r2.json']},
        original_owner=(ROOT/'04_wiring/src/main.rs').read_text(),original_prompt=(ROOT/'00_nucleo/prompts/wiring.md').read_text()))

def command(name,argv):
    before=state();verify(before);start=now();tick=time.monotonic()
    env={**os.environ,'CARGO_TARGET_DIR':TARGET,'PYTHONDONTWRITEBYTECODE':'1'}
    p=subprocess.run(argv,cwd=ROOT,env=env,capture_output=True,text=True,timeout=2400)
    after=state();verify(after)
    save(name,dict(at=start,end=now(),seconds=time.monotonic()-tick,argv=argv,cwd=str(ROOT),target=TARGET,
        before=before,after=after,exit=p.returncode,stdout=p.stdout,stderr=p.stderr,
        manifest_sha256=sha(D/'p1323-manifest.json') if (D/'p1323-manifest.json').exists() else None,
        binary=dict(path=TARGET+'/release/typst',sha256=sha(TARGET+'/release/typst'))))
    if name=='unit-red':
        print('Unit RED process exit:',p.returncode,'Private test output retained for reviewer; not displayed to implementer.')
    else:
        print(p.stdout[-1800:],p.stderr[-800:])
    sys.exit(p.returncode)

if __name__=='__main__':
    if sys.argv[1]=='init':init()
    else:command(sys.argv[1],sys.argv[2:])
