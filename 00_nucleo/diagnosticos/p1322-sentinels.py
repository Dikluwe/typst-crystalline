"""Fresh functional supplement, separately counted; no edits to old oracles."""
import base64
from collections import Counter
from concurrent.futures import ThreadPoolExecutor
import hashlib
import json
from pathlib import Path
import subprocess
import sys
import tempfile
import time
sys.dont_write_bytecode=True
D=Path(__file__).resolve().parent
import importlib.util
def load(name,path):
    spec=importlib.util.spec_from_file_location(name,path);module=importlib.util.module_from_spec(spec);spec.loader.exec_module(module);return module
r=load('p1322_record',D/'p1322-record.py')
m=load('p1322_matrix',D/'p1322-matrix.py')
legacy=load('p1307_adapter',D/'p1307-r4-oracle.py')
modules=load('p1306_modules',D/'p1306-oracle.py')
csv_audit=load('p1320_cases_only',D/'p1320-audit.py')
FIX=D/'p1322-fixtures'

def create(path,body):
    if path.exists():raise FileExistsError(path)
    patch='*** Begin Patch\n*** Add File: '+str(path)+'\n'+''.join('+'+line+'\n' for line in body.rstrip('\n').split('\n'))+'*** End Patch\n'
    subprocess.run(['apply_patch'],input=patch,text=True,capture_output=True,check=True,cwd=r.ROOT)

def freeze():
    before=r.state();r.verify(before)
    tmp=Path(tempfile.mkdtemp(prefix='p1322-functional-',dir='/tmp'))
    # Byte-for-byte fixture copies made by cp, never edits to historical inputs.
    source=D/'p1320-fixtures'
    assert not FIX.exists()
    subprocess.run(['cp','-a',str(source),str(FIX)],check=True)
    for path,body in modules.FIXTURES.items():create(FIX/path,body)
    cases=[]
    def add(id,expression,family,**extra):
        cases.append(dict(id=id,expression=expression,family=family,route='eval',cwd=str(r.ROOT),**extra))
    old=json.loads((D/'p1308-r2-oracle.json').read_text())
    for i,c in enumerate(old['cases']):
        item={**c,'family':'p1307-p1308','cwd':str(r.ROOT),'legacy_projection':True}
        if item['route']=='compile':
            item['fixture']=str(tmp/f'context-{i}.typ');create(Path(item['fixture']),item['document'])
        cases.append(item)
    for c in modules.cases():
        add('p1306.'+c['id'],c['expression'],'modules')
        cases[-1]['cwd']=str(FIX)
    for name in ['p1311-ab-cases.json','p1321-ab2-cases.json','p1321-ab2-r2-cases.json']:
        data=json.loads((D/name).read_text());data=data.get('cases',[]) if isinstance(data,dict) else data
        for c in data:
            add(name.split('-')[0]+'.'+('r2.' if '-r2-' in name else '')+c['id'],
                c['expr'].replace('00_nucleo/diagnosticos/p1320-fixtures/','00_nucleo/diagnosticos/p1322-fixtures/'),name)
    for c in csv_audit.cases():
        if c['id']=='json-roundtrip':c={**c,'expression':'json(bytes(json.encode((a: (1, true, none)))))'}
        add('p1320.'+c['id'],c['expression'],'p1320-lacunas')
        cases[-1]['cwd']=str(FIX)
        if 'cwd' in c:
            # Dedicated binary fixture copy, not a read from a disappearing old RAM target.
            path=FIX/'binary'/'data-lf.csv'
            if not path.exists():
                path.parent.mkdir(parents=True,exist_ok=True)
                subprocess.run(['cp',str(csv_audit.BINARY_FIXTURE),str(path)],check=True)
            cases[-1]['cwd']=str(path.parent)
    for n in [39,40,41,42,81,256]:add('array-integrity.'+str(n),f'{{let a=range({n}); (a.len(), a, repr(a))}}','repr-integrity')
    for value in ['42','1.5','true','none','(:)','(1,)','x => x','int','bytes("a")','[x]','2pt']:
        add('read-cast.'+str(len(cases)),f'read({value})','read-cast')
    for index,expression in enumerate([
        'cbor.encode(1, 2)','cbor.encode(1, foo: 2)','cbor.encode(foo: 2, 1)','cbor.encode()',
        '{let f=cbor.encode; f(1, foo: 2)}','{let f=cbor.encode.with(foo: 2); f(1)}',
        '{let f=cbor.encode.with(1, 2); f()}','{let a=arguments(foo: 2); cbor.encode(1, ..a)}',
        '{let f(..a)=cbor.encode(..a); f(1, foo: 2)}','cbor(cbor.encode((a: (1, true, none))))',
        'read("../p1322-outside-not-created.txt")',
    ]):add('causal-cbor.'+str(index),expression,'causal-cbor')
    assert len({c['id'] for c in cases})==len(cases)
    for c in cases:
        c.setdefault('source_sha256',hashlib.sha256(c.get('document',c['expression']).encode()).hexdigest())
    r.save('sentinels-cases',dict(at=r.now(),cases=cases,profiles=m.PROFILES,fixture_root=str(FIX),temp=str(tmp)))
    inputs={str(p):r.sha(p) for p in FIX.rglob('*') if p.is_file()}
    inputs.update({str(p):r.sha(p) for p in tmp.rglob('*.typ')})
    for name in ['p1322-sentinels-cases.json','p1322-sentinels.py','p1322-record.py','p1322-matrix.py',
        'p1309-matrix.py','p1309-record-r2.py','p1307-r4-oracle.py','p1308-r2-oracle.json','p1306-oracle.py',
        'p1320-audit.py','p1311-ab-cases.json','p1321-ab2-cases.json','p1321-ab2-r2-cases.json']:
        inputs[str(D/name)]=r.sha(D/name)
    r.save('sentinels-freeze',dict(at=r.now(),manifest_sha256=r.sha(D/'p1322-manifest.json'),inputs=inputs,
        binaries=dict(vanilla=r.baseline()['vanilla'],crystalline=json.loads((D/'p1322-build.json').read_text())['candidate']),
        interpretation='raw bilateral equality separate from frozen legacy projection/preservation; no blanket closure of known debts',
        before=before,after=r.state(),profiles=m.PROFILES))

def run(phase):
    frozen=json.loads((D/'p1322-sentinels-freeze.json').read_text());m.verify(frozen)
    corpus=json.loads((D/'p1322-sentinels-cases.json').read_text());before=r.state();start=r.now();tick=time.monotonic()
    jobs=[(c,p) for c in corpus['cases'] for p in m.PROFILES if not c.get('observations') or p in c['observations']]
    if phase=='reverse':jobs.reverse()
    def pair(job):
        c,profile=job
        row=dict(id=c['id'],expression=c['expression'],profile=profile,family=c['family'],phase=phase,universe='supplement')
        for side in (('crystalline','vanilla') if phase=='reverse' else ('vanilla','crystalline')):
            binary=frozen['binaries'][side]
            argv=[binary['path'],'--color=never',c['route']]
            if c['route']=='eval':argv += [c['expression'],'--format','json']
            else:argv += [c['fixture'],str(Path(corpus['temp'])/f"out-{hashlib.sha256(c['id'].encode()).hexdigest()[:16]}-{profile}-{phase}-{side}.pdf"),'--format','pdf']
            argv += m.old.features(profile)
            reason=None;begin=r.now();clock=time.monotonic()
            try:
                p=subprocess.run(argv,cwd=c['cwd'],capture_output=True,timeout=30)
                code,out,err=p.returncode,p.stdout,p.stderr
                if code not in (0,1):reason='CLI_EXECUTION_FAILURE'
            except (OSError,subprocess.TimeoutExpired) as exc:
                code,out,err=None,b'',str(exc).encode();reason='EXECUTION_UNKNOWN'
            if code==0 and c['route']=='eval':
                try:json.loads(out)
                except (ValueError,UnicodeError):reason='UNPARSEABLE_JSON'
            if code==1 and not err.strip():reason='MISSING_DIAGNOSTIC'
            o=dict(id=c['id'],profile=profile,side=side,argv=argv,cwd=c['cwd'],started_at=begin,
                duration_seconds=time.monotonic()-clock,exit_code=code,stdout=out.decode(errors='replace'),stderr=err.decode(errors='replace'),
                stdout_base64=base64.b64encode(out).decode(),stderr_base64=base64.b64encode(err).decode(),
                stdout_sha256=hashlib.sha256(out).hexdigest(),stderr_sha256=hashlib.sha256(err).hexdigest(),
                complete=reason is None,reason_code=reason,source_sha256=c['source_sha256'],features=m.PROFILES[profile],
                binary_path=binary['path'],binary_sha256=binary['sha256'])
            if c.get('legacy_projection'):
                o['observable']=legacy.envelope(code,o['stdout'],o['stderr'],c)
                if o['observable']['kind']=='Unknown':o.update(complete=False,reason_code='LEGACY_OBSERVABLE_UNKNOWN')
            row[side]=o
        row['runtime_class']=m.classify(row['vanilla'],row['crystalline'])
        if c.get('legacy_projection'):
            row['historical_preservation']=legacy.classify(c['observations'][profile]['future_expected'],row['crystalline']['observable'])
            row['language_projection']=legacy.classify(row['vanilla']['observable'],row['crystalline']['observable'])
        return row
    with ThreadPoolExecutor(max_workers=4) as pool:rows=list(pool.map(pair,jobs))
    m.verify(frozen)
    r.save('sentinels-'+phase,dict(at=start,end=r.now(),seconds=time.monotonic()-tick,manifest_sha256=r.sha(D/'p1322-manifest.json'),
        freeze_sha256=r.sha(D/'p1322-sentinels-freeze.json'),before=before,after=r.state(),binaries=frozen['binaries'],rows=rows,
        counts=dict(Counter(row['runtime_class'] for row in rows))))
    print(Counter(row['runtime_class'] for row in rows),flush=True)

if __name__=='__main__':
    if sys.argv[1]=='freeze':freeze()
    else:
        assert sys.argv[1] in ['normal','repeat','reverse'];run(sys.argv[1])
