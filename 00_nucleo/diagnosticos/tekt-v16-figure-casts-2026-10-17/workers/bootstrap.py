from pathlib import Path
import json,gzip,hashlib,datetime,subprocess,shutil
out=Path('00_nucleo/diagnosticos/tekt-v16-figure-casts-2026-10-17');out.mkdir(exist_ok=False);(out/'workers').mkdir();prior=Path('00_nucleo/diagnosticos/tekt-v16-catalogos-2026-10-17');old=json.load(gzip.open(prior/'final-state.json.gz'));sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest();assert all(sha(p)==h for p,h in old['source_hashes'].items())
sources=['01_core/src/compiler/stdlib/figure_image.rs','01_core/src/compiler/stdlib/layout/casting.rs'];owners={sources[0]:'00_nucleo/prompts/compiler/stdlib/figure_image.md',sources[1]:'00_nucleo/prompts/compiler/stdlib/layout/casting.md'}
b=dict(utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),head=subprocess.check_output(['git','rev-parse','HEAD'],text=True).strip(),hashes=old['source_hashes'],files={p:Path(p).read_text() for p in sources+list(owners.values())},status=subprocess.check_output(['git','status','--porcelain=v1'],text=True),diffstat=subprocess.check_output(['git','diff','HEAD','--stat'],text=True),linter=dict(path='/tmp/tekt-linter-none-target/debug/crystalline-lint',sha256=sha('/tmp/tekt-linter-none-target/debug/crystalline-lint')))
with gzip.open(out/'before.json.gz','wt') as f:json.dump(b,f)
(out/'scope.json').write_text(json.dumps(dict(sources=sources,owners=owners,mode='Investigation; scope pending measured classification'),indent=2)+'\n');(out/'empty.typ').write_text('')
for name in ['run.py','strict.py','suite.py','rustdoc.py','rust_matches.py']:
 s=(prior/'workers'/name).read_text().replace('tekt-v16-catalogos-2026-10-17','tekt-v16-figure-casts-2026-10-17').replace('TEKT_CATALOGUE_MEMORY_LIMIT','TEKT_FIGURE_CAST_MEMORY_LIMIT');(out/'workers'/name).write_text(s)
shutil.copy('/tmp/bootstrap-v16-figure-casts.py',out/'workers/bootstrap.py')
