from pathlib import Path
import gzip,json,hashlib,datetime,subprocess,sys
out=Path(__file__).resolve().parents[1];phase=sys.argv[1];inventory=json.load(gzip.open(out/'inventory.json.gz'));b=json.load(gzip.open(out/'before.json.gz'));sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest();hashes={p:sha(p) for p in b['hashes']};bins={'tekt':str(Path('target/debug/typst').resolve()),'vanilla':str(Path('lab/typst-original/target/release/typst').resolve())};assert sha(bins['vanilla'])=='7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea';rows=[r for r in inventory['rows'] if r['kind']=='function'];record=dict(start=datetime.datetime.now(datetime.timezone.utc).isoformat(),head=b['head'],source_hashes=hashes,status=subprocess.check_output(['git','status','--porcelain=v1'],text=True),diffstat=subprocess.check_output(['git','diff','HEAD','--stat'],text=True),binaries={k:dict(path=p,sha256=sha(p)) for k,p in bins.items()},method='Runtime public paths from all eight Tekt feature profiles. type(), where() and figure-kind admission; no element constructor body invoked.',cases=[])
with (out/(phase+'-rows.jsonl')).open('w') as stream:
 for i,row in enumerate(rows):
  profile=row['profile'];expr=row['expression'];features=[name for bit,name in [(1,'html'),(2,'a11y-extras'),(4,'bundle')] if profile&bit];data=dict(profile=profile,expression=expr,features=features,registered_name=row['name'],queries={})
  for kind,bin in bins.items():
   for operation,query in [('type',f'type({expr})'),('where',f'type(({expr}).where())'),('kind',f'type(figure([audit],kind:({expr})))')]:
    if operation!='type' and data['queries'][kind]['type']['exit']!=0:continue
    cmd=[bin,'eval',query,'--in',str(out/'empty.typ'),'--format','json']+(['--features',','.join(features)] if features else []);r=subprocess.run(cmd,capture_output=True,text=True,timeout=30);assert r.returncode in [0,1,2],(cmd,r.returncode,r.stderr);data['queries'].setdefault(kind,{})[operation]=dict(utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),argv=cmd,exit=r.returncode,stdout=r.stdout,stderr=r.stderr)
  record['cases'].append(data);stream.write(json.dumps(data,ensure_ascii=False)+'\n');stream.flush()
  if (i+1)%100==0:print('measured',i+1,'/',len(rows),'profile',profile,flush=True)
record.update(end=datetime.datetime.now(datetime.timezone.utc).isoformat(),sources_unchanged=all(sha(p)==h for p,h in hashes.items()));assert record['sources_unchanged']
with gzip.open(out/(phase+'-measurement.json.gz'),'wt') as f:json.dump(record,f)
print('COMPLETE',len(rows),flush=True)
