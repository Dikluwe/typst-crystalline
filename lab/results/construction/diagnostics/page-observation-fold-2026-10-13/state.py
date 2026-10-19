import datetime,gzip,hashlib,json,subprocess,sys
from pathlib import Path
D=Path(__file__).resolve().parent; R=D.parents[2]
sha=lambda p:hashlib.sha256(p.read_bytes()).hexdigest()
files=[p for base in ['01_core','02_shell','03_infra','04_wiring','00_nucleo/prompts'] for p in (R/base).rglob('*') if p.is_file() and p.suffix in ['.rs','.md','.toml']]
x={'utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'head':subprocess.check_output(['git','rev-parse','HEAD'],cwd=R,text=True).strip(),'status':subprocess.check_output(['git','status','--short','--untracked-files=all'],cwd=R,text=True),'diffstat':subprocess.check_output(['git','diff','HEAD','--stat'],cwd=R,text=True),'sha256':{str(p.relative_to(R)):sha(p) for p in files},'binaries':{str(p):sha(p) for p in [R/'target/release/typst',R/'lab/typst-original/target/release/typst']},'owner_originals':{str(p.relative_to(R)):p.read_text() for p in [R/'01_core/src/compiler/introspect/from_tags/counter_updates.rs',R/'00_nucleo/prompts/compiler/introspect/from_tags/counter_updates.md']}}
with gzip.open(D/(sys.argv[1]+'.json.gz'),'wt') as f:json.dump(x,f)
