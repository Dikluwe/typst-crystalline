"""P1322 sampled export/geometry plus closure and causal boundary checks."""
from collections import Counter
import hashlib
import json
from pathlib import Path
import subprocess
import sys
import tempfile
sys.dont_write_bytecode=True
D=Path(__file__).resolve().parent
import importlib.util
def load(name,path):
    spec=importlib.util.spec_from_file_location(name,path);module=importlib.util.module_from_spec(spec);spec.loader.exec_module(module);return module
r=load('p1322_record',D/'p1322-record.py')
sys.path.insert(0,str(r.ROOT/'lab/parity/matrix'))
matrix=load('historical_transversal',r.ROOT/'lab/parity/matrix/runner.py')
functional=load('p1322_functions',D/'p1322-sentinels.py')
PROFILES={p:dict(features=f) for p,f in functional.m.PROFILES.items()}

def freeze():
    before=r.state();r.verify(before)
    manifest=matrix.load_manifest(matrix.HERE/'manifest.yaml')
    assert not matrix.validate_manifest(manifest)
    tmp=Path(tempfile.mkdtemp(prefix='p1322-transversal-',dir='/tmp'))
    subprocess.run(['cp','-a',str(matrix.HERE/'fixtures'),str(tmp/'fixtures')],check=True)
    extra=[]
    for i,expression in enumerate([
        'csv(bytes("\\\"a\\nb\\\",c\\n1"))','csv(bytes("a,b\\r\\n1"))',
        'csv(bytes("á,😀\\r\\n1"))','csv(bytes((97,44,98,10,49,10,255)))',
        '{let f=json; f(42)}','{let f=json.with(42); f()}',
        '{let a=arguments(42); json(..a)}','{let f(..a)=json(..a); f(42)}',
    ]):extra.append(dict(id='boundary-'+str(i),expression=expression))
    r.save('transversal-cases',dict(at=r.now(),matrix=manifest,profiles=PROFILES,extra=extra,temp=str(tmp)))
    paths=[Path(__file__),D/'p1322-transversal-cases.json',D/'p1322-record.py',D/'p1322-sentinels.py',D/'p1322-matrix.py',
        r.ROOT/'lab/parity/matrix/runner.py',r.ROOT/'lab/parity/matrix/svg_morphology.py',r.ROOT/'lab/parity/matrix/manifest.yaml']
    paths += [p for p in (tmp/'fixtures').rglob('*') if p.is_file()]
    paths += [p for p in functional.FIX.rglob('*') if p.is_file()]
    r.save('transversal-freeze',dict(at=r.now(),manifest_sha256=r.sha(D/'p1322-manifest.json'),inputs={str(p):r.sha(p) for p in paths},
        binaries=dict(vanilla=r.baseline()['vanilla'],crystalline=json.loads((D/'p1322-build.json').read_text())['candidate']),
        before=before,after=r.state(),profiles=PROFILES,scope='sample only, no full layout/export claim; exact PDF bytes not criterion'))

def run():
    frozen=json.loads((D/'p1322-transversal-freeze.json').read_text());functional.m.verify(frozen)
    data=json.loads((D/'p1322-transversal-cases.json').read_text());before=r.state();start=r.now()
    tmp=Path(data['temp']);matrix.HERE=tmp
    rows=[];extra=[];closures=[]
    bins={s:Path(v['path']) for s,v in frozen['binaries'].items()}
    for phase in ['normal','repeat','reverse']:
        cases=data['matrix']['cases']
        if phase=='reverse':cases=list(reversed(cases))
        for profile in PROFILES:
            result=[matrix.run_case(c,bins['vanilla'],bins['crystalline'],tmp/phase/profile/c['id'],profile,PROFILES) for c in cases]
            rows.append(dict(phase=phase,profile=profile,results=result,counts=matrix.closed_counts(result)))
            print(phase,profile,matrix.closed_counts(result),flush=True)
            for c in data['extra']:
                row=dict(id=c['id'],expression=c['expression'],profile=profile,phase=phase,universe='supplement')
                for side,binary in bins.items():
                    o=functional.modules.execute(binary,c,profile,phase,side,functional.FIX)
                    reason=None
                    if not o['complete'] or o['exit_code'] not in (0,1):reason='EXECUTION_UNKNOWN'
                    o.update(binary_path=str(binary),binary_sha256=frozen['binaries'][side]['sha256'],features=PROFILES[profile]['features'],
                        reason_code=reason,complete=reason is None,source_sha256=hashlib.sha256(c['expression'].encode()).hexdigest())
                    row[side]=o
                row['runtime_class']=functional.m.classify(row['vanilla'],row['crystalline']);extra.append(row)
            for name in ['captured-only','closure-only']:
                row=dict(id=name,profile=profile,phase=phase,observation='document assertions only')
                for side,binary in bins.items():
                    args=[str(binary),'--color=never','compile',str(functional.FIX/(name+'.typ')),str(tmp/f'{name}-{phase}-{profile}-{side}.svg'),'--format','svg']
                    args += functional.m.old.features(profile)
                    row[side]=functional.csv_audit.invoke(args,functional.FIX)
                row['state']=functional.csv_audit.classify(row['vanilla'],row['crystalline'],'compile_assertions');closures.append(row)
    functional.m.verify(frozen)
    r.save('transversal',dict(at=start,end=r.now(),manifest_sha256=r.sha(D/'p1322-manifest.json'),freeze_sha256=r.sha(D/'p1322-transversal-freeze.json'),
        before=before,after=r.state(),binaries=frozen['binaries'],matrix=rows,extra=extra,closures=closures,
        artifacts={str(p):r.sha(p) for p in tmp.rglob('*') if p.is_file()},temp=str(tmp)))

if __name__=='__main__':
    if sys.argv[1]=='freeze':freeze()
    elif sys.argv[1]=='run':run()
    else:raise ValueError(sys.argv[1])
