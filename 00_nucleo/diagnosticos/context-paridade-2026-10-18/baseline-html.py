from pathlib import Path
import gzip,json,subprocess,sys
p=Path(__file__).resolve().parent
baseline=json.loads(gzip.decompress((p/'text-final-sources.json.gz').read_bytes()))['files']
current=json.loads(gzip.decompress((p/'occurrences-candidate2-sources.json.gz').read_bytes()))['files']
assert all(Path(f).read_text()==s for f,s in current.items())
changed={f:s for f,s in current.items() if baseline.get(f)!=s}
try:
 for f in changed:
  if f in baseline:Path(f).write_text(baseline[f])
  else:Path(f).unlink()
 with (p/'build-html-baseline.log').open('w') as out:
  subprocess.run(['cargo','build','--offline','--locked','--workspace'],stdout=out,stderr=subprocess.STDOUT,check=True)
 subprocess.run(['python3',str(p/'probe-html.py'),'occurrences-html-before'],check=True)
finally:
 for f,s in changed.items():Path(f).write_text(s)
 assert all(Path(f).read_text()==s for f,s in current.items())
