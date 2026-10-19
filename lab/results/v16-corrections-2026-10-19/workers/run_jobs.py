from pathlib import Path
import datetime,gzip,hashlib,json,os,subprocess,sys,time
root=Path.cwd();out=root/sys.argv[1];out.mkdir(parents=True,exist_ok=True)
now=lambda:datetime.datetime.now(datetime.timezone.utc).isoformat()
cg=Path('/sys/fs/cgroup')/Path('/proc/self/cgroup').read_text().strip().split('::',1)[1].lstrip('/')
mem=lambda:{k:(cg/k).read_text().strip() for k in ['memory.max','memory.high','memory.swap.max','memory.peak','memory.events']}
assert mem()['memory.max']==str(12*1024**3) and mem()['memory.swap.max']=='0',mem()
env=dict(os.environ,CARGO_BUILD_JOBS='1',CARGO_PROFILE_DEV_DEBUG='0',CARGO_PROFILE_TEST_DEBUG='0',CARGO_INCREMENTAL='0',TYPST_VANILLA=str(root/'lab/typst-original/target/release/typst'))
inputs=lambda:{str(p.relative_to(root)):hashlib.sha256(p.read_bytes()).hexdigest() for top in ['01_core','02_shell','03_infra','04_wiring','00_nucleo/prompts','lab/benches'] for p in (root/top).rglob('*') if p.is_file() and p.suffix in ['.rs','.md','.toml']}
start=inputs();state={'utc':now(),'head':subprocess.check_output(['git','rev-parse','HEAD'],text=True).strip(),'status':subprocess.check_output(['git','status','--short'],text=True),'diff_head_stat':subprocess.check_output(['git','diff','HEAD','--stat'],text=True),'inputs':start,'memory_before':mem()}
with gzip.open(out/'state.json.gz','wt') as f:json.dump(state,f)
results=[]
for job in json.loads(Path(sys.argv[2]).read_text()):
 print(now(),job['name'],'START',flush=True);t=time.monotonic()
 with (out/(job['name']+'.log')).open('w') as log:r=subprocess.run(job['command'],cwd=job.get('cwd',root),env={**env,**job.get('environment',{})},stdout=log,stderr=subprocess.STDOUT)
 row={**job,'utc_end':now(),'exit':r.returncode,'seconds':time.monotonic()-t,'memory':mem()};results.append(row)
 (out/'commands.json').write_text(json.dumps(results,indent=2)+'\n');print(now(),job['name'],'END',r.returncode,flush=True)
 if r.returncode!=job.get('expected_exit',0):print((out/(job['name']+'.log')).read_text()[-8000:],flush=True);break
end=inputs();changed=sorted(k for k in start.keys()|end.keys() if start.get(k)!=end.get(k))
(out/'freeze.json').write_text(json.dumps({'utc':now(),'changed':changed,'memory_after':mem()},indent=2)+'\n')
sys.exit(int(len(results)!=len(json.loads(Path(sys.argv[2]).read_text())) or any(r['exit']!=r.get('expected_exit',0) for r in results)))
