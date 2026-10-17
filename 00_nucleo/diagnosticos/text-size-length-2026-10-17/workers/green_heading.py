from pathlib import Path
import subprocess,json,gzip,hashlib,datetime
out=Path(__file__).resolve().parents[1];b=json.load(gzip.open(out/'before.json.gz'));s=json.load(open(out/'scope.json'));commands=[['cargo','fmt','--all'],[b['linter']['path'],'.','--checks','v15,v26','--fail-on','warning'],['python3','00_nucleo/diagnosticos/tekt-v16-table-cells-2026-10-17/workers/seal_owned.py']+s['sources'],['cargo','test','--offline','--locked','--workspace','ts1_','--','--test-threads=1']];results=[]
for cmd in commands:
 r=subprocess.run(cmd);results.append(dict(argv=cmd,exit=r.returncode));(out/'heading-green-result.json').write_text(json.dumps(dict(utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),commands=results),indent=2)+'\n');assert r.returncode==0,cmd
