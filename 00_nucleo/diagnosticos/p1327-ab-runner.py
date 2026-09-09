#!/usr/bin/env python3
"""Frozen P1327 A/B oracle. Never reads runtime or candidate source."""
import argparse
import datetime
import hashlib
import json
import os
import pathlib
import subprocess

ROOT = pathlib.Path('/repos/Antigravity/typst-crystalline')
CWD = '/tmp/p1327-ab-fixtures'
PROFILES = {'default': [], 'html': ['--features', 'html'],
            'a11y': ['--features', 'a11y-extras'],
            'html+a11y': ['--features', 'html,a11y-extras']}
BASELINE = '/tmp/p1326-target.6Vi4Km/release/typst'
VANILLA = '/usr/local/bin/typst'
CASES = [
 ('std', '{ import std; std.calc.abs(-7) }', 'correction'),
 ('alias', '{ let renamed = std; import renamed; renamed.calc.abs(-8) }', 'correction'),
 ('calc', '{ import calc; calc.abs(-9) }', 'correction'),
 ('ordinary', '{ import "ordinary.typ" as original; import original; original.x }', 'correction'),
 ('math-name', '{ import "math.typ" as math; import math; math.x }', 'correction'),
 ('shadow', '{ import "ordinary.typ" as original; let std = original; import std; std.x }', 'correction'),
 ('closure', '{ let f() = { import std; std.calc.abs(-11) }; f() }', 'correction'),
 ('same-span-twice', '{ let f() = { import std; std.calc.abs(-11) }; (f(), f()) }', 'correction'),
 ('distinct-spans', '{ import std; import std; std.calc.abs(-12) }', 'correction'),
 ('utf8-offset', '{\n let prefix = "á🦀";\n let módulo = std;\n import módulo; módulo.calc.abs(-13)\n}', 'correction'),
 ('later-error', '{ import std; global }', 'correction'),
 ('alias-later-error', '{ let renamed = std; import renamed; global }', 'correction'),
 ('field', '{ let box = (saved: std); import box.saved; saved.calc.abs(-14) }', 'parity'),
 ('rename', '{ import std as chosen; chosen.calc.abs(-15) }', 'parity'),
 ('redundant-rename', '{ import std as std; std.calc.abs(-16) }', 'preserved-debt'),
 ('items', '{ import std: calc; calc.abs(-17) }', 'parity'),
 ('wildcard', '{ import calc: *; abs(-18) }', 'parity'),
 ('file', '{ import "ordinary.typ"; ordinary.x }', 'parity'),
 ('dynamic-bare', '{ import (std); none }', 'parity'),
 ('dynamic-block', '{ import { std }; none }', 'parity'),
 ('dynamic-call', '{ let f() = std; import f(); none }', 'parity'),
 ('dynamic-rename', '{ import (std) as chosen; chosen.calc.abs(-20) }', 'parity'),
 ('dynamic-items', '{ import (std): calc; calc.abs(-21) }', 'parity'),
 ('dynamic-wildcard', '{ import (calc): *; abs(-22) }', 'parity'),
 ('resolution-error', '{ import absent; none }', 'parity'),
 ('type-error', '{ let specimen = 42; import specimen; none }', 'preserved-debt'),
 ('error-before', '{ global; import std; none }', 'parity'),
 ('imported-route', '{ import "warning.typ" as warned; warned.x }', 'correction'),
]

def sha(path):
    return hashlib.sha256(pathlib.Path(path).read_bytes()).hexdigest()

def save(path, data):
    body = json.dumps(data, ensure_ascii=False, indent=2) + '\n'
    relative = str(pathlib.Path(path).relative_to(ROOT))
    patch = '*** Begin Patch\n*** Add File: ' + relative + '\n'
    patch += ''.join('+' + line + '\n' for line in body.splitlines()) + '*** End Patch\n'
    subprocess.run(['apply_patch'], input=patch, text=True, cwd=ROOT, check=True)

def run(binary, case, profile):
    argv = [binary, 'eval', '--format', 'json', *PROFILES[profile], case[1]]
    started = datetime.datetime.now(datetime.timezone.utc).isoformat()
    env = os.environ.copy()
    env['NO_COLOR'] = '1'
    p = subprocess.run(argv, cwd=CWD, env=env, capture_output=True, timeout=30)
    return {'id': case[0], 'profile': profile, 'class': case[2], 'argv': argv,
            'cwd': CWD, 'started': started, 'exit': p.returncode,
            'stdout': p.stdout.decode(), 'stderr': p.stderr.decode()}

def observable(row):
    return [row['exit'], row['stdout'], row['stderr']]

def main():
    parser = argparse.ArgumentParser()
    parser.add_argument('mode', choices=['freeze', 'candidate'])
    parser.add_argument('output')
    parser.add_argument('--candidate')
    parser.add_argument('--oracle')
    args = parser.parse_args()
    result = {'regime': 'A/B, executed without isolation attestation',
              'runner_sha256': sha(__file__), 'cases': CASES, 'profiles': PROFILES,
              'fixtures': {p.name: {'sha256': sha(p), 'text': p.read_text()}
                           for p in sorted(pathlib.Path(CWD).glob('*.typ'))},
              'unknown_policy': 'Any missing result, invalid fixture or execution blocks acceptance',
              'normalization': 'none: full exit/stdout/stderr equality'}
    if args.mode == 'freeze':
        assert sha(BASELINE) == 'bd86b34602323a390a60a8f41b97b180f93077affcd30b6b1e85aa5926dee811'
        assert sha(VANILLA) == '7b4f40c56d6fa95082ebcfd893e275d418ebcaed1b97b62785f78284c63ff7b8'
        result['binaries'] = {p: sha(p) for p in [BASELINE, VANILLA]}
        result['baseline'] = [run(BASELINE, c, p) for p in PROFILES for c in CASES]
        result['vanilla'] = [run(VANILLA, c, p) for p in PROFILES for c in CASES]
        result['expected'] = [b if b['class'] == 'preserved-debt' else v
                              for b, v in zip(result['baseline'], result['vanilla'])]
        result['precheck'] = [{'id': b['id'], 'profile': b['profile'], 'class': b['class'],
                               'baseline_equals_vanilla': observable(b) == observable(v)}
                              for b, v in zip(result['baseline'], result['vanilla'])]
    else:
        oracle = json.loads(pathlib.Path(args.oracle).read_text())
        assert oracle['runner_sha256'] == sha(__file__)
        result['oracle_sha256'] = sha(args.oracle)
        result['candidate_sha256'] = sha(args.candidate)
        expected = {(r['profile'], r['id']): r for r in oracle['expected']}
        result['runs'] = []
        order = [(c, p) for p in PROFILES for c in CASES]
        for name, sequence in [('normal', order), ('repeat', order), ('reverse', order[::-1])]:
            for c, p in sequence:
                row = run(args.candidate, c, p)
                row['order'] = name
                row['verdict'] = 'Preserved' if observable(row) == observable(expected[p, c[0]]) else 'Violated'
                result['runs'].append(row)
    save(args.output, result)

if __name__ == '__main__':
    main()
