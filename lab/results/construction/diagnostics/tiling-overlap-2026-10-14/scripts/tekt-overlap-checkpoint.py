from pathlib import Path
import re,json,gzip,hashlib,subprocess,datetime
out=Path('00_nucleo/diagnosticos/tiling-overlap-2026-10-14');paths=json.loads((out/'owned-sources.json').read_text())
subprocess.run(['rustfmt','--edition','2021','--config','skip_children=true',*paths],check=True)
owned=[s for s in paths if re.search(r'^//! @prompt ',Path(s).read_text(),re.M)]
with (out/'seal-3.log').open('w') as f:subprocess.run(['python3','/tmp/tekt-seal-owned.py',*owned],check=True,stdout=f,stderr=subprocess.STDOUT)
names=paths.copy()+['00_nucleo/prompts/_nuclei/pdf-solid-opacity.toml']
for s in owned:names.append(re.search(r'^//! @prompt (.+)$',Path(s).read_text(),re.M)[1])
snapshot=dict(utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),head=subprocess.check_output(['git','rev-parse','HEAD'],text=True),status=subprocess.check_output(['git','status','--porcelain=v1'],text=True),diffstat=subprocess.check_output(['git','diff','HEAD','--stat'],text=True),files={p:dict(text=Path(p).read_text(),sha256=hashlib.sha256(Path(p).read_bytes()).hexdigest()) for p in sorted(set(names))})
with gzip.open(out/'candidate-final.json.gz','wt') as f:json.dump(snapshot,f)
s=Path('/tmp/tekt-prepare-cell-suite.py').read_text().replace('tiling-preparacao-2026-10-14','tiling-overlap-2026-10-14')
Path('/tmp/tekt-overlap-suite.py').write_text(s)
print('snapshot and scoped seals saved')
