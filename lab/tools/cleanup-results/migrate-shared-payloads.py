#!/usr/bin/env python3
"""Build a verified candidate only; originals remain until explicit validation/commit."""
from pathlib import Path
import json,gzip,hashlib,subprocess,datetime
import payload_store as store
ROOT=store.ROOT;OUT=ROOT/'lab/results/storage-shared-migration-2026-11-15';OUT.mkdir(exist_ok=False)
def sha(p):return store.digest(p)
tracked=set(subprocess.check_output(['git','ls-files','-z'],cwd=ROOT).decode().split('\0'));new=set(subprocess.check_output(['git','ls-files','--others','--exclude-standard','-z'],cwd=ROOT).decode().split('\0'))
before={'utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'head':subprocess.check_output(['git','rev-parse','HEAD'],cwd=ROOT,text=True).strip(),'diffstat':subprocess.check_output(['git','diff','HEAD','--stat'],cwd=ROOT,text=True),'working_sha256':{p:sha(ROOT/p) for p in tracked|new if p and (ROOT/p).is_file()},'du_bytes':int(subprocess.check_output(['du','-sx','-B1',str(ROOT)],text=True).split()[0])}
with gzip.open(OUT/'before.json.gz','wt') as f:json.dump(before,f)
specs={
 'backups':{'archive':'lab/results/project-cleanup-2026-10-22/source-copies.packed.tar.zst','index':'lab/results/storage-backups-2026-11-14/index.jsonl.gz','hash_field':'payload_sha256','encoding':'identity'},
 'snapshots':{'archive':'lab/results/storage-snapshots-2026-11-14/snapshots.tar.zst','index':'lab/results/storage-snapshots-2026-11-14/index.jsonl.gz','hash_field':'content_sha256','encoding':'gzip'},
 'data':{'archive':'lab/results/storage-historical-data-2026-11-14/data.tar.zst','index':'lab/results/storage-historical-data-2026-11-14/index.jsonl.gz','hash_field':'content_sha256','encoding':'gzip'}}
catalog=store.build(specs,store.OUT)
old_bytes=sum((ROOT/s['archive']).stat().st_size for s in catalog['sets'].values());new_bytes=int(subprocess.check_output(['du','-sx','-B1',str(store.OUT)],text=True).split()[0])
if new_bytes>=old_bytes:raise RuntimeError('candidate not smaller; original archives intact')
for p,h in before['working_sha256'].items():assert sha(ROOT/p)==h,p
(OUT/'candidate-receipt.json').write_text(json.dumps({'utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'old_archives_bytes':old_bytes,'new_store_allocated_bytes':new_bytes,'originals_still_present':True,'unique_payloads':catalog['unique_payloads'],'file_counts':catalog['file_counts'],'all_payloads_verified':True,'working_files_verified':True},indent=2));print('CANDIDATE VERIFIED',old_bytes,new_bytes,flush=True)
