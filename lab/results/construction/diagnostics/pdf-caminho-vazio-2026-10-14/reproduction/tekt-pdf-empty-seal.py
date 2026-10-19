from pathlib import Path
import json,subprocess,gzip,hashlib,datetime
out=Path('00_nucleo/diagnosticos/pdf-caminho-vazio-2026-10-14');owners=json.loads((out/'owners.json').read_text())
with (out/'seal.log').open('w') as f:subprocess.run(['python3','/tmp/tekt-seal-owned.py']+list(owners),stdout=f,stderr=subprocess.STDOUT,check=True)
paths=list(owners)+list(owners.values());r=dict(utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),head=subprocess.check_output(['git','rev-parse','HEAD'],text=True),status=subprocess.check_output(['git','status','--porcelain=v1'],text=True),diffstat=subprocess.check_output(['git','diff','HEAD','--stat'],text=True),files={n:dict(text=Path(n).read_text(),sha256=hashlib.sha256(Path(n).read_bytes()).hexdigest()) for n in paths})
with gzip.open(out/'candidate-final.json.gz','wt') as f:json.dump(r,f)
print('Sealed only draw.rs and its owner')
