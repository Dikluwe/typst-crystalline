from pathlib import Path
import subprocess,shutil,tempfile,os,json,gzip,hashlib,re,sys,datetime
root=Path.cwd();out=root/'00_nucleo/diagnosticos/tekt-v16-introspect-walk-2026-10-16';phase=sys.argv[1];assert phase in ['baseline','green'];before=json.load(gzip.open(out/'before.json.gz'));scope=json.load(open(out/'scope.json'));sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest();hashes={p:sha(p) for p in before['hashes']}
anchors=[('W1','pub(crate) fn reserve_locations','match content {'),('W2','let _ = content.map_content','match node {'),('W3','fn table_root','match c {'),('W4','fn walk_at','match content {'),('W5','let fields = match content','match content {'),('W6','fn old_reserve','match node {')]
text=Path(scope['sources'][0]).read_text();targets=[]
for id,start,needle in anchors:
 at=text.index(start);pos=text.index(needle,at);targets.append({'id':id,'source':scope['sources'][0],'line':text[:pos].count('\n')+1})
record={'utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'head':before['head'],'phase':phase,'targets':targets,'source_hashes':hashes}
with tempfile.TemporaryDirectory(prefix='tekt-walk-probe-') as temp:
 stage=Path(temp)
 for top in ['01_core','02_shell','03_infra','04_wiring','benches']:shutil.copytree(root/top,stage/top,ignore=shutil.ignore_patterns('target','.git','__pycache__'))
 for name in ['Cargo.toml','Cargo.lock','clippy.toml','rustfmt.toml','rust-toolchain.toml','.cargo/config.toml']:
  dst=stage/name;dst.parent.mkdir(parents=True,exist_ok=True);shutil.copyfile(root/name,dst)
 p=stage/'01_core/src/entities/content.rs';s=p.read_text();p.write_text(s.replace('pub enum Content {','pub enum Content {\n    TektWalkProbe,',1));cmd=['cargo','test','--offline','--locked','-p','typst-core','--lib','--no-run','--message-format=json'];env=dict(os.environ,CARGO_TARGET_DIR=str(root/'target'),CARGO_BUILD_JOBS='1');record.update(command=cmd,workspace=str(stage))
 with gzip.open(out/('probe-'+phase+'-input.json.gz'),'wt') as f:json.dump({'record':record,'mutated_content':p.read_text(),'walk':text},f)
 with (out/('probe-'+phase+'.log')).open('w') as log:r=subprocess.run(cmd,cwd=stage,env=env,stdout=log,stderr=subprocess.STDOUT)
 errors=[]
 for line in (out/('probe-'+phase+'.log')).read_text().splitlines():
  try:x=json.loads(line)
  except ValueError:continue
  if x.get('reason')=='compiler-message' and x['message']['level']=='error':errors.append(x['message'])
 for t in targets:t['e0004']=any((e.get('code') or {}).get('code')=='E0004' and any(s['is_primary'] and s['file_name'].removeprefix(str(stage)+'/')==t['source'] and s['line_start']==t['line'] for s in e['spans']) for e in errors)
 aux_source='01_core/src/compiler/eval/call_dispatch/content_methods.rs';aux_text=Path(aux_source).read_text();aux_at=aux_text.find('fn outline_entry_receiver(')
 aux_line=aux_text[:aux_text.index('match content {',aux_at)].count('\n')+1 if aux_at>=0 else None
 auxiliary=any((e.get('code') or {}).get('code')=='E0004' and any(sp['is_primary'] and sp['file_name'].removeprefix(str(stage)+'/')==aux_source and sp['line_start']==aux_line for sp in e['spans']) for e in errors)
 record['auxiliary_receiver_e0004']=auxiliary
 if phase=='green':assert auxiliary
 record.update(exit=r.returncode,total_e0004=sum((e.get('code') or {}).get('code')=='E0004' for e in errors),selected=sum(t['e0004'] for t in targets),other_errors=[e for e in errors if (e.get('code') or {}).get('code')!='E0004'],sources_unchanged=all(sha(p)==h for p,h in hashes.items()))
 (out/('probe-'+phase+'-result.json')).write_text(json.dumps(record,indent=2)+'\n');print(json.dumps({k:record[k] for k in ['phase','exit','total_e0004','selected','sources_unchanged']}),flush=True);assert r.returncode!=0 and not record['other_errors'] and record['sources_unchanged'];assert record['selected']==(0 if phase=='baseline' else 6),targets
