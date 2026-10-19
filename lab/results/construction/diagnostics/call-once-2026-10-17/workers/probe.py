from pathlib import Path
import json,gzip,hashlib,subprocess,sys,datetime
out=Path(__file__).resolve().parents[1];phase=sys.argv[1];b=json.load(gzip.open(out/'before.json.gz'));sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest();bins={'tekt':str(Path('target/debug/typst').resolve()),'vanilla':str(Path('lab/typst-original/target/release/typst').resolve())};assert sha(bins['vanilla'])=='7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea';cases=[]
for receiver,method,arg,result in [('calc','abs','-3','3'),('array','len','(1,2)','2'),('str','from-unicode','65','"A"'),('[abc]','has','"text"','true'),('(1,2)','at','1','2'),('(a:1)','at','"a"','1'),('"abc"','at','1','"b"'),('version(1,2,3)','at','0','1')]:
 expr='{ let n = 0; let a = 0; let r = ({ n += 1; '+receiver+' }).'+method+'({ a += 1; '+arg+' }); (n,a,r) }';cases.append(dict(label='effects/'+receiver+'.'+method,expression=expr,expected='(1, 1, '+result+')'))
for method in ['f','abs','measure','layout','within','before','after','func','has','body']:
 expr='{ import "sample.typ" as m; let n=0; let a=0; let r=({ n+=1; m }).'+method+('()' if method=='func' else '({ a+=1; 42 })')+'; (n,a,r) }';cases.append(dict(label='module/'+method,expression=expr,expected='(1, '+('0' if method=='func' else '1')+', 42)'))
for method in ['insert','remove','push','pop']:
 expr='{ import "sample.typ" as sample; let m=sample; let a=0; let r=m.'+method+'({a+=1; 42}); (a,r) }';cases.append(dict(label='mutating-fallback/'+method,expression=expr,expected='(1, 42)'))
for receiver in ['calc','array','[abc]','(1,2)','(a:1)','"abc"','42','version(1,2,3)']:
 cases.append(dict(label='missing/'+receiver,expression='('+receiver+').__missing(panic("argument executed"))',expected=None))
for expr in ['{ import "sample.typ" as m; m.data(panic("argument executed")) }','{ let a=(1,); a.push(2); a }','{ let d=(a:1); d.insert("b",2); d }','{ let n=0; let a=0; let r=({n+=1;heading}).where(level: {a+=1;1}); (n,a,type(r)) }','{ let n=0; let a=0; let r=({n+=1;calc.abs}).with({a+=1;-3}); (n,a,r()) }','{ let n=0; let a=0; let r=({n+=1;heading.where()}).or({a+=1;figure.where()}); (n,a,type(r)) }']:
 cases.append(dict(label='control/'+str(len(cases)),expression=expr,expected=None))
record=dict(start=datetime.datetime.now(datetime.timezone.utc).isoformat(),head=b['head'],source_hashes={p:sha(p) for p in b['hashes']},fixtures={str(p):sha(p) for p in out.glob('*.typ')},binaries={k:dict(path=p,sha256=sha(p)) for k,p in bins.items()},cases=[])
for c in cases:
 results={}
 for kind,bin in bins.items():
  cmd=[bin,'eval',c['expression'],'--in',str(out/'empty.typ'),'--format','json'];r=subprocess.run(cmd,capture_output=True,text=True,timeout=30);results[kind]=dict(argv=cmd,exit=r.returncode,stdout=r.stdout,stderr=r.stderr)
 record['cases'].append(dict(**c,results=results))
record['end']=datetime.datetime.now(datetime.timezone.utc).isoformat();assert all(sha(p)==h for p,h in record['source_hashes'].items())
with gzip.open(out/(phase+'-probe.json.gz'),'wt') as f:json.dump(record,f)
for c in record['cases']:print(c['label'],[(k,r['exit'],r['stdout'].strip(),r['stderr'].splitlines()[:1]) for k,r in c['results'].items()])
