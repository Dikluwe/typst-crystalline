from pathlib import Path
import json,gzip,hashlib,shutil,tempfile,subprocess,datetime,re,sys,os
root=Path.cwd();out=root/'00_nucleo/diagnosticos/tekt-v16-introspect-origins-2026-10-16';scope=json.load(open(out/'scope.json'));before=json.load(gzip.open(out/'before.json.gz'));phase,enum=sys.argv[1:];assert phase in ['baseline','green'] and enum in ['Value','Content'];targets=[x for x in scope['targets'] if x['enum']==enum]
sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest();now=lambda:datetime.datetime.now(datetime.timezone.utc).isoformat();hashes={p:sha(p) for p in before['hashes']};record={'start':now(),'phase':phase,'enum':enum,'targets':[],'source_hashes':hashes}
with tempfile.TemporaryDirectory(prefix='tekt-origins-probe-') as temp:
 stage=Path(temp)
 for top in ['01_core','02_shell','03_infra','04_wiring','benches']:shutil.copytree(root/top,stage/top,ignore=shutil.ignore_patterns('target','.git','__pycache__'))
 for name in ['Cargo.toml','Cargo.lock','clippy.toml','rustfmt.toml','rust-toolchain.toml','.cargo/config.toml']:
  dest=stage/name;dest.parent.mkdir(parents=True,exist_ok=True);shutil.copyfile(root/name,dest)
 definition=stage/('01_core/src/entities/'+enum.lower()+'.rs');text=definition.read_text();needle='pub enum '+enum+' {';assert text.count(needle)==1
 definition.write_text(text.replace(needle,needle+'\n    TektOrigins'+enum+'Probe,',1));inputs={str(definition.relative_to(stage)):definition.read_text()}
 for target in targets:
  text=(stage/target['source']).read_text();matches=list(re.finditer(r'^\s*(?:pub(?:\([^)]*\))? )?fn '+target['function']+r'\(',text,re.M));assert len(matches)==1,target
  start=matches[0].start();after=re.search(r'^\s*(?:pub(?:\([^)]*\))? )?fn |^#\[cfg\(test\)\]',text[matches[0].end():],re.M);end=matches[0].end()+after.start() if after else len(text)
  record['targets'].append({**target,'start_line':text[:start].count('\n')+1,'end_line':text[:end].count('\n')+1});inputs[target['source']]=text
 cmd=['cargo','check','--offline','--locked','-p','typst-core','--message-format=json'];env=dict(os.environ,CARGO_TARGET_DIR=str(root/'target'),CARGO_BUILD_JOBS='1');record.update(workspace=str(stage),command=cmd,environment={'CARGO_TARGET_DIR':env['CARGO_TARGET_DIR'],'CARGO_BUILD_JOBS':'1'})
 with gzip.open(out/('probe-'+phase+'-'+enum+'-input.json.gz'),'wt') as f:json.dump({'record':record,'mutated_sources':inputs},f)
 with (out/('probe-'+phase+'-'+enum+'.log')).open('w') as log:r=subprocess.run(cmd,cwd=stage,env=env,stdout=log,stderr=subprocess.STDOUT)
 errors=[]
 for line in (out/('probe-'+phase+'-'+enum+'.log')).read_text().splitlines():
  try:item=json.loads(line)
  except ValueError:continue
  if item.get('reason')=='compiler-message' and item['message']['level']=='error':errors.append(item['message'])
 for target in record['targets']:
  target['e0004']=any((error.get('code') or {}).get('code')=='E0004' and any(span['is_primary'] and span['file_name'].removeprefix(str(stage)+'/')==target['source'] and target['start_line']<=span['line_start']<target['end_line'] for span in error['spans']) for error in errors)
 record.update(end=now(),exit=r.returncode,total_e0004=sum((e.get('code') or {}).get('code')=='E0004' for e in errors),selected_e0004=sum(x['e0004'] for x in record['targets']),other_errors=[e for e in errors if (e.get('code') or {}).get('code')!='E0004'],productive_sources_unchanged=all(sha(p)==h for p,h in hashes.items()))
 (out/('probe-'+phase+'-'+enum+'-result.json')).write_text(json.dumps(record,indent=2)+'\n');print(json.dumps({k:record[k] for k in ['phase','enum','exit','total_e0004','selected_e0004','productive_sources_unchanged']}),flush=True)
 assert r.returncode!=0 and not record['other_errors'] and record['productive_sources_unchanged']
 assert record['selected_e0004']==(0 if phase=='baseline' else len(targets)),record['targets']
