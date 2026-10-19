"""Replay the strengthened legacy grid test against restored original parsers."""
from pathlib import Path
import datetime, gzip, hashlib, json, os, re, shutil, subprocess

ROOT = Path.cwd()
OUT = ROOT/'lab/results/v16-corrections-2026-10-19/batch-f/legacy-grid-red'
STAGE = ROOT/'lab/scratch/v16-batch-f-legacy-grid-red'
OUT.mkdir(exist_ok=False)
assert not STAGE.exists()
sha = lambda p: hashlib.sha256(Path(p).read_bytes()).hexdigest()
utc = lambda: datetime.datetime.now(datetime.timezone.utc).isoformat()
for top in ['01_core','02_shell','03_infra','04_wiring','lab/benches','lab/corpus','lab/parity/corpus','lab/typst-original/tests']:
    shutil.copytree(ROOT/top,STAGE/top)
for name in ['Cargo.toml','Cargo.lock','clippy.toml','rustfmt.toml','rust-toolchain.toml','.cargo/config.toml']:
    dest = STAGE/name
    dest.parent.mkdir(parents=True,exist_ok=True)
    shutil.copyfile(ROOT/name,dest)
inputs = {str(p.relative_to(STAGE)):sha(p) for p in STAGE.rglob('*') if p.is_file()}
restores = {}
for name in ['01_core/src/compiler/stdlib/layout/grid.rs','01_core/src/compiler/stdlib/structural/table_grid.rs']:
    content = subprocess.check_output(['git','show','HEAD:'+name])
    (STAGE/name).write_bytes(content)
    restores[name] = {'original_current_sha256':inputs[name], 'restored_sha256':sha(STAGE/name), 'content':content.decode()}
state = {'utc':utc(),'head':subprocess.check_output(['git','rev-parse','HEAD'],text=True).strip(),'cwd':str(STAGE),'inputs_before_restore':inputs,'restores':restores,'method':'Current complete source copy with only the coordinated grid/table parser owners restored to HEAD; original malformed-input behavior, not a whole-baseline claim.'}
with gzip.open(OUT/'inputs.json.gz','wt') as f:
    json.dump(state,f)
command = ['cargo','test','--offline','--locked','-p','typst-core','--lib','p633_grid_columns_invalid_string_rejected','--','--test-threads=1']
env = dict(os.environ,CARGO_TARGET_DIR=str(ROOT/'lab/target/product'))
with (OUT/'test.log').open('w') as f:
    run = subprocess.run(command,cwd=STAGE,env=env,stdout=f,stderr=subprocess.STDOUT)
log = (OUT/'test.log').read_text()
outcomes = re.findall(r'^test [^\n]*::p633_grid_columns_invalid_string_rejected \.\.\. (ok|FAILED|ignored)\s*$',log,re.M)
changed = [p for p,h in inputs.items() if sha(STAGE/p) != restores.get(p,{}).get('restored_sha256',h)]
receipt = {'utc':utc(),'command':command,'cwd':str(STAGE),'exit':run.returncode,'outcomes':outcomes,'changed':changed,'log_sha256':sha(OUT/'test.log'),'inputs_sha256':sha(OUT/'inputs.json.gz')}
(OUT/'result.json').write_text(json.dumps(receipt,indent=2)+'\n')
assert run.returncode == 101 and outcomes == ['FAILED']
assert '1 failed' in log and 'unwrap_err()' in log and 'error[E' not in log and 'could not compile' not in log
assert not changed
print('PASS: strengthened legacy regression rejects the restored silent acceptance')
