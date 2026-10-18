from pathlib import Path
import gzip,json,hashlib,subprocess,re,datetime,os,sys,shutil
p=Path(__file__).resolve().parent;r=Path.cwd();now=lambda:datetime.datetime.now(datetime.timezone.utc).isoformat()
def sha(f):
 with Path(f).open('rb') as stream:return hashlib.file_digest(stream,'sha256').hexdigest()
old=json.loads(gzip.decompress((p/'broad-suite-sources.json.gz').read_bytes()));oldfiles=old['files'];results=[];env=dict(os.environ,PATH='/tmp/tekt-linter-none-target/debug:'+os.environ['PATH'])
def run(label,cmd,accepted=(0,)):
 start=now()
 with (p/('final-'+label+'.log')).open('w') as log:z=subprocess.run(cmd,env=env,stdout=log,stderr=subprocess.STDOUT)
 row={'label':label,'command':cmd,'start':start,'end':now(),'exit':z.returncode};results.append(row);(p/'final-gates-results.json').write_text(json.dumps(results,indent=2)+'\n');print(json.dumps(row),flush=True)
 if z.returncode not in accepted:sys.exit(z.returncode or 1)
run('seal',['python3',str(p/'seal.py')])
changed=[f for f,s in oldfiles.items() if Path(f).read_text()!=s]
expected={'01_core/src/compiler/layout/bibliography.rs','01_core/src/compiler/layout/bib_csl/plan.rs','00_nucleo/prompts/compiler/layout/bibliography.md','00_nucleo/prompts/compiler/layout/bib_csl/plan.md'};assert set(changed)==expected,changed
production=[]
for f in changed:
 if not f.endswith('.rs'):continue
 before=oldfiles[f];after=Path(f).read_text()
 # Both known files place all productive declarations before cfg(test) modules.
 marker='\n#[cfg(test)]\n';assert marker in before and marker in after
 normalize=lambda s:re.sub(r'^//! @prompt-hash [a-f0-9]+\n','',s.split(marker,1)[0],flags=re.M)
 a,b=normalize(before),normalize(after);assert a==b,'productive body changed: '+f
 production.append({'file':f,'normalized_productive_sha256':hashlib.sha256(a.encode()).hexdigest(),'test_modules_begin':before.index(marker),'method':'Exact productive prefix through first cfg(test) module; omit only prompt-hash comment. All remaining edits are inside the known test modules.'})
proof={'utc':now(),'broad_suite_sources':'broad-suite-sources.json.gz','changed_after_broad':changed,'production_unchanged':True,'productive_comparison':production,'only_changes':'Fixture guard + explicit identity asserts, no-prefix legacy projection coverage for false/true hanging, owner L0 prescriptions/hashes.'};(p/'production-equivalence.json').write_text(json.dumps(proof,indent=2)+'\n')
files={f:sha(f) for f in oldfiles};freeze=dict(old['freeze'],utc=now(),sources=files,status=subprocess.check_output(['git','status','--porcelain=v1','--untracked-files=all'],text=True),diffstat=subprocess.check_output(['git','diff','HEAD','--stat'],text=True));(p/'final-freeze.json.gz').write_bytes(gzip.compress(json.dumps(freeze).encode()))
run('focused',['cargo','test','--offline','--locked','-p','typst-core','--lib','a2_','--','--test-threads=1'])
run('clippy',['cargo','clippy','--offline','--locked','--workspace','--all-targets','--','-D','warnings','-D','unfulfilled_lint_expectations'])
run('fmt',['cargo','fmt','--all','--check']);run('lineage',['crystalline-lint','.','--checks','v1,v5,v15,v26','--fail-on','warning']);run('reverse',['python3','tools/ci/check_reverse_lineage.py'])
run('architecture',['crystalline-lint','.','--format','sarif'],accepted=(0,1));run('lint-diff',['python3',str(p/'lint-diff.py'),'final-architecture.log']);run('diff',['git','diff','--check'])
assert all(sha(f)==h for f,h in files.items())
suite=[{'status':a,'passed':int(b),'failed':int(c),'ignored':int(d)} for a,b,c,d in re.findall(r'^test result: (ok|FAILED)\. (\d+) passed; (\d+) failed; (\d+) ignored;',(p/'complete-workspace.log').read_text(),re.M)]
broad=json.loads((p/'broad-results.json').read_text());keep={'build','probe','pagination','extra','legacy','textual','custom','colors','audit','workspace','doctest','rustdoc'}
effective=[dict(x,cohort='broad') for x in broad if x['label'] in keep]+[dict(x,cohort='final-fixtures') for x in results]
summary={'utc':now(),'sources_unchanged_during_each_cohort':True,'production_unchanged_after_broad':True,'broad_freeze':'complete-freeze.json.gz','final_freeze':'final-freeze.json.gz','equivalence':'production-equivalence.json','broad_results':'broad-results.json','final_results':'final-gates-results.json','suite':suite,'passed':sum(x['passed'] for x in suite),'failed':sum(x['failed'] for x in suite),'ignored':sum(x['ignored'] for x in suite),'results':effective,'binary_sha256':sha('target/debug/typst')};(p/'complete-summary.json').write_text(json.dumps(summary,indent=2)+'\n');print(json.dumps({'passed':summary['passed'],'failed':summary['failed'],'ignored':summary['ignored'],'production_unchanged':True}),flush=True)
