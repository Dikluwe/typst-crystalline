"""Archive final validation output with exact current source and lineage hashes."""
import datetime,gzip,hashlib,json,subprocess
from pathlib import Path
HERE=Path(__file__).resolve().parent;ROOT=HERE.parents[2]
git=lambda *args:subprocess.check_output(['git',*args],cwd=ROOT,text=True)
files=[ROOT/name for name in git('diff','--name-only').splitlines() if name.endswith(('.rs','.md','.toml'))]
files += [ROOT/'01_core/src/compiler/realize/mod.rs',ROOT/'00_nucleo/prompts/compiler/realize/mod.md',ROOT/'00_nucleo/prompts/_nuclei/realization-order.toml',ROOT/'01_core/src/compiler/layout/realized.rs',ROOT/'00_nucleo/prompts/compiler/layout/realized.md']
logs={
'infra':('/tmp/tekt-initial-layout-2026-10-05/infra.log','cargo test --offline -p typst-infra --lib'),
'build':('/tmp/tekt-initial-layout-2026-10-05/build.log','cargo build --offline --release'),
'lint':('/tmp/tekt-initial-layout-2026-10-05/lint.sarif','crystalline-lint --format sarif .'),
'ownership':('/tmp/tekt-realize-nucleus-gate.log','crystalline-lint --checks v15,v26 .'),
 'reuse-red':('/tmp/tekt-initial-layout-2026-10-05/red.log','cargo test --offline -p typst-infra --lib table_reuses_first_layout'),
'reuse-green':('/tmp/tekt-initial-layout-2026-10-05/green.log','cargo test --offline -p typst-infra --lib independent_table_reuses_first_layout; cargo test --offline -p typst-infra --lib numbered_table_keeps_second_layout; cargo test --offline -p typst-infra --lib initial_layout_proof'),

}
r={'collected':datetime.datetime.now(datetime.timezone.utc).isoformat(),'head':git('rev-parse','HEAD').strip(),'status':git('status','--short'),'diff_stat':git('diff','HEAD','--stat'),'working_tree_hashes':{str(p.relative_to(ROOT)):hashlib.sha256(p.read_bytes()).hexdigest() for p in files},'note':'Source hashes describe final tree; RED logs are intermediate failures, not benchmarks of final tree. Log mtime is completion timestamp of redirected output. Final suite runs followed the last productive change and sealing.','logs':{}}
for name,(path,command) in logs.items():
 p=Path(path)
 r['logs'][name]={'command':command,'log_finished_utc':datetime.datetime.fromtimestamp(p.stat().st_mtime,datetime.timezone.utc).isoformat(),'sha256':hashlib.sha256(p.read_bytes()).hexdigest(),'output':p.read_text()}
with gzip.open(HERE/'execution.json.gz','wt') as f:json.dump(r,f,ensure_ascii=False,indent=2)
print('Archived execution logs and final source provenance')
