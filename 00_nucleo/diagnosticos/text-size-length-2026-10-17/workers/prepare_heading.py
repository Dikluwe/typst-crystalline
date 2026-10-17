from pathlib import Path
import subprocess
out=Path(__file__).resolve().parents[1]
for command in [['python3',str(out/'workers/archive_candidate.py')],['python3',str(out/'workers/extended.py'),'refuters-before']]:
 result=subprocess.run(command);assert result.returncode==0,command
