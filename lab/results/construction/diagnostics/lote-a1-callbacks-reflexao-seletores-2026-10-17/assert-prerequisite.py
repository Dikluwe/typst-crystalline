from pathlib import Path
import subprocess,json,gzip,datetime
p=Path(__file__).resolve().parent;root=p.parents[2]
rows=[]
for expression in ['assert.eq(1pt,0%+1pt)','assert.eq((1,),(1.0,))']:
 row={'expression':expression,'results':{}}
 for name,binary in [('tekt-before',p/'outputs/typst-before'),('vanilla',root/'lab/typst-original/target/release/typst')]:
  cmd=[str(binary),'eval',expression,'--in',str(p/'outputs/text-fields.typ'),'--format','json'];r=subprocess.run(cmd,capture_output=True,text=True);row['results'][name]={'command':cmd,'exit':r.returncode,'stdout':r.stdout,'stderr':r.stderr}
 rows.append(row);print(expression,[(n,r['exit']) for n,r in row['results'].items()])
(p/'assert-prerequisite-before.json.gz').write_bytes(gzip.compress(json.dumps({'utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'cases':rows}).encode()))
