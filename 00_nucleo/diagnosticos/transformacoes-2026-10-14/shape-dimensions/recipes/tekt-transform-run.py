from pathlib import Path
import sys,subprocess,json,gzip,hashlib,datetime,os
out=Path('00_nucleo/diagnosticos/transformacoes-2026-10-14/implementation');label=sys.argv[1];cmd=sys.argv[2:]
sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest();now=lambda:datetime.datetime.now(datetime.timezone.utc).isoformat()
files={str(p):sha(p) for top in ['01_core','02_shell','03_infra','04_wiring','benches','00_nucleo/prompts'] for p in Path(top).rglob('*') if p.is_file() and p.suffix in ['.rs','.md','.toml']}
r=dict(start=now(),command=cmd,head=subprocess.check_output(['git','rev-parse','HEAD'],text=True),status=subprocess.check_output(['git','status','--porcelain'],text=True),diffstat=subprocess.check_output(['git','diff','HEAD','--stat'],text=True),files=files)
with gzip.open(out/(label+'-pre.json.gz'),'wt') as f:json.dump(r,f)
env=os.environ.copy();env['CARGO_BUILD_JOBS']='2'
with (out/(label+'.log')).open('w') as f:q=subprocess.run(cmd,env=env,stdout=f,stderr=subprocess.STDOUT)
r.update(end=now(),exit=q.returncode,source_guard=all(sha(p)==h for p,h in files.items()))
with gzip.open(out/(label+'-result.json.gz'),'wt') as f:json.dump(r,f)
print(label,'exit',q.returncode,'guard',r['source_guard'],flush=True);assert r['source_guard'];sys.exit(q.returncode)
