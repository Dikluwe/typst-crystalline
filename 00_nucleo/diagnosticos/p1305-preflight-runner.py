#!/usr/bin/env python3
"""Reproduce named-module collision before any P1305 product edit."""
import argparse
import base64
import datetime
import hashlib
import json
import pathlib
import subprocess
import tempfile
import time

ROOT = pathlib.Path(__file__).resolve().parents[2]
DIAG = ROOT / '00_nucleo/diagnosticos'
FIXTURES = {
    'ordinary/std.typ': '#let x = 7\n',
    'ordinary/global.typ': '#let x = 8\n',
    'ordinary/map.typ': '#let x = 9\n',
    'reexport/std.typ': '#import std: *\n',
    'main.typ': '''#let global-alias = std
#import "ordinary/std.typ" as ordinary
#import "ordinary/global.typ" as named-global
#import "ordinary/map.typ" as named-map
#import "reexport/std.typ" as reexport
#let reexport-alias = reexport
#metadata((
  global: repr(std),
  global-alias: repr(global-alias),
  ordinary: repr(ordinary),
  named-global: repr(named-global),
  named-map: repr(named-map),
  reexport: repr(reexport),
  reexport-alias: repr(reexport-alias),
  kinds: (repr(type(std)), repr(type(ordinary)), repr(type(reexport))),
  values: (ordinary.x, named-global.x, named-map.x),
  builtin: (std.calc.abs(-7), reexport.calc.abs(-7)),
  color-map: repr(color.map),
)) <p1305-preflight>
''',
}
PROFILES = {'default': [], 'html': ['html'], 'a11y': ['a11y-extras'], 'html+a11y': ['html', 'a11y-extras']}


def pin(path):
    p = pathlib.Path(path)
    raw = p.read_bytes()
    return {'path': str(path), 'sha256': hashlib.sha256(raw).hexdigest(), 'bytes': len(raw)}


def run(argv, side, profile, order):
    start = datetime.datetime.now(datetime.timezone.utc).isoformat()
    clock = time.monotonic_ns()
    p = subprocess.run(argv, cwd=ROOT, capture_output=True, timeout=90)
    row = {'side': side, 'profile': profile, 'order': order, 'argv': argv, 'cwd': str(ROOT), 'started_at': start, 'duration_ns': time.monotonic_ns()-clock, 'exit_code': p.returncode}
    for name, raw in [('stdout', p.stdout), ('stderr', p.stderr)]:
        row[name] = raw.decode('utf-8', errors='replace')
        row[name+'_base64'] = base64.b64encode(raw).decode()
        row[name+'_sha256'] = hashlib.sha256(raw).hexdigest()
    try:
        row['value'] = json.loads(p.stdout) if p.returncode == 0 else None
    except ValueError:
        row['value'] = None
    return row


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument('--crystalline', required=True)
    args = parser.parse_args()
    previous_path = DIAG/'p1305-pre-measurement.json'
    previous = json.loads(previous_path.read_text()) if previous_path.exists() else None
    tmp = pathlib.Path(tempfile.mkdtemp(prefix='p1305-modules-', dir='/tmp'))
    for relative, text in FIXTURES.items():
        p = tmp / relative
        p.parent.mkdir(parents=True, exist_ok=True)
        p.write_text(text)
    binaries = {'vanilla': '/usr/local/bin/typst', 'crystalline': args.crystalline}
    records = []
    for order, profiles, sides in [('normal', list(PROFILES), list(binaries)), ('inverted', list(reversed(PROFILES)), list(reversed(binaries)))]:
        for profile in profiles:
            for side in sides:
                argv = [binaries[side], 'query', str(tmp/'main.typ'), '<p1305-preflight>', '--field', 'value', '--one']
                if PROFILES[profile]:
                    argv += ['--features', ','.join(PROFILES[profile])]
                records.append(run(argv, side, profile, order))
    repeats = []
    for profile in PROFILES:
        for side in binaries:
            pair = [r for r in records if r['profile'] == profile and r['side'] == side]
            repeats.append({'profile': profile, 'side': side, 'same_exit_and_raw_channels': all(pair[0][k] == pair[1][k] for k in ['exit_code', 'stdout_sha256', 'stderr_sha256'])})
    output = {'schema': 'p1305-module-preflight-v1', 'manifest': pin(DIAG/'p1305-manifest.json'), 'runner': pin(__file__), 'baseline': json.loads((DIAG/'p1305-manifest.json').read_text())['baseline'], 'binary_pins': {s:pin(p) for s,p in binaries.items()}, 'fixtures': [{'relative_path': p, 'content': t, **pin(tmp/p)} for p,t in FIXTURES.items()], 'profiles': PROFILES, 'runs': records, 'repetition': repeats, 'claim_limit': 'Named module public values only; query deprecation warnings retained, not classified as new parity failures; source audit establishes carrier structure separately.'}
    if previous is not None:
        output['calibration_previous_attempt'] = previous
        output['revision_reason'] = 'Focal fixture correction only: str(type) unsupported in crystalline; repr(type) observes kind without that cast. Previous outputs and fixtures preserved, not counted as successful measurement.'
    (DIAG/'p1305-pre-measurement.json').write_text(json.dumps(output, indent=2, ensure_ascii=False)+'\n')
    print(json.dumps({'runs': len(records), 'successful': sum(r['exit_code']==0 and r['value'] is not None for r in records), 'stable': all(r['same_exit_and_raw_channels'] for r in repeats), 'values_default': {r['side']:r['value'] for r in records if r['order']=='normal' and r['profile']=='default'}}, ensure_ascii=False))


if __name__ == '__main__':
    main()
