from pathlib import Path
import shutil,tempfile,subprocess,sys,re,hashlib,json,datetime
root=Path.cwd();sources=[Path(x) for x in sys.argv[1:]]
allowed=set(sources)
for source in sources:
 owner=re.search(r'^//! @prompt (.+)$',(root/source).read_text(),re.M).group(1)
 allowed.add(Path(owner))
with tempfile.TemporaryDirectory(prefix='tekt-owned-seal-') as directory:
 stage=Path(directory)
 for top in ['01_core','02_shell','03_infra','04_wiring','benches','00_nucleo/prompts']:
  for p in (root/top).rglob('*'):
   if p.is_file() and p.suffix in ['.rs','.md','.toml']:
    dst=stage/p.relative_to(root);dst.parent.mkdir(parents=True,exist_ok=True);shutil.copyfile(p,dst)
 for p in ['Cargo.toml','crystalline.toml']:
  shutil.copyfile(root/p,stage/p)
 for source in sources:
  p=stage/source;t=p.read_text();t,n=re.subn(r'^//! @prompt-hash [0-9a-f]{8}$','//! @prompt-hash 00000000',t,count=1,flags=re.M);assert n==1,source;p.write_text(t)
 r=subprocess.run(['/tmp/tekt-linter-ci-target/debug/crystalline-lint',str(stage),'--checks','v1,v5,v15,v26','--fix-hashes'],capture_output=True,text=True)
 print(r.stdout);print(r.stderr,file=sys.stderr)
 if r.returncode:sys.exit(r.returncode)
 changes=[]
 for p in stage.rglob('*'):
  if not p.is_file():continue
  rel=p.relative_to(stage);old=(root/rel).read_bytes();new=p.read_bytes()
  if old==new:continue
  assert rel in allowed,rel
  if rel.suffix=='.rs':
   norm=lambda x:re.sub(rb'^//! @prompt-hash [^\n]*\n',b'',x,count=1,flags=re.M)
  else:
   norm=lambda x:re.sub(rb'^Hash do C\xc3\xb3digo: [^\n]*\n',b'',x,count=1,flags=re.M)
  assert norm(old)==norm(new),rel
  changes.append((rel,old,new))
 for rel,old,new in changes:
  assert (root/rel).read_bytes()==old,rel
  (root/rel).write_bytes(new)
 print(json.dumps({'utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'sealed':[str(c[0]) for c in changes]}))
