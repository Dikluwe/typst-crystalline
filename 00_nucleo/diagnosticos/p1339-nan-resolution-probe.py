"""Bounded source-informed investigation; no edits to product, policy, or old receipts."""
import datetime
import hashlib
import json
import os
from pathlib import Path
import subprocess
import time

ROOT = Path(__file__).resolve().parents[2]
D = ROOT / '00_nucleo/diagnosticos'

def now():
    return datetime.datetime.now(datetime.timezone.utc).isoformat()

def sha(path):
    with Path(path).open('rb') as stream:
        return hashlib.file_digest(stream, 'sha256').hexdigest()

def git(*args):
    return subprocess.check_output(['git', *args], cwd=ROOT, text=True)

def save(name, data):
    path = D / ('p1339-nan-resolution-' + name + '.json')
    assert not path.exists(), path
    body = json.dumps(data, ensure_ascii=False, indent=2) + '\n'
    patch = '*** Begin Patch\n*** Add File: ' + str(path) + '\n' + ''.join('+' + l + '\n' for l in body.splitlines()) + '*** End Patch\n'
    subprocess.run(['apply_patch'], input=patch, cwd=ROOT, text=True, capture_output=True, check=True)
    return sha(path)

def provenance():
    return dict(at=now(), head=git('rev-parse', 'HEAD').strip(), status=git('status', '--short'), diff_stat=git('diff', 'HEAD', '--stat'))

expressions = [
    ('multiply-nan', 'float("nan") * 1rad'),
    ('multiply-zero-infinity', '0deg * float("inf")'),
    ('subtract-infinities', 'float("inf") * 1deg - float("inf") * 1deg'),
    ('add-opposite-infinities', 'float("inf") * 1deg + -float("inf") * 1deg'),
    ('divide-by-nan', '1deg / float("nan")'),
    ('divide-infinities', '(float("inf") * 1deg) / float("inf")'),
    ('atan-nan', 'calc.atan(float("nan"))'),
    ('atan2-nan-y', 'calc.atan2(float("nan"), 1.0)'),
    ('atan2-nan-x', 'calc.atan2(1.0, float("nan"))'),
    ('asin-nan', 'calc.asin(float("nan"))'),
    ('acos-nan', 'calc.acos(float("nan"))'),
    ('control-positive-infinity', 'float("inf") * 1rad'),
    ('control-negative-infinity', '-float("inf") * 1rad'),
    ('control-negative-zero', '-0deg'),
    ('control-finite', '90deg'),
]
cases = [dict(id=name, expression='{ let a = ' + expr + '; repr((type(a), a, a == a, a / 1deg, (a / 1deg) == (a / 1deg))) }') for name, expr in expressions]
cases.append(dict(id='control-float-nan', expression='{ let n = float("nan"); repr((type(n), n, n == n)) }'))
profiles = {'default': [], 'html': ['--features', 'html'], 'a11y': ['--features', 'a11y-extras'], 'html+a11y': ['--features', 'html,a11y-extras']}
sources = [
    'lab/typst-original/crates/typst-utils/src/scalar.rs',
    'lab/typst-original/crates/typst-library/src/layout/angle.rs',
    'lab/typst-original/crates/typst-library/src/foundations/calc.rs',
    '00_nucleo/materialization/typst-passo-1339.md',
    '00_nucleo/diagnosticos/p1339-retification-check.json',
]
a0 = json.loads((D / 'p1339-a0.json').read_text())
assert all(sha(ROOT / p) == h for p, h in a0['product_inventory'].items())
manifest_sha = save('manifest', dict(
    provenance=provenance(), cases=cases, profiles=profiles,
    source_hashes={p: sha(ROOT / p) for p in sources}, script_sha256=sha(__file__),
    purpose='Test additional public Angle producers and distinguish unreachable domain from a mere missed fixture; not an oracle or seal',
    authority='User: Tente resolver o problema; no presumed permission to weaken gates',
    budget='One focal corpus, normal/reverse, four profiles; no complete P1339 rerun',
))
for binary_name, binary in [('vanilla', '/usr/local/bin/typst'), ('crystalline', '/tmp/p1338-target.vlNAmp/release/typst')]:
    initial = provenance()
    binary_sha = sha(binary)
    rows = []
    for order, sequence in [('normal', cases), ('reverse', list(reversed(cases)))]:
        for case in sequence:
            for profile, flags in profiles.items():
                argv = [binary, 'eval', case['expression'], *flags]
                start = now()
                tick = time.monotonic()
                try:
                    result = subprocess.run(argv, cwd=ROOT, env={**os.environ, 'LC_ALL': 'C', 'TZ': 'UTC'}, text=True, capture_output=True, timeout=20)
                    output = dict(exit=result.returncode, stdout=result.stdout, stderr=result.stderr, execution='completed')
                except subprocess.TimeoutExpired as error:
                    output = dict(exit=None, stdout=str(error.stdout), stderr=str(error.stderr), execution='timeout')
                rows.append(dict(id=case['id'], order=order, profile=profile, argv=argv, cwd=str(ROOT), start=start, end=now(), seconds=time.monotonic() - tick, **output))
    assert sha(binary) == binary_sha
    receipt_sha = save(binary, dict(provenance=initial, end_provenance=provenance(), binary=dict(path=binary, sha256=binary_sha), manifest_sha256=manifest_sha, rows=rows))
    print(json.dumps(dict(binary=binary_name, runs=len(rows), receipt_sha256=receipt_sha)), flush=True)
assert all(sha(ROOT / p) == h for p, h in a0['product_inventory'].items())
