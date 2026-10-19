from pathlib import Path
import subprocess,json,datetime,hashlib
p=Path(__file__).resolve().parent;out=p/'outputs/regex-oracle';out.mkdir(parents=True,exist_ok=True)
source=r'''#show regex("\\d+"): it => strong(it)
#show regex("\\d+"): it => emph(it)
abc123def'''
f=out/'nested.typ';f.write_text(source);binary=Path('lab/typst-original/target/release/typst').resolve();target=f.with_suffix('.html');cmd=[str(binary),'compile',str(f),str(target),'--features','html'];z=subprocess.run(cmd,text=True,capture_output=True)
r=dict(utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),head=subprocess.check_output(['git','rev-parse','HEAD'],text=True).strip(),diffstat=subprocess.check_output(['git','diff','HEAD','--stat'],text=True),pin='bd5ebec285505889e8ff1d4f68804ab147a699f3',binary_sha256=hashlib.sha256(binary.read_bytes()).hexdigest(),source=source,source_sha256=hashlib.sha256(f.read_bytes()).hexdigest(),command=cmd,exit=z.returncode,stderr=z.stderr,html=target.read_text() if z.returncode==0 else None)
(p/'regex-oracle.json').write_text(json.dumps(r,indent=2)+'\n');print(r['exit'],r['html'] or r['stderr'])
