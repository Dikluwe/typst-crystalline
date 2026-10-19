from pathlib import Path
import subprocess,sys,gzip,json
p=Path(__file__).resolve().parent
freeze=json.loads(gzip.decompress((p/'candidate9-sources.json.gz').read_bytes()))
def check():assert all(Path(f).read_text()==s for f,s in freeze['files'].items())
def run(name,cmd):
 check();print(name,flush=True)
 with (p/(name+'.log')).open('w') as out:z=subprocess.run(cmd,stdout=out,stderr=subprocess.STDOUT)
 if z.returncode:print((p/(name+'.log')).read_text()[-12000:],flush=True);sys.exit(z.returncode)
 check()
run('exports-candidate9-worker',['python3',str(p/'exports9.py'),'candidate9'])
run('workspace-candidate9-worker',['python3',str(p/'validate-workspace9.py')])
run('mixed-candidate9-worker',['python3',str(p/'mixed-regressions9.py')])
run('alias-proof-candidate9-worker',['python3',str(p/'alias-proof9.py')])
run('ram-candidate9-worker',['python3',str(p/'ram-probe9.py')])
