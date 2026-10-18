from pathlib import Path
import subprocess,sys,gzip,json,datetime
p=Path(__file__).resolve().parent
pin='/tmp/tekt-linter-none-target/debug/crystalline-lint'
for label,cmd in [('ownership-test10',[pin,'.','--checks','v15,v26','--fail-on','warning']),('fmt-test10',['cargo','fmt','--all']),('seal-test10',[pin,'.','--fix-hashes']),('lineage-test10',[pin,'.','--checks','v1,v5,v15,v26','--fail-on','warning'])]:
 print(label,flush=True)
 with (p/(label+'.log')).open('w') as f:z=subprocess.run(cmd,stdout=f,stderr=subprocess.STDOUT)
 if z.returncode:sys.exit(z.returncode)
b=json.loads(gzip.decompress((p/'before-sources.json.gz').read_bytes()))
names=list(b['files'])+['00_nucleo/prompts/_nuclei/eval/textual-recipe-frontier.toml','00_nucleo/prompts/compiler/realize/recipes.md','01_core/src/compiler/realize/recipes.rs']
(p/'candidate10-sources.json.gz').write_bytes(gzip.compress(json.dumps(dict(utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),files={f:Path(f).read_text() for f in names}),ensure_ascii=False).encode()))
for label,cmd in [('v17-green-candidate10',[pin,'.','--checks','v17','--fail-on','warning']),('architecture-candidate10',[pin,'.','--format','sarif'])]:
 print(label,flush=True)
 with (p/(label+'.log')).open('w') as out:z=subprocess.run(cmd,stdout=out,stderr=subprocess.STDOUT)
 if label.startswith('v17') and z.returncode:sys.exit(z.returncode)
 if label.startswith('architecture') and z.returncode not in (0,1):sys.exit(z.returncode)
sys.exit(subprocess.run(['python3',str(p/'audit10.py')]).returncode)
