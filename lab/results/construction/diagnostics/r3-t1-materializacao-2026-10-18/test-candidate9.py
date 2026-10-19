from pathlib import Path
import subprocess,sys,gzip,json,datetime
p=Path(__file__).resolve().parent
pin='/tmp/tekt-linter-none-target/debug/crystalline-lint'
for label,cmd in [('ownership-test9',[pin,'.','--checks','v15,v26','--fail-on','warning']),('fmt-test9',['cargo','fmt','--all']),('seal-test9',[pin,'.','--fix-hashes']),('lineage-test9',[pin,'.','--checks','v1,v5,v15,v26','--fail-on','warning'])]:
 print(label,flush=True)
 with (p/(label+'.log')).open('w') as f:z=subprocess.run(cmd,stdout=f,stderr=subprocess.STDOUT)
 if z.returncode:sys.exit(z.returncode)
b=json.loads(gzip.decompress((p/'before-sources.json.gz').read_bytes()))
names=list(b['files'])+['00_nucleo/prompts/_nuclei/eval/textual-recipe-frontier.toml','00_nucleo/prompts/compiler/realize/recipes.md','01_core/src/compiler/realize/recipes.rs']
(p/'candidate9-sources.json.gz').write_bytes(gzip.compress(json.dumps(dict(utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),files={f:Path(f).read_text() for f in names}),ensure_ascii=False).encode()))
for test in ['composition_show_par_overrides_outer_set_with_page_configuration','math_bare_insertion_context','p1493_']:
 z=subprocess.run(['cargo','test','--offline','--locked','-p','typst-infra','--lib',test,'--','--test-threads=1'])
 if z.returncode:sys.exit(z.returncode)
