from pathlib import Path
import datetime,gzip,hashlib,json,subprocess,sys
out=Path(__file__).resolve().parents[1];phase=sys.argv[1];assert phase in ['supplement-before','supplement-after'];b=json.load(gzip.open(out/'before.json.gz'));sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest();hashes={p:sha(p) for p in b['hashes']}
if phase=='before':assert hashes==b['hashes']
cases={'int-min':'calc.abs(int.min)','int-min-extra':'calc.abs(int.min,2)','bound-int-min':'calc.abs.with(int.min)(2)'}
bins={'tekt':Path('target/debug/typst').resolve(),'vanilla':Path('lab/typst-original/target/release/typst').resolve()};assert sha(bins['vanilla'])=='7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea';r=dict(start=datetime.datetime.now(datetime.timezone.utc).isoformat(),head=b['head'],source_hashes=hashes,status=subprocess.check_output(['git','status','--porcelain=v1'],text=True),diffstat=subprocess.check_output(['git','diff','HEAD','--stat'],text=True),binaries={k:dict(path=str(p),sha256=sha(p)) for k,p in bins.items()},cases=[])
for name,expr in cases.items():
 for kind,bin in bins.items():
  cmd=[str(bin),'eval',expr,'--in',str(out/'empty.typ'),'--format','json'];z=subprocess.run(cmd,capture_output=True,text=True,timeout=30);row=dict(name=name,binary=kind,utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),expression=expr,argv=cmd,exit=z.returncode,stdout=z.stdout,stderr=z.stderr);assert z.returncode in [0,1];r['cases'].append(row)
r.update(end=datetime.datetime.now(datetime.timezone.utc).isoformat(),sources_unchanged=all(sha(p)==h for p,h in hashes.items()));assert r['sources_unchanged']
with gzip.open(out/(phase+'-measurement.json.gz'),'wt') as f:json.dump(r,f)
for name in cases:
 print(json.dumps(dict(name=name,observations={x['binary']:dict(exit=x['exit'],stdout=x['stdout'],errors=[l for l in x['stderr'].splitlines() if l.startswith('error:') or 'hint:' in l]) for x in r['cases'] if x['name']==name})),flush=True)
