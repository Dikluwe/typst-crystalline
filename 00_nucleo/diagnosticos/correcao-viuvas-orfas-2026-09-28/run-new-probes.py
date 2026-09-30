from pathlib import Path
import subprocess
p=Path('00_nucleo/diagnosticos/correcao-viuvas-orfas-2026-09-28')
assert 'Finished `release`' in (p/'release-final.log').read_text()
for folder in ['after-pdf','after-extra']:
 with (p/folder/'sondas.log').open('w') as log:subprocess.run(['python3',str(p/folder/'sondas.py')],stdout=log,stderr=subprocess.STDOUT,check=True)
 print(folder+' completed',flush=True)
