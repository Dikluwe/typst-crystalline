from pathlib import Path
import json,gzip,hashlib,datetime,subprocess
out=Path('00_nucleo/diagnosticos/raster-mascaras-2026-10-14');out.mkdir(exist_ok=False)
owners={'03_infra/src/export/render.rs':'00_nucleo/prompts/infra/export/render.md'};(out/'owners.json').write_text(json.dumps(owners,indent=2));paths=list(owners)+list(owners.values())
r=dict(utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),head=subprocess.check_output(['git','rev-parse','HEAD'],text=True),status=subprocess.check_output(['git','status','--porcelain=v1'],text=True),diffstat=subprocess.check_output(['git','diff','HEAD','--stat'],text=True),files={n:dict(text=Path(n).read_text(),sha256=hashlib.sha256(Path(n).read_bytes()).hexdigest()) for n in paths})
with gzip.open(out/'before-source.json.gz','wt') as f:json.dump(r,f)
