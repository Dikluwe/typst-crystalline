from pathlib import Path
import os,subprocess,json,gzip,hashlib,datetime,shutil,tarfile
root=Path.cwd();out=Path(__file__).resolve().parent
now=lambda:datetime.datetime.now(datetime.timezone.utc).isoformat()
def load(name):return json.loads(gzip.decompress((out/name).read_bytes()))
def gz(name,data):(out/name).write_bytes(gzip.compress(json.dumps(data,ensure_ascii=False,indent=2).encode()))
def sha(path):
 h=hashlib.sha256()
 with path.open('rb') as fp:
  for b in iter(lambda:fp.read(1024*1024),b''):h.update(b)
 return h.hexdigest()
def git(*args):return subprocess.check_output(['git',*args],text=True)
start=now();before=load('before.json.gz');plan=load('plan.json.gz')
cache_roots=[root/x['path'] for x in plan['roots'] if x['group']=='C07']
cached=[];productive=[]
for item in before['productive_hashes']:
 q=root/item['path']
 if '__pycache__' in q.parts and q.suffix=='.pyc' and any(q.is_relative_to(c) for c in cache_roots):
  assert not q.exists();cached.append(item['path'])
 else:assert sha(q)==item['sha256'];productive.append(item)
assert all(not (root/x['path']).exists() for x in plan['roots'])
assert all(os.path.lexists(root/f) for f in before['tracked']+before['untracked'])
assert all(sha(root/f)==h for f,h in plan['anchors'].items())
current_modified=set(git('diff','HEAD','--name-only').splitlines())
prior_modified=set(line[3:] for line in before['status'].splitlines() if not line.startswith('??'))
assert current_modified==prior_modified
print('Sources/configuration unchanged:',len(productive),'expected Python caches removed:',len(cached),flush=True)
smoke=out/'smoke';smoke.mkdir(exist_ok=True);(smoke/'input.typ').write_text('CACHE OK\n');commands=[]
for label,exe in [('tekt',root/'target/debug/typst'),('vanilla',root/'lab/typst-original/target/release/typst')]:
 cmd=[str(exe),'compile',str(smoke/'input.typ'),str(smoke/(label+'.pdf'))]
 z=subprocess.run(cmd,text=True,capture_output=True);assert z.returncode==0,(cmd,z.stderr)
 text=subprocess.check_output(['pdftotext',str(smoke/(label+'.pdf')),'-'],text=True).strip();assert text=='CACHE OK',text
 commands.append(dict(label=label,command=cmd,exit=z.returncode,text=text,binary_sha256=sha(exe),output_sha256=sha(smoke/(label+'.pdf'))))
print('Smoke compile PDF: Tekt and vanilla OK',flush=True)
archive=out/'retencao/artefatos.tar.zst';temp=out/'retencao/artefatos.tar.zst.partial'
assert not archive.exists() and not temp.exists()
print('Compressing independently retained objects with zstd, one thread',flush=True)
with (out/'archive-build.log').open('w') as log:
 tar=subprocess.Popen(['tar','-C',str(out/'retencao'),'-cf','-','objetos'],stdout=subprocess.PIPE,stderr=log)
 compress=subprocess.Popen(['zstd','-q','-T1','-3','-o',str(temp),'-'],stdin=tar.stdout,stderr=log)
 tar.stdout.close()
 assert compress.wait()==0
 assert tar.wait()==0
temp.rename(archive)
expected={Path(x['path']).name:x for x in plan['objects']};seen=set();total=0
print('Verifying all archived object contents against SHA-256 before retiring staging copies',flush=True)
with (out/'archive-verify.log').open('w') as log:
 decoded=subprocess.Popen(['zstd','-q','-dc',str(archive)],stdout=subprocess.PIPE,stderr=log)
 with tarfile.open(fileobj=decoded.stdout,mode='r|') as tf:
  for member in tf:
   if member.isdir():assert member.name.rstrip('/')=='objetos';continue
   assert member.isfile() and member.name.startswith('objetos/')
   key=Path(member.name).name;assert key in expected and key not in seen
   fp=tf.extractfile(member);h=hashlib.sha256()
   for block in iter(lambda:fp.read(1024*1024),b''):h.update(block)
   assert h.hexdigest()==expected[key]['sha256']
   assert member.size==expected[key]['logical_bytes']
   seen.add(key);total+=member.size
 while decoded.stdout.read(1024*1024):pass
 decoded.stdout.close();assert decoded.wait()==0
assert seen==set(expected)
archive_record=dict(utc=now(),path=str(archive.relative_to(root)),sha256=sha(archive),compressed_bytes=archive.stat().st_size,uncompressed_object_bytes=total,objects_verified=len(seen),members={h:'objetos/'+h for h in sorted(seen)},restore_mapping=plan['retained'])
gz('retencao.json.gz',archive_record)
# These are the independent staging copies made by this cleanup, not originals.
shutil.rmtree(out/'retencao/objetos')
assert all(sha(root/f)==h for f,h in plan['anchors'].items())
assert all(sha(root/x['path'])==x['sha256'] for x in productive)
subprocess.run(['git','diff','--check'],check=True)
result=dict(start=start,end=now(),head=before['head'],before='before.json.gz',plan='plan.json.gz',roots_removed=[x['path'] for x in plan['roots']],removed_paths_allocated_bytes=sum(x['allocated_bytes'] for x in plan['roots']),retention=archive_record,productive_files_unchanged=len(productive),expected_deleted_python_cache_entries=cached,git_visible_paths_preserved=len(set(before['tracked'])|set(before['untracked'])),anchors=plan['anchors'],smoke=commands,disk_free_before_bytes=before['disk_free_bytes'],disk_free_after_bytes=shutil.disk_usage(root).free,status=git('status','--short'),diffstat=git('diff','HEAD','--stat'),rebuild_performed=False)
gz('result.json.gz',result)
print(json.dumps(dict(deleted_roots=len(result['roots_removed']),removed_cache_paths_gib=result['removed_paths_allocated_bytes']/1024**3,compressed_retention_gib=archive.stat().st_size/1024**3,archived_objects=len(seen),disk_free_increase_gib=(result['disk_free_after_bytes']-result['disk_free_before_bytes'])/1024**3,productive_unchanged=len(productive)),ensure_ascii=False),flush=True)
