from pathlib import Path
import collections,datetime,gzip,hashlib,json,os,re,shutil,subprocess,sys,tempfile
root=Path.cwd();out=Path(__file__).resolve().parents[1];phase=sys.argv[1];assert phase in ['baseline','green'];b=json.load(gzip.open(out/'before.json.gz'));scope=json.load(open(out/'scope.json'));sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest();hashes={p:sha(p) for p in b['hashes']};now=lambda:datetime.datetime.now(datetime.timezone.utc).isoformat();helper='/tmp/tekt-whitespace-latent';recipe=json.load(open(out/'audit-recipe.json'));cmd=recipe['command'].copy();cmd[-3:]=[str(out/'workers/latent.rs'),'-o',helper];assert all(sha(v['path'])==v['sha256'] for v in recipe['externs'].values());subprocess.run(cmd,check=True)
def ast(path):return json.loads(subprocess.check_output([helper,str(path),'matches'],text=True))
def selected(source,path):
 current=path.read_text();nodes=ast(path);marker=current.find('\n#[cfg(test)]\nmod v16_whitespace_tests');assert marker>=0;cut=current[:marker].count('\n')+1;nodes=[n for n in nodes if n['line']<cut];aux=[]
 if phase=='green' and source.endswith('/space.rs'):
  start=current.index('fn space_boundary_content(');line=current[:current.index('match content {',start)].count('\n')+1;aux=[n for n in nodes if n['line']==line];assert len(aux)==1;nodes=[n for n in nodes if n['line']!=line]
 old_nodes=old_ast[source];assert len(nodes)==len(old_nodes),(source,len(nodes),len(old_nodes));rows=[]
 for target in scope['targets']:
  if target['source']!=source:continue
  idx=[i for i,n in enumerate(old_nodes) if n['line']==target['match_line'] and n['end_line']==target['match_end_line']];assert len(idx)==1;node=nodes[idx[0]];rows.append(dict(id=target['id'],source=source,line=node['line'],match_index=idx[0]))
 if aux:rows.append(dict(id='new-boundary-helper',source=source,line=aux[0]['line']))
 return rows
old_ast={}
with tempfile.TemporaryDirectory(prefix='tekt-whitespace-old-ast-') as d:
 for i,source in enumerate(scope['sources']):
  p=Path(d)/(str(i)+'.rs');p.write_text(b['files'][source]);old_ast[source]=ast(p)
fixtures=json.load(open(out/'probe-fixtures-repair.json'))['fixtures'];assert all(sha(p)==h for p,h in fixtures.items())
record=dict(phase=phase,start=now(),head=b['head'],source_hashes=hashes,fixtures=fixtures,worker_sha256=sha(__file__),command_recipe=cmd,original_arms=12,original_matches=11,domains={},mechanism='Actual future Content/SpaceSupport variants; cfg(test)-only E0004 witness. Infra compilation dependencies repaired only outside scoped targets in a discarded tree; no fixture executed.')
for domain in ['core','infra']:
 with tempfile.TemporaryDirectory(prefix='tekt-whitespace-probe-'+domain+'-') as d:
  stage=Path(d)
  for top in ['01_core','02_shell','03_infra','04_wiring','benches']:shutil.copytree(root/top,stage/top,ignore=shutil.ignore_patterns('target','.git','__pycache__'))
  for name in ['Cargo.toml','Cargo.lock','clippy.toml','rustfmt.toml','rust-toolchain.toml','.cargo/config.toml']:
   dst=stage/name;dst.parent.mkdir(parents=True,exist_ok=True);shutil.copyfile(root/name,dst)
  for name,h in fixtures.items():
   dst=stage/name;dst.parent.mkdir(parents=True,exist_ok=True);shutil.copyfile(root/name,dst);assert sha(dst)==h
  content=stage/'01_core/src/entities/content.rs';s=content.read_text();assert s.count('pub enum Content {')==1;content.write_text(s.replace('pub enum Content {','pub enum Content {\n    TektWhitespaceProbe,',1))
  targets_sources=[s for s in scope['sources'] if s.startswith('01_core/' if domain=='core' else '03_infra/')]
  witness_source=targets_sources[-1];p=stage/witness_source
  witness='\n#[cfg(test)]\nenum TektWhitespaceWitness { Current, Future }\n#[cfg(test)]\nfn tekt_whitespace_witness(input: TektWhitespaceWitness) {\n    match input { TektWhitespaceWitness::Current => {} }\n}\n';p.write_text(p.read_text()+witness)
  if domain=='infra':
   s=p.read_text();assert s.count('enum SpaceSupport {')==1;p.write_text(s.replace('enum SpaceSupport {','enum SpaceSupport {\n    TektWhitespaceProbe,',1))
  state=dict(start=now(),workspace=str(stage),iterations=[],repairs=[]);max_iterations=12 if domain=='infra' else 1;cmd=['cargo','test','--offline','--locked','-p','typst-core' if domain=='core' else 'typst-infra','--lib','--no-run','--message-format=json'];state['command']=cmd
  for iteration in range(max_iterations):
   targets=[t for source in targets_sources for t in selected(source,stage/source)];positions={(t['source'],t['line']) for t in targets};witness_line=(stage/witness_source).read_text().split('    match input { TektWhitespaceWitness::Current => {} }')[0].count('\n')+1
   log=out/f'probe-{phase}-{domain}-{iteration}.log';env=dict(os.environ,CARGO_TARGET_DIR=str(root/'target'),CARGO_BUILD_JOBS='1')
   modified={}
   for top in ['01_core','02_shell','03_infra','04_wiring','benches']:
    for p in (stage/top).rglob('*.rs'):
     rel=str(p.relative_to(stage))
     if sha(p)!=sha(root/rel):modified[rel]=p.read_text()
   with gzip.open(out/f'probe-{phase}-{domain}-{iteration}-input.json.gz','wt') as f:json.dump(dict(record=record,state=state,modified=modified,targets=targets,witness_line=witness_line),f)
   with log.open('w') as f:r=subprocess.run(cmd,cwd=stage,env=env,stdout=f,stderr=subprocess.STDOUT)
   errors=[]
   for line in log.read_text().splitlines():
    try:x=json.loads(line)
    except ValueError:continue
    if x.get('reason')=='compiler-message' and x['message']['level']=='error':errors.append(x['message'])
   assert errors and all((e.get('code') or {}).get('code')=='E0004' for e in errors),errors
   primary=[]
   for e in errors:
    spans=[s for s in e['spans'] if s['is_primary']];assert len(spans)==1,e;sp=spans[0];source=sp['file_name'].removeprefix(str(stage)+'/');primary.append((source,sp['line_start']))
   hit=set(primary)&positions;witness_seen=(witness_source,witness_line) in primary
   item=dict(iteration=iteration,exit=r.returncode,selected=sorted(hit),witness_seen=witness_seen,total_e0004=len(errors),targets=targets,primary_locations=primary);state['iterations'].append(item)
   (out/f'probe-{phase}-{domain}-progress.json').write_text(json.dumps(state,indent=2)+'\n')
   if domain=='core':
    assert witness_seen;expected=0 if phase=='baseline' else 6;assert len(hit)==expected,(phase,domain,hit);break
   other=set(primary)-positions-{(witness_source,witness_line)}
   if not other:
    assert witness_seen;expected=0 if phase=='baseline' else 6;assert len(hit)==expected,(phase,domain,hit);break
   grouped=collections.defaultdict(set)
   for source,line in other:grouped[source].add(line)
   for source,lines in grouped.items():
    p=stage/source;data=p.read_bytes();nodes=ast(p);edits=[]
    for line in lines:
     found=[n for n in nodes if n['line']==line];assert len(found)==1,(source,line,found);n=found[0];assert (source,n['line']) not in positions;insert=b'\n_ => panic!("unrelated future-variant compile barrier in disposable probe"),\n';edits.append((n['body_end_byte'],insert));state['repairs'].append(dict(iteration=iteration,source=source,match_line=line,match_start_byte=n['start_byte'],body_end_byte=n['body_end_byte'],insert=insert.decode(),before_sha256=sha(p)))
    for at,extra in sorted(edits,reverse=True):data=data[:at]+extra+data[at:]
    p.write_bytes(data)
  else:raise AssertionError('probe dependency repair did not converge')
  state.update(end=now(),selected_matches=len(hit),witness_seen=witness_seen,final_e0004=len(errors),sources_unchanged=all(sha(p)==h for p,h in hashes.items()))
  assert state['sources_unchanged'];record['domains'][domain]=state
  modified={}
  for top in ['01_core','02_shell','03_infra','04_wiring','benches']:
   for p in (stage/top).rglob('*.rs'):
    rel=str(p.relative_to(stage))
    if sha(p)!=sha(root/rel):modified[rel]=p.read_text()
  with gzip.open(out/f'probe-{phase}-{domain}-input.json.gz','wt') as f:json.dump(dict(record=state,modified=modified,original_consumers={p:Path(p).read_text() for p in scope['sources']},AST_before=old_ast,AST_after={p:ast(root/p) for p in scope['sources']}),f)
record.update(end=now(),selected_matches=sum(x['selected_matches'] for x in record['domains'].values()),sources_unchanged=all(sha(p)==h for p,h in hashes.items()));(out/f'probe-{phase}-result.json').write_text(json.dumps(record,indent=2)+'\n');assert record['selected_matches']==(0 if phase=='baseline' else 12);print(json.dumps(dict(phase=phase,selected_matches=record['selected_matches'],infra_repairs=len(record['domains']['infra']['repairs']),sources_unchanged=record['sources_unchanged'])),flush=True)
