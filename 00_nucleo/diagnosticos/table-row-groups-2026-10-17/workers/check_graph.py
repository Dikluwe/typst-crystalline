from pathlib import Path
import datetime,gzip,hashlib,json,re,shutil,subprocess,tempfile
root=Path.cwd();out=Path(__file__).resolve().parents[1];package=root/'00_nucleo/materialization/table-row-groups-2026-10-17';scope=json.load(open(package/'scope.json'));b=json.load(gzip.open(out/'before.json.gz'));sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest();assert all(sha(p)==h for p,h in b['hashes'].items());linter=b['linter']['binary'];assert sha(linter)==b['linter']['sha256'];record=dict(start=datetime.datetime.now(datetime.timezone.utc).isoformat(),head=b['head'],source_hashes=b['hashes'],purpose='Validate candidate ownership/nucleus graph and calculate preview pins only; existing code bodies are copied unchanged. This is not implementation or a functional GREEN.',commands=[])
with tempfile.TemporaryDirectory(prefix='tekt-row-groups-graph-') as d:
 stage=Path(d)
 for top in ['01_core','02_shell','03_infra','04_wiring','benches','00_nucleo/prompts']:
  for p in (root/top).rglob('*'):
   if p.is_file() and p.suffix in ['.rs','.md','.toml']:
    dst=stage/p.relative_to(root);dst.parent.mkdir(parents=True,exist_ok=True);shutil.copyfile(p,dst)
 for name in ['Cargo.toml','crystalline.toml']:shutil.copyfile(root/name,stage/name)
 overlays={}
 for p in (package/'l0').rglob('*'):
  if p.is_file():
   rel=p.relative_to(package/'l0');dst=stage/rel;dst.parent.mkdir(parents=True,exist_ok=True);shutil.copyfile(p,dst);overlays[str(rel)]=sha(p)
 record['overlays']=overlays
 with (out/'graph-v26-before.json').open('w') as f:subprocess.run([linter,str(stage),'--checks','v26','--format','sarif','--fail-on','warning'],stdout=f,stderr=subprocess.STDOUT)
 for label,cmd in [('fix',[linter,str(stage),'--checks','v1,v5,v15,v26','--fix-hashes']),('strict',[linter,str(stage),'--checks','v1,v5,v15,v26','--fail-on','warning'])]:
  with (out/('graph-'+label+'.log')).open('w') as f:r=subprocess.run(cmd,stdout=f,stderr=subprocess.STDOUT)
  record['commands'].append(dict(label=label,argv=cmd,exit=r.returncode));assert r.returncode==0,(label,(out/('graph-'+label+'.log')).read_text()[-3000:])
 pins={};allowed={v['source'] for v in scope['owners'].values()}
 norm=lambda s:re.sub(r'^//! @prompt-hash [^\n]*\n','',s,count=1,flags=re.M)
 for source in allowed:
  a=(root/source).read_text();z=(stage/source).read_text();assert norm(a)==norm(z),source
  pins[source]=dict(owner=re.search(r'^//! @prompt (.+)$',z,re.M).group(1),old=re.search(r'^//! @prompt-hash (.+)$',a,re.M).group(1),candidate=re.search(r'^//! @prompt-hash (.+)$',z,re.M).group(1))
 # Any source outside the planned owners must have the same exact bytes.
 for top in ['01_core','02_shell','03_infra','04_wiring','benches']:
  for p in (stage/top).rglob('*.rs'):
   rel=str(p.relative_to(stage))
   if rel not in allowed:assert sha(p)==sha(root/rel),rel
 # Save the exact L0 metadata preview, not a new implementation.
 l0_preview={p:(stage/p).read_text() for p in scope['owners']}
 with gzip.open(out/'graph-l0-preview.json.gz','wt') as f:json.dump(l0_preview,f)
 (package/'header-pins.json').write_text(json.dumps(pins,indent=2)+'\n')
record.update(end=datetime.datetime.now(datetime.timezone.utc).isoformat(),sources_unchanged=all(sha(p)==h for p,h in b['hashes'].items()),candidate_graph_valid=True,productive_bodies_changed=0,live_l0_changed=0,headers_previewed=len(pins));assert record['sources_unchanged'];(out/'graph-result.json').write_text(json.dumps(record,indent=2)+'\n');print(json.dumps({k:record[k] for k in ['candidate_graph_valid','productive_bodies_changed','live_l0_changed','headers_previewed','sources_unchanged']}),flush=True)
