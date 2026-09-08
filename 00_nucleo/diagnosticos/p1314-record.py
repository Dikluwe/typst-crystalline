"""Bounded P1314 commands with immutable, full provenance receipts."""
import datetime
import hashlib
import json
import os
from pathlib import Path
import subprocess
import sys
import time

sys.dont_write_bytecode = True
ROOT = Path(__file__).resolve().parents[2]
D = ROOT / '00_nucleo/diagnosticos'
TARGET = '/dev/shm/p1314-target.cswujn'
BASE_BINARY = '/dev/shm/p1313-target.keFg93/release/typst'

def sha(path):
    return hashlib.sha256(Path(path).read_bytes()).hexdigest()

def now():
    return datetime.datetime.now(datetime.timezone.utc).isoformat()

def git(*args):
    return subprocess.check_output(['git', *args], cwd=ROOT, text=True)

def state():
    return {'head': git('rev-parse', 'HEAD').strip(), 'at': now(),
            'diff_stat': git('diff', 'HEAD', '--stat'),
            'diff': git('diff', 'HEAD', '--binary'),
            'staged': git('diff', '--cached', '--binary'),
            'status': git('status', '--short', '--untracked-files=all')}

def save(name, data):
    path = D / ('p1314-' + name + '.json')
    assert not path.exists(), path
    payload = json.dumps(data, ensure_ascii=False, indent=2) + '\n'
    patch = '*** Begin Patch\n*** Add File: ' + str(path) + '\n'
    patch += ''.join('+' + line + '\n' for line in payload.splitlines())
    patch += '*** End Patch\n'
    subprocess.run(['apply_patch'], input=patch, cwd=ROOT, text=True,
                   capture_output=True, check=True)
    print(json.dumps({'path': str(path), 'sha256': sha(path)}), flush=True)

def init():
    s = state()
    assert not s['staged']
    assert sha(BASE_BINARY) == 'cefb4b485cc25ae98871cfbd7a76925d0333d6d26906dc7f7426868e899285ce'
    assert sha(ROOT/'01_core/src/compiler/stdlib/loading.rs') == 'a8ef0db2cd9531d6f86aa5c346c3860495a002a8244b4e9594cc9fd6f15b3468'
    assert sha(ROOT/'00_nucleo/prompts/compiler/stdlib/loading.md') == '103a2035a2c4c534e3e905bf168ec50d06dc6c2fc484cfc362a74b22f7bf9c5c'
    old = json.loads((D/'p1313-implementation-baseline.json').read_text())
    files = {p: sha(ROOT/p) for p in old['files']}
    prior = {str(p.relative_to(ROOT)): sha(p) for prefix in ('p1309-*','p1310-*','p1311-*','p1312-*','p1313-*') for p in D.glob(prefix) if p.is_file()}
    save('baseline', {'state': s, 'files': files, 'prior_artifacts': prior,
        'request': 'Escreva o próximo e implemente; não autoriza commit/stage/push.',
        'vanilla': {'path': '/usr/local/bin/typst', 'sha256': sha('/usr/local/bin/typst'), 'upstream': 'a51e02804'},
        'baseline_binary': {'path': BASE_BINARY, 'sha256': sha(BASE_BINARY)},
        'target': TARGET, 'recorder_sha256': sha(__file__),
        'regime': 'A/B; executado sem atestação de isolamento técnico'})

def command(name, argv):
    before = state(); start = time.monotonic()
    env = {'CARGO_TARGET_DIR': TARGET, 'PYTHONDONTWRITEBYTECODE': '1'}
    try:
        p = subprocess.run(argv, cwd=ROOT, env={**os.environ, **env},
                           capture_output=True, text=True, timeout=1200)
        code, out, err = p.returncode, p.stdout, p.stderr
    except subprocess.TimeoutExpired as e:
        code, out, err = 124, str(e.stdout), str(e.stderr)
    data = {'before': before, 'after': state(), 'argv': argv, 'cwd': str(ROOT),
            'env': env, 'seconds': time.monotonic()-start,
            'exit': code, 'stdout': out, 'stderr': err,
            'baseline_sha256': sha(D/'p1314-baseline.json'),
            'recorder_sha256': sha(__file__)}
    for label, path in [('baseline_binary', Path(BASE_BINARY)),
                        ('candidate', Path(TARGET)/'release/typst')]:
        if path.exists(): data[label] = {'path': str(path), 'sha256': sha(path)}
    save(name, data)
    print(out[-1500:]); print(err[-1500:])
    raise SystemExit(code)

if __name__ == '__main__':
    if sys.argv[1] == 'init': init()
    else: command(sys.argv[1], sys.argv[2:])
