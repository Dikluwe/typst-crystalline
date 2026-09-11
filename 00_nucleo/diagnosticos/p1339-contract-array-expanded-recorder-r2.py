#!/usr/bin/env python3
"""Mechanical expansion of frozen Array inputs; never reads candidate source."""
import argparse
import base64
import datetime
import hashlib
import json
import os
from pathlib import Path
import subprocess
import time

ROOT = Path('/repos/Antigravity/typst-crystalline')
D = ROOT / '00_nucleo/diagnosticos'
PREFIX = 'p1339-contract-array-expanded-'
PINS = {
    'p1339-contract-array-inputs-r2.json': '8a0c77118c80d220b58dd8070fe0da2399353c4ae0f46610b245224b389bb3a0',
    'p1339-contract-array-supplement-r2.json': '3f6050dbd7b3373d7972f58e1343860697d2af7c672845cc06033cd5383767fc',
    'p1339-contract-array-focal-raw-r1.json': 'e969bffc9da5f796498fdd5bf635eaea468efac7cb67c4d0f5d04810c4a71196',
    'p1339-contract-array-oracles-focal-r1.json': '9fde62b1664bdda6d83aece3580f9acbebeb2768113a204ebeb8889117b3c533',
}
BINS = {
    'vanilla': ('/usr/local/bin/typst', '7b4f40c56d6fa95082ebcfd893e275d418ebcaed1b97b62785f78284c63ff7b8'),
    'baseline': ('/tmp/p1338-target.vlNAmp/release/typst', 'f7c8085f8453ecb6b10841b698092d41e7fbec64d9d46f9341b9b9b538bfefa1'),
}
PROFILES = {'default': [], 'html': ['--features', 'html'], 'a11y': ['--features', 'a11y-extras'], 'html+a11y': ['--features', 'html,a11y-extras']}
ENV = {**os.environ, 'PYTHONDONTWRITEBYTECODE': '1', 'NO_COLOR': '1'}

def sha(path):
    return hashlib.sha256(Path(path).read_bytes()).hexdigest()

def now():
    return datetime.datetime.now(datetime.timezone.utc).isoformat()

def lossless(value):
    if value is None:
        return ''
    try:
        return value.decode('utf-8')
    except UnicodeDecodeError:
        return {'encoding': 'base64', 'data': base64.b64encode(value).decode('ascii')}

def run(argv):
    start = now()
    try:
        result = subprocess.run(argv, cwd=ROOT, env=ENV, input=b'', capture_output=True, timeout=30)
        return dict(argv=argv, stdin='', stdout=lossless(result.stdout), stderr=lossless(result.stderr), exit=result.returncode, utc_start=start, utc_end=now())
    except subprocess.TimeoutExpired as error:
        return dict(argv=argv, stdin='', stdout=lossless(error.stdout), stderr=lossless(error.stderr), exit=None, timeout_seconds=30, utc_start=start, utc_end=now())

def git_state():
    return {key: run(argv) for key, argv in {
        'head': ['git', 'rev-parse', 'HEAD'],
        'status': ['git', 'status', '--short'],
        'diffstat': ['git', 'diff', 'HEAD', '--stat'],
    }.items()}

def save(suffix, data):
    path = D / (PREFIX + suffix)
    assert not path.exists(), path
    content = json.dumps(data, ensure_ascii=False, indent=2) + '\n'
    patch = '*** Begin Patch\n*** Add File: ' + str(path) + '\n' + ''.join('+' + line + '\n' for line in content.splitlines()) + '*** End Patch\n'
    subprocess.run(['apply_patch'], cwd=ROOT, input=patch, text=True, capture_output=True, check=True)
    return {'path': str(path), 'sha256': sha(path)}

def main():
    parser = argparse.ArgumentParser()
    parser.add_argument('--manifest-sha256', required=True)
    parser.add_argument('--go-file', required=True)
    parser.add_argument('--go-sha256', required=True)
    parser.add_argument('--private-green-file', required=True)
    parser.add_argument('--private-green-sha256', required=True)
    args = parser.parse_args()
    manifest_path = D / (PREFIX + 'manifest-r2.json')
    assert sha(manifest_path) == args.manifest_sha256
    manifest = json.loads(manifest_path.read_text())
    assert sha(__file__) == manifest['recorder_sha256']
    assert manifest['expected_cli_processes'] == 672
    dependencies = {str(D / name): pin for name, pin in PINS.items()}
    dependencies[str(manifest_path)] = args.manifest_sha256
    dependencies[str(Path(__file__).resolve())] = manifest['recorder_sha256']
    for filename, pin in [(args.go_file, args.go_sha256), (args.private_green_file, args.private_green_sha256)]:
        path = Path(filename).resolve()
        assert path.parent == D and path.name.startswith('p1339-') and path.suffix == '.json'
        assert len(pin) == 64 and sha(path) == pin
        dependencies[str(path)] = pin
    for binary, pin in BINS.values():
        dependencies[binary] = pin
    assert all(sha(path) == pin for path, pin in dependencies.items())
    for suffix in ['raw-r1.json', 'oracles-r1.json', 'receipt-r1.json']:
        assert not (D / (PREFIX + suffix)).exists()
    inputs = json.loads((D / 'p1339-contract-array-inputs-r2.json').read_text())
    cases = inputs['cases']
    assert len(cases) == 28 and len({case['id'] for case in cases}) == 28
    raw = {'schema': 'p1339-array-expanded-raw-v1', 'utc_start': now(), 'author': '/root/p1312_tests', 'authority': 'Same combined contract/oracle author, mechanical reference expansion only; shared filesystem and inherited context, no technical isolation attestation; no candidate reads or executions.', 'manifest_sha256': args.manifest_sha256, 'dependencies_before': dependencies, 'effective_relevant_environment': {key: value for key, value in ENV.items() if key in ['PATH', 'LANG', 'LC_ALL', 'LC_CTYPE', 'TZ', 'HOME', 'NO_COLOR', 'PYTHONDONTWRITEBYTECODE', 'SOURCE_DATE_EPOCH'] or key.startswith(('TYPST_', 'FONTCONFIG_', 'XDG_'))}, 'environment_overrides': {'PYTHONDONTWRITEBYTECODE': '1', 'NO_COLOR': '1'}, 'git_before': git_state(), 'runs': []}
    started = time.monotonic()
    for label, (binary, _) in BINS.items():
        for order in ['normal', 'repeat', 'reverse']:
            ordered = list(reversed(cases)) if order == 'reverse' else cases
            for profile, flags in PROFILES.items():
                for case in ordered:
                    argv = [binary, 'eval', *flags, case['source']]
                    if time.monotonic() - started >= 600:
                        result = dict(argv=argv, stdin='', stdout='', stderr='', exit=None, not_executed_reason='600-second envelope exhausted', utc_start=now(), utc_end=now())
                    else:
                        result = run(argv)
                    result.update(id=case['id'], source=case['source'], source_sha256=hashlib.sha256(case['source'].encode()).hexdigest(), binary=label, profile=profile, order=order, policy=case['policy'])
                    raw['runs'].append(result)
    raw['elapsed_seconds'] = time.monotonic() - started
    raw['utc_end'] = now()
    raw['git_after'] = git_state()
    raw['dependencies_after'] = {path: sha(path) for path in dependencies}
    raw['integrity_ok'] = raw['dependencies_after'] == dependencies
    raw_ref = save('raw-r1.json', raw)
    lookup = {(row['id'], row['binary'], row['profile'], row['order']): row for row in raw['runs']}
    channels = lambda row: {key: row[key] for key in ['exit', 'stdout', 'stderr']}
    oracles = []
    for order in ['normal', 'repeat', 'reverse']:
        for profile in PROFILES:
            for case in cases:
                v = lookup[case['id'], 'vanilla', profile, order]
                b = lookup[case['id'], 'baseline', profile, order]
                selected = v if case['policy'] == 'vanilla' else b
                reason = None
                if case['policy'] == 'normative_l0':
                    reason = 'E08 preserves occurrence.span; original claimed detached obligation is not constructed. Historical Unknown retained, no guessed expected.'
                elif not raw['integrity_ok']:
                    reason = 'Pinned dependency changed.'
                elif any(row['exit'] is None for row in [v, b]):
                    reason = 'Timeout or envelope exhaustion.'
                elif any(row['exit'] not in (0, 1) for row in [v, b]):
                    reason = 'UnknownTransport: exit outside accepted {0,1}; raw exit and channels retained.'
                elif any(not isinstance(row[key], str) for row in [v, b] for key in ['stdout', 'stderr']):
                    reason = 'Non-UTF8 output preserved losslessly but not a validated public fixture.'
                elif case['role'] == 'positive' and not (v['exit'] == 0 and b['exit'] == 1 and b['stderr'].startswith('error: type array does not have a constructor\n')):
                    reason = 'Positive fixture did not reach expected direct prerequisite RED.'
                elif any('unexpected argument \'--features\'' in row['stderr'] or 'unrecognized option' in row['stderr'] for row in [v, b]):
                    reason = 'CLI transport rejected feature arguments.'
                oracles.append({**case, 'profile': profile, 'order': order, 'source_sha256': v['source_sha256'], 'status': 'Unknown' if reason else 'FrozenLiteralCell', 'reason': reason, 'expected': None if reason else channels(selected), 'vanilla_observed': channels(v), 'baseline_observed': channels(b)})
    oracle_ref = save('oracles-r1.json', {'schema': 'p1339-array-expanded-literal-oracles-v1', 'manifest_sha256': args.manifest_sha256, 'raw': raw_ref, 'inputs_sha256': PINS['p1339-contract-array-inputs-r2.json'], 'policy': 'Choose actual observed vanilla/baseline channels per frozen case policy, separately for each profile/order. E08 normative policy remains Unknown. No default extrapolation or candidate observation.', 'cases': oracles})
    disagreement = []
    for case in cases:
        for profile in PROFILES:
            for binary in BINS:
                normal = channels(lookup[case['id'], binary, profile, 'normal'])
                for order in ['repeat', 'reverse']:
                    if channels(lookup[case['id'], binary, profile, order]) != normal:
                        disagreement.append([case['id'], binary, profile, order])
    default_old = json.loads((D / 'p1339-contract-array-oracles-focal-r1.json').read_text())
    old_lookup = {row['id']: row for row in default_old['cases']}
    default_disagreement = [row['id'] for row in oracles if row['profile'] == 'default' and row['order'] == 'normal' and any(row[key] != old_lookup[row['id']][key] for key in ['vanilla_observed', 'baseline_observed'])]
    receipt_ref = save('receipt-r1.json', {'schema': 'p1339-array-expanded-receipt-v1', 'utc': now(), 'manifest_sha256': args.manifest_sha256, 'raw': raw_ref, 'oracles': oracle_ref, 'status': 'Recorded expansion; no successor seal or candidate verdict', 'scheduled_cells': len(raw['runs']), 'executed_cli_processes': sum('not_executed_reason' not in row for row in raw['runs']), 'timeout_cells': sum('timeout_seconds' in row for row in raw['runs']), 'not_executed_cells': sum('not_executed_reason' in row for row in raw['runs']), 'literal_oracle_cells': sum(row['status'] == 'FrozenLiteralCell' for row in oracles), 'unknown_oracle_cells': sum(row['status'] == 'Unknown' for row in oracles), 'repeat_reverse_disagreements': disagreement, 'prior_focal_default_disagreements': default_disagreement, 'integrity_ok': raw['integrity_ok'], 'elapsed_seconds': raw['elapsed_seconds'], 'budget': 'Same prospectively declared 672-process expansion; no new contract revision, no focal reset. Private focal costs belong to adversary/verifier receipts. Original70/840 and all other W/F remain due; E08 historical Unknown persists even if distinct owner harness proves fallback.'})
    print(json.dumps({'raw': raw_ref, 'oracles': oracle_ref, 'receipt': receipt_ref, 'processes': len(raw['runs']), 'literal': sum(row['status'] == 'FrozenLiteralCell' for row in oracles), 'unknown': sum(row['status'] == 'Unknown' for row in oracles), 'order_disagreements': len(disagreement), 'default_disagreements': len(default_disagreement)}))

if __name__ == '__main__':
    main()
