from pathlib import Path
import subprocess,json,datetime
out=Path('00_nucleo/diagnosticos/tekt-v16-whitespace-2026-10-17');cmd=['cargo','clippy','--offline','--locked','--workspace','--all-targets','--message-format=json','--','-D','warnings','-D','unfulfilled_lint_expectations']
with (out/'strict-clippy.log').open('w') as f:r=subprocess.run(cmd,stdout=f,stderr=subprocess.STDOUT)
messages=[]
for line in (out/'strict-clippy.log').read_text().splitlines():
 try:x=json.loads(line)
 except ValueError:continue
 if x.get('reason')=='compiler-message' and x['message']['level'] in ['warning','error']:messages.append(x['message'])
record=dict(utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),command=cmd,exit=r.returncode,diagnostics=messages);(out/'strict-details.json').write_text(json.dumps(record,indent=2)+'\n');assert r.returncode==0 and not messages,record;print(json.dumps(record),flush=True)
