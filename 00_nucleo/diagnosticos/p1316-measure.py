"""Source-origin observations before the P1316 contract or candidate."""
import importlib.util
import subprocess
from pathlib import Path

D = Path(__file__).resolve().parent
spec = importlib.util.spec_from_file_location('record', D/'p1316-record.py')
r = importlib.util.module_from_spec(spec)
spec.loader.exec_module(r)
expressions = [
    'csv(bytes("a,b\\n1"))',
    'csv(bytes((255,)))',
    'csv(bytes("\\"a\\nb\\",c\\n1"), row-type: dictionary)',
    '{ let f = csv.with(bytes("a,b\\n1")); f() }',
    '{ let a = arguments(bytes("a,b\\n1")); csv(..a) }',
    'csv(delimiter: ";", bytes("a;b\\n1"))',
    'csv(..arguments(bytes("a,b\\n1")).map((..a) => bytes("a,b\\n1")))',
    'csv(bytes("a,b\\n1"), delimiter: "ab")',
    'csv(bytes("a,b\\n1"), unknown: true)',
    'csv("unequal.csv")',
    'csv(path("unequal.csv"))',
    'csv(bytes("a,b\\n1,2"))',
]
before=r.state()
rows=[]
binaries={'baseline':r.BASE, 'vanilla':'/usr/local/bin/typst'}
cwd='/tmp/p1314-ab-fixtures'
for expr in expressions:
    for product,binary in binaries.items():
        argv=[binary,'eval',expr]
        p=subprocess.run(argv,cwd=cwd,capture_output=True,text=True,timeout=30)
        rows.append(dict(expr=expr,product=product,argv=argv,cwd=cwd,exit=p.returncode,
                         stdout=p.stdout,stderr=p.stderr))
r.save('measurement',dict(before=before,after=r.state(),rows=rows,
    binaries={k:dict(path=v,sha256=r.sha(v)) for k,v in binaries.items()},
    upstream='a51e02804', fixture_sha256=r.sha(Path(cwd)/'unequal.csv'),
    sources={n:dict(sha256=r.sha(r.ROOT/n),text=(r.ROOT/n).read_text()) for n in (r.OWNER,r.L0)},
    prior_artifacts={str(p.relative_to(r.ROOT)):r.sha(p) for p in D.glob('p1315-*') if p.is_file()},
    script_sha256=r.sha(__file__)))
for row in rows:
    print(row['expr'],row['product'],row['stderr'] or row['stdout'])
