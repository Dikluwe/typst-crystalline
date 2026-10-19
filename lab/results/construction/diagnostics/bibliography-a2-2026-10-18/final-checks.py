from pathlib import Path
import subprocess,os,json,gzip,hashlib,datetime,re,sys
r=Path.cwd();p=Path(__file__).resolve().parent;now=lambda:datetime.datetime.now(datetime.timezone.utc).isoformat();sha=lambda f:hashlib.sha256(Path(f).read_bytes()).hexdigest()
freeze=json.loads(gzip.decompress((p/'complete-freeze.json.gz').read_bytes()));results=json.loads((p/'complete-results.json').read_text())
prior_audit=results[-1]
assert prior_audit['label']=='custom-csl' and prior_audit['exit']==1
results=results[:-1]
assert [x['label'] for x in results]==['build','probe','textual-controls','probe-audit','clippy','workspace','doctest','rustdoc']
assert all(x['exit']==0 for x in results)
original=json.loads((p/'complete-resources.json').read_text());assert original['exit']==1
log=(p/'complete.log').read_text();assert 'AssertionError' in log and "'typst_wiring'" in log
assert all(sha(f)==h for f,h in freeze['sources'].items())
# Cargo package typst-wiring exposes a bin named typst, as its manifest declares.
metadata=json.loads(subprocess.check_output(['cargo','metadata','--offline','--no-deps','--format-version','1'],text=True))
indices=[]
for package in metadata['packages']:
 if package['id'] in metadata['workspace_members']:
  for target in package['targets']:
   if target.get('doc') and any(k in target['kind'] for k in ('lib','bin')):
    index=r/'target/doc'/target['name'].replace('-','_')/'index.html'
    assert index.is_file(),index;indices.append(str(index))
assert len(indices)==4,indices
(p/'rustdoc-indices.json').write_text(json.dumps({'utc':now(),'indices':indices,'source':'cargo metadata target names, including bin typst from typst-wiring','corrected_driver_assertion':'Original verifier expected nonexistent typst_wiring/index.html. Product Rustdoc command succeeded; verify the four declared targets instead.'},indent=2)+'\n')
env=dict(os.environ,PATH='/tmp/tekt-linter-none-target/debug:'+os.environ['PATH'])
def run(label,cmd,accepted=(0,)):
 start=now()
 with (p/('complete-'+label+'.log')).open('w') as output:z=subprocess.run(cmd,env=env,stdout=output,stderr=subprocess.STDOUT)
 row={'label':label,'command':cmd,'start':start,'end':now(),'exit':z.returncode};results.append(row);(p/'complete-results.json').write_text(json.dumps(results,indent=2)+'\n');print(json.dumps(row),flush=True)
 if z.returncode not in accepted:sys.exit(z.returncode or 1)
run('custom-csl',['python3',str(p/'custom-csl.py'),'custom-csl-results'])
run('geometry-controls',['python3',str(p/'geometry-controls.py')])
run('fmt',['cargo','fmt','--all','--check'])
run('lineage',['crystalline-lint','.','--checks','v1,v5,v15,v26','--fail-on','warning'])
run('reverse',['python3','tools/ci/check_reverse_lineage.py'])
run('architecture',['crystalline-lint','.','--format','sarif'],accepted=(0,1))
run('lint-diff',['python3',str(p/'lint-diff.py')])
run('diff',['git','diff','--check'])
assert all(sha(f)==h for f,h in freeze['sources'].items())
suite=[{'status':a,'passed':int(b),'failed':int(c),'ignored':int(d)} for a,b,c,d in re.findall(r'^test result: (ok|FAILED)\. (\d+) passed; (\d+) failed; (\d+) ignored;', (p/'complete-workspace.log').read_text(),re.M)]
summary={'utc':now(),'sources_unchanged':True,'suite':suite,'passed':sum(x['passed'] for x in suite),'failed':sum(x['failed'] for x in suite),'ignored':sum(x['ignored'] for x in suite),'results':results,'binary_sha256':sha(r/'target/debug/typst'),'rustdoc_indices':indices,'prior_custom_audit_failure':prior_audit,'custom_default_title_parity':'OPEN_PREEXISTING_LAYOUT, exact before comparison; not counted GREEN','original_driver_exit':original['exit'],'original_driver_error':'wrong expected Rustdoc index typst_wiring; no failing product command, all source hashes unchanged; remaining checks resumed without repeating passed suites'}
(p/'complete-summary.json').write_text(json.dumps(summary,indent=2)+'\n');print(json.dumps(summary),flush=True)
