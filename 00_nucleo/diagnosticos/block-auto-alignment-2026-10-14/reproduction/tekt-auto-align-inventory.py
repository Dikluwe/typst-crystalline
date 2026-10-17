from pathlib import Path
import json,re,gzip,datetime,subprocess,hashlib
out=Path('00_nucleo/diagnosticos/block-auto-alignment-2026-10-14');out.mkdir(exist_ok=False)
names=['mod','block','sub_frame','checkpoint','placement','tests']
paths=[f'01_core/src/compiler/layout/{n}.rs' for n in names]
owners={n:re.search(r'^//! @prompt (.+)$',Path(n).read_text(),re.M).group(1) for n in paths};(out/'owners.json').write_text(json.dumps(owners,indent=2))
paths+=list(owners.values())
r=dict(utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),head=subprocess.check_output(['git','rev-parse','HEAD'],text=True),status=subprocess.check_output(['git','status','--porcelain=v1'],text=True),diffstat=subprocess.check_output(['git','diff','HEAD','--stat'],text=True),files={p:dict(sha256=hashlib.sha256(Path(p).read_bytes()).hexdigest(),text=Path(p).read_text()) for p in paths})
with gzip.open(out/'before-source.json.gz','wt') as f:json.dump(r,f)
