"""Read-only product probes for the proposed CSV DataSource contract."""
import datetime
import hashlib
import json
from pathlib import Path
import subprocess
import sys

sys.dont_write_bytecode = True
ROOT = Path(__file__).resolve().parents[2]
D = ROOT/'00_nucleo/diagnosticos'
BASE = '/dev/shm/p1312-target.B8uLa9/release/typst'
VANILLA = '/usr/local/bin/typst'
def sha(path):
    return hashlib.sha256(Path(path).read_bytes()).hexdigest()
def now():
    return datetime.datetime.now(datetime.timezone.utc).isoformat()
def state():
    def git(*args):
        return subprocess.check_output(['git', *args], cwd=ROOT, text=True)
    return {'at': now(), 'head': git('rev-parse', 'HEAD').strip(),
            'diff_stat': git('diff', 'HEAD', '--stat'), 'diff': git('diff', 'HEAD', '--binary'),
            'status': git('status', '--short', '--untracked-files=all'),
            'staged': git('diff', '--cached', '--binary')}
def save(name, data):
    path = D/('p1313-'+name+'.json')
    assert not path.exists(), path
    payload = json.dumps(data, ensure_ascii=False, indent=2)+'\n'
    subprocess.run(['apply_patch'], input='*** Begin Patch\n*** Add File: '+str(path)+'\n'+
                   ''.join('+'+line+'\n' for line in payload.splitlines())+'*** End Patch\n',
                   cwd=ROOT, text=True, capture_output=True, check=True)
    print(path, sha(path), flush=True)
def baseline():
    previous = json.loads((D/'p1312-verification.json').read_text())
    assert sha(BASE) == previous['candidate']['sha256']
    for path, key in [('01_core/src/compiler/stdlib/loading.rs', 'source_sha256'),
                      ('00_nucleo/prompts/compiler/stdlib/loading.md', 'L0_sha256')]:
        assert sha(ROOT/path) == previous['candidate'][key]
    prior = json.loads((D/'p1312-baseline.json').read_text())
    s = state(); assert not s['staged']
    save('baseline', {'state': s, 'files': {p: sha(ROOT/p) for p in prior['files']},
        'prior_artifacts': {str(p.relative_to(ROOT)): sha(p) for prefix in ('p1309-*', 'p1310-*', 'p1311-*', 'p1312-*') for p in D.glob(prefix) if p.is_file()},
        'baseline_binary': {'path': BASE, 'sha256': sha(BASE)},
        'vanilla': {'path': VANILLA, 'sha256': sha(VANILLA), 'upstream': 'a51e02804'},
        'script_sha256': sha(__file__)})
def measure():
    expressions = [
        'csv(42)', 'csv(bytes("a,b\\n1,2"))', 'csv(bytes(""))',
        'csv(bytes("a;b\\n1;2"), delimiter: ";", row-type: dictionary)',
        'csv(bytes("a,b\\n1"))', 'csv(bytes((255,)))',
        'csv(bytes("a,b"), delimiter: "::")', 'csv(42, delimiter: "::")',
        'csv(sym.alpha)', 'csv()', 'csv(42, unknown: true)',
        '{ let f = csv.with(bytes("a,b")); f() }',
        'read(bytes("a,b"))', 'csv.encode',
    ]
    before = state(); rows = []
    for expression in expressions:
        for binary in (BASE, VANILLA):
            argv = [binary, 'eval', expression, '--format', 'json']
            at = now()
            p = subprocess.run(argv, cwd=ROOT, text=True, capture_output=True, timeout=30)
            rows.append({'source': expression, 'argv': argv, 'cwd': str(ROOT), 'at': at,
                         'exit': p.returncode, 'stdout': p.stdout, 'stderr': p.stderr,
                         'binary_sha256': sha(binary)})
    save('measurement', {'before': before, 'after': state(), 'rows': rows,
        'baseline_sha256': sha(D/'p1313-baseline.json'), 'script_sha256': sha(__file__)})
    for row in rows:
        print(row['source'], row['argv'][0], row['stdout'] or row['stderr'])
if __name__ == '__main__':
    {'baseline': baseline, 'measure': measure}[sys.argv[1]]()
