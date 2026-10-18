from pathlib import Path
import subprocess,json,gzip,datetime,hashlib,sys
p=Path(__file__).resolve().parent
before=json.loads(gzip.decompress((p/'before-sources.json.gz').read_bytes()))
def sha(f):
 with Path(f).open('rb') as x:return hashlib.file_digest(x,'sha256').hexdigest()
files={f:sha(f) for f in before['files']};changed={f:Path(f).read_text() for f,s in before['files'].items() if hashlib.sha256(s.encode()).hexdigest()!=files[f]}
(p/'candidate1-sources.json.gz').write_bytes(gzip.compress(json.dumps(dict(utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),source_hashes=files,changed_texts=changed)).encode()))
for label,cmd in [('build',['cargo','build','--offline','--locked','--workspace']),('first-after',['python3',str(p/'probe.py'),'first-after']),('first-bib-after',['python3',str(p/'bib-probe.py'),'first-bib-after']),('first-regions-after',['python3',str(p/'regions.py'),'first-regions-after'])]:
 print(label,flush=True)
 with (p/(label+'.log')).open('w') as log:z=subprocess.run(cmd,stdout=log,stderr=subprocess.STDOUT)
 if z.returncode:sys.exit(z.returncode)
assert all(sha(f)==h for f,h in files.items());print('candidate1 source hashes unchanged')
