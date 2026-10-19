from pathlib import Path
import json,gzip,hashlib,subprocess,shutil,tempfile,os,sys,datetime,difflib
root=Path.cwd();out=Path(__file__).resolve().parents[1];phase=sys.argv[1];assert phase in ['baseline','green'];b=json.load(gzip.open(out/'before.json.gz'));scope=json.load(open(out/'scope.json'));sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest();hashes={p:sha(p) for p in b['hashes']};source=scope['sources'][0];original=b['files'][source];current=Path(source).read_text();mapping={i+1:j+1 for block in difflib.SequenceMatcher(None,original.splitlines(),current.splitlines(),autojunk=False).get_matching_blocks() for i,j in zip(range(block.a,block.a+block.size),range(block.b,block.b+block.size))};targets=[{'id':r['id'],'enum':r['enum'],'source':source,'line':mapping[r['match_line']]} for r in scope['targets']]
record={'phase':phase,'start':datetime.datetime.now(datetime.timezone.utc).isoformat(),'head':b['head'],'source_hashes':hashes,'targets':targets}
with tempfile.TemporaryDirectory(prefix='tekt-eval-helper-probe-') as directory:
 stage=Path(directory)
 for top in ['01_core','02_shell','03_infra','04_wiring','benches']:shutil.copytree(root/top,stage/top,ignore=shutil.ignore_patterns('target','.git','__pycache__'))
 for name in ['Cargo.toml','Cargo.lock','clippy.toml','rustfmt.toml','rust-toolchain.toml','.cargo/config.toml']:
  dst=stage/name;dst.parent.mkdir(parents=True,exist_ok=True);shutil.copyfile(root/name,dst)
 mutated={}
 for enum,file in [('Content','01_core/src/entities/content.rs'),('Value','01_core/src/entities/value.rs')]:
  p=stage/file;s=p.read_text();needle='pub enum '+enum+' {';assert s.count(needle)==1;p.write_text(s.replace(needle,needle+'\n    TektEvalHelpersProbe,',1));mutated[file]=p.read_text()
 cmd=['cargo','test','--offline','--locked','-p','typst-core','--lib','--no-run','--message-format=json'];record.update(command=cmd,workspace=str(stage));env=dict(os.environ,CARGO_TARGET_DIR=str(root/'target'),CARGO_BUILD_JOBS='1')
 with gzip.open(out/('probe-'+phase+'-input.json.gz'),'wt') as f:json.dump({'record':record,'mutated':mutated,'test_consumer':current},f)
 with (out/('probe-'+phase+'.log')).open('w') as f:r=subprocess.run(cmd,cwd=stage,env=env,stdout=f,stderr=subprocess.STDOUT)
 errors=[]
 for line in (out/('probe-'+phase+'.log')).read_text().splitlines():
  try:x=json.loads(line)
  except ValueError:continue
  if x.get('reason')=='compiler-message' and x['message']['level']=='error':errors.append(x['message'])
 for t in targets:t['e0004']=any((e.get('code') or {}).get('code')=='E0004' and any(s['is_primary'] and s['file_name'].removeprefix(str(stage)+'/')==t['source'] and s['line_start']==t['line'] for s in e['spans']) for e in errors)
 record.update(exit=r.returncode,total_e0004=sum((e.get('code') or {}).get('code')=='E0004' for e in errors),selected=sum(t['e0004'] for t in targets),other_errors=[e for e in errors if (e.get('code') or {}).get('code')!='E0004'],sources_unchanged=all(sha(p)==h for p,h in hashes.items()),end=datetime.datetime.now(datetime.timezone.utc).isoformat())
 (out/('probe-'+phase+'-result.json')).write_text(json.dumps(record,indent=2)+'\n');print(json.dumps({k:record[k] for k in ['phase','exit','total_e0004','selected','sources_unchanged']}),flush=True);assert r.returncode!=0 and not record['other_errors'] and record['sources_unchanged'];assert record['selected']==(0 if phase=='baseline' else 11),targets
