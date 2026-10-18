from pathlib import Path
import subprocess,gzip,json,datetime,hashlib,resource
p=Path(__file__).resolve().parent;r=p.parents[2];source=p/'outputs/shape.typ';rows=[]
for name,b in [('tekt',r/'target/debug/typst'),('vanilla',r/'lab/typst-original/target/release/typst')]:
 out=p/('outputs/shape-'+name+'.pdf');cmd=[str(b),'compile',str(source),str(out)];z=subprocess.run(cmd,capture_output=True,text=True);text=subprocess.run(['pdftotext',str(out),'-'],capture_output=True,text=True).stdout if z.returncode==0 else '';row={'name':name,'argv':cmd,'exit':z.returncode,'stderr':z.stderr,'text':text};rows.append(row);print(row,flush=True)
(p/'shape.json.gz').write_bytes(gzip.compress(json.dumps({'utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'source':source.read_text(),'stack_limit':resource.getrlimit(resource.RLIMIT_STACK),'rows':rows},indent=2).encode()))
