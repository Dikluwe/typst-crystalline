from pathlib import Path
import subprocess,json,datetime,hashlib,gzip,sys
out=Path('00_nucleo/diagnosticos/clippy-expressions-2026-10-16');b=json.load(gzip.open(out/'before.json.gz'));sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest();hashes={p:sha(p) for p in b['all_source_hashes']};records=[]
for label,cmd in [('observer-green',['cargo','test','--offline','--locked','-p','typst-infra','--lib','pdf_object_observer','--','--test-threads=1']),('green-clippy',['python3',str(out/'workers/lint_gate.py')])]:
 start=datetime.datetime.now(datetime.timezone.utc).isoformat()
 with (out/(label+'.log')).open('w') as f:r=subprocess.run(cmd,stdout=f,stderr=subprocess.STDOUT)
 rec=dict(label=label,command=cmd,start=start,end=datetime.datetime.now(datetime.timezone.utc).isoformat(),exit=r.returncode,sources_unchanged=all(sha(p)==h for p,h in hashes.items()));records.append(rec);(out/(label+'-result.json')).write_text(json.dumps(rec,indent=2)+'\n');print(json.dumps(rec),flush=True)
 assert rec['sources_unchanged']
 if r.returncode:sys.exit(r.returncode)
