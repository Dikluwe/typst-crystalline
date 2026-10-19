from pathlib import Path
import subprocess,sys,gzip,json
p=Path(__file__).resolve().parent
freeze=json.loads(gzip.decompress((p/'occurrences-verified-sources.json.gz').read_bytes()))
def check():
 assert all(Path(f).read_text()==s for f,s in freeze['files'].items())
for label,cmd in [
 ('clippy-occurrences-verified',['cargo','clippy','--offline','--locked','--workspace','--all-targets','--','-D','warnings','-D','unfulfilled_lint_expectations']),
 ('build-occurrences-verified',['cargo','build','--offline','--locked','--workspace']),
 ('workspace-occurrences-verified-worker',['python3',str(p/'validate-occurrences-verified.py')]),
 ('external-probe-occurrences-verified',['python3',str(p/'probe-occurrences.py'),'occurrences-verified']),
 ('html-probe-occurrences-verified',['python3',str(p/'probe-html.py'),'occurrences-html-verified'])]:
 check();print(label,flush=True)
 with (p/(label+'.log')).open('w') as out:
  z=subprocess.run(cmd,stdout=out,stderr=subprocess.STDOUT)
 if z.returncode:
  print((p/(label+'.log')).read_text()[-10000:]);sys.exit(z.returncode)
 check()
