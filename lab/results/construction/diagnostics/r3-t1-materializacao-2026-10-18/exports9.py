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
for name,cmd in [('after-candidate9',['python3',str(p/'survey9.py'),'after-candidate9']),('style-after-candidate9',['python3',str(p/'style-probe9.py'),'style-after-candidate9']),('html-after-candidate9',['python3',str(p/'html-survey9.py')]),('exports-check-candidate9',['python3',str(p/'check-exports9.py')])]:
 print(name,flush=True)
 with (p/(name+'.log')).open('w') as out:z=subprocess.run(cmd,stdout=out,stderr=subprocess.STDOUT)
 if z.returncode:print((p/(name+'.log')).read_text()[-8000:]);sys.exit(z.returncode)
check()
