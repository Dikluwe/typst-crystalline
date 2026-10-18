from pathlib import Path
import json,gzip,subprocess,sys
p=Path(__file__).resolve().parent
base=json.loads(gzip.decompress((p/'before-sources.json.gz').read_bytes()))['files']
expected={f'{root}/{name}.{ext}' for root,ext in [('01_core/src/compiler/layout','rs'),('00_nucleo/prompts/compiler/layout','md')] for name in ['block','heading']}
assert {f for f,s in base.items() if Path(f).read_text()!=s}==expected
for cmd in [['/tmp/tekt-linter-none-target/debug/crystalline-lint','.','--checks','v15,v26','--fail-on','warning'],['/tmp/tekt-linter-none-target/debug/crystalline-lint','--fix-hashes','.'],['/tmp/tekt-linter-none-target/debug/crystalline-lint','.','--checks','v1,v5,v15,v26','--fail-on','warning'],['python3','tools/ci/check_reverse_lineage.py']]:
 print(cmd,flush=True);z=subprocess.run(cmd)
 assert z.returncode==0,cmd
assert {f for f,s in base.items() if Path(f).read_text()!=s}==expected
