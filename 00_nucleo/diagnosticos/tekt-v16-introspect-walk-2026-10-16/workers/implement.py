from pathlib import Path
import json,subprocess
out=Path('00_nucleo/diagnosticos/tekt-v16-introspect-walk-2026-10-16');scope=json.load(open(out/'scope.json'))
for label,cmd in [('materialize',['python3',str(out/'workers/materialize.py')]),('fmt',['cargo','fmt','--all']),('seal',['python3',str(out/'workers/seal_owned.py'),*scope['sources']]),('guard',['python3',str(out/'workers/guard.py')])]:
 print(label,flush=True)
 with (out/('implementation-'+label+'.log')).open('w') as log:r=subprocess.run(cmd,stdout=log,stderr=subprocess.STDOUT)
 assert r.returncode==0,(label,r.returncode)
print('IMPLEMENTATION PASS',flush=True)
