"""Archive final validation output with exact current source and lineage hashes."""
import datetime,gzip,hashlib,json,subprocess
from pathlib import Path
HERE=Path(__file__).resolve().parent;ROOT=HERE.parents[2]
git=lambda *args:subprocess.check_output(['git',*args],cwd=ROOT,text=True)
files=[ROOT/name for name in git('diff','--name-only').splitlines() if name.endswith(('.rs','.md','.toml'))]
files += [ROOT/'01_core/src/compiler/realize/mod.rs',ROOT/'00_nucleo/prompts/compiler/realize/mod.md',ROOT/'00_nucleo/prompts/_nuclei/realization-order.toml']
logs={
'core':('/tmp/tekt-realize-core-receipt.log','cargo test --offline -p typst-core --lib'),
'infra':('/tmp/tekt-realize-infra-receipt.log','cargo test --offline -p typst-infra --lib'),
'build':('/tmp/tekt-realize-release-receipt.log','cargo build --offline --release'),
'lint':('/tmp/tekt-realize-final-lint.sarif','crystalline-lint --format sarif .'),
'ownership':('/tmp/tekt-realize-nucleus-gate.log','crystalline-lint --checks v15,v26 .'),
'common-red':('/tmp/tekt-single-eval-red.log','initial single_evaluation sentinels'),
'causal-red':('/tmp/tekt-realize-causal-red.log','single_evaluation sentinels before causal-order correction'),
'context-id-red':('/tmp/tekt-realize-context-id-red.log','single_evaluation_recipe_context_ids_do_not_collide_with_captured_content'),
'measurement-red':('/tmp/tekt-realize-measurement-red.log','cargo test --offline -p typst-core --lib realization_transports_preserve_constrained_measurement'),
'legacy-measurement-red':('/tmp/tekt-realize-legacy-measurement-red.log','cargo test --offline -p typst-core --lib generated_shape_keeps_legacy_dimensions'),
}
r={'collected':datetime.datetime.now(datetime.timezone.utc).isoformat(),'head':git('rev-parse','HEAD').strip(),'status':git('status','--short'),'diff_stat':git('diff','HEAD','--stat'),'working_tree_hashes':{str(p.relative_to(ROOT)):hashlib.sha256(p.read_bytes()).hexdigest() for p in files},'note':'Source hashes describe final tree; RED logs are intermediate failures, not benchmarks of final tree. Log mtime is completion timestamp of redirected output. Only lineage/document metadata changed after final suite runs.','logs':{}}
for name,(path,command) in logs.items():
 p=Path(path)
 r['logs'][name]={'command':command,'log_finished_utc':datetime.datetime.fromtimestamp(p.stat().st_mtime,datetime.timezone.utc).isoformat(),'sha256':hashlib.sha256(p.read_bytes()).hexdigest(),'output':p.read_text()}
with gzip.open(HERE/'execution.json.gz','wt') as f:json.dump(r,f,ensure_ascii=False,indent=2)
print('Archived execution logs and final source provenance')
