from pathlib import Path
import subprocess,sys,gzip,json
p=Path(__file__).resolve().parent
freeze=json.loads(gzip.decompress((p/'candidate10-sources.json.gz').read_bytes()))
def check():
 assert all(Path(f).read_text()==s for f,s in freeze['files'].items())
for label,cmd in [
 ('clippy-candidate10',['cargo','clippy','--offline','--locked','--workspace','--all-targets','--','-D','warnings','-D','unfulfilled_lint_expectations']),
 ('build-candidate10',['cargo','build','--offline','--locked','--workspace']),
 ('workspace-candidate10-worker',['python3',str(p/'validate-workspace10.py')]),
 ('audit-candidate10-final',['python3',str(p/'audit10.py')])]:
 check();print(label,flush=True)
 with (p/(label+'.log')).open('w') as out:
  z=subprocess.run(cmd,stdout=out,stderr=subprocess.STDOUT)
 if z.returncode:
  print((p/(label+'.log')).read_text()[-10000:]);sys.exit(z.returncode)
 check()
