"""Additional byte/character column boundary, before tests or candidate."""
import importlib.util
import subprocess
from pathlib import Path
D = Path(__file__).resolve().parent
spec = importlib.util.spec_from_file_location('r', D/'p1318-record.py')
r = importlib.util.module_from_spec(spec)
spec.loader.exec_module(r)
before = r.state()
rows = []
for name,data in [('invalid-unicode-column', 'a,b\né,😀\r'.encode()+b'1,\xff'),
                  ('valid-unicode-column', 'a,b\né,😀\r\n1'.encode())]:
    for mode in ['array','dictionary']:
        expression = 'csv(bytes(('+','.join(map(str,data))+',)), row-type: '+mode+')'
        for product,binary in [('baseline',r.BASE),('vanilla','/usr/local/bin/typst')]:
            argv=[binary,'eval',expression]
            p=subprocess.run(argv,cwd=r.ROOT,capture_output=True,text=True,timeout=30)
            rows.append(dict(id=name,mode=mode,data=list(data),product=product,argv=argv,
                cwd=str(r.ROOT),exit=p.returncode,stdout=p.stdout,stderr=p.stderr))
r.save('measurement-columns',dict(before=before,after=r.state(),rows=rows,
    binaries={k:dict(path=v,sha256=r.sha(v)) for k,v in [('baseline',r.BASE),('vanilla','/usr/local/bin/typst')]},script_sha256=r.sha(__file__)))
for row in rows:
    print(row['id'],row['mode'],row['product'],row['stderr'].splitlines()[0])
