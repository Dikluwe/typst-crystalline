from pathlib import Path
import subprocess,sys,json,gzip
p=Path(__file__).resolve().parent;b=json.loads(gzip.decompress((p/'before-sources.json.gz').read_bytes()))['files']
roles=json.loads((p/'prescription.json').read_text())['owners'];expected=set()
for owner in roles:
 if owner=='compiler/layout/bib_csl/plan.md':continue
 expected.add('00_nucleo/prompts/'+owner)
 consumer='mod.rs' if owner=='compiler/layout.md' else owner.removeprefix('compiler/layout/').removesuffix('.md')+'.rs'
 expected.add('01_core/src/compiler/layout/'+consumer)

assert {f for f,s in b.items() if Path(f).read_text()!=s}==expected
for cmd in [['/tmp/tekt-linter-none-target/debug/crystalline-lint','.','--checks','v15,v26','--fail-on','warning'],['/tmp/tekt-linter-none-target/debug/crystalline-lint','--fix-hashes','.'],['/tmp/tekt-linter-none-target/debug/crystalline-lint','.','--checks','v1,v5,v15,v26','--fail-on','warning'],['python3','tools/ci/check_reverse_lineage.py']]:
 print(cmd,flush=True);z=subprocess.run(cmd)
 if z.returncode:sys.exit(z.returncode)
assert {f for f,s in b.items() if Path(f).read_text()!=s}==expected
