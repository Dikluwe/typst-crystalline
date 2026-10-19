from pathlib import Path
import subprocess,sys,gzip,json
p=Path(__file__).resolve().parent
freeze=json.loads(gzip.decompress((p/'occurrences-final-sources.json.gz').read_bytes()))
def check():
 assert all(Path(f).read_text()==s for f,s in freeze['files'].items())
for label,cmd in [
 ('clippy-occurrences-final',['cargo','clippy','--offline','--locked','--workspace','--all-targets','--','-D','warnings','-D','unfulfilled_lint_expectations']),
 ('build-occurrences-final',['cargo','build','--offline','--locked','--workspace']),
 ('workspace-occurrences-final-worker',['python3',str(p/'validate-occurrences.py')]),
 ('external-probe-occurrences-final',['python3',str(p/'probe-occurrences.py'),'occurrences-final']),
 ('html-probe-occurrences-final',['python3',str(p/'probe-html.py'),'occurrences-html-final'])]:
 check();print(label,flush=True)
 with (p/(label+'.log')).open('w') as out:
  z=subprocess.run(cmd,stdout=out,stderr=subprocess.STDOUT)
 if z.returncode:
  print((p/(label+'.log')).read_text()[-10000:]);sys.exit(z.returncode)
 check()
