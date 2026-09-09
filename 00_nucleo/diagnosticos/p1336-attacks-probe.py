"""Frozen pre-C sources and independent vanilla/baseline observations."""
import datetime
import hashlib
import json
from pathlib import Path
import subprocess
import time

ROOT = Path('/repos/Antigravity/typst-crystalline')
MANIFEST = ROOT / '00_nucleo/diagnosticos/p1336-manifest.json'
CASES = [
    ('i-direct', '(37).unavailable', 'vanilla'),
    ('i-alias', '{ let inteiro = -57;\n (\n inteiro\n ).ausência }', 'vanilla'),
    ('s-direct', '"ábç".unavailable', 'vanilla'),
    ('s-alias', '{ let cadeia = "";\n (\n cadeia\n ).ausência }', 'vanilla'),
    ('bool-boundary', 'true.ausência', 'baseline'),
    ('int-namespace', 'int.min', 'baseline'),
    ('str-namespace', 'str.len("ábç")', 'baseline'),
    ('int-method', '(37).signum()', 'baseline'),
    ('str-method', '"ábç".len()', 'baseline'),
    ('int-type-absent', 'int.ausência', 'baseline'),
    ('str-type-absent', 'str.ausência', 'baseline'),
]

def sha(path):
    return hashlib.sha256(Path(path).read_bytes()).hexdigest()

def utc():
    return datetime.datetime.now(datetime.timezone.utc).isoformat()

if __name__ == '__main__':
    m = json.loads(MANIFEST.read_text())
    assert sha(MANIFEST) == '54864bfd67d50197560011d5aae055a0303fcc9cadc4230221dfd4f7a6daea88'
    result = {'at_start': utc(), 'manifest_sha256': sha(MANIFEST),
              'cases': CASES, 'commands': [], 'observations': []}
    for name, argv in [('head', ['git', 'rev-parse', 'HEAD']),
                       ('diff_stat', ['git', 'diff', 'HEAD', '--stat']),
                       ('status', ['git', 'status', '--short', '--untracked-files=all'])]:
        p = subprocess.run(argv, cwd=ROOT, capture_output=True, text=True)
        result[name] = p.stdout
        result['commands'].append({'argv': argv, 'exit': p.returncode, 'stdout': p.stdout, 'stderr': p.stderr})
    for role, identity in [('baseline', m['baseline_binary']), ('vanilla', m['vanilla'])]:
        assert sha(identity['path']) == identity['sha256']
        for case, source, expected_role in CASES:
            argv = [identity['path'], 'eval', source]
            started = utc()
            t = time.monotonic()
            p = subprocess.run(argv, cwd=ROOT, capture_output=True, text=True, timeout=30)
            result['observations'].append({'role': role, 'binary_sha256': identity['sha256'],
                'case': case, 'source': source, 'expected_role': expected_role, 'argv': argv,
                'at_start': started, 'at_end': utc(), 'seconds': time.monotonic()-t,
                'exit': p.returncode, 'stdout': p.stdout, 'stderr': p.stderr})
    result['at_end'] = utc()
    out = ROOT / '00_nucleo/diagnosticos/p1336-attacks-oracles.json'
    out.write_text(json.dumps(result, ensure_ascii=False, indent=2) + '\n')
    print(json.dumps({'path': str(out), 'sha256': sha(out), 'observations': len(result['observations'])}))
