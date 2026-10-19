from pathlib import Path
import subprocess,sys,gzip,json
p=Path(__file__).resolve().parent
freeze=json.loads(gzip.decompress((p/'candidate8-sources.json.gz').read_bytes()))
def check():assert all(Path(f).read_text()==s for f,s in freeze['files'].items())
def run(name,cmd):
 check();print(name,flush=True)
 with (p/(name+'.log')).open('w') as out:z=subprocess.run(cmd,stdout=out,stderr=subprocess.STDOUT)
 if z.returncode:print((p/(name+'.log')).read_text()[-12000:],flush=True);sys.exit(z.returncode)
 check()
run('core-candidate8',['cargo','test','--offline','--locked','-p','typst-core','--lib','--no-fail-fast','--','--test-threads=1'])
run('exports-candidate8-worker',['python3',str(p/'exports8.py'),'candidate8'])
run('workspace-candidate8-worker',['python3',str(p/'validate-workspace8.py')])
run('mixed-candidate8-worker',['python3',str(p/'mixed-regressions8.py')])
run('alias-proof-candidate8-worker',['python3',str(p/'alias-proof8.py')])
run('ram-candidate8-worker',['python3',str(p/'ram-probe8.py')])
