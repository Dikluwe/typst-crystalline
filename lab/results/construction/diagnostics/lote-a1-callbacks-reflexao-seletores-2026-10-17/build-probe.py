from pathlib import Path
import subprocess,json,os,datetime
p=Path(__file__).resolve().parent;rows=[]
for label,cmd in [
 ('candidate-build',['cargo','build','--offline','--locked','--workspace']),
 ('value-shape',['python3',str(p/'value-shape.py'),'after']),
 ('candidate-main',['python3',str(p/'probe.py'),'candidate']),
 ('candidate-state',['python3',str(p/'state-extended.py')]),
 ('candidate-targets',['python3',str(p/'target-controls.py')]),
 ('candidate-extra',['python3',str(p/'extra-probe.py')]),
]:
 start=datetime.datetime.now(datetime.timezone.utc).isoformat()
 with (p/(label+'.log')).open('w') as log:r=subprocess.run(cmd,stdout=log,stderr=subprocess.STDOUT)
 row={'label':label,'command':cmd,'start':start,'end':datetime.datetime.now(datetime.timezone.utc).isoformat(),'exit':r.returncode};rows.append(row);print(row,flush=True)
 (p/'candidate-results.json').write_text(json.dumps(rows,indent=2))
 if r.returncode:raise SystemExit(r.returncode)
