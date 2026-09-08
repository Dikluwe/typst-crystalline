"""Corrected constructions; initial measurements remain immutable."""
import importlib.util
import subprocess
from pathlib import Path

D = Path(__file__).resolve().parent
spec = importlib.util.spec_from_file_location('record', D/'p1317-record.py')
r = importlib.util.module_from_spec(spec)
spec.loader.exec_module(r)
before = r.state()
rows = []
for expression in [
    'csv(bytes((97,44,98,10,49,44,255)))',
    'csv(bytes((97,44,98,10,49,44,255)), row-type: dictionary)',
    'csv(bytes((97,44,98,10,255)))',
    'csv(bytes((97,44,98,10,255)), row-type: dictionary)',
]:
    for product, binary in [('baseline', r.BASE), ('vanilla', '/usr/local/bin/typst')]:
        argv = [binary, 'eval', expression]
        p = subprocess.run(argv, cwd=r.ROOT, capture_output=True, text=True, timeout=30)
        rows.append(dict(expr=expression, product=product, argv=argv, cwd=str(r.ROOT),
                         exit=p.returncode, stdout=p.stdout, stderr=p.stderr))
r.save('measurement-focal', dict(before=before, after=r.state(), rows=rows,
    binaries={k:dict(path=v,sha256=r.sha(v)) for k,v in [('baseline',r.BASE),('vanilla','/usr/local/bin/typst')]},
    script_sha256=r.sha(__file__), reason='Bytes concatenation is unsupported in baseline; use literal byte arrays. Initial file cases measured missing-file, not parsing.'))
for row in rows:
    print(row['expr'], row['product'], row['stderr'])
