from pathlib import Path
import subprocess,sys
p=Path(__file__).resolve().parent
for script,label in [('controls.py','controls-after'),('controls_extra.py','controls-extra-after')]:
 print(label,flush=True)
 with (p/(label+'.log')).open('w') as log:z=subprocess.run(['python3',str(p/script),label],stdout=log,stderr=subprocess.STDOUT)
 if z.returncode:sys.exit(z.returncode)
