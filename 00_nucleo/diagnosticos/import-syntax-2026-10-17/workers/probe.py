from pathlib import Path
import json,gzip,hashlib,datetime,subprocess,sys
out=Path(__file__).resolve().parents[1];phase=sys.argv[1];b=json.load(gzip.open(out/'before.json.gz'));scope=json.load(open(out/'scope.json'));measured_files=set(b['hashes'])|set(scope['sources'])|set(scope['owners'].values())|set(scope['nuclei']);sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest();bins={'tekt':'target/debug/typst','vanilla':'lab/typst-original/target/release/typst'};assert sha(bins['vanilla'])=='7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea';cases=[]
for file in ['reserved','string','missing','multi','prefix','warning','mixed','nested','runtime','valid']:
 for route,expr in [('alias','{ import "'+file+'.typ" as sample; 42 }'),('items','{ import "'+file+'.typ": x; 42 }'),('wildcard','{ import "'+file+'.typ": *; 42 }'),('include','include "'+file+'.typ"'),('include-path','include path("'+file+'.typ")')]:cases.append(dict(label=file+'/'+route,expression=expr,features=[]))
for name,feature in [('html','html'),('bundle','bundle')]:
 for enabled in [False,True]:cases.append(dict(label=name+'/'+str(enabled),expression='{ import "'+name+'.typ" as sample; sample.result }',features=[feature] if enabled else []))
for file in ['reserved','warning','mixed']:
 cases.append(dict(label=file+'/main',expression='42',features=[],input=file+'.typ'))
cases.append(dict(label='warning/repeated',expression='{ import \"warning.typ\" as a; import \"warning.typ\" as b; 42 }',features=[]))
record=dict(start=datetime.datetime.now(datetime.timezone.utc).isoformat(),head=b['head'],source_hashes={p:sha(p) for p in measured_files},fixtures={str(p):sha(p) for p in out.glob('*.typ')},binaries={n:dict(path=p,sha256=sha(p)) for n,p in bins.items()},cases=[])
for c in cases:
 results={}
 for name,bin in bins.items():
  cmd=[bin,'eval',c['expression'],'--in',str(out/c.get('input','empty.typ')),'--format','json']+(['--features',','.join(c['features'])] if c['features'] else []);r=subprocess.run(cmd,capture_output=True,text=True,timeout=30);results[name]=dict(argv=cmd,exit=r.returncode,stdout=r.stdout,stderr=r.stderr)
 record['cases'].append(dict(**c,results=results))
record.update(end=datetime.datetime.now(datetime.timezone.utc).isoformat(),sources_unchanged=all(sha(p)==h for p,h in record['source_hashes'].items()));assert record['sources_unchanged']
with gzip.open(out/(phase+'-probe.json.gz'),'wt') as f:json.dump(record,f)
for c in record['cases']:print(c['label'],[(k,r['exit'],r['stdout'].strip(),r['stderr'].splitlines()[:2]) for k,r in c['results'].items()])
