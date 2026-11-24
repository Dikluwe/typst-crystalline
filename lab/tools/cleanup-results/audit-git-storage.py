#!/usr/bin/env python3
"""Capture/verify Git IDs, working files and recovery copy before safe repacking."""
from pathlib import Path
import os,json,hashlib,subprocess,gzip,datetime,shutil,sys
ROOT=Path(__file__).resolve().parents[3];OUT=ROOT/'lab/results/storage-git-packs-2026-11-14';BACKUP=Path('/tmp/tekt-git-objects-before-repack-20261114')
def sha(p):
 with open(p,'rb') as f:return hashlib.file_digest(f,'sha256').hexdigest()
def utc():return datetime.datetime.now(datetime.timezone.utc).isoformat()
def objects():return sorted(subprocess.check_output(['git','cat-file','--batch-all-objects','--batch-check=%(objectname) %(objecttype) %(objectsize)'],cwd=ROOT,text=True).splitlines())
def metadata():return {str(p.relative_to(ROOT/'.git')):sha(p) for p in (ROOT/'.git').rglob('*') if p.is_file() and 'objects' not in p.relative_to(ROOT/'.git').parts}
def working():
 names=set(subprocess.check_output(['git','ls-files','-z'],cwd=ROOT).decode().split('\0'))|set(subprocess.check_output(['git','ls-files','--others','--exclude-standard','-z'],cwd=ROOT).decode().split('\0'))
 return {p:sha(ROOT/p) for p in names if p and (ROOT/p).is_file()}
if sys.argv[1]=='before':
 OUT.mkdir(exist_ok=False);BACKUP.mkdir(exist_ok=False)
 data={'utc':utc(),'head':subprocess.check_output(['git','rev-parse','HEAD'],cwd=ROOT,text=True).strip(),'diffstat':subprocess.check_output(['git','diff','HEAD','--stat'],cwd=ROOT,text=True),'status':subprocess.check_output(['git','status','--porcelain=v1','-z'],cwd=ROOT).decode(),'metadata_sha256':metadata(),'working_sha256':working(),'objects':objects(),'du_bytes':int(subprocess.check_output(['du','-sx','-B1',str(ROOT)],text=True).split()[0]),'objects_du_bytes':int(subprocess.check_output(['du','-sx','-B1',str(ROOT/'.git/objects')],text=True).split()[0])}
 with gzip.open(OUT/'before.json.gz','wt') as f:json.dump(data,f)
 shutil.copytree(ROOT/'.git/objects',BACKUP/'objects',copy_function=shutil.copy2)
 manifest={str(p.relative_to(BACKUP)):sha(p) for p in BACKUP.rglob('*') if p.is_file()}
 for name,h in manifest.items():assert sha(ROOT/'.git'/name)==h,name
 with gzip.open(OUT/'recovery-manifest.json.gz','wt') as f:json.dump(manifest,f)
 (OUT/'ready.json').write_text(json.dumps({'utc':utc(),'objects':len(data['objects']),'recovery_path':str(BACKUP),'recovery_copy_verified':True}));print('PREPARED',len(data['objects']),flush=True)
else:
 with gzip.open(OUT/'before.json.gz','rt') as f:before=json.load(f)
 assert objects()==before['objects'],'object IDs/types/sizes changed'
 assert metadata()==before['metadata_sha256'],'Git index/refs/reflogs/metadata changed'
 assert working()==before['working_sha256'],'working files changed'
 assert subprocess.check_output(['git','diff','HEAD','--stat'],cwd=ROOT,text=True)==before['diffstat']
 with (OUT/'fsck.log').open('w') as log:
  p=subprocess.run(['git','fsck','--full','--strict','--no-progress'],cwd=ROOT,stdout=log,stderr=subprocess.STDOUT)
 assert p.returncode==0,'fsck failure: retain recovery backup'
 after={'utc':utc(),'head':before['head'],'du_bytes':int(subprocess.check_output(['du','-sx','-B1',str(ROOT)],text=True).split()[0]),'objects_du_bytes':int(subprocess.check_output(['du','-sx','-B1',str(ROOT/'.git/objects')],text=True).split()[0]),'object_ids_types_sizes_preserved':len(before['objects']),'git_metadata_preserved':True,'working_files_preserved':True,'fsck_exit':p.returncode}
 after['reclaimed_gib']=(before['du_bytes']-after['du_bytes'])/1024**3
 (OUT/'receipt.json').write_text(json.dumps(after,indent=2))
 # Remove only the exact recovery tree created by this run, after full verification.
 assert BACKUP.name=='tekt-git-objects-before-repack-20261114';shutil.rmtree(BACKUP)
 after['temporary_recovery_copy_removed']=True;(OUT/'receipt.json').write_text(json.dumps(after,indent=2));print(json.dumps(after),flush=True)
