from pathlib import Path
import json,gzip,hashlib,datetime,subprocess
p=Path(__file__).resolve().parent
b=json.loads(gzip.decompress((p/'before-sources.json.gz').read_bytes()));files=list(b['files'])+['01_core/src/compiler/layout/paragraph_indent.rs','00_nucleo/prompts/compiler/layout/paragraph_indent.md','00_nucleo/prompts/_nuclei/layout/paragraph-hanging-indent.toml']
d=dict(utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),head=subprocess.check_output(['git','rev-parse','HEAD'],text=True).strip(),status=subprocess.check_output(['git','status','--porcelain=v1','--untracked-files=all'],text=True),diffstat=subprocess.check_output(['git','diff','HEAD','--stat'],text=True),scope='Final source cohort after decoration refinement and empty-body guard; initial cohort is preserved in initial-validation.',sources={f:hashlib.sha256(Path(f).read_bytes()).hexdigest() for f in files})
(p/'freeze.json.gz').write_bytes(gzip.compress(json.dumps(d).encode()));print(len(files),'sources frozen')
