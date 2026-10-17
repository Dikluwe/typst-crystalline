from pathlib import Path
import os,sys,subprocess,json,datetime,hashlib,gzip
root=Path.cwd();out=root/'00_nucleo/diagnosticos/vanilla-resync-2026-10-17'
now=lambda:datetime.datetime.now(datetime.timezone.utc).isoformat()
def sha(p):return hashlib.sha256(Path(p).read_bytes()).hexdigest()
files={str(p):sha(p) for top in ['01_core','02_shell','03_infra','04_wiring','benches','00_nucleo/prompts'] for p in Path(top).rglob('*') if p.is_file() and p.suffix in ['.rs','.md','.toml']}
state={'utc':now(),'head':subprocess.check_output(['git','rev-parse','HEAD'],text=True).strip(),'status':subprocess.check_output(['git','status','--porcelain=v1','--untracked-files=all'],text=True),'diffstat':subprocess.check_output(['git','diff','HEAD','--stat'],text=True),'source_hashes':files,'oracle':json.loads((out/'oracle-build.json').read_text())}
with gzip.open(out/'validation-state.json.gz','wt') as f:json.dump(state,f,indent=2)
env=dict(os.environ,PATH='/tmp/tekt-linter-none-target/debug:'+os.environ['PATH'],CARGO_BUILD_JOBS='1')
results=[]
def run(label,command,extra=None):
 start=now()
 with (out/(label+'.log')).open('w') as f:r=subprocess.run(command,env=dict(env,**(extra or {})),stdout=f,stderr=subprocess.STDOUT)
 item={'label':label,'command':command,'start':start,'end':now(),'exit':r.returncode};results.append(item);print(json.dumps(item),flush=True)
 (out/'validation-results.json').write_text(json.dumps(results,indent=2)+'\n')
 return r.returncode
run('build',['cargo','build','--offline','--locked','--workspace'])
run('workspace',['python3',str(out/'suite.py')])
run('vanilla-smoke-build',['rustc','--edition=2021','--test','lab/parity/tests/vanilla_cli_smoke.rs','-o',str(out/'vanilla-smoke')],{'CARGO_MANIFEST_DIR':str(root/'lab/parity')})
run('vanilla-smoke',[str(out/'vanilla-smoke'),'--nocapture'])
run('fmt',['cargo','fmt','--all','--check'])
run('clippy',['cargo','clippy','--offline','--locked','--workspace','--all-targets','--','-D','warnings','-D','unfulfilled_lint_expectations'])
run('rustdoc',['cargo','doc','--offline','--locked','--workspace','--no-deps'],{'RUSTDOCFLAGS':'-D warnings'})
run('lineage',['crystalline-lint','.','--checks','v1,v5,v15,v26','--fail-on','warning'])
run('reverse',['python3','tools/ci/check_reverse_lineage.py'])
run('architecture',['crystalline-lint','.','--format','sarif'])
run('diff',['git','diff','--check'])
assert all(sha(p)==h for p,h in files.items()),'Sources drifted during validation'
indices={str(p):p.exists() and p.stat().st_size>0 for p in [Path('target/doc')/n/'index.html' for n in ['typst_core','typst_shell','typst_infra','typst']]}
(out/'validation-integrity.json').write_text(json.dumps({'utc':now(),'sources_unchanged':True,'source_count':len(files),'rustdoc_indices':indices},indent=2)+'\n')
sys.exit(0 if all(r['exit']==0 for r in results if r['label']!='architecture') and all(indices.values()) else 1)
