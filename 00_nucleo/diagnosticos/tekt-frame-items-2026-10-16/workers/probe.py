from pathlib import Path
import json,gzip,hashlib,subprocess,tempfile,shutil,os,sys,re,datetime
root=Path.cwd();out=root/'00_nucleo/diagnosticos/tekt-frame-items-2026-10-16';phase=sys.argv[1];assert phase in ['baseline','green'];now=lambda:datetime.datetime.now(datetime.timezone.utc).isoformat();sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest();scope=json.loads((out/'scope.json').read_text())
ids={'F002','F003','F007','F009','F010','F011','F036','F037','F038','F039','F050','F051'}
rows=json.loads((out/'classification.json').read_text());contexts=json.loads((out/'contexts.json').read_text());lookup={(r['source'],r['line'],r['column']):r for r in contexts}
targets=[(r,lookup[(r['source'],r['line'],r['column'])]) for r in rows if r['id'] in ids]
assert len(targets)==12

files={p:sha(p) for p in json.load(gzip.open(out/'before.json.gz'))['hashes']};record=dict(start=now(),phase=phase,head=subprocess.check_output(['git','rev-parse','HEAD'],text=True).strip(),source_hashes=files,command=['cargo','check','--offline','--locked','-p','typst-core','--message-format=json'],targets=[])
with tempfile.TemporaryDirectory(prefix='tekt-frame-probe-') as directory:
 stage=Path(directory)
 for top in ['01_core','02_shell','03_infra','04_wiring','benches']:
  shutil.copytree(root/top,stage/top,ignore=shutil.ignore_patterns('target','.git','__pycache__'))
 for p in ['Cargo.toml','Cargo.lock','clippy.toml','rustfmt.toml','rust-toolchain.toml','.cargo/config.toml']:
  if (root/p).is_file():dest=stage/p;dest.parent.mkdir(exist_ok=True,parents=True);shutil.copyfile(root/p,dest)
 p=stage/'01_core/src/entities/layout_types.rs';t=p.read_text();anchor='pub enum FrameItem {';assert t.count(anchor)==1;p.write_text(t.replace(anchor,anchor+'\n    TektFrameProbe,'))
 inputs={}
 for row,ctx in targets:
  path=row['source'];t=(stage/path).read_text()
  prefix=ctx['match_source'].rsplit(row['arm_source'],1)[0]
  prefix=prefix.split('#[allow(unreachable_patterns)]')[0].rstrip()
  pattern=r'\s+'.join(re.escape(x) for x in prefix.split())
  matches=list(re.finditer(pattern,t));assert len(matches)==1,(row['id'],len(matches))
  line=t[:matches[0].start()].count('\n')+1
  record['targets'].append(dict(id=row['id'],source=path,name=row['function'],line=line));inputs[path]=t
 record['temporary_workspace']=str(stage);record['environment']={'CARGO_TARGET_DIR':str(root/'target'),'CARGO_BUILD_JOBS':'1','TEKT_FRAME_MEMORY_LIMIT':os.environ.get('TEKT_FRAME_MEMORY_LIMIT')}
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
