from pathlib import Path
import datetime,gzip,hashlib,json,os,re,shutil,subprocess,sys
root=Path.cwd();phase=sys.argv[1];assert phase in ['baseline','green']
out=root/'lab/results/v16-corrections-2026-10-19/batch-a'/('structural-'+phase);out.mkdir(parents=True,exist_ok=True)
base=root/'lab/scratch/v16-batch-a-base-2026-10-19' if phase=='baseline' else root
stage=root/'lab/scratch/v16-batch-a-probe-2026-10-19';stage.mkdir(parents=True,exist_ok=True)
now=lambda:datetime.datetime.now(datetime.timezone.utc).isoformat()
sha=lambda p:hashlib.sha256(p.read_bytes()).hexdigest()
scope=json.loads((out.parent/'scope.json').read_text());source_freeze={p:sha(base/p) for p in scope['sources']}
# Copy actual sources/manifests; no substitute functions, enums, or dependencies.
for top in ['01_core','02_shell','03_infra','04_wiring','lab/benches']:
 shutil.copytree(base/top,stage/top,dirs_exist_ok=True)
for p in ['Cargo.toml','Cargo.lock','clippy.toml','rustfmt.toml','rust-toolchain.toml','.cargo/config.toml']:
 dst=stage/p;dst.parent.mkdir(parents=True,exist_ok=True);shutil.copyfile(base/p,dst)
original={str(p.relative_to(stage)):p.read_text() for top in ['01_core','02_shell','03_infra','04_wiring'] for p in (stage/top).rglob('*.rs')}
core='01_core/src/'
# Target function and occurrence locate the current source after shifts; Rust primary diagnostic must be at its actual match.
targets={
 'V16-048':(core+'compiler/parse/math.rs','math_op',0,0),
 'V16-071':(core+'entities/ast/expr.rs','from_kind',0,0),
 'V16-072':(core+'entities/ast/expr.rs','from_kind',1,0),
 'V16-073-Pattern':(core+'entities/ast/expr.rs','bindings',0,0),
 'V16-073-Expr':(core+'entities/ast/expr.rs','bindings',0,1 if phase=='green' else 0),
 'V16-078':(core+'entities/font_variations.rs','from_validated_dict',0,0),
 'V16-079':(core+'entities/gradient/radial.rs','sample_precise',0,0),
 'V16-080':(core+'entities/operators.rs','from_kind',0,0),
 'V16-081':(core+'entities/operators.rs','from_kind',1,0),
 'V16-082':(core+'entities/syntax_node/representation.rs','spanless_eq',0,0),
 'V16-086':('03_infra/src/export/builder/metadata.rs','collect_top',0,1),
 'V16-087':('03_infra/src/export/gradients/adaptive.rs','sample_gradient_precise_for_svg',0,'hue'),
 'V16-090':('03_infra/src/fonts/coverage.rs','collect',0,0),
 'V16-104':('03_infra/tests/armed_watch_contract.rs','signature_mentions_id',0,0),
 'V16-105':('03_infra/tests/watch_snapshot_contract.rs','signature_mentions_id',0,0)}

def enum_probe(path,decl):
 p=stage/path;t=p.read_text();start=t.index(decl);end=t.index('\n}',start)
 p.write_text(t[:end]+'\n    TektV16EvolutionProbe,'+t[end:])

def omit(path,pattern):
 p=stage/path;t=p.read_text();t,n=re.subn(pattern,'',t,count=1,flags=re.M);assert n==1,(path,pattern);p.write_text(t)

def locate(key):
 path,fn,nth,which=targets[key];t=(stage/path).read_text()
 functions=list(re.finditer(r'\bfn '+fn+r'\s*(?:<[^\n]*?>)?\s*\(',t));assert len(functions)>nth,(key,functions)
 start=functions[nth].start();nxt=re.search(r'\n\s*(?:pub(?:\([^)]*\))?\s+)?(?:const\s+)?fn\s+',t[functions[nth].end():]);end=functions[nth].end()+nxt.start() if nxt else len(t)
 matches=list(re.finditer(r'\bmatch\s+',t[start:end]));
 if key=='V16-073-Expr' and phase=='green':
  # Bind the diagnostic to the nested Expr dispatch, not a numbered match.
  anchor=re.search(r'Self::Normal\((\w+)\)\s*=>\s*(?:\{\s*)?match\s+\1\s*\{',t[start:end])
  assert anchor,(key,'missing direct Normal payload dispatch')
  pos=start+anchor.start()+anchor.group().index('match ')
 elif which=='hue':
  anchor=t.index('if let Some(hue_index)',start);pos=t.index('match ',anchor)
 else:
  assert len(matches)>which,(key,matches);pos=start+matches[which].start()
 return {'id':key,'file':path,'function':fn,'match_line':t[:pos].count('\n')+1,'match_column':pos-t.rfind('\n',0,pos),'source_anchor':t[pos:t.index('{',pos)+1]}

sets=[('core-domains',['V16-048','V16-071','V16-072','V16-078','V16-079','V16-080','V16-081','V16-082'],'core'),('pattern',['V16-073-Pattern'],'core'),('expr',['V16-073-Expr'],'core'),('infra-domain-omissions',['V16-086','V16-087','V16-090'],'infra'),('watch-omissions',['V16-104','V16-105'],'tests')]
records=[]
for name,ids,kind in sets:
 for path,t in original.items():(stage/path).write_text(t)
 if name=='core-domains':
  for path,decl in [('entities/syntax_kind.rs','pub enum SyntaxKind {'),('entities/value.rs','pub enum Value {'),('entities/color.rs','pub enum ColorSpace {'),('entities/syntax_node/representation.rs','enum NodeKind {')]:enum_probe(core+path,decl)
 elif name in ['pattern','expr']:enum_probe(core+'entities/ast/expr.rs',"pub enum "+('Pattern' if name=='pattern' else 'Expr')+"<'a> {")
 elif kind=='infra':
  omit(targets['V16-086'][0],r'^\s*typst_core::entities::layout_types::SemanticKind::ParbreakBoundary => \{\n[\s\S]*?^\s*return;\n\s*\}\n')
  omit(targets['V16-087'][0],r'^\s*ColorSpace::Oklch => Some\(2\),\n')
  omit(targets['V16-090'][0],r'^\s*Format::SegmentMappingToDeltaValues\(_\) => Some\(4\),\n')
 else:
  for key in ids:omit(targets[key][0],r'^\s*Value::Array\(values\) => \{\n\s*values\.iter\(\)\.any\(\|value\| signature_mentions_id\(value, target\)\)\n\s*\}\n')
 locations=[locate(key) for key in ids]
 changes={p:{'original_sha256':hashlib.sha256(t.encode()).hexdigest(),'mutated':(stage/p).read_text()} for p,t in original.items() if t!=(stage/p).read_text()}
 cmd=['cargo','check','--offline','--locked','-p','typst-core' if kind=='core' else 'typst-infra','--message-format=json']
 cmd+=['--test','armed_watch_contract','--test','watch_snapshot_contract','--keep-going'] if kind=='tests' else ['--lib']
 env=dict(os.environ,CARGO_BUILD_JOBS='1',CARGO_PROFILE_DEV_DEBUG='0',CARGO_PROFILE_TEST_DEBUG='0',CARGO_INCREMENTAL='0',CARGO_TARGET_DIR=str(root/'lab/target/product'))
 rec={'name':name,'phase':phase,'utc_start':now(),'head':subprocess.check_output(['git','rev-parse','HEAD'],text=True).strip(),'cwd':str(stage),'command':cmd,'source_base':str(base),'targets':locations,'changed_paths':sorted(changes),'source_hashes':{p:hashlib.sha256(t.encode()).hexdigest() for p,t in original.items()},'real_lock_sha256':sha(stage/'Cargo.lock')}
 with gzip.open(out/(name+'-inputs.json.gz'),'wt') as f:json.dump({'record':rec,'mutations':changes},f)
 print(now(),name,'START',flush=True)
 with (out/(name+'.log')).open('w') as f:r=subprocess.run(cmd,cwd=stage,env=env,stdout=f,stderr=subprocess.STDOUT)
 errors=[]
 for line in (out/(name+'.log')).read_text().splitlines():
  try:item=json.loads(line)
  except ValueError:continue
  if item.get('reason')=='compiler-message' and item['message']['level']=='error':errors.append(item['message'])
 for target in locations:
  target['diagnostics']=[e for e in errors if (e.get('code') or {}).get('code')=='E0004' and any(s['is_primary'] and s['file_name'].removeprefix(str(stage)+'/')==target['file'] and s['line_start']==target['match_line'] and s['column_start']>=target['match_column'] for s in e['spans'])]
  target['attributed_e0004']=bool(target['diagnostics'])
 rec.update(utc_end=now(),exit=r.returncode,selected_e0004=sum(t['attributed_e0004'] for t in locations),other_errors=[e for e in errors if (e.get('code')or{}).get('code')!='E0004'],error_count=len(errors))
 records.append(rec);(out/'results.json').write_text(json.dumps(records,indent=2)+'\n')
 print(now(),name,'END',r.returncode,'selected',rec['selected_e0004'],'other',len(rec['other_errors']),flush=True)
 assert not rec['other_errors'],name
 assert rec['selected_e0004']==(0 if phase=='baseline' else len(locations)),locations
 assert r.returncode==(0 if phase=='baseline' and kind!='core' else 101),name
assert all(sha(base/p)==h for p,h in source_freeze.items()),'productive source changed during structural proof'
print('PASS',phase,'15 obligations across14 V16 occurrences',flush=True)
