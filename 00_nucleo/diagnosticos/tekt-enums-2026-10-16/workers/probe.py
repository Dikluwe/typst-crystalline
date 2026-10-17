from pathlib import Path
import json,gzip,hashlib,subprocess,tempfile,shutil,os,sys,re,datetime
root=Path.cwd();out=root/'00_nucleo/diagnosticos/tekt-enums-2026-10-16';phase=sys.argv[1];assert phase in ['baseline','green'];now=lambda:datetime.datetime.now(datetime.timezone.utc).isoformat();sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest();scope=json.loads((out/'scope.json').read_text())
targets=[
 ('color','eq','impl PartialEq for Color',r'match \(self, other\)'),
 ('color/adjustments','darken','pub fn darken',r'match self\s*\{'),
 ('color/adjustments','rotate','pub fn rotate',r'match self\.to_space\(space\)\.ok\(\)\?\s*\{'),
 ('color/adjustments','saturate','pub fn saturate',r'match self\s*\{'),
 ('color/conversion','to_oklab','pub(super) fn to_oklab_p476',r'match c\s*\{'),
 ('color/conversion','to_oklch','pub(super) fn to_oklch_p476',r'match c\s*\{'),
 ('color/conversion','to_space_spot','pub fn to_space',r'match self\s*\{'),
 ('color/conversion','to_process','pub fn to_process',r'match self\s*\{'),
 ('color/mixing','mix_default','pub fn mix(',r'match \(&self, &other\)'),
 ('color/mixing','mix_hue','pub fn mix(',r'match &space\s*\{'),
 ('color/mixing','weighted_default','pub(crate) fn mix_weighted',r'match &colors\[0\]\.0\s*\{'),
 ('color/mixing','weighted_hue','pub(crate) fn mix_weighted',r'match &space\s*\{'),
 ('math_style','map_glyph_vs','pub fn map_glyph_vs',r'match kind\s*\{'),
 ('math_style','letter_base','fn letter_base',r'match \(kind, bold, italic\)'),
 ('math_style','digit_base','fn digit_base',r'match \(kind, bold\)'),
 ('math_style','bmp_exception','fn bmp_exception',r'match \(c, kind, bold, italic\)|match \(kind, bold, italic, c\)'),
 ('func','element_name','pub fn element_name',r'match self\.0\.as_ref\(\)'),
 ('func','namespace','pub fn namespace',r'match self\.0\.as_ref\(\)'),
 ('func','eq','impl PartialEq for Func',r'match \(self\.0\.as_ref\(\), other\.0\.as_ref\(\)\)'),
 ('func','hash','impl Hash for Func',r'match self\.0\.as_ref\(\)'),
 ('state_update','eq','impl PartialEq for StateUpdate',r'match \(self, other\)'),
]
files={p:sha(p) for p in json.load(gzip.open(out/'before.json.gz'))['hashes']};record=dict(start=now(),phase=phase,head=subprocess.check_output(['git','rev-parse','HEAD'],text=True).strip(),source_hashes=files,command=['cargo','check','--offline','--locked','-p','typst-core','--message-format=json'],targets=[])
with tempfile.TemporaryDirectory(prefix='tekt-v16-enums-probe-') as directory:
 stage=Path(directory)
 for top in ['01_core','02_shell','03_infra','04_wiring','benches']:
  shutil.copytree(root/top,stage/top,ignore=shutil.ignore_patterns('target','.git','__pycache__'))
 for p in ['Cargo.toml','Cargo.lock','clippy.toml','rustfmt.toml','rust-toolchain.toml','.cargo/config.toml']:
  if (root/p).is_file():dest=stage/p;dest.parent.mkdir(exist_ok=True,parents=True);shutil.copyfile(root/p,dest)
 enums=[('color','pub enum Color {'),('color','pub enum ColorSpace {'),('math_style','pub enum MathStyleKind {'),('func','pub(crate) enum FuncRepr {'),('state_update','pub enum StateUpdate {')]
 for module,anchor in enums:
  p=stage/f'01_core/src/entities/{module}.rs';t=p.read_text();assert t.count(anchor)==1;p.write_text(t.replace(anchor,anchor+'\n    TektV16Probe,'))
 inputs={}
 for module,name,fn_anchor,pattern in targets:
  path=f'01_core/src/entities/{module}.rs';t=(stage/path).read_text();a=t.index(fn_anchor);m=re.search(pattern,t[a:]);assert m,(path,name);line=t[:a+m.start()].count('\n')+1;record['targets'].append(dict(source=path,name=name,line=line));inputs[path]=t
 record['temporary_workspace']=str(stage);record['environment']={'CARGO_TARGET_DIR':str(root/'target'),'CARGO_BUILD_JOBS':'1','TEKT_V16_MEMORY_LIMIT':os.environ.get('TEKT_V16_MEMORY_LIMIT')}
 with gzip.open(out/f'probe-{phase}-input.json.gz','wt') as f:json.dump(dict(record=record,mutated_sources=inputs),f)
 env=dict(os.environ);env.update(CARGO_TARGET_DIR=str(root/'target'),CARGO_BUILD_JOBS='1')
 with (out/f'probe-{phase}.log').open('w') as f:r=subprocess.run(record['command'],cwd=stage,env=env,stdout=f,stderr=subprocess.STDOUT)
 errors=[]
 for line in (out/f'probe-{phase}.log').read_text().splitlines():
  try:x=json.loads(line)
  except ValueError:continue
  if x.get('reason')=='compiler-message' and x['message']['level']=='error':errors.append(x['message'])
 primary=set()
 for m in errors:
  if (m.get('code') or {}).get('code')!='E0004':continue
  for s in m['spans']:
   if not s['is_primary']:continue
   path=s['file_name'];path=path.removeprefix(str(stage)+'/');primary.add((path,s['line_start']))
 for t in record['targets']:t['e0004']=(t['source'],t['line']) in primary
 record.update(end=now(),exit=r.returncode,selected_e0004=sum(t['e0004'] for t in record['targets']),total_e0004=sum((e.get('code') or {}).get('code')=='E0004' for e in errors),other_errors=[e for e in errors if (e.get('code') or {}).get('code')!='E0004'],sources_unchanged=all(sha(p)==h for p,h in files.items()))
 (out/f'probe-{phase}-details.json').write_text(json.dumps(record,ensure_ascii=False,indent=2)+'\n');print(json.dumps({k:record[k] for k in ['phase','exit','selected_e0004','total_e0004','sources_unchanged']}),flush=True)
 assert r.returncode!=0 and record['sources_unchanged'] and not record['other_errors'],record.get('other_errors')
 expected=0 if phase=='baseline' else len(targets);assert record['selected_e0004']==expected,record['targets']
