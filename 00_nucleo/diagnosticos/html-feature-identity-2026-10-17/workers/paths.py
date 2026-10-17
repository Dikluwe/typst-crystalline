from pathlib import Path
import gzip,json,subprocess,hashlib,datetime
out=Path(__file__).resolve().parents[1];b=json.load(gzip.open(out/'before.json.gz'));prior=json.load(gzip.open(out/'before-measurement.json.gz'));sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest();bin=str(Path('target/debug/typst').resolve());vanilla=prior['binaries']['vanilla'];assert sha(vanilla['path'])==vanilla['sha256'];hashes={p:sha(p) for p in b['hashes']};record=dict(start=datetime.datetime.now(datetime.timezone.utc).isoformat(),head=b['head'],source_hashes=hashes,binaries=dict(tekt=dict(path=bin,sha256=sha(bin)),vanilla=vanilla),reuse_vanilla='Exact pinned binary, expression, feature flags and empty file verified; archived results from before-measurement',cases=[])
assert (out/'empty.typ').read_text()==''
for old in prior['cases']:
 if old['expression']!='std.html' and not old['expression'].startswith('std.html.'):continue
 flags=['--features',','.join(old['features'])] if old['features'] else [];expr=old['expression'];results={}
 for label,query in [('type',f'type({expr})'),('where',f'type(({expr}).where())'),('kind',f'type(figure([audit],kind:({expr})))')]:
  if label!='type' and results['type']['exit']!=0:continue
  cmd=[bin,'eval',query,'--in',str(out/'empty.typ'),'--format','json']+flags;r=subprocess.run(cmd,capture_output=True,text=True,timeout=30);results[label]=dict(argv=cmd,exit=r.returncode,stdout=r.stdout,stderr=r.stderr)
 disabled=not old['profile']&1
 if disabled:
  v=old['queries']['vanilla']['type'];t=results['type'];assert t['exit']==v['exit']==1 and t['stderr'].removesuffix('warning: ficheiro vazio: sem conteúdo\n\n')==v['stderr'] and t['stdout']==v['stdout'],(old,t)
 else:
  for label,r in results.items():
   o=old['queries']['tekt'][label];assert (r['exit'],r['stdout'],r['stderr'])==(o['exit'],o['stdout'],o['stderr']),(expr,label)
 record['cases'].append(dict(profile=old['profile'],expression=expr,disabled=disabled,results=results))
record.update(end=datetime.datetime.now(datetime.timezone.utc).isoformat(),sources_unchanged=all(sha(p)==h for p,h in hashes.items()),disabled_rejections=sum(c['disabled'] for c in record['cases']),active_controls=sum(not c['disabled'] for c in record['cases']));assert record['sources_unchanged'];assert record['disabled_rejections']==460;assert record['active_controls']==460
with gzip.open(out/'paths-verification.json.gz','wt') as f:json.dump(record,f)
print('PATHS',record['disabled_rejections'],'disabled rejections;',record['active_controls'],'active controls')
