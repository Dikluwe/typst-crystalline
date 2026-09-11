"""Record local implementation checks; never an independent verdict."""
import datetime, hashlib, json, os, pathlib, subprocess, sys, time

ROOT = pathlib.Path('/repos/Antigravity/typst-crystalline')
label, *argv = sys.argv[1:]
assert label.replace('-', '').isalnum() and argv
paths = ['01_core/src/compiler/stdlib/foundations/float.rs',
         '01_core/src/compiler/stdlib/primitives_constructors/version.rs',
         '01_core/src/compiler/stdlib/primitives_constructors.rs',
         '01_core/src/compiler/stdlib/mod.rs']
def sha(p):
    return hashlib.sha256((ROOT / p).read_bytes()).hexdigest()
def git(*args):
    return subprocess.check_output(['git', *args], cwd=ROOT, text=True)
def state():
    return {'utc': datetime.datetime.now(datetime.timezone.utc).isoformat(),
            'head': git('rev-parse', 'HEAD').strip(),
            'diff_stat': git('diff', 'HEAD', '--stat'),
            'status': git('status', '--short'),
            'source_sha256': {p: sha(p) for p in paths}}
before = state()
start = time.monotonic()
run = subprocess.run(argv, cwd=ROOT, text=True, capture_output=True)
after = state()
receipt = {'executor': '/root/p1339_remaining_l0', 'role':'implementation-local-check',
           'regime':'executado sem atestacao de isolamento', 'label':label, 'argv':argv,
           'target_dir':os.environ.get('CARGO_TARGET_DIR'), 'before':before, 'after':after,
           'seconds':time.monotonic()-start, 'exit':run.returncode,
           'stdout':run.stdout, 'stderr':run.stderr,
           'sources_changed_during_run':before['source_sha256'] != after['source_sha256'],
           'authority_sha256':sha('00_nucleo/diagnosticos/p1339-implementation-authorities.json'),
           'seal_sha256':sha('00_nucleo/diagnosticos/p1339-seal.json'),
           'no_claims':['Not independent verification', 'No final P1339 verdict']}
out = ROOT / f'00_nucleo/diagnosticos/p1339-implementation-numerics-{label}.json'
with out.open('x') as f:
    json.dump(receipt, f, indent=2)
    f.write('\n')
print(run.stdout)
print(run.stderr[-4000:])
print(json.dumps({'receipt':str(out), 'sha256':sha(out), 'exit':run.returncode, 'seconds':receipt['seconds']}))
sys.exit(run.returncode)
