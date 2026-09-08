"""Position suffix measurements before P1318 specification/candidate."""
import importlib.util
import subprocess
from pathlib import Path

D = Path(__file__).resolve().parent
spec = importlib.util.spec_from_file_location('record', D/'p1318-record.py')
r = importlib.util.module_from_spec(spec)
spec.loader.exec_module(r)
samples = [
    ('lf', b'a,b\n1'), ('crlf', b'a,b\r\n1'), ('cr', b'a,b\r1'),
    ('blank', b'\n\na,b\n1'), ('quoted', b'"a\nb",c\n1'),
    ('utf8-first', bytes([255])), ('utf8-late', b'a,b\n1,\xff'),
    ('utf8-crlf', b'a,b\r\n1,\xff'), ('utf8-cr', b'a,b\r1,\xff'),
    ('unequal-wins', b'a,b\n\xff'),
    ('invalid-after-earlier-unequal', b'a,b\r1\r\xff,2'),
    ('bom', b'\xef\xbb\xbfa,b\n1'),
    ('unicode-line', '"é\u2028😀",b\r1'.encode()),
    ('unicode-line-invalid', '"é\u2028😀",b\r'.encode()+b'1,\xff'),
    ('vt-ff-nel', '"a\v\f\u0085b",c\n1'.encode()),
]
expressions = []
for name, data in samples:
    source = 'bytes((' + ','.join(map(str,data)) + ',))'
    for mode in ['array','dictionary']:
        expressions.append((name+'-'+mode, 'csv('+source+', row-type: '+mode+')'))
expressions += [
    ('with', '{ let f = csv.with(bytes("a,b\\n1")); f() }'),
    ('map', 'csv(..arguments(bytes("a,b\\n1")).map((..a) => bytes("a,b\\n1")))'),
    ('excess', 'csv(bytes("a,b\\n1"), 42)'),
    ('invalid-option', 'csv(bytes("a,b\\n1"), delimiter: "ab")'),
    ('path', 'csv("/tmp/p1314-ab-fixtures/unequal.csv")'),
]
before = r.state()
rows = []
binaries = {'baseline':r.BASE, 'vanilla':'/usr/local/bin/typst'}
for name, expression in expressions:
    for product, binary in binaries.items():
        argv = [binary, 'eval', expression]
        p = subprocess.run(argv, cwd=r.ROOT, capture_output=True, text=True, timeout=30)
        rows.append(dict(id=name, expr=expression, product=product, argv=argv,
                         cwd=str(r.ROOT), exit=p.returncode, stdout=p.stdout, stderr=p.stderr))
r.save('measurement', dict(before=before, after=r.state(), rows=rows,
    binaries={k:dict(path=v,sha256=r.sha(v)) for k,v in binaries.items()}, upstream='a51e02804',
    sources={n:dict(sha256=r.sha(r.ROOT/n),text=(r.ROOT/n).read_text()) for n in (r.OWNER,r.L0)},
    prior_artifacts={str(p.relative_to(r.ROOT)):r.sha(p) for prefix in ('p1315-*','p1316-*','p1317-*') for p in D.glob(prefix) if p.is_file()},
    samples={name:list(data) for name,data in samples}, script_sha256=r.sha(__file__)))
for row in rows:
    print(row['id'], row['product'], (row['stderr'] or row['stdout']).splitlines()[0])
