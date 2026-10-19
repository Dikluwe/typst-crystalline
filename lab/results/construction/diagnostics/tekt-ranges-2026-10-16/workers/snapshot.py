from pathlib import Path
import json,gzip,hashlib,subprocess,datetime
out=Path('00_nucleo/diagnosticos/tekt-ranges-2026-10-16');sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest();prior=Path('00_nucleo/diagnosticos/clippy-enums-publicos-2026-10-16/final-state.json');previous=json.loads(prior.read_text());assert all(sha(p)==h for p,h in previous['source_hashes'].items())
files={str(p):p.read_text() for top in ['01_core','02_shell','03_infra','04_wiring','benches','00_nucleo/prompts'] for p in Path(top).rglob('*') if p.is_file() and p.suffix in ['.rs','.md','.toml']}
for p in ['Cargo.toml','Cargo.lock','crystalline.toml','rustfmt.toml','clippy.toml','.cargo/config.toml','rust-toolchain.toml']:
 if Path(p).is_file():files[p]=Path(p).read_text()
record=dict(utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),head=subprocess.check_output(['git','rev-parse','HEAD'],text=True).strip(),status=subprocess.check_output(['git','status','--porcelain=v1'],text=True),diffstat=subprocess.check_output(['git','diff','HEAD','--stat'],text=True),files=files,hashes={p:hashlib.sha256(t.encode()).hexdigest() for p,t in files.items()},prior_validation=str(prior),prior_validation_sha256=sha(prior))
with gzip.open(out/'before.json.gz','wt') as f:json.dump(record,f)
sources=['01_core/src/compiler/eval/call_dispatch/call_spans.rs','03_infra/src/watch.rs'];owners=['00_nucleo/prompts/compiler/eval/call_dispatch/call_spans.md','00_nucleo/prompts/shell/watch.md'];(out/'scope.json').write_text(json.dumps(dict(sources=sources,owners=owners),indent=2)+'\n')
print(json.dumps(dict(utc=record['utc'],head=record['head'],files=len(files),previous_stage_verified=True)))
