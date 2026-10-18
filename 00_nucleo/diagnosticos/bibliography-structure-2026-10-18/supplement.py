from pathlib import Path
import subprocess,json,datetime,gzip,hashlib
p=Path(__file__).resolve().parent
now=lambda:datetime.datetime.now(datetime.timezone.utc).isoformat()
summary=json.loads((p/'complete-summary.json').read_text());(p/'summary-before-supplement.json').write_text(json.dumps(summary,indent=2)+'\n')
rows=[]
for label,cmd in [('build',['cargo','build','--offline','--locked','--workspace']),('lint-diff',['python3',str(p/'lint-diff.py'),'final-architecture.log']),('diff',['git','diff','--check'])]:
 start=now()
 with (p/('supplement-'+label+'.log')).open('w') as f:done=subprocess.run(cmd,stdout=f,stderr=subprocess.STDOUT)
 rows.append(dict(label=label,command=cmd,start=start,end=now(),exit=done.returncode));(p/'supplement-results.json').write_text(json.dumps(rows,indent=2)+'\n')
 assert done.returncode==0,(label,done.returncode)
freeze=json.loads(gzip.decompress((p/'final-freeze.json.gz').read_bytes()))
def sha(f):
 with Path(f).open('rb') as s:return hashlib.file_digest(s,'sha256').hexdigest()
assert all(sha(f)==h for f,h in freeze['sources'].items())
summary['binary_before_supplement_sha256']=summary['binary_sha256'];summary['binary_sha256']=sha('target/debug/typst');summary['supplement_results']='supplement-results.json';summary['results'] += [dict(x,cohort='final-build-supplement') for x in rows]
(p/'complete-summary.json').write_text(json.dumps(summary,indent=2)+'\n')
print(json.dumps(dict(results=rows,binary_sha256=summary['binary_sha256'],source_freeze_verified=True)))
