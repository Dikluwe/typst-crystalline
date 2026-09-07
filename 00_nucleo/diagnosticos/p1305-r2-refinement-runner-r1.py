#!/usr/bin/env python3
"""Recover P1304's calibrated ratio observation adapter; no product reads/writes."""
import argparse
import base64
import datetime
import hashlib
import json
import pathlib
import subprocess
import time

ROOT = pathlib.Path('/repos/Antigravity/typst-crystalline')
DIAG = ROOT / '00_nucleo/diagnosticos'
VANILLA = '/usr/local/bin/typst'
BASELINE = '/dev/shm/p1305-target.3j4tck2g/release/typst'
VANILLA_SHA = '7b4f40c56d6fa95082ebcfd893e275d418ebcaed1b97b62785f78284c63ff7b8'
BASELINE_SHA = '4a4e1bd46c053dd19bfcae2230537c9478fbfb2ff26a94dfd87d9f29fee2835d'
HISTORICAL_SHA = '6b300e95e456fd36828e68f8b15889af2a8184e514ee5be57ad2c8c1c0b4f6d2'
FAILED_SHA = '6c32031d30aec9aa5cce532038a6b353b9ed99d050386ea3ec456163a5710e37'
PROFILES = {'default': [], 'html': ['html'], 'a11y': ['a11y-extras'], 'html+a11y': ['html', 'a11y-extras']}

def sha(data):
    return hashlib.sha256(data).hexdigest()

def pin(path):
    data = pathlib.Path(path).read_bytes()
    return {'path': str(path), 'sha256': sha(data), 'bytes': len(data)}

def run(binary, expression, profile, phase, side, path):
    argv = [binary, 'eval', expression, '--format', 'json']
    if PROFILES[profile]:
        argv += ['--features', ','.join(PROFILES[profile])]
    started = datetime.datetime.now(datetime.timezone.utc).isoformat()
    clock = time.monotonic_ns()
    try:
        process = subprocess.run(argv, cwd=ROOT, capture_output=True, timeout=90)
        code, stdout, stderr, complete = process.returncode, process.stdout, process.stderr, True
    except subprocess.TimeoutExpired as error:
        code, stdout, stderr, complete = None, error.stdout or b'', error.stderr or b'', False
    row = {'id': path, 'expression': expression, 'side': side, 'profile': profile, 'phase': phase, 'argv': argv, 'cwd': str(ROOT), 'started_at': started, 'duration_ns': time.monotonic_ns() - clock, 'exit_code': code, 'complete': complete}
    for name, raw in [('stdout', stdout), ('stderr', stderr)]:
        row[name] = raw.decode('utf-8', errors='replace')
        row[name + '_base64'] = base64.b64encode(raw).decode()
        row[name + '_sha256'] = sha(raw)
    try:
        row['value'] = json.loads(stdout) if code == 0 else None
        row['json_parsed'] = code == 0
    except ValueError:
        row['value'], row['json_parsed'] = None, False
    return row

def normalize(row, previous, expected_hex_digest):
    value = row['value']
    try:
        if row['exit_code'] != 0 or not row['complete'] or not row['json_parsed'] or row['stderr']:
            raise ValueError('invalid execution or unexpected stderr')
        if value['kind'] != 'array' or value['n'] != len(value['rgba']) or value['n'] != len(value['hex']):
            raise ValueError('incomplete structured array')
        tokens = []
        for rgba, color in zip(value['rgba'], value['hex']):
            raw = color.removeprefix('#').lower()
            if len(rgba) != 4:
                raise ValueError('incomplete component tuple')
            if len(raw) == 6 and rgba[3] == 1.0:
                raw += 'ff'
            if len(raw) != 8 or any(c not in '0123456789abcdef' for c in raw):
                raise ValueError('unknown color token')
            tokens.append('0x' + raw)
        row['digests'] = {'rgba8_sha256': sha(','.join(tokens).encode()), 'full_components_sha256': sha(json.dumps(value['rgba'], separators=(',', ':'), ensure_ascii=False).encode()), 'cardinality': value['n']}
        row['indices'] = {str(i): {'rgba': value['rgba'][i], 'hex': value['hex'][i]} for i in [39, 40, value['n'] - 2, value['n'] - 1] if 0 <= i < value['n']}
        row['historical_data_preserved'] = (row['digests']['full_components_sha256'] == previous['full_components_sha256'] and value['n'] == previous['cardinality'] and row['digests']['rgba8_sha256'] == expected_hex_digest)
    except (TypeError, KeyError, ValueError) as error:
        row['normalization_unknown'] = str(error)
        row['historical_data_preserved'] = False

def main():
    parser = argparse.ArgumentParser()
    parser.add_argument('--crystalline', default=BASELINE)
    parser.add_argument('--crystalline-sha256', default=BASELINE_SHA)
    parser.add_argument('--maps', default='cividis,spectral', help='Comma-separated existing P1304 map names, or all')
    parser.add_argument('--output', default=str(DIAG / 'p1305-r2-repr-refinement-r1.json'))
    args = parser.parse_args()
    output = pathlib.Path(args.output)
    if output.exists():
        raise SystemExit('Refusing to replace existing evidence')
    historical_path = DIAG / 'p1304-repr-refinement.json'
    rejected_path = DIAG / 'p1305-r2-repr-refinement.json'
    if pin(historical_path)['sha256'] != HISTORICAL_SHA or pin(rejected_path)['sha256'] != FAILED_SHA:
        raise SystemExit('Immutable predecessor identity changed')
    historical = json.loads(historical_path.read_text())
    binaries = {'vanilla': VANILLA, 'crystalline': args.crystalline}
    binaries_pins = {side: pin(path) for side, path in binaries.items()}
    if binaries_pins['vanilla']['sha256'] != VANILLA_SHA or binaries_pins['crystalline']['sha256'] != args.crystalline_sha256:
        raise SystemExit('Binary pin mismatch')
    selected = [m for m in historical['map_measurements'] if args.maps == 'all' or m['path'].removeprefix('color.map.') in args.maps.split(',')]
    if len(selected) != (15 if args.maps == 'all' else len(args.maps.split(','))):
        raise SystemExit('Unknown or duplicate requested map')
    records = []
    jobs = [(m, profile) for m in selected for profile in PROFILES]
    for phase, ordered, sides in [('normal', jobs, list(binaries)), ('inverted', list(reversed(jobs)), list(reversed(binaries)))]:
        for m, profile in ordered:
            path = m['path']
            expression = f'(kind: repr(type({path})), n: {path}.len(), rgba: {path}.map(c => c.components().map(x => x / 100%)), hex: {path}.map(c => c.to-hex()), repr: repr({path}))'
            for side in sides:
                row = run(binaries[side], expression, profile, phase, side, path)
                normalize(row, m['profiles'][profile][side], m['l0_expected_rgba8_sha256'])
                records.append(row)
    repetitions = []
    for m, profile in jobs:
        for side in binaries:
            pair = [r for r in records if (r['id'], r['profile'], r['side']) == (m['path'], profile, side)]
            repetitions.append({'id': m['path'], 'profile': profile, 'side': side, 'stable': all(pair[0][k] == pair[1][k] for k in ['exit_code', 'stdout_sha256', 'stderr_sha256', 'complete'])})
    result = {'schema': 'p1305-r2-refinement-adapter-repair-v1', 'scope': 'Auxiliary observation adapter only; sealed productive contract and38oracles unchanged; no candidate source access', 'public_cause': 'Initial auxiliary runner mistakenly reused historical pre-calibration float(ratio), unsupported by crystalline. Canonical P1304 revision already exposes unchanged ratios by division by100%.', 'minimal_change': 'Only expression component adapter float(x) -> x / 100%; full rgba/hex/cardinality/repr channels and P1304 canonical digest policy preserved.', 'historical': pin(historical_path), 'historical_revision_script_sha256': historical['revision_script_sha256'], 'rejected_predecessor': pin(rejected_path), 'rejected_predecessor_disposition': 'All240runs preserved, including120invalid crystalline float(ratio) executions. Never reclassified as product failure or data equality.', 'script': pin(__file__), 'binary_pins': binaries_pins, 'preexisting_baseline_provenance': pin(DIAG / 'p1305-r2-preflight-manifest.json'), 'runs': records, 'repetition': repetitions, 'adapter_observation': {'runs': len(records), 'valid_executions': sum(r['complete'] and r['exit_code'] == 0 and r['json_parsed'] and not r['stderr'] for r in records), 'all_historical_data_preserved': all(r['historical_data_preserved'] for r in records), 'all_repetitions_stable': all(r['stable'] for r in repetitions)}, 'acceptance_limit': 'This artifact records observations only. Independent verifier owns candidate verdict. No full627corpus repetition and no modified sealed artifacts.'}
    with output.open('x') as stream:
        json.dump(result, stream, indent=2, ensure_ascii=False)
        stream.write('\n')
    print(json.dumps({'output': str(output), 'sha256': pin(output)['sha256'], **result['adapter_observation']}), flush=True)

if __name__ == '__main__':
    main()
