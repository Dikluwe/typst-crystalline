"""Real-workspace enum-extension and cross-crate positive-arm omission witnesses."""
from pathlib import Path
import datetime,gzip,hashlib,json,os,re,shutil,subprocess,sys
ROOT=Path.cwd(); OUT=ROOT/'lab/results/v16-corrections-2026-10-19/batch-e'; PHASE=sys.argv[1]
assert PHASE in ('baseline','green')
BASE=ROOT/'lab/scratch/v16-batch-e-base-2026-10-19' if PHASE=='baseline' else ROOT
SUFFIX=sys.argv[2] if len(sys.argv)>2 else PHASE
assert SUFFIX in ('baseline','green','query-green')
STAGE=ROOT/('lab/scratch/v16-batch-e-structural-'+SUFFIX); RESULT=OUT/('structural-'+SUFFIX);RESULT.mkdir(parents=True,exist_ok=False)
now=lambda:datetime.datetime.now(datetime.timezone.utc).isoformat()
sha=lambda p:hashlib.sha256(p.read_bytes()).hexdigest()
def gz(p,x):
 with gzip.open(p,'wt') as f:json.dump(x,f)
scope=json.loads((OUT/'scope.json').read_text()); frozen={p:sha(BASE/p) for p in scope['sources']+scope['owners']}
for top in ['01_core','02_shell','03_infra','04_wiring','lab/benches','lab/corpus','lab/parity/corpus','lab/typst-original/tests']:shutil.copytree(BASE/top,STAGE/top,dirs_exist_ok=True)
for path in ['Cargo.toml','Cargo.lock','clippy.toml','rustfmt.toml','rust-toolchain.toml','.cargo/config.toml']:
 dst=STAGE/path;dst.parent.mkdir(parents=True,exist_ok=True);shutil.copyfile(BASE/path,dst)
original={str(p.relative_to(STAGE)):p.read_text() for top in ['01_core','02_shell','03_infra','04_wiring'] for p in (STAGE/top).rglob('*.rs')}
gz(RESULT/'original-sources.json.gz',original);gz(RESULT/'inputs.json.gz',{'utc':now(),'base':str(BASE),'inputs':{str(p.relative_to(STAGE)):sha(p) for p in STAGE.rglob('*') if p.is_file()},'frozen_owners':frozen})
def locate(t):
 text=(STAGE/t['file']).read_text();spec=t.get(PHASE,t); start=text.index(spec['after']) if spec.get('after') else 0;pos=text.index(spec['anchor'],start)
 return dict(t,line=text[:pos].count('\n')+1,byte_start=len(text[:pos].encode()),byte_end=len(text[:pos+len(spec['anchor'])].encode()))
targets=json.loads((OUT/'structural-targets.json').read_text()); records=[]
for name,package in [('core-extension','typst-core'),('shell-omissions','typst-shell')]:
 for p,text in original.items():(STAGE/p).write_text(text)
 selected=[t for t in targets if t['group']==name and (PHASE in t or 'anchor' in t)]
 if name=='core-extension':
  p=STAGE/'01_core/src/entities/content.rs';text=p.read_text();pos=text.index('\n}',text.index('pub enum Content {'));p.write_text(text[:pos]+'\n    TektV16EvolutionProbe,'+text[pos:])
 else:
  for t in selected:
   p=STAGE/t['file'];text=p.read_text();assert text.count(t['remove_exact'])==1,t;p.write_text(text.replace(t['remove_exact'],''))
 selected=[locate(t) for t in selected]
 changes={p:(STAGE/p).read_text() for p,text in original.items() if text!=(STAGE/p).read_text()};gz(RESULT/(name+'-mutations.json.gz'),changes)
 cmd=['cargo','test','--offline','--locked','-p',package,'--lib','--no-run','--message-format=json'];env=dict(os.environ,CARGO_BUILD_JOBS='1',CARGO_PROFILE_DEV_DEBUG='0',CARGO_PROFILE_TEST_DEBUG='0',CARGO_INCREMENTAL='0',CARGO_TARGET_DIR=str(ROOT/'lab/target/product'))
 rec={'name':name,'phase':PHASE,'utc_start':now(),'head':subprocess.check_output(['git','rev-parse','HEAD'],text=True).strip(),'cwd':str(STAGE),'command':cmd,'targets':selected,'mutated_paths':sorted(changes),'environment':{k:env[k] for k in ['CARGO_BUILD_JOBS','CARGO_PROFILE_DEV_DEBUG','CARGO_PROFILE_TEST_DEBUG','CARGO_INCREMENTAL','CARGO_TARGET_DIR']}}
 print(now(),name,'START',flush=True)
 with (RESULT/(name+'.log')).open('w') as f:r=subprocess.run(cmd,cwd=STAGE,env=env,stdout=f,stderr=subprocess.STDOUT)
 errors=[]
 for line in (RESULT/(name+'.log')).read_text().splitlines():
  try:item=json.loads(line)
  except ValueError:continue
  if item.get('reason')=='compiler-message' and item['message']['level']=='error':errors.append(item['message'])
 for t in selected:t['diagnostics']=[e for e in errors if (e.get('code') or {}).get('code')=='E0004' and any(s['is_primary'] and s['file_name'].removeprefix(str(STAGE)+'/')==t['file'] and s['line_start']==t['line'] and t['byte_start']<=s['byte_start']<t['byte_end'] for s in e['spans'])]
 rec.update(utc_end=now(),exit=r.returncode,selected_e0004=sum(bool(t['diagnostics']) for t in selected),other_errors=[e for e in errors if (e.get('code') or {}).get('code')!='E0004'],error_count=len(errors));records.append(rec);(RESULT/'results.json').write_text(json.dumps(records,indent=2)+'\n')
 print(now(),name,'END',r.returncode,'selected',rec['selected_e0004'],'other',len(rec['other_errors']),flush=True)
 assert not rec['other_errors'],rec['other_errors'];assert rec['selected_e0004']==(0 if PHASE=='baseline' else len(selected)),selected
 assert r.returncode==(0 if PHASE=='baseline' and name=='shell-omissions' else 101)
changed=[p for p,value in frozen.items() if sha(BASE/p)!=value];(RESULT/'freeze.json').write_text(json.dumps({'utc':now(),'changed':changed},indent=2)+'\n');assert not changed
print('PASS',PHASE,flush=True)
