from pathlib import Path
import subprocess,sys
p=Path(__file__).resolve().parent
pin='/tmp/tekt-linter-none-target/debug/crystalline-lint'
for cmd in [[pin,'.','--checks','v15,v26','--fail-on','warning'],[pin,'--fix-hashes','.'],[pin,'.','--checks','v1,v5,v15,v26','--fail-on','warning'],['python3','tools/ci/check_reverse_lineage.py'],['cargo','test','--offline','--locked','-p','typst-core','--lib','hanging_indent_','--','--test-threads=1']]:
 print(cmd,flush=True);z=subprocess.run(cmd)
 if z.returncode:sys.exit(z.returncode)
