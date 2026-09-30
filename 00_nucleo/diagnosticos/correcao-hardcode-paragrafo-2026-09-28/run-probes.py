from pathlib import Path
import subprocess
p=Path('00_nucleo/diagnosticos/correcao-hardcode-paragrafo-2026-09-28')
assert 'Finished' in (p/'release-final.log').read_text()
for folder in ['after-pdf','regression-hyphen','regression-selection','regression-multi','regression-composition','regression-precedence','regression-policy','regression-lexical']:
 with (p/(folder+'.log')).open('w') as log:subprocess.run(['python3',str(p/folder/'sondas.py')],stdout=log,stderr=subprocess.STDOUT,check=True)
 print(folder+' completed',flush=True)
with (p/'regression-empty.log').open('w') as log:subprocess.run(['python3',str(p/'regression-empty-runner.py')],stdout=log,stderr=subprocess.STDOUT,check=True)
for script in ['regression-acceptance.py','regression-main-acceptance.py','regressions-acceptance.py','acceptance.py']:
 subprocess.run(['python3',str(p/script)],check=True)
