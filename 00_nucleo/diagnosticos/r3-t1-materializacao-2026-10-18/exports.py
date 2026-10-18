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
shutil.copyfile(freeze,p/'candidate-sources.json.gz')
for name,cmd in [('after',['python3',str(p/'survey.py'),'after']),('style-after',['python3',str(p/'style-probe.py'),'style-after']),('html-after',['python3',str(p/'html-survey.py')]),('exports-check',['python3',str(p/'check-exports.py')])]:
 print(name,flush=True)
 with (p/(name+'.log')).open('w') as out:z=subprocess.run(cmd,stdout=out,stderr=subprocess.STDOUT)
 if z.returncode:print((p/(name+'.log')).read_text()[-8000:]);sys.exit(z.returncode)
check()
