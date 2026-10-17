from pathlib import Path
import subprocess
p=Path(__file__).resolve().parent
subprocess.run(['python3',str(p/'materialize.py')],check=True)
subprocess.run(['python3',str(p/'seal_owned.py'),'01_core/src/compiler/eval/tests.rs','01_core/src/compiler/introspect/walk.rs'],check=True)
subprocess.run(['python3',str(p/'guard.py')],check=True)
