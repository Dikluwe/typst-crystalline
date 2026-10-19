from pathlib import Path
import subprocess,json,gzip,datetime,sys
p=Path(__file__).resolve().parent
pin='/tmp/tekt-linter-none-target/debug/crystalline-lint'
def run(label,cmd,allowed=(0,)):
 print(label,flush=True)
 with (p/(label+'.log')).open('w') as log:z=subprocess.run(cmd,stdout=log,stderr=subprocess.STDOUT)
 assert z.returncode in allowed,(label,z.returncode)
 return z.returncode
run('ownership-pre-red',[pin,'.','--checks','v15,v26','--fail-on','warning'])
run('fmt-red',['cargo','fmt','--all'])
run('seal-red',[pin,'.','--fix-hashes'])
run('lineage-red',[pin,'.','--checks','v1,v5,v15,v26','--fail-on','warning'])
before=json.loads(gzip.decompress((p/'before-sources.json.gz').read_bytes()))
filter=sys.argv[1] if len(sys.argv)>1 else 'r3_t1_'
names=list(before['files'])+['00_nucleo/prompts/_nuclei/eval/textual-recipe-frontier.toml','00_nucleo/prompts/compiler/realize/recipes.md','01_core/src/compiler/realize/recipes.rs']
snapshot='style-red-sources.json.gz' if filter=='r3_t1_style_' else 'red-sources.json.gz'
(p/snapshot).write_bytes(gzip.compress(json.dumps(dict(utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),files={f:Path(f).read_text() for f in names}),ensure_ascii=False).encode()))
result=run('style-unit-red' if filter=='r3_t1_style_' else 'acceptance-red',['cargo','test','--offline','--locked','-p','typst-core','--lib',filter,'--','--test-threads=1'],(0,101))
sys.exit(result)
