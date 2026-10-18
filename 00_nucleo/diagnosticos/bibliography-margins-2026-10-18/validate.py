from pathlib import Path
import json,gzip,datetime,subprocess,hashlib,re,os,sys,tempfile
p=Path(__file__).resolve().parent
os.environ['PATH']='/tmp/tekt-linter-none-target/debug:'+os.environ['PATH']
os.environ['TYPST_VANILLA']=str(Path.cwd()/'lab/typst-original/target/release/typst')
now=lambda:datetime.datetime.now(datetime.timezone.utc).isoformat()
def sha(f):
 with Path(f).open('rb') as s:return hashlib.file_digest(s,'sha256').hexdigest()
resume='--resume' in sys.argv
previous=json.loads((p/'first-results.json').read_text()) if resume else []
results=[]
def run(label,cmd,accepted=(0,)):
 if resume and label in {'seal','focused','build','probe','extra'}:
  row=next(x for x in previous if x['label']==label);assert row['exit']==0
  results.append(row);print('reuse verified unchanged source: '+label,flush=True);return
 start=now();print(label,flush=True)
 with (p/(label+'.log')).open('w') as log:z=subprocess.run(cmd,stdout=log,stderr=subprocess.STDOUT)
 results.append(dict(label=label,command=cmd,start=start,end=now(),exit=z.returncode));(p/'results.json').write_text(json.dumps(results,indent=2)+'\n')
 assert z.returncode in accepted,(label,z.returncode)
if resume:
 frozen=json.loads(gzip.decompress((p/'freeze.json.gz').read_bytes()))
 assert all(sha(f)==h for f,h in frozen['sources'].items())
run('seal',['python3',str(p/'seal.py')])
base=json.loads(gzip.decompress((p/'before-sources.json.gz').read_bytes()))
files={f:sha(f) for f in base['files']};files['00_nucleo/prompts/_nuclei/layout/block-margin-priority.toml']=sha('00_nucleo/prompts/_nuclei/layout/block-margin-priority.toml')
freeze={'utc':now(),'head':base['head'],'status':subprocess.check_output(['git','status','--porcelain=v1','--untracked-files=all'],text=True),'diffstat':subprocess.check_output(['git','diff','HEAD','--stat'],text=True),'sources':files}
if not resume:(p/'freeze.json.gz').write_bytes(gzip.compress(json.dumps(freeze).encode()))
run('focused',['cargo','test','--offline','--locked','-p','typst-core','--lib','a2_','--','--test-threads=1'])
run('build',['cargo','build','--offline','--locked','--workspace'])
run('probe',['python3',str(p/'probe.py'),'after'])
run('extra',['python3',str(p/'extra.py'),'extra-after'])
run('audit',['python3',str(p/'audit.py')])
run('clippy',['cargo','clippy','--offline','--locked','--workspace','--all-targets','--','-D','warnings','-D','unfulfilled_lint_expectations'])
with tempfile.TemporaryDirectory(prefix='tekt-a2-margin-tests-',dir='/dev/shm',ignore_cleanup_errors=True) as temp:
 os.environ['TMPDIR']=temp
 run('workspace',['cargo','test','--offline','--locked','--workspace','--lib','--bins','--tests','--no-fail-fast','--','--test-threads=1'])
 os.environ.pop('TMPDIR',None)
run('doctest',['cargo','test','--offline','--locked','--workspace','--doc','--no-fail-fast','--','--test-threads=1'])
os.environ['RUSTDOCFLAGS']='-D warnings';run('rustdoc',['cargo','doc','--offline','--locked','--workspace','--no-deps'])
run('fmt',['cargo','fmt','--all','--check'])
run('lineage',['crystalline-lint','.','--checks','v1,v5,v15,v26','--fail-on','warning'])
run('reverse',['python3','tools/ci/check_reverse_lineage.py'])
run('architecture',['crystalline-lint','.','--format','sarif'],accepted=(0,1))
run('lint-diff',['python3',str(p/'lint-diff.py'),'architecture.log'])
run('diff',['git','diff','--check'])
assert all(sha(f)==h for f,h in files.items())
suite=[{'status':a,'passed':int(b),'failed':int(c),'ignored':int(d)} for a,b,c,d in re.findall(r'^test result: (ok|FAILED)\. (\d+) passed; (\d+) failed; (\d+) ignored;',(p/'workspace.log').read_text(),re.M)]
assert suite and all(x['failed']==0 for x in suite)
summary={'utc':now(),'suite':suite,'passed':sum(x['passed'] for x in suite),'failed':sum(x['failed'] for x in suite),'ignored':sum(x['ignored'] for x in suite),'binary_sha256':sha('target/debug/typst'),'sources_count':len(files),'sources_unchanged':True,'results':results}
(p/'summary.json').write_text(json.dumps(summary,indent=2)+'\n');print(json.dumps({k:v for k,v in summary.items() if k not in ['suite','results']}),flush=True)
