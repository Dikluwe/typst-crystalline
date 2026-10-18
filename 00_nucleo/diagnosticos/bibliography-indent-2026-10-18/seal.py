from pathlib import Path
import gzip,json,subprocess,sys
p=Path(__file__).resolve().parent;base=json.loads(gzip.decompress((p/'before-sources.json.gz').read_bytes()))['files']
expected={'01_core/src/compiler/layout/bibliography.rs','01_core/src/compiler/eval/rules/set.rs','01_core/src/compiler/stdlib/structural/par.rs','01_core/src/compiler/eval/tests.rs','00_nucleo/prompts/compiler/layout/bibliography.md','00_nucleo/prompts/compiler/eval/rules/set.md','00_nucleo/prompts/compiler/stdlib/structural/par.md','00_nucleo/prompts/compiler/eval/tests.md'}
assert {f for f,s in base.items() if Path(f).read_text()!=s}==expected
for cmd in [['cargo','fmt','--all'],['/tmp/tekt-linter-none-target/debug/crystalline-lint','.','--checks','v15,v26','--fail-on','warning'],['/tmp/tekt-linter-none-target/debug/crystalline-lint','--fix-hashes','.'],['/tmp/tekt-linter-none-target/debug/crystalline-lint','.','--checks','v1,v5,v15,v26','--fail-on','warning'],['python3','tools/ci/check_reverse_lineage.py']]:
 print(cmd,flush=True);z=subprocess.run(cmd)
 if z.returncode:sys.exit(z.returncode)
assert {f for f,s in base.items() if Path(f).read_text()!=s}==expected
