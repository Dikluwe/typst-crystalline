import subprocess,sys
from pathlib import Path
p=Path(__file__).resolve().parent
for command in [['cargo','build','--offline','--locked','--workspace'],['python3',str(p/'full-probe.py'),sys.argv[1] if len(sys.argv)>1 else 'r2s-measurement']]:
 print(command,flush=True)
 result=subprocess.run(command)
 if result.returncode:sys.exit(result.returncode)
