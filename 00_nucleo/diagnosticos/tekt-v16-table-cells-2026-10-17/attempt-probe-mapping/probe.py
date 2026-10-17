from pathlib import Path
import collections,datetime,gzip,hashlib,json,os,re,shutil,subprocess,sys,tempfile,tomllib
root=Path.cwd();out=Path(__file__).resolve().parents[1];phase=sys.argv[1];assert phase in ['baseline','green'];scope=json.load(open(out/'scope.json'));b=json.load(gzip.open(out/'before.json.gz'));checkpoint=json.load(open(out/'tests-before-code.json'))['expected_files'];sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest();hashes={p:sha(p) for p in b['hashes']};now=lambda:datetime.datetime.now(datetime.timezone.utc).isoformat();target=Path('/tmp/tekt-table-cells-probe-target');target.mkdir(exist_ok=True)
assert target.resolve()!=(root/'target').resolve()
if not (out/'target-seed.json').exists():
 packages={tomllib.loads((root/top/'Cargo.toml').read_text())['package']['name'] for top in ['01_core','02_shell','03_infra','04_wiring','benches']};rust_names={p.replace('-','_') for p in packages};debug=target/'debug';(debug/'deps').mkdir(parents=True,exist_ok=True);count=0
 for p in (root/'target/debug/deps').iterdir():
  if p.is_file() and p.suffix in ['.rlib','.rmeta','.so','.d'] and p.stem.removeprefix('lib').rsplit('-',1)[0] not in rust_names:shutil.copy2(p,debug/'deps'/p.name);count+=1
 for sub in ['.fingerprint','build']:
  (debug/sub).mkdir(exist_ok=True)
  for p in (root/'target/debug'/sub).iterdir():
   if p.name.rsplit('-',1)[0] in packages:continue
   dest=debug/sub/p.name
   if p.is_dir():shutil.copytree(p,dest)
   elif p.is_file():shutil.copy2(p,dest)
 (out/'target-seed.json').write_text(json.dumps(dict(utc=now(),target=str(target),workspace_packages_excluded=sorted(packages),external_files=count),indent=2)+'\n')
def ast(path):return json.loads(subprocess.check_output(['/tmp/tekt-whitespace-latent',str(path),'matches'],text=True))
def fn(text,node):return re.findall(r'\bfn\s+(\w+)\s*(?:<|\()',text.encode()[:node['start_byte']].decode())[-1]
def norm(p):return re.sub(r'\s+','',p)
def selected(source,path):
 text=path.read_text();nodes=[n for n in ast(path) if n['line']<text[:text.index('\n#[cfg(test)]\nmod v16_table_tests')].count('\n')+1];rows=[]
 for t in scope['targets']:
  if t['source']!=source:continue
  if phase=='baseline':found=[n for n in nodes if n['line']==t['match_line']]
  elif t['mode']=='snapshot':continue
  else:found=[n for n in nodes if fn(text,n)==t['function'] and any(norm(a['pattern'])==norm(t['replacement']) for a in n['arms'])]
  assert len(found)==1,(phase,t['id'],found);rows.append(dict(id=t['id'],source=source,line=found[0]['line'],function=fn(text,found[0])))
 if phase=='green':
  extra=[n for n in nodes if fn(text,n) in ['captured_recipe','table_group_view'] or (fn(text,n) in ['native_table_header','collect'] and all(a['pattern'].startswith('TableCellKind::') for a in n['arms']))]
  rows.extend(dict(id='new-'+fn(text,n)+'-'+str(i),source=source,line=n['line'],function=fn(text,n)) for i,n in enumerate(extra))
 return rows
record=dict(phase=phase,start=now(),head=b['head'],source_hashes=hashes,cargo_target_dir=str(target),mechanism='Future variants in actual Content, Value and TableCellKind enums, test target and cfg(test) E0004 witness. Disposable workspace and isolated Cargo artifacts.',worker_sha256=sha(__file__))
with tempfile.TemporaryDirectory(prefix='tekt-table-cells-probe-') as directory:
 stage=Path(directory)
 for top in ['01_core','02_shell','03_infra','04_wiring','benches']:shutil.copytree(root/top,stage/top,ignore=shutil.ignore_patterns('target','.git','__pycache__'))
 for name in ['Cargo.toml','Cargo.lock','clippy.toml','rustfmt.toml','rust-toolchain.toml','.cargo/config.toml']:
  dst=stage/name;dst.parent.mkdir(parents=True,exist_ok=True);shutil.copyfile(root/name,dst)
 # Preserve fixture includes used by all test modules.
 fixtures=json.load(open('00_nucleo/diagnosticos/tekt-v16-whitespace-2026-10-17/probe-fixtures-repair.json'))['fixtures']
 for name,h in fixtures.items():
  dst=stage/name;dst.parent.mkdir(parents=True,exist_ok=True);shutil.copyfile(root/name,dst);assert sha(dst)==h
 if phase=='baseline':
  for name,text in checkpoint.items():(stage/name).write_text(text)
 for name,enum in [('01_core/src/entities/content.rs','Content'),('01_core/src/entities/value.rs','Value'),('01_core/src/entities/elements/table_cell.rs','TableCellKind')]:
  p=stage/name;s=p.read_text();assert s.count('pub enum '+enum+' {')==1;p.write_text(s.replace('pub enum '+enum+' {','pub enum '+enum+' {\n    TektTableCellsFuture,',1))
 targets=[t for s in scope['sources'] for t in selected(s,stage/s)];assert len(targets)==(15 if phase=='baseline' else 18),targets
 positions={(t['source'],t['line']) for t in targets};assert len(positions)==len(targets)
 witness_source=scope['sources'][-1];p=stage/witness_source;p.write_text(p.read_text()+'\n#[cfg(test)]\nenum TektTableWitness { Current, Future }\n#[cfg(test)]\nfn tekt_table_witness(input: TektTableWitness) {\n    match input { TektTableWitness::Current => {} }\n}\n');witness_line=p.read_text().split('    match input { TektTableWitness::Current => {} }')[0].count('\n')+1
 modified={str(p.relative_to(stage)):p.read_text() for top in ['01_core','02_shell','03_infra','04_wiring','benches'] for p in (stage/top).rglob('*.rs') if sha(p)!=sha(root/p.relative_to(stage))}
 with gzip.open(out/f'probe-{phase}-input.json.gz','wt') as f:json.dump(dict(record=record,modified=modified,targets=targets,witness=(witness_source,witness_line),fixtures=fixtures),f)
 cmd=['cargo','test','--offline','--locked','-p','typst-core','--lib','--no-run','--message-format=json'];log=out/f'probe-{phase}.log'
 with log.open('w') as f:r=subprocess.run(cmd,cwd=stage,env=dict(os.environ,CARGO_TARGET_DIR=str(target),CARGO_BUILD_JOBS='1'),stdout=f,stderr=subprocess.STDOUT)
 errors=[]
 for line in log.read_text().splitlines():
  try:x=json.loads(line)
  except ValueError:continue
  if x.get('reason')=='compiler-message' and x['message']['level']=='error':errors.append(x['message'])
 primary=[]
 for e in errors:
  for sp in e['spans']:
   if sp['is_primary']:primary.append((sp['file_name'].removeprefix(str(stage)+'/'),sp['line_start']))
 hit=set(primary)&positions;witness_seen=(witness_source,witness_line) in primary
 record.update(end=now(),exit=r.returncode,command=cmd,targets=targets,selected=sorted(hit),selected_matches=len(hit),witness_seen=witness_seen,total_errors=len(errors),errors_only_e0004=all((e.get('code') or {}).get('code')=='E0004' for e in errors),primary_locations=primary,sources_unchanged=all(sha(p)==h for p,h in hashes.items()))
 (out/f'probe-{phase}-result.json').write_text(json.dumps(record,indent=2)+'\n');print(json.dumps({k:record[k] for k in ['phase','exit','selected_matches','total_errors','witness_seen','errors_only_e0004','sources_unchanged']}),flush=True)
 assert record['sources_unchanged'] and errors and record['errors_only_e0004'] and witness_seen and len(hit)==(0 if phase=='baseline' else 18)
