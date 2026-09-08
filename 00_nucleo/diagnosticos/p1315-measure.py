"""Measure CSV record ordinal separately from physical error coordinates."""
import importlib.util
import subprocess
from pathlib import Path

D = Path(__file__).resolve().parent
spec = importlib.util.spec_from_file_location('record', D/'p1315-record.py')
r = importlib.util.module_from_spec(spec)
spec.loader.exec_module(r)
expressions = [
    'csv(bytes("a,b\\n1"))',
    'csv(bytes("a,b\\n\\n1"))',
    'csv(bytes("\\"a\\nb\\",c\\n1"))',
    'csv(bytes("\\"a\\nb\\",c\\n1"), row-type: dictionary)',
    'csv(bytes("a,b\\n\\"1\\n2\\",3\\n4"))',
    'csv(bytes("a,b\\n\\"1\\n2\\",3\\n4"), row-type: dictionary)',
    'csv(bytes("\\n\\na,b\\n1"))',
    'csv(bytes("a,b\\r\\n\\"1\\r\\n2\\",3\\r\\n4"))',
    'csv(bytes("a;b\\n1;2;3"), delimiter: ";")',
    'csv(bytes("\\"a\\nb\\",c\\n1,2"))',
    'csv(bytes((255,)))',
]
before=r.state()
rows=[]
binaries={'baseline':r.BASE, 'vanilla':'/usr/local/bin/typst'}
for expr in expressions:
    for product,binary in binaries.items():
        argv=[binary,'eval',expr]
        p=subprocess.run(argv,cwd=r.ROOT,capture_output=True,text=True,timeout=30)
        rows.append(dict(expr=expr,product=product,argv=argv,exit=p.returncode,
                         stdout=p.stdout,stderr=p.stderr))
sources=['lab/typst-original/crates/typst-library/src/loading/csv.rs',r.OWNER,r.L0]
r.save('measurement',dict(before=before,after=r.state(),rows=rows,
    binaries={k:dict(path=v,sha256=r.sha(v)) for k,v in binaries.items()},
    upstream='a51e02804',sources={s:r.sha(r.ROOT/s) for s in sources},
    script_sha256=r.sha(__file__)))
for row in rows:
    print(row['expr'],row['product'],(row['stderr'] or row['stdout']).splitlines()[0])
