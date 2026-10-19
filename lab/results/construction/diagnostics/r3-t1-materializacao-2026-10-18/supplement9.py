from pathlib import Path
import subprocess,sys,json,gzip
p=Path(__file__).resolve().parent
for label,cmd,allowed in [('style-order-candidate9',['python3',str(p/'style-order-probe.py')],[0]),('context-frontiers-candidate9',['python3',str(p/'context-frontiers.py')],[0]),('audit-candidate9',['python3',str(p/'audit9.py')],[0,1]),('v17-red-candidate9',['/tmp/tekt-linter-none-target/debug/crystalline-lint','.','--checks','v17','--fail-on','warning'],[1])]:
 print(label,flush=True)
 with (p/(label+'.log')).open('w') as out:z=subprocess.run(cmd,stdout=out,stderr=subprocess.STDOUT)
 if z.returncode not in allowed:print((p/(label+'.log')).read_text()[-6000:]);sys.exit(z.returncode)
