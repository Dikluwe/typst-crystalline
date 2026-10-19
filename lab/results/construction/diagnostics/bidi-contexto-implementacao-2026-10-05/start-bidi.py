from pathlib import Path
import hashlib,json,gzip,subprocess,datetime,re
root=Path.cwd(); dest=root/'00_nucleo/diagnosticos/bidi-contexto-implementacao-2026-10-05'; dest.mkdir(exist_ok=True)
files={str(p):p.read_text() for folder in ['01_core/src','02_shell/src','03_infra/src','04_wiring/src','00_nucleo/prompts'] for p in Path(folder).rglob('*') if p.is_file() and p.suffix in ['.rs','.md','.toml']}
receipt={'utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'head':subprocess.check_output(['git','rev-parse','HEAD'],text=True).strip(),'diff_stat':subprocess.check_output(['git','diff','HEAD','--stat'],text=True),'sources':files,'approval':{'message':'confirmo','manifest_sha256':hashlib.sha256(Path('00_nucleo/materialization/bidi-contexto-l0-manifest-2026-10-05.json').read_bytes()).hexdigest()}}
with gzip.open(dest/'before.json.gz','wt') as f:json.dump(receipt,f)
Path('/tmp/tekt-bidi-materializacao').mkdir(exist_ok=True)
import shutil
shutil.copy2('target/release/typst','/tmp/tekt-bidi-materializacao/typst-before')
Path('00_nucleo/materialization/bidi-contexto-aprovacao-2026-10-05.json').write_text(json.dumps(receipt['approval']|{'utc_recorded':receipt['utc'],'status':'human-confirmed'},indent=2)+'\n')
p=Path('01_core/src/entities/mod.rs');p.write_text(p.read_text().replace('pub mod bib_entry;','pub mod bidi_context;\npub mod bib_entry;'))
