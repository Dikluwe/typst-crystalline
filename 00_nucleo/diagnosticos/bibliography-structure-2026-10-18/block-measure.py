from pathlib import Path
import subprocess,json,gzip,hashlib,datetime,sys
p=Path(__file__).resolve().parent
sha=lambda f:hashlib.file_digest(Path(f).open('rb'),'sha256').hexdigest()
freeze={str(f):sha(f) for folder in ('01_core','02_shell','03_infra','04_wiring','00_nucleo/prompts') for f in Path(folder).rglob('*') if f.is_file() and f.suffix in ('.rs','.md','.toml')}
(p/'block-freeze.json.gz').write_bytes(gzip.compress(json.dumps({'utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'head':subprocess.check_output(['git','rev-parse','HEAD'],text=True).strip(),'sources':freeze,'files':{f:Path(f).read_text() for f in freeze}}).encode()))
for label,cmd in [('extra-before',['python3',str(p/'extra.py'),'extra-before']),('build-block',['cargo','build','--offline','--locked','--workspace']),('pagination-block',['python3',str(p/'pagination.py'),'pagination-block']),('extra-block',['python3',str(p/'extra.py'),'extra-block'])]:
 with (p/(label+'.log')).open('w') as log:z=subprocess.run(cmd,stdout=log,stderr=subprocess.STDOUT)
 print(label,z.returncode,flush=True)
 if z.returncode:sys.exit(z.returncode)
assert all(sha(f)==h for f,h in freeze.items())
print('Block candidate',sha('target/debug/typst'))
