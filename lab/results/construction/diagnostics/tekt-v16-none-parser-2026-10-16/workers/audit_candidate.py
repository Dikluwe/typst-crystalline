from pathlib import Path
import subprocess,json,hashlib,collections,datetime
out=Path('00_nucleo/diagnosticos/tekt-v16-none-parser-2026-10-16').resolve()
root=Path('/tmp/tekt-linter-none-src')
cmd=['/tmp/tekt-linter-none-target/debug/crystalline-lint','.','--checks','v16,v17,v19,v20','--format','sarif','--fail-on','error']
p=subprocess.run(cmd,cwd=root,stdout=subprocess.PIPE,stderr=subprocess.PIPE,text=True)
assert p.returncode==0,p.stderr
(out/'ratchet-before.sarif').write_text(p.stdout)
rows=json.loads(p.stdout)['runs'][0]['results']
identity=lambda r:hashlib.sha256((r['ruleId']+'|./'+r['locations'][0]['physicalLocation']['artifactLocation']['uri'].removeprefix('./')+'|'+' '.join(r['message']['text'].split())+' ['+r['ruleId']+']').encode()).hexdigest()
key=lambda r:(r['ruleId'],r['locations'][0]['physicalLocation']['artifactLocation']['uri'].removeprefix('./'),identity(r))
current=collections.Counter(map(key,rows));old=collections.Counter()
for line in (root/'00_nucleo/assessments/0037-self-complexity.tsv').read_text().splitlines()[1:]:
 fields=line.split('\t')
 if fields[8].startswith('ACCEPT-'):old[(fields[1],fields[3],fields[5])]+=1
added=[dict(result=r,identity=identity(r)) for r in rows if key(r) in current-old]
record={'utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'command':cmd,'added':added,'removed':list((old-current).elements())}
(out/'ratchet-delta.json').write_text(json.dumps(record,indent=2)+'\n')
print(json.dumps(record,indent=2),flush=True)
