"""Measure existing real Path/Str fixtures before P1319 L0 and candidate."""
import importlib.util
import subprocess
from pathlib import Path
import time

D = Path(__file__).resolve().parent
spec = importlib.util.spec_from_file_location('record', D/'p1319-record.py')
r = importlib.util.module_from_spec(spec)
spec.loader.exec_module(r)
cwd = Path('/tmp/p1318-ab-fixtures')
expressions = []
for name in ['crlf-between', 'invalid-after-earlier-ls']:
    for mode in ['array', 'dictionary']:
        for route in ['str', 'path', 'with', 'args', 'detached', 'excess']:
            path = '"'+name+'.csv"'
            opts = 'row-type: '+mode
            expr = {
                'str':f'csv({path}, {opts})',
                'path':f'csv(path({path}), {opts})',
                'with':f'{{let f=csv.with({path}, {opts}); f()}}',
                'args':f'{{let a=arguments({opts}, {path}); csv(..a)}}',
                'detached':f'csv(..arguments(0).map((..a) => {path}), {opts})',
                'excess':f'csv({path}, 42, {opts})',
            }[route]
            expressions.append((name+'-'+mode+'-'+route, expr))
expressions += [('missing','csv("absent-p1319.csv")'),
                ('invalid-option','csv("invalid-after-earlier-ls.csv", delimiter: "xx")')]
before = r.state()
rows = []
binaries = {'baseline':r.BASE, 'vanilla':'/usr/local/bin/typst'}
started = time.monotonic()
for name, expr in expressions:
    for product, binary in binaries.items():
        argv = [binary, 'eval', expr]
        p = subprocess.run(argv, cwd=cwd, capture_output=True, text=True, timeout=30)
        rows.append(dict(id=name, expr=expr, product=product, argv=argv,
                         cwd=str(cwd), exit=p.returncode, stdout=p.stdout, stderr=p.stderr))
r.save('measurement', dict(before=before, after=r.state(), rows=rows,
    seconds=time.monotonic()-started,
    binaries={k:dict(path=v,sha256=r.sha(v)) for k,v in binaries.items()}, upstream='a51e02804',
    sources={n:dict(sha256=r.sha(r.ROOT/n),text=(r.ROOT/n).read_text()) for n in (r.OWNER,r.L0)},
    fixtures={str(cwd/(n+'.csv')):dict(sha256=r.sha(cwd/(n+'.csv')),bytes=list((cwd/(n+'.csv')).read_bytes())) for n in ['crlf-between','invalid-after-earlier-ls']},
    script_sha256=r.sha(__file__)))
for row in rows:
    print(row['id'], row['product'], repr(row['stderr'] or row['stdout']))
