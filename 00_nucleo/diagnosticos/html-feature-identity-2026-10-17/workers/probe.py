from pathlib import Path
import json,gzip,subprocess,hashlib,datetime,sys
out=Path(__file__).resolve().parents[1];phase=sys.argv[1];b=json.load(gzip.open(out/'before.json.gz'));sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest();hashes={p:sha(p) for p in b['hashes']};bins={'tekt':str(Path('target/debug/typst').resolve()),'vanilla':str(Path('lab/typst-original/target/release/typst').resolve())};assert sha(bins['vanilla'])=='7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea'
cases=[]
for feature,field,roots in [('html','html',['std','std.format','format']),('bundle','asset',['std']),('bundle','bundle',['std.format','format'])]:
 for root in roots:
  for route,expr in [('direct',f'{root}.{field}'),('alias',f'{{ let s = {root}; s.{field} }}'),('closure',f'{{ let s = {root}; let get() = s.{field}; let std = "shadow"; get() }}'),('reexport',f'{{ import "bridge.typ" as bridge; '+f'bridge.{"root" if root=="std" else "formats"}.{field}'+' }')]:
   cases.append(dict(label=f'{root}.{field}/{route}',expression=f'type({expr})',feature=feature,gated=True))
for source,field in [('global.typ','html'),('global.typ','asset'),('format.typ','html'),('format.typ','bundle')]:
 expr=f'{{ import "{source}" as m; '+f'type(m.{field})'+' }';cases.append(dict(label=f'user-{source}/{field}',expression=expr,gated=False))
for field in ['html','asset','bundle']:
 cases.append(dict(label=f'dict/{field}',expression='{ let std = ('+field+': 42); std.'+field+' }',gated=False))
for expr in ['repr(std)','repr(format)','std == std','std.format == format','std.type == type','{ import "bridge.typ" as bridge; bridge.root == std }','{ let f() = 42; f() }']:
 cases.append(dict(label='control/'+expr,expression=expr,gated=False))
record=dict(start=datetime.datetime.now(datetime.timezone.utc).isoformat(),head=b['head'],source_hashes=hashes,fixtures={str(p):sha(p) for p in out.glob('*.typ')},binaries={k:dict(path=p,sha256=sha(p)) for k,p in bins.items()},cases=[])
for profile in range(8):
 flags=[n for bit,n in [(1,'html'),(2,'a11y-extras'),(4,'bundle')] if profile&bit]
 for c in cases:
  results={}
  for kind,bin in bins.items():
   cmd=[bin,'eval',c['expression'],'--in',str(out/'empty.typ'),'--format','json']+(['--features',','.join(flags)] if flags else []);r=subprocess.run(cmd,capture_output=True,text=True,timeout=30);assert r.returncode in [0,1,2];results[kind]=dict(argv=cmd,exit=r.returncode,stdout=r.stdout,stderr=r.stderr)
  record['cases'].append(dict(**c,profile=profile,results=results))
 print('profile',profile,flush=True)
record.update(end=datetime.datetime.now(datetime.timezone.utc).isoformat(),sources_unchanged=all(sha(p)==h for p,h in hashes.items()));assert record['sources_unchanged']
with gzip.open(out/(phase+'-probe.json.gz'),'wt') as f:json.dump(record,f)
for r in record['cases']:
 if r['profile']==0:print(r['label'],[(k,x['exit'],x['stdout'].strip(),x['stderr'].splitlines()[:1]) for k,x in r['results'].items()])
