from pathlib import Path
import gzip,json,subprocess,hashlib,datetime,sys
out=Path(__file__).resolve().parents[1];phase=sys.argv[1];b=json.load(gzip.open(out/'before.json.gz'));sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest();bins={'tekt':'target/debug/typst','vanilla':'lab/typst-original/target/release/typst'};queries=['{ let n=0; let r=math.mat({n+=1;1}); n }','{ let n=(1,); let r=math.mat({n.push(1);1}); n }','{ let n=(); let r=math.mat({n.push(1);1}); n }','{ let n=0; let r=math.frac({n+=1;1},2); n }','{ let n=0; let a=0; let r=({n+=1;math}).frac({a+=1;1},2); (n,a) }','{ let n=0; let r=math.vec({n+=1;1}); n }','{ let n=0; let r=({n+=1; [abc]}).fields(); n }'];cases=[]
for expr in queries:
 results={}
 for name,bin in bins.items():
  cmd=[bin,'eval',expr,'--in',str(out/'empty.typ'),'--format','json'];r=subprocess.run(cmd,capture_output=True,text=True,timeout=30);results[name]=dict(argv=cmd,exit=r.returncode,stdout=r.stdout,stderr=r.stderr)
 cases.append(dict(expression=expr,results=results))
record=dict(utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),head=b['head'],source_hashes={p:sha(p) for p in b['hashes']},binaries={n:dict(path=p,sha256=sha(p)) for n,p in bins.items()},cases=cases)
with gzip.open(out/(phase+'-math-probe.json.gz'),'wt') as f:json.dump(record,f)
for c in cases:print(c['expression'],[(k,r['exit'],r['stdout'].strip()) for k,r in c['results'].items()])
