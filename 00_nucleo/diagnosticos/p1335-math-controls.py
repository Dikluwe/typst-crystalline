"""Supplemental causal controls requested before classification; no product mutation."""
import importlib.util,json,sys
from pathlib import Path
sys.dont_write_bytecode=True
D=Path(__file__).resolve().parent
spec=importlib.util.spec_from_file_location('m',D/'p1335-matrix.py');m=importlib.util.module_from_spec(spec);spec.loader.exec_module(m);r=m.r
cases=[dict(id='math-generic-content',expression='{let f(x)=[ok]; $f(1)$}'),dict(id='math-unshadowed-abs',expression='$abs(1)$'),dict(id='math-unshadowed-sqrt',expression='$sqrt(1)$')]
if sys.argv[1]=='freeze':
    f=json.loads((D/'p1335-runtime-freeze.json').read_text());m.verify(f)
    r.save('math-controls-freeze',dict(at=r.now(),manifest_sha256=r.sha(D/'p1335-manifest.json'),inputs={str(D/'p1335-math-controls.py'):r.sha(__file__),str(D/'p1335-runtime-freeze.json'):r.sha(D/'p1335-runtime-freeze.json')},binaries=f['binaries'],cases=cases,profiles=m.PROFILES,policy='Generic Content-returning function versus unshadowed math functions; complements pre-frozen shadow controls boundary8–11. Raw bilateral equality, separate denominator.'))
else:
    f=json.loads((D/'p1335-math-controls-freeze.json').read_text());m.verify(f);before=r.state();rows=[]
    for phase in ['normal','repeat','reverse']:
        for c in reversed(f['cases']) if phase=='reverse' else f['cases']:
            for p in m.PROFILES:
                row=dict(id=c['id'],expression=c['expression'],profile=p,phase=phase,universe='supplement')
                for side in ['crystalline','vanilla'] if phase=='reverse' else ['vanilla','crystalline']:row[side]=m.observe(f['binaries'][side],c,p,side,phase)
                row['runtime_class']=m.classify(row['vanilla'],row['crystalline']);rows.append(row)
    m.verify(f);r.save('math-controls',dict(at=r.now(),manifest_sha256=r.sha(D/'p1335-manifest.json'),freeze_sha256=r.sha(D/'p1335-math-controls-freeze.json'),before=before,after=r.state(),rows=rows))
