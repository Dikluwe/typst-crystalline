from pathlib import Path
import subprocess,json,gzip,hashlib,datetime,tarfile,shutil,sys
p=Path(__file__).resolve().parent;r=Path.cwd()
def sha(f):
 with Path(f).open('rb') as stream:return hashlib.file_digest(stream,'sha256').hexdigest()
assert sha('target/debug/typst')=='c2b1b2e93cf34f3c597a0a0fbbd51c8f8ac96a3c6bc0a4d9bd72373599acf899'
(p/'bin').mkdir(exist_ok=True);shutil.copy2('target/debug/typst',p/'bin/typst-before')
with tarfile.open(p/'before-artifacts.tar.gz','w:gz') as tar:tar.add(p/'outputs',arcname='outputs')
freeze={str(f):sha(f) for folder in ('01_core','02_shell','03_infra','04_wiring','00_nucleo/prompts') for f in Path(folder).rglob('*') if f.is_file() and f.suffix in ('.rs','.md','.toml')}
(p/'grid-freeze.json.gz').write_bytes(gzip.compress(json.dumps({'utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'head':subprocess.check_output(['git','rev-parse','HEAD'],text=True).strip(),'sources':freeze,'baseline_binary_source':'predecessor bibliography-indent complete-freeze, also copied as this stage before-sources prior to edits'}).encode()))
for label,cmd in [('pagination-before',['python3',str(p/'pagination.py'),'pagination-before']),('build-grid',['cargo','build','--offline','--locked','--workspace']),('probe-grid',['python3',str(p/'probe.py'),'after-grid']),('pagination-grid',['python3',str(p/'pagination.py'),'pagination-grid'])]:
 with (p/(label+'.log')).open('w') as log:z=subprocess.run(cmd,stdout=log,stderr=subprocess.STDOUT)
 print(label,z.returncode,flush=True)
 if z.returncode:sys.exit(z.returncode)
assert all(sha(f)==h for f,h in freeze.items())
print('Grid measurement complete',sha('target/debug/typst'))
