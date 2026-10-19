from pathlib import Path
import json,subprocess,datetime
out=Path('00_nucleo/diagnosticos/tekt-v16-introspect-origins-2026-10-16');scope=json.load(open(out/'scope.json'));record={'commands':[]}
for label,cmd in [('materialize',['python3',str(out/'workers/materialize.py')]),('seal',['python3',str(out/'workers/seal_owned.py'),*scope['sources']]),('guard',['python3',str(out/'workers/guard.py')])]:
 item={'label':label,'command':cmd,'start':datetime.datetime.now(datetime.timezone.utc).isoformat()}
 with (out/('implementation-'+label+'.log')).open('w') as log:r=subprocess.run(cmd,stdout=log,stderr=subprocess.STDOUT)
 item.update(exit=r.returncode,end=datetime.datetime.now(datetime.timezone.utc).isoformat());record['commands'].append(item)
 (out/'implementation-result.json').write_text(json.dumps(record,indent=2)+'\n');assert r.returncode==0,item
 print(json.dumps(item),flush=True)
