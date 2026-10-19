from pathlib import Path
import subprocess,json,gzip,datetime,sys
p=Path(__file__).resolve().parent;pin='/tmp/tekt-linter-none-target/debug/crystalline-lint'
commands=[('ownership-candidate',[pin,'.','--checks','v15,v26','--fail-on','warning']),('fmt-candidate',['cargo','fmt','--all']),('seal-candidate',[pin,'.','--fix-hashes']),('lineage-candidate',[pin,'.','--checks','v1,v5,v15,v26','--fail-on','warning']),('check-candidate',['cargo','check','--offline','--locked','-p','typst-core','--lib'])]
for label,cmd in commands:
 print(label,flush=True)
 with (p/(label+'.log')).open('w') as log:z=subprocess.run(cmd,stdout=log,stderr=subprocess.STDOUT)
 if z.returncode:sys.exit(z.returncode)
