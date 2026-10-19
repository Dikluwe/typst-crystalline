from pathlib import Path
import json,subprocess,collections,datetime
out=Path('00_nucleo/diagnosticos/tekt-v16-introspect-origins-2026-10-16');scope=json.load(open(out/'scope.json'));record={'commands':[]}
for label,cmd,expected in [
 ('seal',['python3',str(out/'workers/seal_owned.py'),*scope['sources']],0),
 ('guard',['python3',str(out/'workers/guard.py')],0),
 ('architecture',['/tmp/tekt-linter-none-target/debug/crystalline-lint','.','--format','sarif'],1)]:
 item={'label':label,'command':cmd,'start':datetime.datetime.now(datetime.timezone.utc).isoformat()}
 with (out/('flat-'+label+'.log')).open('w') as log:r=subprocess.run(cmd,stdout=log,stderr=subprocess.STDOUT)
 item.update(exit=r.returncode,end=datetime.datetime.now(datetime.timezone.utc).isoformat());record['commands'].append(item);(out/'flat-check.json').write_text(json.dumps(record,indent=2)+'\n');assert r.returncode==expected,item
rows=json.loads((out/'flat-architecture.log').read_text())['runs'][0]['results'];counts=dict(collections.Counter(x['ruleId']+':'+x['level'] for x in rows));assert counts=={'V16:error':239,'V19:note':900,'V20:note':871},counts
record.update(counts=counts);(out/'flat-check.json').write_text(json.dumps(record,indent=2)+'\n');print(json.dumps(counts),flush=True)
