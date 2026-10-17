from pathlib import Path
import gzip,json,hashlib,datetime,subprocess,sys
out=Path('00_nucleo/diagnosticos/tiling-cobertura-2026-10-14');files=[Path('01_core/src/compiler/layout/tiling.rs'),Path('00_nucleo/prompts/compiler/layout/tiling.md')]
r=dict(utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),head=subprocess.check_output(['git','rev-parse','HEAD'],text=True),diffstat=subprocess.check_output(['git','diff','HEAD','--stat'],text=True),files={str(p):dict(text=p.read_text(),sha256=hashlib.sha256(p.read_bytes()).hexdigest()) for p in files})
with gzip.open(out/(sys.argv[1]+'.json.gz'),'wt') as f:json.dump(r,f)
