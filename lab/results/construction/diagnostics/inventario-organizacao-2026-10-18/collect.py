from pathlib import Path
import os,stat,json,gzip,hashlib,subprocess,datetime,collections
root=Path.cwd();out=Path(__file__).resolve().parent
utc=lambda:datetime.datetime.now(datetime.timezone.utc).isoformat()
def git(*args):return subprocess.check_output(['git',*args],text=True)
def gz(name,data):(out/name).write_bytes(gzip.compress(json.dumps(data,ensure_ascii=False,indent=2).encode()))
def sha(path):
 h=hashlib.sha256()
 with path.open('rb') as f:
  for block in iter(lambda:f.read(1024*1024),b''):h.update(block)
 return h.hexdigest()
tracked=set(git('ls-files','-z').split('\0'))-{''}
untracked=set(git('ls-files','--others','--exclude-standard','-z').split('\0'))-{''}
start=utc();head=git('rev-parse','HEAD').strip()
state=dict(utc=start,head=head,branch=git('branch','--show-current').strip(),status=git('status','--short'),diffstat=git('diff','HEAD','--stat'),modified=git('diff','HEAD','--name-only').splitlines(),tracked=sorted(tracked),untracked=sorted(untracked))
gz('git-before.json.gz',state)
# Sum allocated blocks per directory entry, no following symlinks. Shared extents
# and hardlinks can overlap: these are estimates, not a guaranteed reclaim size.
agg=collections.defaultdict(lambda:dict(files=0,logical_bytes=0,allocated_bytes=0,tracked=0,untracked=0,ignored=0,git_metadata=0,symlinks=0))
errors=[];symlinks=[];docs=[];groups=collections.defaultdict(list);metadata=[]
count=0;stack=[root];excluded={'.agents','.aws','.codex'}
with gzip.open(out/'files.tsv.gz','wt',encoding='utf-8') as rows:
 rows.write('path\ttype\tgit_state\tlogical_bytes\tallocated_bytes\tmtime_ns\n')
 while stack:
  folder=stack.pop()
  try: entries=os.scandir(folder)
  except OSError as e:errors.append(dict(path=str(folder.relative_to(root)),error=str(e)));continue
  with entries:
   for ent in entries:
    path=Path(ent.path);rel=path.relative_to(root).as_posix();parts=rel.split('/')
    if len(parts)==1 and parts[0] in excluded:continue
    try:s=ent.stat(follow_symlinks=False)
    except OSError as e:errors.append(dict(path=rel,error=str(e)));continue
    if stat.S_ISDIR(s.st_mode):stack.append(path);continue
    kind='symlink' if stat.S_ISLNK(s.st_mode) else 'file' if stat.S_ISREG(s.st_mode) else 'special'
    gs='git_metadata' if parts[0]=='.git' else 'tracked' if rel in tracked else 'untracked' if rel in untracked else 'ignored'
    alloc=s.st_blocks*512
    keys={'.',parts[0]}
    if len(parts)>1:keys.add('/'.join(parts[:2]))
    if len(parts)>2 and (parts[0]=='lab' or parts[:2]==['00_nucleo','diagnosticos'] or parts[:2]==['00_nucleo','materialization']):keys.add('/'.join(parts[:3]))
    if len(parts)>3 and parts[0]=='lab':keys.add('/'.join(parts[:4]))
    for key in keys:
     a=agg[key];a['files']+=1;a['logical_bytes']+=s.st_size;a['allocated_bytes']+=alloc;a[gs]+=1;a['symlinks']+=kind=='symlink'
    rows.write(f'{rel}\t{kind}\t{gs}\t{s.st_size}\t{alloc}\t{s.st_mtime_ns}\n');count+=1
    if kind=='symlink':symlinks.append(dict(path=rel,target=os.readlink(path)));continue
    if kind!='file':continue
    if parts[0] in ['01_core','02_shell','03_infra','04_wiring'] and path.suffix=='.rs' or rel.startswith('00_nucleo/prompts/') or rel in ['Cargo.toml','Cargo.lock','crystalline.toml','CLAUDE.md'] or rel.startswith('.github/workflows/') or rel.startswith('tools/ci/'):
     metadata.append(dict(path=rel,git_state=gs,sha256=sha(path),bytes=s.st_size))
    if parts[:2] in [['00_nucleo','diagnosticos'],['00_nucleo','materialization'],['00_nucleo','context']] and path.suffix=='.md':
     text=path.read_text(errors='replace');lines=text.splitlines()
     title=next((line.lstrip('# ').strip() for line in lines if line.startswith('# ')),path.name)
     sections=[line for line in lines if line.startswith('##')]
     h=hashlib.sha256(text.encode()).hexdigest();groups[(s.st_size,h)].append(rel)
     docs.append(dict(path=rel,title=title,git_state=gs,bytes=s.st_size,sha256=h,sections=sections))
summary=dict(start=start,end=utc(),head=head,files=count,excluded_root_paths=sorted(excluded),method='lstat/scandir sem seguir symlinks; st_size e st_blocks*512 por entrada; sem deduplicar hardlinks/extents compartilhados; não é tamanho recuperável garantido',directories=dict(sorted(agg.items())),errors=errors,symlinks=symlinks)
gz('filesystem.json.gz',summary);gz('documents.json.gz',docs);gz('productive-state.json.gz',metadata)
gz('duplicate-documents.json.gz',[dict(bytes=size,sha256=h,paths=paths) for (size,h),paths in groups.items() if len(paths)>1])
previous=root/'00_nucleo/diagnosticos/r3-t1-materializacao-2026-10-18/candidate10-sources.json.gz'
frozen=json.loads(gzip.decompress(previous.read_bytes()))['files']
changed=[f for f,s in frozen.items() if not (root/f).is_file() or (root/f).read_text()!=s]
gz('closure-comparison.json.gz',dict(utc=utc(),reference=str(previous.relative_to(root)),reference_sha256=sha(previous),protected_files=len(frozen),changed=changed))
print(json.dumps(dict(start=start,end=summary['end'],files=count,top={k:v for k,v in summary['directories'].items() if '/' not in k},errors=errors,protected_changed=changed),ensure_ascii=False))
