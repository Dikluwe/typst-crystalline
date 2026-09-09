"""Independent P1327-R2 eval ordering corpus, no runtime source access."""
import argparse
import datetime
import hashlib
import importlib.util
import json
import os
import pathlib
import subprocess

ROOT = pathlib.Path('/repos/Antigravity/typst-crystalline')
PREFIX = ROOT / '00_nucleo/diagnosticos'
spec = importlib.util.spec_from_file_location('original', PREFIX / 'p1327-ab-runner.py')
original = importlib.util.module_from_spec(spec)
spec.loader.exec_module(original)
C1 = '/tmp/p1327-target.k9Mq0s/release/typst'
VANILLA = '/usr/local/bin/typst'
CASES = [
    ('deprecated-error', '{ let x = sym.join; global }', 'json', 'order-correction'),
    ('deprecated-warning-only', '{ let x = sym.join; 7 }', 'json', 'parity'),
    ('mixed-warnings-error', '{ let x = sym.join; import std; global }', 'json', 'order-correction'),
    ('two-deprecated-error', '{ let x = sym.join; let y = sym.join; global }', 'json', 'order-correction'),
    ('error-only', 'global', 'json', 'parity'),
    ('no-warning', '{ let x = 2; x + 5 }', 'json', 'parity'),
    ('serialization-warning', '{ let x = sym.join; 7 }', 'raw', 'preserve-success-path'),
    ('serialization-no-warning', '7', 'raw', 'preserve-success-path'),
]

def run(binary, case, profile):
    argv = [binary, 'eval', '--format', case[2], *original.PROFILES[profile], case[1]]
    env = os.environ.copy()
    env['NO_COLOR'] = '1'
    start = datetime.datetime.now(datetime.timezone.utc).isoformat()
    p = subprocess.run(argv, cwd=original.CWD, env=env, capture_output=True, timeout=30)
    return {'id': case[0], 'expression': case[1], 'format': case[2], 'class': case[3],
            'profile': profile, 'argv': argv, 'cwd': original.CWD, 'started': start,
            'exit': p.returncode, 'stdout': p.stdout.decode(), 'stderr': p.stderr.decode()}

def main():
    p = argparse.ArgumentParser()
    p.add_argument('mode', choices=['measure', 'candidate'])
    p.add_argument('output')
    p.add_argument('--candidate')
    p.add_argument('--oracle')
    args = p.parse_args()
    result = {'runner_sha256': original.sha(__file__), 'cases': CASES,
              'profiles': original.PROFILES, 'normalization': 'none, full exit/stdout/stderr',
              'regime': 'A/B without technical isolation attestation'}
    if args.mode == 'measure':
        assert original.sha(C1) == '5f00502b5055fedf8ca1dc2c879112ac1300e12621e11fdb2c731364450a62f9'
        assert original.sha(VANILLA) == '7b4f40c56d6fa95082ebcfd893e275d418ebcaed1b97b62785f78284c63ff7b8'
        result['binaries'] = {binary: original.sha(binary) for binary in [C1, VANILLA]}
        result['c1'] = [run(C1, c, profile) for profile in original.PROFILES for c in CASES]
        result['vanilla'] = [run(VANILLA, c, profile) for profile in original.PROFILES for c in CASES]
    else:
        oracle = json.loads(pathlib.Path(args.oracle).read_text())
        assert oracle['runner_sha256'] == original.sha(__file__)
        result['oracle_sha256'] = original.sha(args.oracle)
        result['candidate_sha256'] = original.sha(args.candidate)
        expected = {(r['profile'], r['id']): r for r in oracle['expected']}
        sequence = [(c, profile) for profile in original.PROFILES for c in CASES]
        result['runs'] = []
        for order, seq in [('normal', sequence), ('repeat', sequence), ('reverse', sequence[::-1])]:
            for c, profile in seq:
                row = run(args.candidate, c, profile)
                row['order'] = order
                row['verdict'] = 'Preserved' if original.observable(row) == original.observable(expected[profile, c[0]]) else 'Violated'
                result['runs'].append(row)
    original.save(args.output, result)

if __name__ == '__main__':
    main()
