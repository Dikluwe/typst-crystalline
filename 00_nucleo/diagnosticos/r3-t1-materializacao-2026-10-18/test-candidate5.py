from pathlib import Path
import subprocess,sys,gzip,json,datetime
p=Path(__file__).resolve().parent
pin='/tmp/tekt-linter-none-target/debug/crystalline-lint'
for label,cmd in [('ownership-test5',[pin,'.','--checks','v15,v26','--fail-on','warning']),('fmt-test5',['cargo','fmt','--all']),('seal-test5',[pin,'.','--fix-hashes']),('lineage-test5',[pin,'.','--checks','v1,v5,v15,v26','--fail-on','warning'])]:
 print(label,flush=True)
 with (p/(label+'.log')).open('w') as f:z=subprocess.run(cmd,stdout=f,stderr=subprocess.STDOUT)
 if z.returncode:sys.exit(z.returncode)
b=json.loads(gzip.decompress((p/'before-sources.json.gz').read_bytes()))
names=list(b['files'])+['00_nucleo/prompts/_nuclei/eval/textual-recipe-frontier.toml','00_nucleo/prompts/compiler/realize/recipes.md','01_core/src/compiler/realize/recipes.rs']
(p/'candidate5-sources.json.gz').write_bytes(gzip.compress(json.dumps(dict(utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),files={f:Path(f).read_text() for f in names}),ensure_ascii=False).encode()))
sys.exit(subprocess.run(['cargo','test','--offline','--locked','-p','typst-core','--lib','--no-fail-fast','--','--test-threads=1']).returncode)
