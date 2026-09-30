from pathlib import Path
import subprocess
p=Path('00_nucleo/diagnosticos/correcao-viuvas-orfas-2026-09-28')
subprocess.run(['python3',str(p/'run-new-probes.py')],check=True)
with (p/'additional-scopes/sondas.log').open('w') as log:subprocess.run(['python3',str(p/'additional-scopes/sondas.py')],stdout=log,stderr=subprocess.STDOUT,check=True)
print('additional-scopes completed',flush=True)
