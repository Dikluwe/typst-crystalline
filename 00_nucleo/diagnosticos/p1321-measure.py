"""Focal baseline measurement before normative change and implementation."""
import importlib.util
from pathlib import Path
import subprocess

spec = importlib.util.spec_from_file_location('record', Path(__file__).with_name('p1321-record.py'))
r = importlib.util.module_from_spec(spec)
spec.loader.exec_module(r)
before = r.state()
expressions = [
    'csv()', 'csv(source: bytes("a,b"))', 'csv(nope: 1, source: 42)',
    'csv(bytes("a,b"), source: 42)', 'csv(bytes("a,b"), 42)',
    'csv(bytes("a,b\\n1"), 42)', 'csv("absent.csv", 42)',
    'csv(42, nope: 1)', 'csv(bytes("a,b"), nope: 1, delimiter: "xx")',
    'csv(bytes("a,b"), nope: 1, 42)', 'csv(bytes("a,b"), 42, nope: 1)',
    '{let f=csv.with(nope: 1); f(bytes("a,b"), 42)}',
]
rows=[]
for expression in expressions:
    for product, binary in [('baseline',r.BASE),('vanilla',r.VANILLA)]:
        argv=[binary,'--color=never','eval',expression]
        p=subprocess.run(argv,cwd=r.ROOT,capture_output=True,text=True,timeout=30)
        rows.append(dict(expression=expression,product=product,argv=argv,cwd=str(r.ROOT),exit=p.returncode,stdout=p.stdout,stderr=p.stderr))
r.save('measurement',dict(before=before,after=r.state(),rows=rows,
    binaries={x:dict(path=p,sha256=r.sha(p)) for x,p in [('baseline',r.BASE),('vanilla',r.VANILLA)]},
    upstream='a51e02804',script_sha256=r.sha(__file__)))
for row in rows:
    if row['product']=='vanilla':
        print(row['expression'],repr(row['stderr']))
