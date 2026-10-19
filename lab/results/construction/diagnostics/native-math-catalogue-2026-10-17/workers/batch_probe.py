from pathlib import Path
import json,gzip,subprocess,hashlib,datetime,sys
out=Path(__file__).resolve().parents[1];phase=sys.argv[1];batch=json.load(open(out/'batch-schema.json'))['cases'];b=json.load(gzip.open(out/'before.json.gz'));sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest();hashes={p:sha(p) for p in b['hashes']};bins={'tekt':'target/debug/typst','vanilla':'lab/typst-original/target/release/typst'};record=dict(utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),head=b['head'],source_hashes=hashes,status=subprocess.check_output(['git','status','--porcelain=v1'],text=True),diffstat=subprocess.check_output(['git','diff','HEAD','--stat'],text=True),binaries={k:sha(p) for k,p in bins.items()},cases=[])
for c in batch:
 expr=c['expression'];features=['--features','bundle'] if expr=='std.asset' else []
 queries=[('empty',f'({expr}).where()'),('kind',f'type(figure([audit],kind:({expr})))'),('counter',f'type(counter({expr}))'),('unknown',f'({expr}).where(__audit_missing: auto)'),('label',f'type(({expr}).where(label: <audit>))')]+[(field,f'({expr}).where({field}: auto)') for field in c['fields']]+[(field,f'({expr}).where({field}: auto)') for field in c['excluded']]+[('partial',f'({expr}).with().where()')]
 for label,query in queries:
  results={}
  for kind,bin in bins.items():
   cmd=[bin,'eval',query,'--in',str(out/'empty.typ'),'--format','json']+features;r=subprocess.run(cmd,capture_output=True,text=True,timeout=30);assert r.returncode in [0,1,2];results[kind]=dict(argv=cmd,exit=r.returncode,stdout=r.stdout,stderr=r.stderr)
  record['cases'].append(dict(expression=expr,label=label,results=results))
record['sources_unchanged']=all(sha(p)==h for p,h in hashes.items());assert record['sources_unchanged']
with gzip.open(out/(phase+'-batch.json.gz'),'wt') as f:json.dump(record,f)
print('COMPLETE',len(record['cases']))
