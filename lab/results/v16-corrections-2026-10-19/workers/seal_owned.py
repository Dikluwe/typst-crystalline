from pathlib import Path
import shutil,tempfile,subprocess,re,json,datetime,gzip,hashlib,sys
root=Path.cwd();out=root/sys.argv[1];scope=json.loads((out/'scope.json').read_text());allowed=set(scope['sources']+scope['owners']);lint=root/'lab/target/linter/debug/crystalline-lint'
r=subprocess.run([str(lint),'.','--checks','v15,v26','--fail-on','warning'],capture_output=True,text=True)
assert r.returncode==0,r.stdout+r.stderr
(out/'preseal-ownership.log').write_text(r.stdout+r.stderr)
with tempfile.TemporaryDirectory(prefix='v16-owned-seal-',dir=root/'lab/scratch') as directory:
 stage=Path(directory)
 for top in ['01_core','02_shell','03_infra','04_wiring','lab/benches','00_nucleo/prompts']:
  for p in (root/top).rglob('*'):
   if p.is_file() and p.suffix in ['.rs','.md','.toml']:
    dst=stage/p.relative_to(root);dst.parent.mkdir(parents=True,exist_ok=True);shutil.copyfile(p,dst)
 for p in ['Cargo.toml','crystalline.toml']:shutil.copyfile(root/p,stage/p)
 for source in scope['sources']:
  p=stage/source;t,n=re.subn(r'^//! @prompt-hash [0-9a-f]{8}$','//! @prompt-hash 00000000',p.read_text(),count=1,flags=re.M);assert n==1;p.write_text(t)
 r=subprocess.run([str(lint),'.','--checks','v1,v5,v15,v26','--fix-hashes'],cwd=stage,capture_output=True,text=True)
 assert r.returncode==0,r.stdout+r.stderr
 changes=[]
 for p in stage.rglob('*'):
  if not p.is_file():continue
  rel=str(p.relative_to(stage));old=(root/rel).read_bytes();new=p.read_bytes()
  if old==new:continue
  assert rel in allowed,rel
  pattern=rb'^//! @prompt-hash [^\n]*\n' if rel.endswith('.rs') else rb'^Hash do C\xc3\xb3digo: [^\n]*\n'
  norm=lambda x:re.sub(pattern,b'',x,count=1,flags=re.M)
  assert norm(old)==norm(new),rel
  changes.append((rel,old,new))
 for rel,old,new in changes:
  assert (root/rel).read_bytes()==old,rel
  (root/rel).write_bytes(new)
 record={'utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'changes':[{'file':rel,'before':hashlib.sha256(old).hexdigest(),'after':hashlib.sha256(new).hexdigest()} for rel,old,new in changes],'linter_output':r.stdout+r.stderr,'ownership_preflight_exit':0}
 records=json.loads((out/'reseal-receipts.json').read_text()) if (out/'reseal-receipts.json').exists() else [];records.append(record)
 (out/'reseal-receipts.json').write_text(json.dumps(records,indent=2)+'\n')
 print('Restricted reseal:',len(changes),'files')
