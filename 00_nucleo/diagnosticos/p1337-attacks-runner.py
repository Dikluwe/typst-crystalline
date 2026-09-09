"""Isolated P1337 mutations, immutable per-round evidence, no main-tree writes."""
import argparse, datetime, difflib, hashlib, json, os, re, shutil, subprocess, tempfile, time
from pathlib import Path
ROOT=Path('/repos/Antigravity/typst-crystalline')
DIAG=ROOT/'00_nucleo/diagnosticos'
SOURCE=Path('01_core/src/compiler/eval/bindings/field_access.rs')
CONFIG='profile.release.package.typst-core.opt-level=0'
def sha(data): return hashlib.sha256(data).hexdigest()
def utc(): return datetime.datetime.now(datetime.timezone.utc).isoformat()
def publish(path, body):
 assert not path.exists(), path
 if not isinstance(body,str): body=json.dumps(body,ensure_ascii=False,indent=2)+'\n'
 patch='*** Begin Patch\n*** Add File: '+str(path)+'\n'+''.join('+'+s+'\n' for s in body.splitlines())+'*** End Patch\n'
 p=subprocess.run(['apply_patch'],input=patch,text=True,capture_output=True)
 assert p.returncode==0,p.stderr
 assert path.read_text()==body
def replace_file(path, body):
 old=path.read_text()
 if old==body: return ''
 diff=list(difflib.unified_diff(old.splitlines(True),body.splitlines(True)))
 hunks='\n'.join('@@' if s.startswith('@@') else s for s in ''.join(diff[2:]).splitlines())+'\n'
 patch='*** Begin Patch\n*** Update File: '+str(path)+'\n'+hunks+'*** End Patch\n'
 p=subprocess.run(['apply_patch'],input=patch,text=True,capture_output=True)
 assert p.returncode==0,p.stderr
 assert path.read_text()==body
 return patch
def tests(text):
 start=text.index('#[cfg(test)]\nmod p1337_tests {')
 end=text.find('#[cfg(test)]',start+15)
 return text[start:] if end<0 else text[start:end]
def prepare(manifest,plan):
 m=json.loads(manifest.read_text()); candidate=(ROOT/SOURCE).read_text()
 base=Path(tempfile.mkdtemp(prefix='p1337-attacks-')); workspace=base/'workspace'; workspace.mkdir()
 copied=['Cargo.toml','Cargo.lock','.cargo','01_core','02_shell','03_infra','04_wiring','benches']
 for name in copied:
  src,dst=ROOT/name,workspace/name
  if src.is_dir(): shutil.copytree(src,dst)
  else: shutil.copy2(src,dst)
 assert (workspace/SOURCE).stat().st_ino!=(ROOT/SOURCE).stat().st_ino
 marker='        other @ ('
 pos=candidate.index(marker,candidate.index('pub(in crate::compiler::eval) fn eval_value_field_access'))
 insert='        Value::Bool(_) => Err(vec![SourceDiagnostic::error(\n            span,\n            "cannot access fields on type bool".to_string(),\n        )]),\n'
 cases=[('C',candidate),('M1',candidate[:pos]+insert+candidate[pos:])]
 for ident,kind in [('M2','Bool(_)'),('M3','None'),('M4','Auto')]:
  needle='            | Value::'+kind+'\n'; assert candidate.count(needle)==1,(ident,candidate.count(needle))
  cases.append((ident,candidate.replace(needle,'',1)))
 needle='            | Value::Str(_)\n'; assert candidate.count(needle)==1
 cases.append(('M5',candidate.replace(needle,needle+'            | Value::Array(_)\n',1)))
 record={'at':utc(),'manifest_path':str(manifest),'manifest_sha256':sha(manifest.read_bytes()),'plan_sha256':sha(plan.read_bytes()),'baseline_sha256':m['baseline_sha256'],'candidate_sha256':sha(candidate.encode()),'tests_sha256':sha(tests(candidate).encode()),'base':str(base),'workspace':str(workspace),'copy_policy':'copy2/copytree distinct inodes; no mutable hardlinks','copied_roots':copied,'profile_config':CONFIG,'mutants':[]}
 for ident,body in cases:
  directory=base/ident; directory.mkdir(); publish(directory/'field_access.rs',body)
  assert tests(body)==tests(candidate)
  patch=''.join(difflib.unified_diff(candidate.splitlines(True),body.splitlines(True),fromfile='C/field_access.rs',tofile=ident+'/field_access.rs'))
  publish(directory/'change.diff',patch)
  record['mutants'].append({'id':ident,'path':str(directory/'field_access.rs'),'source_sha256':sha(body.encode()),'patch_path':str(directory/'change.diff'),'patch_sha256':sha(patch.encode()),'tests_sha256':sha(tests(body).encode())})
 publish(DIAG/'p1337-attacks-prepared.json',record); print(json.dumps({'base':str(base),'tests_sha256':record['tests_sha256'],'candidate_sha256':record['candidate_sha256']}),flush=True)
def run(cache):
 prepared=DIAG/'p1337-attacks-prepared.json'; r=json.loads(prepared.read_text()); base=Path(r['base']); workspace=Path(r['workspace']); target=base/'target'
 assert not target.exists()
 t=time.monotonic(); at=utc(); command=['cp','-a','--reflink=auto',str(Path(cache))+'/.',str(target)]
 p=subprocess.run(command,capture_output=True,text=True,timeout=2700)
 publish(DIAG/'p1337-attacks-cache-copy.json',{'at':at,'seconds':time.monotonic()-t,'argv':command,'exit':p.returncode,'stdout':p.stdout,'stderr':p.stderr,'manifest_sha256':r['manifest_sha256']})
 assert p.returncode==0
 env=os.environ.copy(); env.update({'CARGO_TARGET_DIR':str(target),'CARGO_BUILD_JOBS':'2'})
 provenance=[]
 for command in [['git','rev-parse','HEAD'],['git','diff','HEAD','--stat']]:
  p=subprocess.run(command,cwd=ROOT,capture_output=True,text=True); provenance.append({'argv':command,'exit':p.returncode,'stdout':p.stdout,'stderr':p.stderr})
 runs=[]
 for item in r['mutants']:
  ident=item['id']; output=base/ident; body=Path(item['path']).read_text()
  assert sha(body.encode())==item['source_sha256']; assert sha(tests(body).encode())==r['tests_sha256']
  patch=replace_file(workspace/SOURCE,body); publish(output/'workspace-change.apply_patch',patch)
  old=(workspace/SOURCE).stat().st_mtime_ns; os.utime(workspace/SOURCE,None); fresh=(workspace/SOURCE).stat().st_mtime_ns
  command=['cargo','test','--release','--config',CONFIG,'--locked','--offline','-p','typst-core','--lib','p1337_tests','--','--nocapture']
  at=utc(); t=time.monotonic(); error=None; print(json.dumps({'starting':ident,'at':at}),flush=True)
  with (output/'test.stdout').open('x') as out,(output/'test.stderr').open('x') as err:
   try: code=subprocess.run(command,cwd=workspace,env=env,stdout=out,stderr=err,timeout=2700).returncode
   except (OSError,subprocess.TimeoutExpired) as exc: code=None; error=repr(exc)
  stdout=(output/'test.stdout').read_text(); stderr=(output/'test.stderr').read_text()
  compiled='Compiling typst-core v0.1.0 ('+str(workspace/'01_core')+')' in stderr
  match=re.search(r'Running unittests src/lib.rs \(([^)]+)\)',stderr)
  original=Path(match.group(1)) if match else None
  if original and not original.is_absolute(): original=workspace/original
  binary=output/'test-executable'
  if original and original.exists(): shutil.copy2(original,binary)
  binary_sha=sha(binary.read_bytes()) if binary.exists() else None
  actual=re.search(r'running (\d+) tests',stdout)
  valid=compiled and error is None and 'Finished `release`' in stderr and actual and int(actual.group(1))>0 and binary_sha is not None
  status=('control_passed' if code==0 else 'control_failed') if ident=='C' and valid else ('failed_test_pending_witness_review' if code==101 else 'survived') if valid else 'Unknown'
  record={**item,'at_start':at,'at_end':utc(),'seconds':time.monotonic()-t,'argv':command,'cwd':str(workspace),'env':{'CARGO_TARGET_DIR':str(target),'CARGO_BUILD_JOBS':'2'},'profile_config':CONFIG,'mtime_before_ns':old,'mtime_fresh_ns':fresh,'exit':code,'execution_error':error,'compiled_typst_core':compiled,'valid_execution':bool(valid),'executed_tests':int(actual.group(1)) if actual else None,'original_executable':str(original) if original else None,'preserved_executable':str(binary) if binary.exists() else None,'executable_sha256':binary_sha,'stdout_path':str(output/'test.stdout'),'stderr_path':str(output/'test.stderr'),'stdout_sha256':sha(stdout.encode()),'stderr_sha256':sha(stderr.encode()),'status':status,'manifest_sha256':r['manifest_sha256'],'plan_sha256':r['plan_sha256'],'candidate_sha256':r['candidate_sha256'],'baseline_sha256':r['baseline_sha256'],'tests_sha256':r['tests_sha256'],'runner_sha256':sha(Path(__file__).read_bytes()),'prepared_sha256':sha(prepared.read_bytes()),'provenance':provenance}
  publish(DIAG/('p1337-attacks-run-'+ident+'.json'),record); runs.append(record)
  print(json.dumps({'finished':ident,'status':status,'seconds':record['seconds'],'binary_sha256':binary_sha}),flush=True)
  if not valid or (ident=='C' and code!=0): break
 publish(DIAG/'p1337-attacks-results.json',{'at':utc(),'manifest_sha256':r['manifest_sha256'],'runs':runs,'complete':len(runs)==6,'scope':'instrumental opt-level=0 for typst-core only; not release normal proof; no technical isolation attestation or refinement seal'})
if __name__=='__main__':
 parser=argparse.ArgumentParser(); parser.add_argument('mode',choices=['prepare','run']); parser.add_argument('--manifest',type=Path); parser.add_argument('--plan',type=Path); parser.add_argument('--cache'); args=parser.parse_args()
 if args.mode=='prepare': prepare(args.manifest,args.plan)
 else: run(args.cache)
