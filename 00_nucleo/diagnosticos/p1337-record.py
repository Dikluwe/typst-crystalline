"""P1337 provenance; immutable receipts and protection of the inherited dirty tree."""
import base64, datetime, hashlib, importlib.util, json, os, subprocess, sys, time
from pathlib import Path
sys.dont_write_bytecode = True
ROOT = Path(__file__).resolve().parents[2]
D = ROOT / '00_nucleo/diagnosticos'
L0 = '00_nucleo/prompts/compiler/eval/bindings/field_access.md'
SOURCE = '01_core/src/compiler/eval/bindings/field_access.rs'
TARGET = '/tmp/p1337-target.F7jPNj'
def load(name):
    spec = importlib.util.spec_from_file_location(name, D / (name + '.py'))
    module = importlib.util.module_from_spec(spec); spec.loader.exec_module(module)
    return module
old = load('p1336-record')
state, sha, git, now = old.state, old.sha, old.git, old.now
def read(name): return json.loads((D / ('p1337-' + name + '.json')).read_text())
def save(name, data):
    path = D / ('p1337-' + name + '.json'); assert not path.exists(), path
    body = json.dumps(data, ensure_ascii=True, indent=2) + '\n'
    patch = '*** Begin Patch\n*** Add File: ' + str(path) + '\n' + ''.join('+' + line + '\n' for line in body.splitlines()) + '*** End Patch\n'
    subprocess.run(['apply_patch'], input=patch, text=True, capture_output=True, check=True)
    print(path, sha(path), flush=True)
def verify(current, strict=False):
    base = read('baseline')['state']
    assert all(current[k] == base[k] for k in ['head', 'branch', 'staged'])
    changed = {p for p in set(base['product_inventory']) | set(current['product_inventory']) if base['product_inventory'].get(p) != current['product_inventory'].get(p)}
    assert changed <= (set() if strict else {L0, SOURCE}), changed
def init():
    current = state(); previous = json.loads((D / 'p1336-closure.json').read_text())
    assert all(current[k] == previous['state'][k] for k in ['head', 'branch', 'diff', 'staged', 'product_inventory'])
    history = {str(p): sha(p) for p in D.rglob('*') if p.is_file() and not p.relative_to(D).parts[0].startswith('p1337-') and '__pycache__' not in p.parts}
    retained = {**previous['retained_mutant_artifacts'], **previous['predecessor_active_exports_preserved']}
    for path, digest in {**previous['historical_preserved'], **previous['artifacts'], **retained}.items():
        assert sha(path) == digest, path
    save('baseline', dict(at=now(), state=current, historical_preserved=history, retained_prior_artifacts=retained,
        previous_closure_sha256=sha(D / 'p1336-closure.json'), target=TARGET,
        snapshots={p: (ROOT / p).read_text() for p in [L0, SOURCE]},
        predecessor=dict(path='/tmp/p1336-target.QiOMGq/release/typst', sha256=sha('/tmp/p1336-target.QiOMGq/release/typst')),
        vanilla=dict(path='/usr/local/bin/typst', sha256=sha('/usr/local/bin/typst'), upstream='a51e02804')))
def measure():
    before = state(); verify(before, True); base = read('baseline'); rows = []
    for expr in ['true.nope', 'false.ausência', 'none.nope', 'auto.nope', '{let a=none;\n a.campo-longo}', '{let a=auto;\n a.ausência}', '(1,2).nope', 'bool.nope', '(1).nope', '"abc".len', 'true.nope(panic("arg"))', '(not false, none == none, auto == auto)']:
        row = dict(source=expr, source_sha256=hashlib.sha256(expr.encode()).hexdigest(), observations={})
        for side, binary in [('baseline', base['predecessor']), ('vanilla', base['vanilla'])]:
            argv = [binary['path'], '--color=never', 'eval', expr, '--format', 'json']; at = now()
            p = subprocess.run(argv, cwd=ROOT, capture_output=True, timeout=30)
            row['observations'][side] = dict(at=at, end=now(), argv=argv, cwd=str(ROOT), binary_sha256=sha(binary['path']), exit=p.returncode,
                stdout=p.stdout.decode(), stderr=p.stderr.decode(), stdout_base64=base64.b64encode(p.stdout).decode(), stderr_base64=base64.b64encode(p.stderr).decode())
        rows.append(row)
    after = state(); verify(after, True)
    save('measurement', dict(at=now(), baseline_sha256=sha(D / 'p1337-baseline.json'), before=before, after=after, rows=rows))
    for row in rows:
        print(row['source'], {k: v['stderr'].splitlines()[:2] or v['stdout'].strip() for k, v in row['observations'].items()})
def command(name, argv):
    before = state(); verify(before); at = now(); start = time.monotonic()
    env = {**os.environ, 'CARGO_TARGET_DIR': TARGET, 'PYTHONDONTWRITEBYTECODE': '1'}
    try:
        p = subprocess.run(argv, cwd=ROOT, env=env, capture_output=True, text=True, timeout=2700)
        code, out, err = p.returncode, p.stdout, p.stderr
    except subprocess.TimeoutExpired as exc:
        code = 124; out = exc.stdout or b''; err = exc.stderr or b''
        out = out.decode() if isinstance(out, bytes) else out; err = err.decode() if isinstance(err, bytes) else err
        err += '\nUnknown: timeout\n'
    after = state(); verify(after)
    data = dict(at=at, end=now(), seconds=time.monotonic()-start, argv=argv, cwd=str(ROOT), env={'CARGO_TARGET_DIR': TARGET}, before=before, after=after, exit=code, stdout=out, stderr=err)
    manifest = D / ('p1337-manifest-r1.json' if (D / 'p1337-manifest-r1.json').exists() else 'p1337-manifest.json')
    if manifest.exists():
        data['manifest_sha256'] = sha(manifest)
        data['manifest_path'] = str(manifest)
    binary = Path(TARGET) / 'release/typst'
    if binary.exists(): data['candidate_binary'] = dict(path=str(binary), sha256=sha(binary))
    save(name, data)
    print('Private RED output retained; exit=' + str(code) if name.startswith('unit-red') else out[-700:] + err[-700:])
    return code
if __name__ == '__main__':
    if sys.argv[1] == 'init': init()
    elif sys.argv[1] == 'measure': measure()
    else: sys.exit(command(sys.argv[1], sys.argv[2:]))
