from pathlib import Path
import subprocess,json,hashlib,datetime,re
root=Path.cwd();out=Path('00_nucleo/diagnosticos/raster-mascaras-2026-10-14')
paths=subprocess.check_output(['rg','-l','FrameItem::Shape','01_core','02_shell','03_infra','04_wiring','benches','-g','*.rs'],text=True).splitlines()
x=subprocess.run(['/tmp/tekt-shape-syntax/target/debug/tekt-shape-syntax']+paths,capture_output=True,text=True,check=True)
nodes=[json.loads(s) for s in x.stdout.splitlines()];constructors=[r for r in nodes if r['kind']=='expr'];consumers=sorted({r['path'] for r in constructors})
r=dict(utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),head=subprocess.check_output(['git','rev-parse','HEAD'],text=True),diffstat=subprocess.check_output(['git','diff','HEAD','--stat'],text=True),note='Read-only inventory for pending T1c. Syntax references are not proof of complete semantic dependency closure.',files={p:dict(sha256=hashlib.sha256(Path(p).read_bytes()).hexdigest(),owner=(re.search(r'^//! @prompt (.+)$',Path(p).read_text(),re.M).group(1) if re.search(r'^//! @prompt (.+)$',Path(p).read_text(),re.M) else None)) for p in paths},nodes=nodes)
(out/'next-shape-syntax.json').write_text(json.dumps(r,indent=2));print(json.dumps(dict(referenced_files=len(paths),constructors=len(constructors),constructor_files=len(consumers),files=consumers),indent=2))
