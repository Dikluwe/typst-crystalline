from pathlib import Path
import subprocess,gzip,json,hashlib,datetime
out=Path(__file__).resolve().parents[1];b=json.load(gzip.open(out/'before.json.gz'));s=json.load(open(out/'scope.json'));cmds=[['cargo','fmt','--all'],[b['linter']['path'],'.','--checks','v15,v26','--fail-on','warning'],['python3','00_nucleo/diagnosticos/tekt-v16-table-cells-2026-10-17/workers/seal_owned.py']+s['sources'],['cargo','test','--offline','--locked','--workspace','styled_language_','--','--test-threads=1']];record=dict(start=datetime.datetime.now(datetime.timezone.utc).isoformat(),commands=[])
for cmd in cmds:
 if cmd[0]=='cargo' and 'test' in cmd:
  record['test_source_hashes']={p:hashlib.sha256(Path(p).read_bytes()).hexdigest() for p in b['hashes']}
 r=subprocess.run(cmd);record['commands'].append(dict(argv=cmd,exit=r.returncode));(out/'green-result.json').write_text(json.dumps(record,indent=2)+'\n');assert r.returncode==0,cmd
assert all(hashlib.sha256(Path(p).read_bytes()).hexdigest()==h for p,h in record['test_source_hashes'].items());record['complete']=True;(out/'green-result.json').write_text(json.dumps(record,indent=2)+'\n')
