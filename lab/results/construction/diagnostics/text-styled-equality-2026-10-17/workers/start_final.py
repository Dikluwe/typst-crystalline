from pathlib import Path
import json,subprocess
out=Path(__file__).resolve().parents[1];s=json.load(open(out/'scope.json'));b=json.load(__import__('gzip').open(out/'before.json.gz'))
for cmd in [['cargo','fmt','--all'],[b['linter']['path'],'.','--checks','v15,v26','--fail-on','warning'],['python3','00_nucleo/diagnosticos/tekt-v16-table-cells-2026-10-17/workers/seal_owned.py']+s['sources']]:
 r=subprocess.run(['python3',str(out/'workers/run.py'),'final-prepare-'+('fmt' if cmd[0]=='cargo' else 'graph' if cmd[0]==b['linter']['path'] else 'seal')]+cmd);assert r.returncode==0,cmd
r=subprocess.run(['python3',str(out/'workers/validate.py')]);raise SystemExit(r.returncode)
