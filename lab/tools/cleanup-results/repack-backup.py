#!/usr/bin/env python3
"""Pack nested gzip payloads jointly; restore source copies independently."""
from pathlib import Path
import argparse,os,tarfile,gzip,hashlib,subprocess,json,datetime,io,tempfile,shutil
ROOT=Path(__file__).resolve().parents[3];OUT=ROOT/'lab/results/storage-backups-2026-11-14'
ORIGINAL=ROOT/'lab/results/project-cleanup-2026-10-22/source-copies.tar.zst'
ARCHIVE=ORIGINAL.with_name('source-copies.packed.tar.zst')
def sha(p):
 with open(p,'rb') as f:return hashlib.file_digest(f,'sha256').hexdigest()
def utc():return datetime.datetime.now(datetime.timezone.utc).isoformat()
def restore(prefixes):
 from payload_store import available,restore as shared_restore
 if available():return shared_restore("backups",prefixes)
 with gzip.open(OUT/'index.jsonl.gz','rt') as f:rows={r['path']:r for r in map(json.loads,f)}
 wanted={k:v for k,v in rows.items() if any(k==s or k.startswith(s.rstrip('/')+'/') for s in prefixes)}
 if not wanted:raise ValueError('no indexed path matches')
 needed={r['payload_sha256'] for r in wanted.values()}
 with tempfile.TemporaryDirectory(prefix='tekt-backup-restore-') as temp:
  cache=Path(temp);proc=subprocess.Popen(['zstd','-dc',str(ARCHIVE)],stdout=subprocess.PIPE)
  try:
   with tarfile.open(fileobj=proc.stdout,mode='r|') as archive:
    for item in archive:
     if item.name not in rows:continue
     row=rows[item.name];h=row['payload_sha256']
     if item.isfile() and h in needed:
      with (cache/h).open('wb') as f:shutil.copyfileobj(archive.extractfile(item),f)
      assert sha(cache/h)==h
     if item.name not in wanted:continue
     rel=Path(item.name)
     if rel.is_absolute() or '..' in rel.parts:raise ValueError('unsafe archive member')
     dest=ROOT/rel
     if dest.is_symlink():raise ValueError('refusing symlink')
     if dest.exists():
      if row['encoding']=='gzip':
       with gzip.open(dest,'rb') as f:assert hashlib.file_digest(f,'sha256').hexdigest()==h,'refusing overwrite'
      else:assert sha(dest)==h,'refusing overwrite'
      continue
     dest.parent.mkdir(parents=True,exist_ok=True)
     if dest.parent.resolve()!=dest.parent:raise ValueError('symlink parent')
     fd,name=tempfile.mkstemp(prefix='.backup-restore-',dir=dest.parent);os.close(fd);tmp=Path(name)
     try:
      if row['encoding']=='gzip':
       with (cache/h).open('rb') as src,gzip.open(tmp,'wb',compresslevel=6) as dst:shutil.copyfileobj(src,dst)
       with gzip.open(tmp,'rb') as f:assert hashlib.file_digest(f,'sha256').hexdigest()==h
      else:shutil.copyfile(cache/h,tmp);assert sha(tmp)==h
      tmp.chmod(row['mode']);os.utime(tmp,ns=(row['mtime_ns'],row['mtime_ns']));os.replace(tmp,dest)
     finally:tmp.unlink(missing_ok=True)
   assert proc.wait()==0
  finally:
   if proc.poll() is None:proc.kill();proc.wait()
 print('Restored/verified independent files:',len(wanted))
def pack():
 OUT.mkdir(exist_ok=False)
 tracked=set(subprocess.check_output(['git','ls-files','-z'],cwd=ROOT).decode().split('\0'));new=set(subprocess.check_output(['git','ls-files','--others','--exclude-standard','-z'],cwd=ROOT).decode().split('\0'))
 protected={p:sha(ROOT/p) for p in tracked|new if p and (ROOT/p).is_file()}
 before={'utc':utc(),'head':subprocess.check_output(['git','rev-parse','HEAD'],cwd=ROOT,text=True).strip(),'diffstat':subprocess.check_output(['git','diff','HEAD','--stat'],cwd=ROOT,text=True),'protected_sha256':protected,'du_bytes':int(subprocess.check_output(['du','-sx','-B1',str(ROOT)],text=True).split()[0]),'original_sha256':sha(ORIGINAL),'original_bytes':ORIGINAL.stat().st_size}
 with gzip.open(OUT/'before.json.gz','wt') as f:json.dump(before,f)
 tmp=ARCHIVE.with_suffix('.partial');rows={};seen={};other_meta={};converted=0;count=0
 reader=subprocess.Popen(['zstd','-dc',str(ORIGINAL)],stdout=subprocess.PIPE)
 with tmp.open('xb') as raw:
  writer=subprocess.Popen(['zstd','-q','-6','--long=27','-T1'],stdin=subprocess.PIPE,stdout=raw)
  with tarfile.open(fileobj=reader.stdout,mode='r|') as old,tarfile.open(fileobj=writer.stdin,mode='w|') as newtar,gzip.open(OUT/'index.jsonl.gz','wt') as index:
   for item in old:
    if item.isfile():
     data=old.extractfile(item).read();original_hash=hashlib.sha256(data).hexdigest();encoding='identity';payload=io.BytesIO(data)
     if item.name.endswith('.gz') and data.startswith(b'\x1f\x8b'):
      payload=tempfile.TemporaryFile()
      with gzip.GzipFile(fileobj=io.BytesIO(data)) as src:shutil.copyfileobj(src,payload)
      size=payload.tell();payload.seek(0);encoding='gzip';converted+=1
     else:size=len(data)
     h=hashlib.file_digest(payload,'sha256').hexdigest();payload.seek(0)
     row={'path':item.name,'original_sha256':original_hash,'payload_sha256':h,'encoding':encoding,'mode':item.mode,'mtime_ns':round(item.mtime*1e9),'original_bytes':len(data),'payload_bytes':size}
    elif item.islnk():
     ref=rows[item.linkname];row=dict(ref,path=item.name,mode=item.mode,mtime_ns=round(item.mtime*1e9));h=row['payload_sha256'];payload=None
    else:
     other_meta[item.name]=(item.type.decode('latin1'),item.linkname,item.mode,item.mtime);newtar.addfile(item);continue
    rows[item.name]=row;index.write(json.dumps(row)+'\n');item.pax_headers=dict(item.pax_headers);item.pax_headers['tekt.encoding']=row['encoding']
    if h in seen:item.type=tarfile.LNKTYPE;item.linkname=seen[h];item.size=0;newtar.addfile(item)
    else:
     assert payload is not None;seen[h]=item.name;item.size=size;newtar.addfile(item,payload)
    if payload is not None:payload.close()
    count+=1
    if count%25000==0:print('packed',count,'gzip payloads',converted,flush=True)
  writer.stdin.close();assert writer.wait()==0
 reader.stdout.close();assert reader.wait()==0
 actual={};meta={};proc=subprocess.Popen(['zstd','-dc',str(tmp)],stdout=subprocess.PIPE)
 with tarfile.open(fileobj=proc.stdout,mode='r|') as archive:
  for item in archive:
   if item.isfile():h=hashlib.file_digest(archive.extractfile(item),'sha256').hexdigest()
   elif item.islnk():h=actual[item.linkname]
   else:meta[item.name]=(item.type.decode('latin1'),item.linkname,item.mode,item.mtime);continue
   actual[item.name]=h;row=rows[item.name];assert h==row['payload_sha256'] and item.mode==row['mode'] and item.pax_headers['tekt.encoding']==row['encoding']
 assert proc.wait()==0;assert set(actual)==set(rows);assert meta==other_meta
 assert tmp.stat().st_size<ORIGINAL.stat().st_size,'no gain; original retained'
 assert sha(ORIGINAL)==before['original_sha256']
 os.replace(tmp,ARCHIVE)
 receipt={'utc':utc(),'head':before['head'],'original':str(ORIGINAL.relative_to(ROOT)),'replacement':str(ARCHIVE.relative_to(ROOT)),'before_bytes':before['original_bytes'],'after_bytes':ARCHIVE.stat().st_size,'archive_sha256':sha(ARCHIVE),'preserved_files':len(rows),'unique_payloads':len(seen),'decoded_gzip_payloads':converted,'metadata_entries_verified':len(other_meta),'all_payloads_and_paths_verified':True}
 (OUT/'archive-receipt.json').write_text(json.dumps(receipt,indent=2));ORIGINAL.unlink()
 for p,h in protected.items():assert sha(ROOT/p)==h,p
 assert subprocess.check_output(['git','diff','HEAD','--stat'],cwd=ROOT,text=True)==before['diffstat']
 receipt.update(du_bytes=int(subprocess.check_output(['du','-sx','-B1',str(ROOT)],text=True).split()[0]),protected_files_verified=True);receipt['reclaimed_gib']=(before['du_bytes']-receipt['du_bytes'])/1024**3;(OUT/'receipt.json').write_text(json.dumps(receipt,indent=2));print(json.dumps(receipt),flush=True)
if __name__=='__main__':
 p=argparse.ArgumentParser(description=__doc__);p.add_argument('--restore',nargs='+');args=p.parse_args()
 if args.restore:restore(args.restore)
 else:pack()
