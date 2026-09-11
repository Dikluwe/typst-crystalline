"""Compile isolated actual Array routes; no CLI semantic probes or oracle import."""
import base64, datetime, difflib, hashlib, json, os, pathlib, shutil, signal, subprocess

ROOT = pathlib.Path(__file__).resolve().parents[2]
D = ROOT / '00_nucleo/diagnosticos'
AREA = pathlib.Path('/tmp/p1339-mutant-array.jrXHiv')
SOURCE = AREA / 'source'
TARGET = AREA / 'target'
COMMIT = '2f42d64253547734564513a1159ee6b584c1c4b4'
sha = lambda p: hashlib.sha256(pathlib.Path(p).read_bytes()).hexdigest()
utc = lambda: datetime.datetime.now(datetime.timezone.utc).isoformat()
def write_new(p, value):
    with pathlib.Path(p).open('x') as f:
        json.dump(value, f, ensure_ascii=False, indent=2); f.write('\n')

record_path = D / 'p1339-mutant-array-build-r1.json'
assert not record_path.exists()
pins = {
    'p1339-mutant-array-budget-r1.json': 'f92fe2857944d332b78504779074fc43e25ae6eb2f498ac18f6101af9f2a0692',
    'p1339-contract-array-proposal-r1.md': '8134bbb78a389847d865f93e255db3d8afeb0810b0284a29fa00c065213b109b',
    'p1339-contract-array-inputs-r2.json': '8a0c77118c80d220b58dd8070fe0da2399353c4ae0f46610b245224b389bb3a0',
}
for name, h in pins.items(): assert sha(D / name) == h, name
assert sha(AREA / 'baseline-build.tar') == 'ad3eb3e338d486330d6cc2a49091be3beb83cb4f68269b06bed1560f6b28bc66'
paths = sorted(p for p in SOURCE.rglob('*') if p.is_file())
sources = {str(p.relative_to(SOURCE)): sha(p) for p in paths}
patch = []
changed = ['01_core/src/compiler/eval/call_dispatch.rs', '01_core/src/compiler/eval/p1339_mutant_array.rs']
for path in changed:
    if path.endswith('/p1339_mutant_array.rs'): before = ''
    else: before = subprocess.check_output(['git', 'show', f'{COMMIT}:{path}'], cwd=ROOT).decode()
    after = (SOURCE / path).read_text()
    patch.extend(difflib.unified_diff(before.splitlines(True), after.splitlines(True), fromfile='a/'+path, tofile='b/'+path))
patch_path = D / 'p1339-mutant-array-multiplex-r1.patch'
with patch_path.open('x') as f: f.write(''.join(patch))
# Cache files are separate copied/reflinked files, not shared writable target.
# Force every workspace Rust source newer than copied build artifacts so no
# stale candidate/workspace object can be counted as compiled baseline source.
for path in paths:
    if path.suffix == '.rs': os.utime(path, None)
argv = ['cargo','build','--manifest-path',str(SOURCE/'Cargo.toml'),'-p','typst-wiring',
        '--bin','typst','--release','--locked','--offline','--target-dir',str(TARGET),'-j','3']
env_delta = {'TYPST_COMMIT_SHA': COMMIT}
record = {
    'schema':'p1339-array-isolated-build-v1','author':'/root/p1336_tests',
    'regime':'executado sem atestacao de isolamento',
    'authority_manifest_sha256':'842d6526739014022c073800148a47b3c886831e2198ab65bbfdbe73ce59411b',
    'budget_and_inputs':pins,'runner_sha256':sha(__file__),'start':utc(),
    'argv':argv,'cwd':str(SOURCE),'environment_delta':env_delta,'timeout_seconds':2700,
    'baseline_commit':COMMIT,'archive':{'path':str(AREA/'baseline-build.tar'),'sha256':sha(AREA/'baseline-build.tar')},
    'archive_paths':['Cargo.toml','Cargo.lock','.cargo','01_core','02_shell','03_infra','04_wiring','benches'],
    'source_root':str(SOURCE),'source_sha256':sources,'patch':{'path':str(patch_path),'sha256':sha(patch_path)},
    'cache':{'origin':'/tmp/p1339-target.UD8gh7/release','copy_argv':['cp','-a','--reflink=auto','/tmp/p1339-target.UD8gh7/release',str(TARGET)+'/'],
             'destination':str(TARGET/'release'),'copy_exit':0,'copy_wall_seconds':0.039687152,
             'workspace_rebuild':'Every source .rs mtime touched after copy; compiler log must show all four workspace crates rebuilt. Cache is acceleration, not source/binary provenance.'},
    'mode':'P1339_ARRAY_MUTANT=0 control,1..12 isolated exclusive actual mutations',
    'no_credit':'No semantic fixture has been executed. Control cannot be adopted as solution.'
}
proc = subprocess.Popen(argv,cwd=SOURCE,env={**os.environ,**env_delta},stdout=subprocess.PIPE,stderr=subprocess.PIPE,start_new_session=True)
try:
    stdout, stderr = proc.communicate(timeout=2700); record['transport']='Completed'
except subprocess.TimeoutExpired:
    os.killpg(proc.pid,signal.SIGKILL); stdout,stderr=proc.communicate(); record['transport']='Timeout'
record.update(end=utc(),exit=proc.returncode)
for channel,raw in [('stdout',stdout),('stderr',stderr)]:
    record[channel]=raw.decode(errors='replace');record[channel+'_base64']=base64.b64encode(raw).decode()
    record[channel+'_sha256']=hashlib.sha256(raw).hexdigest()
if proc.returncode == 0:
    output=AREA/'typst-array-multiplex-r1';assert not output.exists()
    shutil.copy2(TARGET/'release/typst',output)
    record['binary']={'path':str(output),'sha256':sha(output)}
write_new(record_path,record)
print(json.dumps({'receipt':str(record_path),'sha256':sha(record_path),'exit':proc.returncode,'binary':record.get('binary')}))
