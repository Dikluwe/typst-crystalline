from pathlib import Path
import gzip,json,datetime,hashlib,subprocess,shutil
out=Path('00_nucleo/diagnosticos/native-element-catalogue-2026-10-17');out.mkdir(exist_ok=False);(out/'workers').mkdir();prior=Path('00_nucleo/diagnosticos/tekt-v16-figure-casts-2026-10-17');old=json.load(gzip.open(prior/'final-state.json.gz'))
def sha(p):
 h=hashlib.sha256()
 with Path(p).open('rb') as f:
  for data in iter(lambda:f.read(1048576),b''):h.update(data)
 return h.hexdigest()
changed=[p for p,h in old['source_hashes'].items() if sha(p)!=h];assert not changed,changed
source='01_core/src/compiler/eval/bindings/value_methods/element_where.rs';owner='00_nucleo/prompts/compiler/eval/bindings/value_methods/element_where.md';b=dict(utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),head=subprocess.check_output(['git','rev-parse','HEAD'],text=True).strip(),hashes=old['source_hashes'],files={p:Path(p).read_text() for p in [source,owner]},status=subprocess.check_output(['git','status','--porcelain=v1'],text=True),diffstat=subprocess.check_output(['git','diff','HEAD','--stat'],text=True),linter=dict(path='/tmp/tekt-linter-none-target/debug/crystalline-lint',sha256=sha('/tmp/tekt-linter-none-target/debug/crystalline-lint')))
with gzip.open(out/'before.json.gz','wt') as f:json.dump(b,f)
(out/'scope.json').write_text(json.dumps(dict(sources=[source],owners={source:owner},mode='Audit first; no code/API decision before registry/runtime/pinned-source measurement'),indent=2)+'\n');(out/'empty.typ').write_text('')
for name in ['run.py','strict.py','suite.py','rustdoc.py','rust_matches.py']:
 s=(prior/'workers'/name).read_text().replace('tekt-v16-figure-casts-2026-10-17','native-element-catalogue-2026-10-17').replace('TEKT_FIGURE_CAST_MEMORY_LIMIT','TEKT_NATIVE_CATALOGUE_MEMORY_LIMIT');(out/'workers'/name).write_text(s)
shutil.copy('/tmp/bootstrap-native-catalogue.py',out/'workers/bootstrap.py')
