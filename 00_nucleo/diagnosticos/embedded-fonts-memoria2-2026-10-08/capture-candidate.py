from pathlib import Path
import gzip,json,subprocess,hashlib,datetime
R=Path.cwd();D=R/'00_nucleo/diagnosticos/embedded-fonts-memoria2-2026-10-08';b=json.loads(gzip.open(D/'before.json.gz','rt').read());files={n:(R/n).read_text() for n in b['files']};changed=[n for n,v in files.items() if v!=b['files'][n]];assert set(changed)=={'03_infra/src/fonts.rs','03_infra/src/embedded_fonts.rs','00_nucleo/prompts/infra/fonts.md','00_nucleo/prompts/infra/embedded_fonts.md'}
x={'utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'head':subprocess.check_output(['git','rev-parse','HEAD'],text=True).strip(),'status':subprocess.check_output(['git','status','--short','--untracked-files=all'],text=True),'diff_stat':subprocess.check_output(['git','diff','HEAD','--stat'],text=True),'files':files,'changed_since_before':changed,'binary_sha256':hashlib.sha256((R/'target/release/typst').read_bytes()).hexdigest()}
with gzip.open(D/'candidate.json.gz','wt') as f:json.dump(x,f)
print(x['binary_sha256'])
