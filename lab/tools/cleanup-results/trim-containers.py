#!/usr/bin/env python3
"""Remove ELF/render payloads inside two old archives; compress historical patches."""
from pathlib import Path
import os,tarfile,hashlib,subprocess,datetime,json,gzip,io
ROOT=Path(__file__).resolve().parents[3];OUT=ROOT/'lab/results/storage-containers-2026-11-14';OUT.mkdir(exist_ok=False)
def sha(p):
 with open(p,'rb') as f:return hashlib.file_digest(f,'sha256').hexdigest()
def utc():return datetime.datetime.now(datetime.timezone.utc).isoformat()
tracked=set(subprocess.check_output(['git','ls-files','-z'],cwd=ROOT).decode().split('\0'));new=set(subprocess.check_output(['git','ls-files','--others','--exclude-standard','-z'],cwd=ROOT).decode().split('\0'));dirty={e[3:] for e in subprocess.check_output(['git','status','--porcelain=v1','-z'],cwd=ROOT).decode().split('\0') if e}
protected={p:sha(ROOT/p) for p in tracked|new if p and (ROOT/p).is_file()}
before={'utc':utc(),'head':subprocess.check_output(['git','rev-parse','HEAD'],cwd=ROOT,text=True).strip(),'diffstat':subprocess.check_output(['git','diff','HEAD','--stat'],cwd=ROOT,text=True),'protected_sha256':protected,'du_bytes':int(subprocess.check_output(['du','-sx','-B1',str(ROOT)],text=True).split()[0])}
with gzip.open(OUT/'before.json.gz','wt') as f:json.dump(before,f)
changed=set();receipts=[]
for rel in ['lab/results/construction/diagnostics/math-fields-r4-2026-10-23/evidence.tar.gz','lab/results/construction/diagnostics/text-size-length-2026-10-17/candidate-1.tar.gz']:
 if rel in dirty or rel in new:raise ValueError('container locally modified')
 p=ROOT/rel;dest=p.with_suffix('').with_suffix('.tar.zst');tmp=dest.with_suffix('.partial');kept={};removed=[];modes={}
 with tmp.open('xb') as raw:
  proc=subprocess.Popen(['zstd','-q','-3','-T1'],stdin=subprocess.PIPE,stdout=raw)
  with tarfile.open(p,'r|gz') as old,tarfile.open(fileobj=proc.stdin,mode='w|') as archive:
   for member in old:
    if member.isfile():
     data=old.extractfile(member).read();h=hashlib.sha256(data).hexdigest()
     if data[:4]==b'\x7fELF' or Path(member.name).suffix.lower() in {'.pdf','.svg','.xml','.html','.ppm'}:
      removed.append({'path':member.name,'sha256':h,'bytes':member.size,'reason':'ELF' if data[:4]==b'\x7fELF' else 'generated render/extraction'});continue
     kept[member.name]=h;modes[member.name]=member.mode;archive.addfile(member,io.BytesIO(data))
    else:archive.addfile(member)
  proc.stdin.close();assert proc.wait()==0
 actual={};proc=subprocess.Popen(['zstd','-dc',str(tmp)],stdout=subprocess.PIPE)
 with tarfile.open(fileobj=proc.stdout,mode='r|') as archive:
  for member in archive:
   if member.isfile():actual[member.name]=hashlib.file_digest(archive.extractfile(member),'sha256').hexdigest();assert member.mode==modes[member.name]
 assert proc.wait()==0;assert kept==actual
 if tmp.stat().st_size>=p.stat().st_size:tmp.unlink();continue
 record={'utc':utc(),'original':rel,'original_sha256':sha(p),'replacement':str(dest.relative_to(ROOT)),'replacement_sha256':sha(tmp),'before_bytes':p.stat().st_size,'after_bytes':tmp.stat().st_size,'retained_file_sha256':kept,'removed_generated':removed}
 os.replace(tmp,dest);(OUT/(p.parent.name+'-receipt.json')).write_text(json.dumps(record,indent=2));p.unlink();changed.add(rel);receipts.append(record)
 print('trimmed',rel,record['before_bytes'],record['after_bytes'],flush=True)
active={'box-launch-bias-2026-11-14','box-launch-control-2026-11-14','box-lifecycle-2026-11-14','vanilla-resync-2026-10-26'};canonical={};patches=[]
for parent,dirs,files in os.walk(ROOT/'lab/results'):
 if any(p in active or p.startswith('storage-') for p in Path(parent).parts):dirs[:]=[];continue
 for n in files:
  if not n.endswith('.patch'):continue
  p=Path(parent)/n;rel=str(p.relative_to(ROOT));dest=Path(str(p)+'.gz')
  if rel in dirty or rel in new or dest.exists() or p.is_symlink():continue
  st=p.stat();h=sha(p);row={'utc':utc(),'path':rel,'sha256':h,'mode':st.st_mode&0o777,'mtime_ns':st.st_mtime_ns,'bytes':st.st_size,'compressed_path':str(dest.relative_to(ROOT))}
  if h in canonical:os.link(canonical[h],dest)
  else:
   with p.open('rb') as src,gzip.open(dest,'wb',compresslevel=6) as dst:
    import shutil;shutil.copyfileobj(src,dst)
   with gzip.open(dest,'rb') as src:assert hashlib.file_digest(src,'sha256').hexdigest()==h
   canonical[h]=dest
  row['compressed_bytes']=dest.stat().st_size;patches.append(row)
  assert sha(p)==h and p.stat().st_mtime_ns==st.st_mtime_ns;p.unlink();changed.add(rel)
with gzip.open(OUT/'patches-index.jsonl.gz','wt') as f:
 for row in patches:f.write(json.dumps(row)+'\n')
for p,h in protected.items():
 if p not in changed:assert sha(ROOT/p)==h,p
after={'utc':utc(),'head':before['head'],'du_bytes':int(subprocess.check_output(['du','-sx','-B1',str(ROOT)],text=True).split()[0]),'trimmed_containers':len(receipts),'compressed_patches':len(patches),'unique_patch_payloads':len(canonical),'protected_files_verified':True};after['reclaimed_gib']=(before['du_bytes']-after['du_bytes'])/1024**3;(OUT/'receipt.json').write_text(json.dumps(after,indent=2));print(json.dumps(after),flush=True)
