from pathlib import Path
import subprocess,os,json,gzip,hashlib,datetime,re,sys,tempfile
r=Path.cwd();p=Path(__file__).resolve().parent
now=lambda:datetime.datetime.now(datetime.timezone.utc).isoformat()
sha=lambda f:hashlib.sha256(Path(f).read_bytes()).hexdigest()
files={str(f):sha(f) for folder in ('01_core','02_shell','03_infra','04_wiring','00_nucleo/prompts') for f in Path(folder).rglob('*') if f.is_file() and f.suffix in ('.rs','.toml','.md')}
freeze={'utc':now(),'head':subprocess.check_output(['git','rev-parse','HEAD'],text=True).strip(),'status':subprocess.check_output(['git','status','--porcelain=v1','--untracked-files=all'],text=True),'diffstat':subprocess.check_output(['git','diff','HEAD','--stat'],text=True),'sources':files}
(p/'complete-freeze.json.gz').write_bytes(gzip.compress(json.dumps(freeze).encode()))
env=dict(os.environ,PATH='/tmp/tekt-linter-none-target/debug:'+os.environ['PATH'],TYPST_VANILLA=str(r/'lab/typst-original/target/release/typst'))
results=[]
def run(label,cmd,extra=None,accepted=(0,)):
 start=now()
 with (p/('complete-'+label+'.log')).open('w') as log:z=subprocess.run(cmd,env=dict(env,**(extra or {})),stdout=log,stderr=subprocess.STDOUT)
 row={'label':label,'command':cmd,'start':start,'end':now(),'exit':z.returncode};results.append(row);(p/'complete-results.json').write_text(json.dumps(results,indent=2)+'\n');print(json.dumps(row),flush=True)
 if z.returncode not in accepted:sys.exit(z.returncode or 1)
run('build',['cargo','build','--offline','--locked','--workspace'])
run('probe',['python3',str(p/'probe.py'),'after'])
run('textual-controls',['python3',str(p/'textual-controls.py'),'textual-controls-after'])
run('probe-audit',['python3',str(p/'audit.py')])
run('clippy',['cargo','clippy','--offline','--locked','--workspace','--all-targets','--','-D','warnings','-D','unfulfilled_lint_expectations'])
with tempfile.TemporaryDirectory(prefix='tekt-a2-tests-',dir='/dev/shm',ignore_cleanup_errors=True) as temp:
 run('workspace',['cargo','test','--offline','--locked','--workspace','--lib','--bins','--tests','--no-fail-fast','--','--test-threads=1'],{'TMPDIR':temp})
run('doctest',['cargo','test','--offline','--locked','--workspace','--doc','--no-fail-fast','--','--test-threads=1'])
run('rustdoc',['cargo','doc','--offline','--locked','--workspace','--no-deps'],{'RUSTDOCFLAGS':'-D warnings'})
assert all((r/'target/doc'/name/'index.html').is_file() for name in ('typst_core','typst_shell','typst_infra','typst_wiring'))
run('fmt',['cargo','fmt','--all','--check'])
run('lineage',['crystalline-lint','.','--checks','v1,v5,v15,v26','--fail-on','warning'])
run('reverse',['python3','tools/ci/check_reverse_lineage.py'])
run('architecture',['crystalline-lint','.','--format','sarif'],accepted=(0,1))
run('lint-diff',['python3',str(p/'lint-diff.py')])
run('diff',['git','diff','--check'])
assert all(sha(f)==h for f,h in files.items()),'sources changed during validation'
suite=[{'status':a,'passed':int(b),'failed':int(c),'ignored':int(d)} for a,b,c,d in re.findall(r'^test result: (ok|FAILED)\. (\d+) passed; (\d+) failed; (\d+) ignored;', (p/'complete-workspace.log').read_text(),re.M)]
summary={'utc':now(),'sources_unchanged':True,'suite':suite,'passed':sum(x['passed'] for x in suite),'failed':sum(x['failed'] for x in suite),'ignored':sum(x['ignored'] for x in suite),'results':results,'binary_sha256':sha(r/'target/debug/typst')}
(p/'complete-summary.json').write_text(json.dumps(summary,indent=2)+'\n');print(json.dumps(summary),flush=True)
