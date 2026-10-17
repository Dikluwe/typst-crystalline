from pathlib import Path
import gzip,json,hashlib,datetime,subprocess
out=Path('00_nucleo/diagnosticos/clippy-default-2026-10-16')
package=Path('00_nucleo/materialization/clippy-default-2026-10-16')
sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest()
manifest='11f3e138f5ba006d04ed4dd1e829e27c004778e691f4a1df63127b1644146d03'
assert sha(package/'manifest.json')==manifest
for p,h in json.loads((package/'manifest.json').read_text())['files'].items():assert sha(package/p)==h,p
base=json.load(gzip.open(package/'base.json.gz'))
for p,v in base['files'].items():assert sha(p)==v['sha256'],p
assert json.loads((package/'approval-receipt.json').read_text())['manifest_sha256']==manifest
old=Path('00_nucleo/diagnosticos/clippy-expressions-2026-10-16')
files={p:dict(sha256=sha(p),text=Path(p).read_text()) for p in base['files']}
all_hashes={str(p):sha(p) for top in ['01_core','02_shell','03_infra','04_wiring','benches','00_nucleo/prompts'] for p in Path(top).rglob('*') if p.is_file() and p.suffix in ['.rs','.md','.toml']}
before=dict(utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),head=subprocess.check_output(['git','rev-parse','HEAD'],text=True).strip(),status=subprocess.check_output(['git','status','--porcelain=v1'],text=True),diffstat=subprocess.check_output(['git','diff','HEAD','--stat'],text=True),files=files,all_source_hashes=all_hashes,cases=[dict(rule=x['code'],source=x['spans'][0]['file_name'],line=x['spans'][0]['line_start']) for x in json.loads((old/'clippy-inventory.json').read_text()) if x['code']=='clippy::new_without_default'],approved_manifest=manifest)
assert len(before['cases'])==3
assert not (out/'before.json.gz').exists()
with gzip.open(out/'before.json.gz','wt') as f:json.dump(before,f)
for name in ['run.py','suite.py','seal_owned.py','seal.py','validate.py']:
 s=(old/'workers'/name).read_text().replace('clippy-expressions-2026-10-16','clippy-default-2026-10-16').replace('TEKT_EXPRESSIONS_MEMORY_LIMIT','TEKT_DEFAULT_MEMORY_LIMIT')
 if name=='validate.py':s=s.replace("00_nucleo/diagnosticos/field-reassign-2026-10-15/clippy-inventory.json","00_nucleo/diagnosticos/clippy-expressions-2026-10-16/clippy-inventory.json")
 (out/'workers'/name).write_text(s)
for p in files:
 if not p.endswith('.md'):continue
 proposed=package/'proposed'/p
 text=proposed.read_text()+f'\nRatificação: dono aprovou o manifesto SHA-256 `{manifest}` em 2026-10-16. Recibo: `00_nucleo/materialization/clippy-default-2026-10-16/approval-receipt.json`. O gate acima está satisfeito exclusivamente para os três impls Default e testes D1–D5 descritos; esta seção está vigente.\n'
 Path(p).write_text(text)
(out/'l0-before-code.json').write_text(json.dumps(dict(utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),approved_manifest=manifest,owners={p:sha(p) for p in files if p.endswith('.md')},sources_still_original=all(sha(p)==v['sha256'] for p,v in files.items() if p.endswith('.rs'))),indent=2)+'\n')
print('Manifesto e bases conferidos; quatro L0 ratificados antes do código; snapshot e workers preservados.')
