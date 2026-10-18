from pathlib import Path
import os,stat,subprocess,json,gzip,hashlib,datetime,shutil,fcntl
root=Path.cwd();out=Path(__file__).resolve().parent
inv=root/'00_nucleo/diagnosticos/inventario-organizacao-2026-10-18'
now=lambda:datetime.datetime.now(datetime.timezone.utc).isoformat()
def git(*args):return subprocess.check_output(['git',*args],text=True)
def gz(name,data):(out/name).write_bytes(gzip.compress(json.dumps(data,ensure_ascii=False,indent=2).encode()))
def sha(path):
 h=hashlib.sha256()
 with path.open('rb') as inp:
  for b in iter(lambda:inp.read(1024*1024),b''):h.update(b)
 return h.hexdigest()
def entries(folder):
 todo=[folder]
 while todo:
  current=todo.pop()
  with os.scandir(current) as iterator:
   for e in iterator:
    q=Path(e.path);s=e.stat(follow_symlinks=False)
    if stat.S_ISDIR(s.st_mode):todo.append(q)
    else:yield q,s

def processes():
 names={'cargo','rustc','rustdoc','typst','crystalline-lint','surface-invento'}
 rows=[]
 for line in subprocess.check_output(['ps','-eo','pid=,comm='],text=True).splitlines():
  a=line.strip().split(None,1)
  if len(a)==2 and a[1] in names:rows.append(dict(pid=int(a[0]),command=a[1]))
 return rows
assert not processes(),processes()
start=now();head=git('rev-parse','HEAD').strip()
tracked=set(git('ls-files','-z').split('\0'))-{''}
untracked=set(git('ls-files','--others','--exclude-standard','-z').split('\0'))-{''}
prod=json.loads(gzip.decompress((inv/'productive-state.json.gz').read_bytes()))
assert all(sha(root/x['path'])==x['sha256'] for x in prod)
state=dict(start=start,head=head,status=git('status','--short'),diffstat=git('diff','HEAD','--stat'),tracked=sorted(tracked),untracked=sorted(untracked),productive_hashes=prod,processes=processes(),disk_free_bytes=shutil.disk_usage(root).free)
gz('before.json.gz',state)
# Independent recovery of the new productive files; no stage/index mutation.
gz('arquivos-novos-protegidos.json.gz',dict(utc=now(),files={x['path']:dict(sha256=x['sha256'],text=(root/x['path']).read_text()) for x in prod if x['git_state']=='untracked'}))
roots=[]
def add(path,group):
 q=root/path
 if q.exists():
  assert q.is_dir() and not q.is_symlink(),path
  assert q.resolve().is_relative_to(root.resolve()),path
  roots.append((q,group))
add('target/debug/incremental','C01')
for f in ['target/doc','target/doc-prefix-nesting-quarantine-20261014']:add(f,'C02')
for base,group in [('target/debug','C03'),('target/release','C05'),('lab/surface-inventory/target/debug','C04'),('lab/surface-inventory/target/release','C04'),('lab/typst-original/target/debug','C06'),('lab/typst-original/target/release','C06')]:
 for name in ['deps','build','.fingerprint','incremental']:
  path=str(Path(base)/name)
  if path!='target/debug/incremental':add(path,group)
# Python cache paths only, without traversing symlinks or tool integrations.
for folder,dirs,files in os.walk(root,followlinks=False):
 rel=Path(folder).relative_to(root)
 dirs[:]=[d for d in dirs if d not in ['.git','.agents','.aws','.codex','retencao'] and not (Path(folder)/d).is_symlink()]
 if '__pycache__' in dirs:
  candidate=Path(folder)/'__pycache__';dirs.remove('__pycache__')
  if not any(candidate.is_relative_to(q) for q,_ in roots):add(candidate.relative_to(root),'C07')
# Acquire existing Cargo profile locks. Never create arbitrary new profile files.
locks=[]
for name in ['target/debug/.cargo-lock','target/release/.cargo-lock','lab/typst-original/target/release/.cargo-lock','lab/surface-inventory/target/release/.cargo-lock']:
 q=root/name
 if q.is_file():
  f=q.open('r+');fcntl.flock(f,fcntl.LOCK_EX|fcntl.LOCK_NB);locks.append(f)
# Ensure no tracked OR nonignored untracked data is inside any deletion root.
for f in tracked|untracked:
 path=root/f
 assert not any(path==q or path.is_relative_to(q) for q,_ in roots),('Git-visible file inside cleanup',f)
plan=[];preserved=[];retained={}
(out/'retencao/objetos').mkdir(parents=True,exist_ok=True)
def retain(path,reason):
 h=sha(path);dest=out/'retencao/objetos'/h
 method='existing'
 if not dest.exists():
  subprocess.run(['cp','--reflink=auto','--preserve=mode,timestamps','--',str(path),str(dest)],check=True);method='independent-copy-or-reflink'
  assert (path.stat().st_dev,path.stat().st_ino)!=(dest.stat().st_dev,dest.stat().st_ino)
 assert sha(dest)==h
 s=path.stat()
 retained[h]=dict(path=str(dest.relative_to(root)),sha256=h,logical_bytes=s.st_size,allocated_bytes=dest.stat().st_blocks*512)
 preserved.append(dict(original=str(path.relative_to(root)),retained=str(dest.relative_to(root)),sha256=h,mode=stat.S_IMODE(s.st_mode),reason=reason,method=method))
# Root CLIs/probes and libraries stay at their original locations; record identity.
anchors={}
for base in ['target/debug','target/release','lab/typst-original/target/release','lab/surface-inventory/target/release']:
 q=root/base
 if q.is_dir():
  for f in q.iterdir():
   if f.is_file() and not f.is_symlink() and (os.access(f,os.X_OK) or f.suffix in ['.rlib','.rmeta']):anchors[str(f.relative_to(root))]=sha(f)
assert anchors.get('lab/typst-original/target/release/typst')=='9b2373155e9c05bbb0daf5755bf4be88a11e64bd6b4f8b376648fa27303373b9'
# Inventory every deletion entry. Retain project libraries, executable ELF files
# (including test/probe binaries) and Rustdoc JSON before deleting build trees.
with gzip.open(out/'removed-files.tsv.gz','wt') as index:
 index.write('path\tgroup\tlogical_bytes\tallocated_bytes\n')
 for q,group in roots:
  count=logical=allocated=0
  for file,s in entries(q):
   count+=1;logical+=s.st_size;allocated+=s.st_blocks*512
   rel=str(file.relative_to(root));index.write(f'{rel}\t{group}\t{s.st_size}\t{s.st_blocks*512}\n')
   if not stat.S_ISREG(s.st_mode):continue
   reason=None
   if group=='C02' and file.suffix=='.json':reason='Rustdoc JSON evidence'
   elif file.name.startswith('libtypst') and file.suffix in ['.rlib','.rmeta']:reason='Typst project library/type metadata'
   elif (s.st_mode & 0o111) and file.suffix not in ['.so','.dylib','.dll']:
    with file.open('rb') as inp:magic=inp.read(4)
    if magic==b'\x7fELF':reason='Executable test/probe/build artifact'
   if reason:retain(file,reason)
  plan.append(dict(path=str(q.relative_to(root)),group=group,files=count,logical_bytes=logical,allocated_bytes=allocated))
gz('plan.json.gz',dict(utc=now(),roots=plan,anchors=anchors,retained=preserved,objects=list(retained.values()),groups='C01-C07: selected ignored cache directories only; no diagnostic outputs/bin or tracked document deleted'))
assert not processes(),processes()
for x in preserved:assert sha(root/x['retained'])==x['sha256']
for rel,h in anchors.items():assert sha(root/rel)==h
# Actual mutation starts only after preservation and the sealed plan exist.
print('Preserved',len(preserved),'artifacts; cleaning',len(plan),'cache directories',flush=True)
removed=[]
for q,group in roots:
 assert not q.is_symlink() and q.resolve().is_relative_to(root.resolve())
 shutil.rmtree(q)
 removed.append(str(q.relative_to(root)))
 print('Removed cache',q.relative_to(root),flush=True)
assert all(not (root/r).exists() for r in removed)
assert all(sha(root/x['path'])==x['sha256'] for x in prod)
assert all(os.path.lexists(root/f) for f in tracked|untracked)
assert all(sha(root/f)==h for f,h in anchors.items())
assert all(sha(root/x['retained'])==x['sha256'] for x in preserved)
# Same executable, small end-to-end smoke after cache removal, no rebuild.
smoke=out/'smoke';smoke.mkdir(exist_ok=True)
(smoke/'input.typ').write_text('CACHE OK\n')
commands=[]
for label,exe in [('tekt',root/'target/debug/typst'),('vanilla',root/'lab/typst-original/target/release/typst')]:
 cmd=[str(exe),'compile',str(smoke/'input.typ'),str(smoke/(label+'.pdf'))]
 z=subprocess.run(cmd,text=True,capture_output=True);assert z.returncode==0,(cmd,z.stderr)
 text=subprocess.check_output(['pdftotext',str(smoke/(label+'.pdf')),'-'],text=True).strip()
 assert text=='CACHE OK',text
 commands.append(dict(label=label,command=cmd,exit=z.returncode,text=text,binary_sha256=sha(exe),output_sha256=sha(smoke/(label+'.pdf'))))
subprocess.run(['git','diff','--check'],check=True)
result=dict(start=start,end=now(),head=head,before='before.json.gz',plan='plan.json.gz',roots_removed=removed,removed_paths_allocated_bytes=sum(x['allocated_bytes'] for x in plan),retained_unique_allocated_bytes=sum(x['allocated_bytes'] for x in retained.values()),retained_count=len(preserved),retained_unique_objects=len(retained),productive_files_unchanged=len(prod),git_visible_paths_preserved=len(tracked|untracked),anchors=anchors,smoke=commands,disk_free_before_bytes=state['disk_free_bytes'],disk_free_after_bytes=shutil.disk_usage(root).free,status=git('status','--short'),diffstat=git('diff','HEAD','--stat'),rebuild_performed=False)
gz('result.json.gz',result)
print(json.dumps({k:result[k] for k in ['roots_removed','removed_paths_allocated_bytes','retained_unique_allocated_bytes','retained_count','retained_unique_objects','productive_files_unchanged','disk_free_before_bytes','disk_free_after_bytes']},ensure_ascii=False),flush=True)
