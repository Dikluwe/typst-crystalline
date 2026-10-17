from pathlib import Path
import subprocess,json,gzip,datetime,hashlib,sys
D=Path(__file__).parent
files=subprocess.check_output(['git','ls-files','--modified'],text=True).splitlines()+subprocess.check_output(['git','ls-files','--others','--exclude-standard','01_core','02_shell','03_infra','04_wiring','00_nucleo/prompts'],text=True).splitlines()
d={'utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'head':subprocess.check_output(['git','rev-parse','HEAD'],text=True).strip(),'diff_stat':subprocess.check_output(['git','diff','HEAD','--stat'],text=True),'status':subprocess.check_output(['git','status','--short'],text=True),'files':{p:Path(p).read_text() for p in files if p.endswith(('.rs','.md','.toml'))},'sha256':{p:hashlib.sha256(Path(p).read_bytes()).hexdigest() for p in files if Path(p).is_file()}}
(D/(sys.argv[1]+'.json.gz')).write_bytes(gzip.compress(json.dumps(d,ensure_ascii=False).encode()))
