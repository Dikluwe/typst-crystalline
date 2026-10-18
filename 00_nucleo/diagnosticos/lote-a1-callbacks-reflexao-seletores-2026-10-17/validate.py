from pathlib import Path
import os,sys,subprocess,json,gzip,datetime,hashlib,tempfile,re
root=Path.cwd();p=root/'00_nucleo/diagnosticos/lote-a1-callbacks-reflexao-seletores-2026-10-17';now=lambda:datetime.datetime.now(datetime.timezone.utc).isoformat()
def sha(path):
 h=hashlib.sha256()
 with Path(path).open('rb') as f:
  for block in iter(lambda:f.read(1024*1024),b''):h.update(block)
 return h.hexdigest()
files={str(f):sha(f) for d in ['01_core','02_shell','03_infra','04_wiring','benches','00_nucleo/prompts'] for f in Path(d).rglob('*') if f.is_file() and f.suffix in ['.rs','.md','.toml']}
freeze={'utc':now(),'head':subprocess.check_output(['git','rev-parse','HEAD'],text=True).strip(),'status':subprocess.check_output(['git','status','--porcelain=v1','--untracked-files=all'],text=True),'diffstat':subprocess.check_output(['git','diff','HEAD','--stat'],text=True),'sources':files,'profiles':{k:os.environ.get(k) for k in ['CARGO_BUILD_JOBS','CARGO_PROFILE_DEV_DEBUG','CARGO_PROFILE_TEST_DEBUG','CARGO_INCREMENTAL']},'oracle_sha256':sha(root/'lab/typst-original/target/release/typst')}
(p/'validation-freeze.json.gz').write_bytes(gzip.compress(json.dumps(freeze,indent=2).encode()))
env=dict(os.environ,PATH='/tmp/tekt-linter-none-target/debug:'+os.environ['PATH'],TYPST_VANILLA=str(root/'lab/typst-original/target/release/typst'))
results=[]
def run(label,command,extra=None):
 start=now()
 with (p/(label+'.log')).open('w') as log:r=subprocess.run(command,env=dict(env,**(extra or {})),stdout=log,stderr=subprocess.STDOUT)
 item={'label':label,'command':command,'start':start,'end':now(),'exit':r.returncode};results.append(item);(p/'validation-results.json').write_text(json.dumps(results,indent=2));print(json.dumps(item),flush=True)
 return r.returncode
if run('build',['cargo','build','--offline','--locked','--workspace']):sys.exit(1)
run('probe',['python3',str(p/'probe.py'),'after'])
with tempfile.TemporaryDirectory(prefix='tekt-a1-tests-',dir='/dev/shm',ignore_cleanup_errors=True) as temp:
 run('workspace',['cargo','test','--offline','--locked','--workspace','--no-fail-fast','--','--test-threads=1'],{'TMPDIR':temp})
run('fmt',['cargo','fmt','--all','--check'])
run('clippy',['cargo','clippy','--offline','--locked','--workspace','--all-targets','--','-D','warnings','-D','unfulfilled_lint_expectations'])
run('rustdoc',['cargo','doc','--offline','--locked','--workspace','--no-deps'],{'RUSTDOCFLAGS':'-D warnings'})
run('lineage',['crystalline-lint','.','--checks','v1,v5,v15,v26','--fail-on','warning'])
run('reverse',['python3','tools/ci/check_reverse_lineage.py'])
run('architecture',['crystalline-lint','.','--format','sarif'])
run('diff',['git','diff','--check'])
assert all(sha(f)==h for f,h in files.items()),'sources changed during validation'
summary=[]
for status,passed,failed,ignored in re.findall(r'^test result: (ok|FAILED)\. (\d+) passed; (\d+) failed; (\d+) ignored;', (p/'workspace.log').read_text(),re.M):summary.append({'status':status,'passed':int(passed),'failed':int(failed),'ignored':int(ignored)})
(p/'validation-summary.json').write_text(json.dumps({'utc':now(),'sources_unchanged':True,'suite':summary,'passed':sum(x['passed'] for x in summary),'failed':sum(x['failed'] for x in summary),'ignored':sum(x['ignored'] for x in summary),'results':results,'binary_sha256':sha(root/'target/debug/typst')},indent=2))
sys.exit(0 if all(r['exit']==0 for r in results if r['label']!='architecture') else 1)
