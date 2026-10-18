from pathlib import Path
import subprocess,sys
p=Path(__file__).resolve().parent
for cmd in [['python3',str(p/'seal.py')],['cargo','test','--offline','--locked','-p','typst-core','--lib','a2_','--','--test-threads=1']]:
 print(cmd,flush=True);z=subprocess.run(cmd)
 if z.returncode:sys.exit(z.returncode)
