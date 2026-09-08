"""Measure CSV option errors before selecting an implementation contract."""
import importlib.util
import json
from pathlib import Path
import subprocess
import sys
import time

sys.dont_write_bytecode = True
D = Path(__file__).resolve().parent
spec = importlib.util.spec_from_file_location('record', D/'p1314-record.py')
r = importlib.util.module_from_spec(spec); spec.loader.exec_module(r)
cases = [
    'csv(bytes("a,b"), delimiter: "ab")',
    'csv(bytes("a,b"), delimiter: "α")',
    'csv(bytes("a,b"), delimiter: false)',
    'csv(bytes("a,b"), row-type: "dictionary")',
    'csv(bytes("a,b"), row-type: str)',
    'csv(bytes("a,b"), delimiter: sym.alpha)',
    'csv(bytes("a,b"), row-type: sym.alpha)',
    'csv(bytes("a,b"), delimiter: "ab", delimiter: ",")',
    'csv(bytes("a,b"), delimiter: ",", delimiter: "ab")',
    'csv(bytes("a,b"), row-type: "array", row-type: array)',
    'csv(bytes("a,b"), row-type: "array", delimiter: "ab")',
    'csv(42, delimiter: "ab")',
    'csv(bytes("a,b"), unknown: 1, delimiter: "ab")',
    'csv(delimiter: "ab")',
    '{ let f = csv.with(delimiter: "ab"); f(bytes("a,b")) }',
    '{ let f = csv.with(delimiter: "ab"); f(bytes("a,b"), delimiter: ",") }',
    '{ let a = arguments(delimiter: "ab"); csv(bytes("a,b"), ..a) }',
    'csv(bytes("a;b"), delimiter: ",", delimiter: ";")',
    'csv(bytes("a,b"), row-type: array)',
    'csv(bytes("a,b\\n1"))',
    'read(bytes("a,b"))',
]
before = r.state(); rows = []; start = time.monotonic()
for expr in cases:
    for product, binary in [('baseline', r.BASE_BINARY), ('vanilla', '/usr/local/bin/typst')]:
        argv = [binary, 'eval', expr]
        at = r.now()
        p = subprocess.run(argv, cwd=r.ROOT, capture_output=True, text=True, timeout=30)
        rows.append({'expr': expr, 'product': product, 'argv': argv, 'cwd': str(r.ROOT), 'at': at,
            'exit': p.returncode, 'stdout': p.stdout, 'stderr': p.stderr, 'binary_sha256': r.sha(binary)})
sources = ['lab/typst-original/crates/typst-library/src/loading/csv.rs',
           'lab/typst-original/crates/typst-library/src/foundations/args.rs',
           '01_core/src/compiler/stdlib/loading.rs', '01_core/src/entities/args.rs']
r.save('measurement', {'before': before, 'after': r.state(), 'rows': rows, 'seconds': time.monotonic()-start,
    'sources': {n:r.sha(r.ROOT/n) for n in sources}, 'baseline_sha256': r.sha(D/'p1314-baseline.json'),
    'script_sha256': r.sha(__file__)})
for row in rows:
    print(row['product'], row['expr'], row['stdout'] or row['stderr'])
