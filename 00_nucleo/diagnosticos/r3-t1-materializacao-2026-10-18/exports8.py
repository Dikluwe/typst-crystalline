from pathlib import Path
import subprocess,gzip,json,sys,shutil
p=Path(__file__).resolve().parent
label=sys.argv[1]
freeze=p/(label+'-sources.json.gz')
def check():
 assert all(Path(f).read_text()==s for f,s in json.loads(gzip.decompress(freeze.read_bytes()))['files'].items())
check()
for name,cmd in [('clippy-'+label,['cargo','clippy','--offline','--locked','--workspace','--all-targets','--','-D','warnings','-D','unfulfilled_lint_expectations']),('build-'+label,['cargo','build','--offline','--locked','--workspace'])]:
 print(name,flush=True)
 with (p/(name+'.log')).open('w') as out:z=subprocess.run(cmd,stdout=out,stderr=subprocess.STDOUT)
 if z.returncode:print((p/(name+'.log')).read_text()[-8000:]);sys.exit(z.returncode)
# Candidate-specific snapshots preserve preceding evidence.
for name,cmd in [('after-candidate8',['python3',str(p/'survey8.py'),'after-candidate8']),('style-after-candidate8',['python3',str(p/'style-probe8.py'),'style-after-candidate8']),('html-after-candidate8',['python3',str(p/'html-survey8.py')]),('exports-check-candidate8',['python3',str(p/'check-exports8.py')])]:
 print(name,flush=True)
 with (p/(name+'.log')).open('w') as out:z=subprocess.run(cmd,stdout=out,stderr=subprocess.STDOUT)
 if z.returncode:print((p/(name+'.log')).read_text()[-8000:]);sys.exit(z.returncode)
check()
