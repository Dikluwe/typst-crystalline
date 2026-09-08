"""UTF-8 CSV observations before the P1317 L0 or candidate."""
import importlib.util
import subprocess
from pathlib import Path

D = Path(__file__).resolve().parent
spec = importlib.util.spec_from_file_location('record', D/'p1317-record.py')
r = importlib.util.module_from_spec(spec)
spec.loader.exec_module(r)
expressions = [
    'csv(bytes((255,)))',
    'csv(bytes((255,)), row-type: dictionary)',
    'csv(bytes("a,b\\n1,") + bytes((255,)))',
    'csv(bytes("a,b\\n1,") + bytes((255,)), row-type: dictionary)',
    'csv(bytes("a,b\\n") + bytes((255,)))',
    '{ let f = csv.with(bytes((255,))); f() }',
    '{ let a = arguments(bytes((255,))); csv(..a) }',
    'csv(bytes((255,)), delimiter: "ab")',
    'csv(bytes((255,)), bytes("valid"))',
    'csv("invalid.csv")',
    'csv(path("invalid.csv"), row-type: dictionary)',
    'csv(bytes("a,b\\n1"))',
    'csv(bytes("a,b\\n1,2"))',
]
before = r.state()
rows = []
binaries = {'baseline': r.BASE, 'vanilla': '/usr/local/bin/typst'}
cwd = '/tmp/p1314-ab-fixtures'
for expr in expressions:
    for product, binary in binaries.items():
        argv = [binary, 'eval', expr]
        p = subprocess.run(argv, cwd=cwd, capture_output=True, text=True, timeout=30)
        rows.append(dict(expr=expr, product=product, argv=argv, cwd=cwd,
                         exit=p.returncode, stdout=p.stdout, stderr=p.stderr))
r.save('measurement', dict(before=before, after=r.state(), rows=rows,
    binaries={k:dict(path=v, sha256=r.sha(v)) for k,v in binaries.items()},
    upstream='a51e02804',
    sources={n:dict(sha256=r.sha(r.ROOT/n), text=(r.ROOT/n).read_text()) for n in (r.OWNER,r.L0)},
    prior_artifacts={str(p.relative_to(r.ROOT)):r.sha(p) for prefix in ('p1315-*','p1316-*') for p in D.glob(prefix) if p.is_file()},
    script_sha256=r.sha(__file__)))
for row in rows:
    print(row['expr'], row['product'], row['stderr'] or row['stdout'])
