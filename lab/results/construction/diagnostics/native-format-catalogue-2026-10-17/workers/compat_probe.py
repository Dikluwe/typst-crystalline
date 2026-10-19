from pathlib import Path
import json,gzip,subprocess,datetime,hashlib,sys
out=Path(__file__).resolve().parents[1];phase=sys.argv[1];b=json.load(gzip.open(out/'before.json.gz'));sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest();scope=json.load(open(out/'scope.json'));hashes={p:sha(p) for p in b['hashes']};hashes.update({p:sha(p) for p in scope.get('nuclei',[])});cases=json.load(open(out/'batch-schema.json'))['cases'];bins={'tekt':'target/debug/typst','vanilla':'lab/typst-original/target/release/typst'};record=dict(utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),head=b['head'],source_hashes=hashes,binaries={k:sha(p) for k,p in bins.items()},status=subprocess.check_output(['git','status','--porcelain=v1'],text=True),diffstat=subprocess.check_output(['git','diff','HEAD','--stat'],text=True),cases=[])
for profile in range(8):
 features=[name for bit,name in [(1,'html'),(2,'a11y-extras'),(4,'bundle')] if profile&bit]
 for c in cases:
  expr=c['expression'];queries=[('name',f'({expr}).name()'),('repr',f'repr({expr})'),('alias',f'{{let f={expr}; (f=={expr},f.name(),f.with().name())}}')]
  if c['format_kind']:queries += [('scope-'+f,f'({expr}).with().{f}') for f in c['fields']]
  for label,query in queries:
   results={}
   for kind,bin in bins.items():
    cmd=[bin,'eval',query,'--in',str(out/'empty.typ'),'--format','json']+(['--features',','.join(features)] if features else []);r=subprocess.run(cmd,capture_output=True,text=True,timeout=30);assert r.returncode in [0,1,2];results[kind]=dict(argv=cmd,exit=r.returncode,stdout=r.stdout,stderr=r.stderr)
   record['cases'].append(dict(profile=profile,expression=expr,label=label,results=results))
record['sources_unchanged']=all(sha(p)==h for p,h in hashes.items());assert record['sources_unchanged']
with gzip.open(out/(phase+'-compat.json.gz'),'wt') as f:json.dump(record,f)
print('COMPLETE',len(record['cases']),flush=True)
