from pathlib import Path
import subprocess
p=Path(__file__).resolve().parent
subprocess.run(['cargo','test','--offline','--locked','-p','typst-core','--lib','fixpoint_propaga_erro_state_update_callback','--','--test-threads=1'],check=True)
subprocess.run(['python3',str(p/'seal-complete.py')],check=True)
