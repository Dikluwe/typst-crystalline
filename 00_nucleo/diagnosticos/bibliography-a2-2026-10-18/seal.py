from pathlib import Path
import json,gzip,subprocess,sys,hashlib,datetime
p=Path(__file__).resolve().parent
base=json.loads(gzip.decompress((p/'before-sources.json.gz').read_bytes()))['files']
expected={'01_core/src/compiler/layout/convergence.rs','00_nucleo/prompts/compiler/layout/convergence.md'}
def changed():return {k for k,v in base.items() if Path(k).read_text()!=v}
assert changed()==expected,changed()
for command in [
 ['cargo','fmt','--all'],
 ['/tmp/tekt-linter-none-target/debug/crystalline-lint','.','--checks','v15,v26','--fail-on','warning'],
 ['/tmp/tekt-linter-none-target/debug/crystalline-lint','--fix-hashes','.'],
 ['/tmp/tekt-linter-none-target/debug/crystalline-lint','.','--checks','v1,v5,v15,v26','--fail-on','warning'],
 ['python3','tools/ci/check_reverse_lineage.py'],
]:
 print(command,flush=True);result=subprocess.run(command)
 if result.returncode:sys.exit(result.returncode)
assert changed()==expected,changed()
print('only the convergence owner and its L0 changed',flush=True)
