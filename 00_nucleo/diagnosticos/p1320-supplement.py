"""Focal correction/control only: no replay of the already measured corpus."""
import importlib.util
from pathlib import Path
import sys
import tempfile

spec = importlib.util.spec_from_file_location('audit', Path(__file__).with_name('p1320-audit.py'))
a = importlib.util.module_from_spec(spec)
spec.loader.exec_module(a)
out = Path(sys.argv[1])
if out.exists():
    raise FileExistsError(out)
before = a.snapshot()
tmp = Path(tempfile.mkdtemp(prefix='p1320-supplement-', dir='/tmp'))
rows = []
for profile, features in [('default', []), ('html', ['html']),
                          ('a11y-extras', ['a11y-extras']), ('html+a11y', ['html', 'a11y-extras'])]:
    runs = {}
    for side, (binary, expected) in a.BINARIES.items():
        assert a.sha(binary) == expected
        argv = [binary, '--color=never', 'eval',
                'json(bytes(json.encode((a: (1, true, none)))))', '--format', 'json']
        if features:
            argv += ['--features', ','.join(features)]
        runs[side] = a.invoke(argv, a.FIX)
    rows.append(dict(id='json-roundtrip-corrected', profile=profile, **runs,
                     state=a.classify(runs['vanilla'], runs['crystalline'], 'value')))
for name in ['captured-only', 'closure-only']:
    runs = {}
    for side, (binary, _) in a.BINARIES.items():
        runs[side] = a.invoke([binary, '--color=never', 'compile', str(a.FIX / (name + '.typ')),
                              str(tmp / (name + '-' + side + '.svg')), '--format', 'svg'], a.FIX)
    rows.append(dict(id=name, profile='default', **runs,
                     state=a.classify(runs['vanilla'], runs['crystalline'], 'compile_assertions')))
after = a.snapshot()
stable = before['inputs'] == after['inputs'] and before['binaries'] == after['binaries']
a.save(out, dict(schema='p1320-focal-supplement/v1', before=before, after=after,
    unchanged=stable, script_sha256=a.sha(__file__), rows=rows,
    artifacts={str(p): a.sha(p) for p in tmp.rglob('*') if p.is_file()},
    reason='json.encode returns a string: wrap bytes for decoding, not a filename; split Path and closure controls',
    previous_receipt_sha256=a.sha(a.D / 'p1320-focal-r1.json')))
for row in rows:
    print(row['id'], row['profile'], row['state'])
sys.exit(0 if stable else 2)
