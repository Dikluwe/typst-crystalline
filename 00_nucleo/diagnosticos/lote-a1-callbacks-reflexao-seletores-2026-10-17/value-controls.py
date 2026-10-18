from pathlib import Path
import subprocess,json,gzip,datetime
p=Path(__file__).resolve().parent;root=p.parents[2]
exprs=['type(h(2pt).amount)','type(v(2pt).amount)','type(box(width:2pt)[A].width)','type(block(width:2pt)[A].width)','type(block(width:50%)[A].width)','counter(page)==counter(selector(page))','counter(math.equation)==counter(selector(math.equation))','text(fill:red)[A].has("text")','text(fill:red)[A].fields().keys()']
rows=[]
for e in exprs:
 cmd=[str(root/'lab/typst-original/target/release/typst'),'eval',e,'--in',str(p/'outputs/text-fields.typ'),'--format','json'];r=subprocess.run(cmd,text=True,capture_output=True);rows.append({'expression':e,'command':cmd,'exit':r.returncode,'stdout':r.stdout,'stderr':r.stderr});print(e,r.returncode,r.stdout.strip(),flush=True)
(p/'value-controls.json.gz').write_bytes(gzip.compress(json.dumps({'utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'cases':rows}).encode()))
