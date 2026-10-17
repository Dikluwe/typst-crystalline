from pathlib import Path
import subprocess,sys,json,datetime,hashlib,gzip
out=Path('00_nucleo/diagnosticos/clippy-locais-2026-10-16');sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest()
before=json.load(gzip.open(out/'before.json.gz'));hashes={p:sha(p) for p in before['all_source_hashes']};records=[]
commands=[('bbox',['cargo','test','--offline','--locked','-p','typst-infra','--lib','p273_6_layouter_parent_bbox_consumed_by_block_arm','--','--test-threads=1']),('clippy',['python3',str(out/'workers/gate.py'),'green'])]
for label,cmd in commands:
 with (out/('green-'+label+'-run.log')).open('w') as f:r=subprocess.run(cmd,stdout=f,stderr=subprocess.STDOUT)
 records.append(dict(label=label,command=cmd,exit=r.returncode))
 unchanged=all(sha(p)==h for p,h in hashes.items());(out/'green-result-details.json').write_text(json.dumps(dict(utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),commands=records,sources_unchanged=unchanged),indent=2)+'\n');assert unchanged
 if r.returncode:sys.exit(r.returncode)
print(json.dumps(records),flush=True)
