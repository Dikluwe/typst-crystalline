from pathlib import Path
import json,subprocess
out=Path(__file__).resolve().parents[1];exprs=['none','true','42','1.5','"A"','(1,)','(a:1)','std','datetime(year:2020,month:1,day:1)','(x)=>x','[x]','auto','1pt','1em + 10%','10%','1deg','red','stroke()','1fr','left','gradient.linear(red,blue)','regex("x")','tiling(size:(1pt,1pt),[x])','bytes((1,2))','decimal("1")','duration(seconds:1)','version(1,2)','heading.where()','sym.alpha','arguments(1,a:2)','state("x")','counter("x")','<x>','ltr','path("a")','int','color.spot("x",fallback:red)'];rows=[]
for expr in exprs:
 cmd=['target/debug/typst','eval','type('+expr+')','--in',str(out/'empty.typ'),'--format','json'];r=subprocess.run(cmd,capture_output=True,text=True);rows.append(dict(expression=expr,argv=cmd,exit=r.returncode,stdout=r.stdout,stderr=r.stderr))
(out/'value-fixtures.json').write_text(json.dumps(rows,indent=2)+'\n');print([(r['expression'],r['exit'],r['stdout'].strip(),r['stderr'].splitlines()[:1] if r['exit'] else []) for r in rows])
