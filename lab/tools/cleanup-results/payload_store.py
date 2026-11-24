#!/usr/bin/env python3
"""Verified shared payloads for historical archives; no productive source imports."""
from pathlib import Path
import os,json,gzip,hashlib,subprocess,tarfile,tempfile,shutil,datetime
ROOT=Path(__file__).resolve().parents[3]
OUT=ROOT/'lab/results/storage-shared-payloads-2026-11-15'
def digest(path):
 with open(path,'rb') as f:return hashlib.file_digest(f,'sha256').hexdigest()
def rows_for(spec):
 with gzip.open(ROOT/spec['index'],'rt') as f:return {r['path']:r for r in map(json.loads,f)}
def available():return (OUT/'catalog.json').is_file()
def build(specs,out):
 out.mkdir(exist_ok=False);seen={};counts={};catalog={'format':1,'created_utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'sets':specs,'payloads':'payloads.tar.zst'}
 with (out/'payloads.tar.zst').open('xb') as raw:
  writer=subprocess.Popen(['zstd','-q','-6','--long=27','-T1'],stdin=subprocess.PIPE,stdout=raw)
  try:
   with tarfile.open(fileobj=writer.stdin,mode='w|') as target:
    for kind,spec in specs.items():
     rows=rows_for(spec);resolved={};nonfiles=0;spec['source_archive_sha256']=digest(ROOT/spec['archive']);spec['index_sha256']=digest(ROOT/spec['index'])
     meta=out/(kind+'-headers.jsonl.gz');reader=subprocess.Popen(['zstd','-dc',str(ROOT/spec['archive'])],stdout=subprocess.PIPE)
     try:
      with tarfile.open(fileobj=reader.stdout,mode='r|') as source,gzip.open(meta,'wt') as log:
       for item in source:
        info=item.get_info();info['type']=info['type'].decode('latin1');info['pax_headers']=item.pax_headers;log.write(json.dumps(info)+'\n')
        if not item.isfile() and not item.islnk():nonfiles+=1;continue
        row=rows[item.name];h=row[spec['hash_field']]
        if item.islnk():
         if resolved.get(item.linkname)!=h:raise ValueError('invalid source alias')
        else:
         with tempfile.TemporaryFile() as blob:
          shutil.copyfileobj(source.extractfile(item),blob);size=blob.tell();blob.seek(0)
          if hashlib.file_digest(blob,'sha256').hexdigest()!=h:raise ValueError('source hash mismatch')
          if h not in seen:
           blob.seek(0);obj=tarfile.TarInfo(h);obj.size=size;obj.mode=0o444;target.addfile(obj,blob);seen[h]=size
        resolved[item.name]=h
      if reader.wait()!=0:raise RuntimeError('source decompression failed')
     finally:
      if reader.poll() is None:reader.kill();reader.wait()
      reader.stdout.close()
     if set(rows)!=set(resolved):raise ValueError('incomplete source path inventory')
     spec.update(headers=meta.name,headers_sha256=digest(meta),files=len(rows),metadata_entries=nonfiles);counts[kind]=len(rows)
     print('collected',kind,len(rows),'unique payloads so far',len(seen),flush=True)
   writer.stdin.close()
   if writer.wait()!=0:raise RuntimeError('store compression failed')
  finally:
   if writer.poll() is None:writer.kill();writer.wait()
 actual={};reader=subprocess.Popen(['zstd','-dc',str(out/'payloads.tar.zst')],stdout=subprocess.PIPE)
 try:
  with tarfile.open(fileobj=reader.stdout,mode='r|') as archive:
   for item in archive:
    if not item.isfile() or item.name not in seen:raise ValueError('unexpected store entry')
    if hashlib.file_digest(archive.extractfile(item),'sha256').hexdigest()!=item.name:raise ValueError('store hash mismatch')
    actual[item.name]=item.size
  if reader.wait()!=0:raise RuntimeError('store decompression failed')
 finally:
  if reader.poll() is None:reader.kill();reader.wait()
  reader.stdout.close()
 if actual!=seen:raise ValueError('incomplete store')
 catalog.update(payloads_sha256=digest(out/'payloads.tar.zst'),unique_payloads=len(seen),file_counts=counts,all_payloads_verified=True)
 (out/'candidate.json').write_text(json.dumps(catalog,indent=2));return catalog
def restore(kind,prefixes):
 catalog=json.loads((OUT/'catalog.json').read_text());spec=catalog['sets'][kind]
 if digest(ROOT/spec['index'])!=spec['index_sha256'] or digest(OUT/spec['headers'])!=spec['headers_sha256']:raise ValueError('index/metadata changed')
 archive=OUT/catalog['payloads']
 if digest(archive)!=catalog['payloads_sha256']:raise ValueError('payload store changed')
 rows=rows_for(spec);wanted={p:r for p,r in rows.items() if any(p==s or p.startswith(s.rstrip('/')+'/') for s in prefixes)}
 if not wanted:raise ValueError('no indexed path matches selection')
 needed={r[spec['hash_field']] for r in wanted.values()}
 with tempfile.TemporaryDirectory(prefix='tekt-shared-restore-') as temp:
  cache=Path(temp);found=set();reader=subprocess.Popen(['zstd','-dc',str(archive)],stdout=subprocess.PIPE)
  try:
   with tarfile.open(fileobj=reader.stdout,mode='r|') as stream:
    for item in stream:
     if item.name not in needed:continue
     obj=cache/item.name
     with obj.open('wb') as f:shutil.copyfileobj(stream.extractfile(item),f)
     if digest(obj)!=item.name:raise ValueError('payload corrupt')
     found.add(item.name)
     if found==needed:break
  finally:
   if reader.poll() is None:reader.terminate()
   reader.stdout.close();reader.wait()
  if found!=needed:raise ValueError('missing payload')
  for name,row in wanted.items():
   rel=Path(name)
   if rel.is_absolute() or '..' in rel.parts:raise ValueError('unsafe path')
   dest=ROOT/rel;h=row[spec['hash_field']];encoding=row.get('encoding',spec.get('encoding','identity'))
   if dest.is_symlink():raise ValueError('refusing symlink')
   if dest.exists():
    with (gzip.open(dest,'rb') if encoding=='gzip' else dest.open('rb')) as f:existing=hashlib.file_digest(f,'sha256').hexdigest()
    if existing!=h:raise ValueError('refusing overwrite '+name)
    continue
   dest.parent.mkdir(parents=True,exist_ok=True)
   if dest.parent.resolve()!=dest.parent:raise ValueError('symlink parent')
   fd,tmpname=tempfile.mkstemp(prefix='.shared-restore-',dir=dest.parent);os.close(fd);tmp=Path(tmpname)
   try:
    if encoding=='gzip':
     with (cache/h).open('rb') as src,gzip.open(tmp,'wb',compresslevel=6) as dst:shutil.copyfileobj(src,dst)
     with gzip.open(tmp,'rb') as f:verified=hashlib.file_digest(f,'sha256').hexdigest()
    else:shutil.copyfile(cache/h,tmp);verified=digest(tmp)
    if verified!=h:raise ValueError('restoration hash mismatch')
    tmp.chmod(row['mode']);mtime=row['mtime_ns'];os.utime(tmp,ns=(row.get('atime_ns',mtime),mtime));os.replace(tmp,dest)
   finally:tmp.unlink(missing_ok=True)
 print('Shared store: restored/verified independent files:',len(wanted))
