from pathlib import Path
import subprocess,json,gzip,datetime,resource,hashlib,sys
p=Path(__file__).resolve().parent;r=p.parents[2];rows=[]
cases={
 'for':'#let c=for i in range(4) {[X]}\n#repr(c)',
 'while':'#let n=0\n#let c=while n < 4 {n += 1;[X]}\n#repr(c)',
 'add':'#repr([X]+[Y]+[Z])',
 'empty-fields':'#repr([].fields())',
 'empty-func':'#repr([].func())',
 'empty-children':'#repr([].children)',
 'single-children':'#repr([X].children)',
 'alias':'#let a=[A]+[B]\n#let b=a+[C]\n#repr(a)\n#repr(b)',
 'both':'#let a=[A]+[B]\n#let b=[C]+[D]\n#repr(a+b)',
 'source-sequence':'#let a=[#strong[A]#emph[B]]\n#repr(a+[C])',
 'source-empty':'#let a=[]\n#repr(a+[C])',
 'empty-left':'#repr([]+[C])',
 'empty-right':'#repr([C]+[])',
 'empty-both':'#repr([]+[])',
 'empty-string-right':'#repr([A]+"")',
 'empty-string-left':'#repr(""+[A])',
 'nonempty-string':'#repr(([A]+[B])+"C")',
 'opaque-right':'#let a=[A]+[B]\n#repr(text(fill:red)[a]+[C])',
 'styled-opaque':'#let a=text(fill:red)[A B]\n#repr(a+[C])',
}
for label,src in cases.items():
 f=p/('outputs/'+label+'.typ');f.write_text(src+'\n');row={'label':label,'source':src,'source_sha256':hashlib.sha256(f.read_bytes()).hexdigest(),'results':{}}
 for name,b in [('tekt',r/'target/debug/typst'),('vanilla',r/'lab/typst-original/target/release/typst')]:
  out=p/('outputs/'+label+'-'+name+'.pdf');cmd=[str(b),'compile',str(f),str(out)];z=subprocess.run(cmd,text=True,capture_output=True,timeout=30);txt=subprocess.run(['pdftotext',str(out),'-'],capture_output=True,text=True).stdout if not z.returncode else '';row['results'][name]={'argv':cmd,'exit':z.returncode,'stderr':z.stderr,'text':' '.join(txt.split())}
 rows.append(row);print(label,[(k,v['exit'],v['text']) for k,v in row['results'].items()],flush=True)
(p/('morphology-'+sys.argv[1]+'.json.gz')).write_bytes(gzip.compress(json.dumps({'utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'stack_limit':resource.getrlimit(resource.RLIMIT_STACK),'cases':rows},indent=2).encode()))
