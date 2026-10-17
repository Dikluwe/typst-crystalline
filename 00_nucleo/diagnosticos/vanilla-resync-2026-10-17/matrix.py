from pathlib import Path
import subprocess,json,datetime,hashlib,sys
root=Path.cwd();out=root/'00_nucleo/diagnosticos/vanilla-resync-2026-10-17'
now=lambda:datetime.datetime.now(datetime.timezone.utc).isoformat()
profiles=['default','html','a11y-extras','html+a11y','bundle','html+bundle'];results=[]
for profile in profiles:
 path=out/('matrix-'+profile.replace('+','-')+'.json')
 cmd=['python3','lab/parity/matrix/runner.py','--profile',profile,'--vanilla',str(root/'lab/typst-original/target/release/typst'),'--crystalline',str(root/'target/debug/typst'),'--output',str(path)]
 start=now()
 with path.with_suffix('.log').open('w') as f:r=subprocess.run(cmd,stdout=f,stderr=subprocess.STDOUT)
 d=json.loads(path.read_text()) if path.exists() else {}
 item={'profile':profile,'command':cmd,'start':start,'end':now(),'exit':r.returncode,'sha256':hashlib.sha256(path.read_bytes()).hexdigest() if path.exists() else None,'summary':d.get('counts'),'order_check':d.get('order_check'),'unexpected':[{'id':x['id'],'state':x['estado'],'expected':x.get('expected_state'),'class':x.get('classe')} for x in d.get('results',[]) if not x.get('expectation_met',True)]}
 results.append(item);print(json.dumps(item),flush=True)
 (out/'matrix-summary.json').write_text(json.dumps(results,indent=2)+'\n')
sys.exit(0 if all(x['exit']==0 for x in results) else 1)
