"""Archive final validation output with exact current source and lineage hashes."""
import datetime,gzip,hashlib,json,subprocess
from pathlib import Path
HERE=Path(__file__).resolve().parent;ROOT=HERE.parents[2]
git=lambda *args:subprocess.check_output(['git',*args],cwd=ROOT,text=True)
files=[ROOT/name for name in git('diff','--name-only').splitlines() if name.endswith(('.rs','.md','.toml'))]
files += [ROOT/'01_core/src/compiler/realize/mod.rs',ROOT/'00_nucleo/prompts/compiler/realize/mod.md',ROOT/'00_nucleo/prompts/_nuclei/realization-order.toml',ROOT/'01_core/src/compiler/layout/realized.rs',ROOT/'00_nucleo/prompts/compiler/layout/realized.md']
logs={
'core':('/tmp/tekt-noop-reuse-2026-10-05/core.log','cargo test --offline -p typst-core --lib'),
'infra':('/tmp/tekt-noop-reuse-2026-10-05/infra.log','cargo test --offline -p typst-infra --lib'),
'build':('/tmp/tekt-noop-reuse-2026-10-05/build.log','cargo build --offline --release'),
'lint':('/tmp/tekt-noop-reuse-2026-10-05/lint.sarif','crystalline-lint --format sarif .'),
'ownership':('/tmp/tekt-realize-nucleus-gate.log','crystalline-lint --checks v15,v26 .'),
 'reuse-red':('/tmp/tekt-noop-reuse-2026-10-05/red.log','cargo test --offline -p typst-infra --lib unchanged_'),
'origins-red':('/tmp/tekt-noop-reuse-2026-10-05/origin-red.log','cargo test --offline -p typst-core --lib prepared_realized_origins'),
'trace-before':('/tmp/tekt-noop-reuse-2026-10-05/trace-before.log','gdb --batch -x /tmp/tekt-noop-reuse-2026-10-05/trace.gdb --args /tmp/tekt-noop-reuse-2026-10-05/typst-baseline compile /tmp/tekt-noop-reuse-2026-10-05/trace.typ /tmp/tekt-noop-reuse-2026-10-05/trace.pdf --pdf-tagged=false --pretty=false'),
'trace-after':('/tmp/tekt-noop-reuse-2026-10-05/trace-after.log','gdb --batch -x /tmp/tekt-noop-reuse-2026-10-05/trace.gdb --args target/release/typst compile /tmp/tekt-noop-reuse-2026-10-05/trace.typ /tmp/tekt-noop-reuse-2026-10-05/trace-after.pdf --pdf-tagged=false --pretty=false'),

}
r={'collected':datetime.datetime.now(datetime.timezone.utc).isoformat(),'head':git('rev-parse','HEAD').strip(),'status':git('status','--short'),'diff_stat':git('diff','HEAD','--stat'),'working_tree_hashes':{str(p.relative_to(ROOT)):hashlib.sha256(p.read_bytes()).hexdigest() for p in files},'note':'Source hashes describe final tree; RED logs are intermediate failures, not benchmarks of final tree. Log mtime is completion timestamp of redirected output. Final suite runs followed the last productive change and sealing.','logs':{},'trace_script':Path('/tmp/tekt-noop-reuse-2026-10-05/trace.gdb').read_text()}
for name,(path,command) in logs.items():
 p=Path(path)
 r['logs'][name]={'command':command,'log_finished_utc':datetime.datetime.fromtimestamp(p.stat().st_mtime,datetime.timezone.utc).isoformat(),'sha256':hashlib.sha256(p.read_bytes()).hexdigest(),'output':p.read_text()}
with gzip.open(HERE/'execution.json.gz','wt') as f:json.dump(r,f,ensure_ascii=False,indent=2)
print('Archived execution logs and final source provenance')
