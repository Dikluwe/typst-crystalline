from pathlib import Path
import json,gzip,subprocess,hashlib,datetime,sys
out=Path(__file__).resolve().parents[1];phase=sys.argv[1];b=json.load(gzip.open(out/'before.json.gz'));scope=json.load(open(out/'scope.json'));measured_files=set(b['hashes'])|set(scope['sources'])|set(scope['owners'].values())|set(scope['nuclei']);sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest();(out/'a11y.typ').write_text('#let result=type(std.pdf.header-cell)\n');(out/'document.typ').write_text('#let result=document("child.html")[hello]\n');(out/'bundle-main.typ').write_text('#import "document.typ" as sample\n#sample.result\n');bins={'tekt':'target/debug/typst','vanilla':'lab/typst-original/target/release/typst'};rows=[]
for profile in range(8):
 flags=[name for bit,name in [(1,'html'),(2,'a11y-extras'),(4,'bundle')] if profile&bit]
 for file in ['html','a11y','bundle']:
  results={}
  for name,bin in bins.items():
   cmd=[bin,'eval','{ import "'+file+'.typ" as sample; sample.result }','--in',str(out/'empty.typ'),'--format','json']+(['--features',','.join(flags)] if flags else []);r=subprocess.run(cmd,capture_output=True,text=True,timeout=30);results[name]=dict(argv=cmd,exit=r.returncode,stdout=r.stdout,stderr=r.stderr)
  rows.append(dict(profile=profile,file=file,results=results))
for target in ['bundle']:
 results={}
 for name,bin in bins.items():
  dest=out/(phase+'-'+name+'-bundle');cmd=[bin,'compile',str(out/'bundle-main.typ'),str(dest),'--format',target,'--features','html,bundle'];r=subprocess.run(cmd,capture_output=True,text=True,timeout=30);results[name]=dict(argv=cmd,exit=r.returncode,stdout=r.stdout,stderr=r.stderr,files={str(p.relative_to(dest)):dict(sha256=sha(p),text=p.read_text()) for p in dest.rglob('*') if p.is_file()} if dest.exists() else {})
 rows.append(dict(target=target,results=results))
record=dict(utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),head=b['head'],source_hashes={p:sha(p) for p in measured_files},fixtures={str(p):sha(p) for p in out.glob('*.typ')},binaries={n:dict(path=p,sha256=sha(p)) for n,p in bins.items()},cases=rows)
record['sources_unchanged']=all(sha(p)==h for p,h in record['source_hashes'].items());assert record['sources_unchanged']
with gzip.open(out/(phase+'-context-probe.json.gz'),'wt') as f:json.dump(record,f)
for c in rows:print({k:v for k,v in c.items() if k!='results'},[(n,r['exit'],r['stdout'].strip(),r['stderr'].splitlines()[:1],list(r.get('files',{}))) for n,r in c['results'].items()])
