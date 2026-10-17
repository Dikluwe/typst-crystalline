from pathlib import Path
import json,gzip,hashlib,subprocess,shutil,tempfile,os,sys,datetime
root=Path.cwd();out=Path(__file__).resolve().parents[1];phase=sys.argv[1];assert phase in ['baseline','green'];b=json.load(gzip.open(out/'before.json.gz'));scope=json.load(open(out/'scope.json'));sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest();hashes={p:sha(p) for p in b['hashes']};source=scope['sources'][0]
recipe=json.load(open(out/'audit-recipe.json'));cmd=recipe['command'][:-3]+[str(out/'workers/latent.rs'),'-o','/tmp/tekt-eval-math-latent'];subprocess.run(cmd,check=True)
with tempfile.TemporaryDirectory(prefix='tekt-match-lines-') as d:
 p=Path(d)/'original.rs';p.write_text(b['files'][source]);before_matches=json.loads(subprocess.check_output(['/tmp/tekt-eval-math-latent',str(p),'matches'],text=True));after_matches=json.loads(subprocess.check_output(['/tmp/tekt-eval-math-latent',source,'matches'],text=True));auxiliary=[x for x in after_matches if x["scrutinee"]=="prime_content"];filtered_matches=[x for x in after_matches if x["scrutinee"]!="prime_content"];assert len(before_matches)==len(filtered_matches);assert len(auxiliary)==(0 if phase=="baseline" else 1)
 targets=[]
 for r in scope['targets']:
  indices=[i for i,x in enumerate(before_matches) if x['line']==r['match_line'] and x['end_line']==r['match_end_line']];assert len(indices)==1,(r['id'],indices);idx=indices[0];targets.append({'id':r['id'],'enum':r['enum'],'source':source,'line':filtered_matches[idx]['line'],'match_index':idx})
if auxiliary:targets.append({'id':'prime-content-auxiliary','enum':'Content','source':source,'line':auxiliary[0]['line'],'match_index':'new auxiliary'})
record={'phase':phase,'start':datetime.datetime.now(datetime.timezone.utc).isoformat(),'head':b['head'],'source_hashes':hashes,'targets':targets,'AST_match_count':len(before_matches)}
with tempfile.TemporaryDirectory(prefix='tekt-eval-math-probe-') as directory:
 stage=Path(directory)
 for top in ['01_core','02_shell','03_infra','04_wiring','benches']:shutil.copytree(root/top,stage/top,ignore=shutil.ignore_patterns('target','.git','__pycache__'))
 for name in ['Cargo.toml','Cargo.lock','clippy.toml','rustfmt.toml','rust-toolchain.toml','.cargo/config.toml']:
  dst=stage/name;dst.parent.mkdir(parents=True,exist_ok=True);shutil.copyfile(root/name,dst)
 mutated={}
 for enum,file in [('Content','01_core/src/entities/content.rs'),('MathAttachSlot','01_core/src/entities/elements/math_attach.rs')]:
  p=stage/file;s=p.read_text();needle='pub enum '+enum+' {';assert s.count(needle)==1;p.write_text(s.replace(needle,needle+'\n    TektEvalMathProbe,',1));mutated[file]=p.read_text()
 cmd=['cargo','test','--offline','--locked','-p','typst-core','--lib','--no-run','--message-format=json'];record.update(command=cmd,workspace=str(stage));env=dict(os.environ,CARGO_TARGET_DIR=str(root/'target'),CARGO_BUILD_JOBS='1')
 with gzip.open(out/('probe-'+phase+'-input.json.gz'),'wt') as f:json.dump({'record':record,'mutated':mutated,'test_consumer':Path(source).read_text(),'before_matches':before_matches,'after_matches':after_matches},f)
 with (out/('probe-'+phase+'.log')).open('w') as f:r=subprocess.run(cmd,cwd=stage,env=env,stdout=f,stderr=subprocess.STDOUT)
 errors=[]
 for line in (out/('probe-'+phase+'.log')).read_text().splitlines():
  try:x=json.loads(line)
  except ValueError:continue
  if x.get('reason')=='compiler-message' and x['message']['level']=='error':errors.append(x['message'])
 for t in targets:t['e0004']=any((e.get('code') or {}).get('code')=='E0004' and any(s['is_primary'] and s['file_name'].removeprefix(str(stage)+'/')==t['source'] and s['line_start']==t['line'] for s in e['spans']) for e in errors)
 record.update(exit=r.returncode,total_e0004=sum((e.get('code') or {}).get('code')=='E0004' for e in errors),selected=sum(t['e0004'] for t in targets),other_errors=[e for e in errors if (e.get('code') or {}).get('code')!='E0004'],sources_unchanged=all(sha(p)==h for p,h in hashes.items()),end=datetime.datetime.now(datetime.timezone.utc).isoformat())
 (out/('probe-'+phase+'-result.json')).write_text(json.dumps(record,indent=2)+'\n');print(json.dumps({k:record[k] for k in ['phase','exit','total_e0004','selected','sources_unchanged']}),flush=True);assert r.returncode!=0 and not record['other_errors'] and record['sources_unchanged'];assert record['selected']==(0 if phase=='baseline' else 71),targets
