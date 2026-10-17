import datetime,gzip,hashlib,json,subprocess
from pathlib import Path
H=Path(__file__).resolve().parent;R=H.parents[2];T=Path('/tmp/tekt-context-cache')
sha=lambda p:hashlib.sha256(p.read_bytes()).hexdigest()
git=lambda *a:subprocess.check_output(['git',*a],cwd=R,text=True)
files=git('diff','--name-only').splitlines()+git('ls-files','--others','--exclude-standard').splitlines()
files=sorted(set(f for f in files if f.endswith(('.rs','.md','.toml')) and f.startswith(('01_core/','03_infra/','00_nucleo/prompts/'))))
d={'utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'head':git('rev-parse','HEAD').strip(),'status':git('status','--short'),'diff_stat':git('diff','HEAD','--stat'),'source_hashes':{f:sha(R/f) for f in files},'sources':{f:(R/f).read_text() for f in files},'cli_sha256':sha(R/'target/release/typst'),'vanilla_sha256':sha(R/'lab/typst-original/target/release/typst'),'logs':{p.name:{'text':p.read_text(),'mtime_utc':datetime.datetime.fromtimestamp(p.stat().st_mtime,datetime.timezone.utc).isoformat()} for p in T.glob('*') if p.suffix in ['.log','.sarif']},'helpers':{p.name:p.read_text() for p in H.glob('*.py')},'scope':json.loads((H/'scope.json').read_text()),'seals':json.loads((H/'scoped-seals.json').read_text())}
with gzip.open(H/'execution.json.gz','wt') as f:json.dump(d,f,ensure_ascii=False,indent=2)
print('Archive saved; CLI SHA-256',d['cli_sha256'])
