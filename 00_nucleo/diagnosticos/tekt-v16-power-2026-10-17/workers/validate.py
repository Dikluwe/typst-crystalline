from pathlib import Path
import datetime,gzip,hashlib,json,os,subprocess,sys,re,tempfile
out=Path(__file__).resolve().parents[1];before=json.load(gzip.open(out/'before.json.gz'));now=lambda:datetime.datetime.now(datetime.timezone.utc).isoformat();sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest();hashes={p:sha(p) for p in before['hashes']};linter='/tmp/tekt-linter-none-target/debug/crystalline-lint';record=dict(start=now(),head=before['head'],hashes=hashes,status=subprocess.check_output(['git','status','--porcelain=v1'],text=True),diffstat=subprocess.check_output(['git','diff','HEAD','--stat'],text=True),commands=[])
commands=[('green',['cargo','test','--offline','--locked','-p','typst-core','--lib','v16_power_','--','--test-threads=1'],0),('probe',['python3',str(out/'workers/probe.py')],0),('build',['cargo','build','--offline','--locked','--workspace'],0),('fmt',['cargo','fmt','--all','--check'],0),('clippy',['cargo','clippy','--offline','--locked','--workspace','--all-targets','--','-D','warnings','-D','unfulfilled_lint_expectations'],0),('lineage',[linter,'.','--checks','v1,v5,v15,v26','--fail-on','warning'],0),('reverse',['python3','tools/ci/check_reverse_lineage.py'],0),('architecture',[linter,'.','--format','sarif'],1),('differential',['python3',str(out/'workers/differential.py')],0),('cli',['python3',str(out/'workers/replay.py')],0),('workspace',['cargo','test','--offline','--locked','--workspace','--no-fail-fast','--','--test-threads=1'],0),('rustdoc',['cargo','doc','--offline','--locked','--workspace','--no-deps'],0),('diff',['git','diff','--check'],0)]
if len(sys.argv)>1:
 previous=json.load(open(out/'validation-result.json'));assert previous['hashes']==hashes
 cut=[c[0] for c in commands].index(sys.argv[1]);assert all(x['exit']==commands[i][2] for i,x in enumerate(previous['commands'][:cut]));record=previous;record['commands']=record['commands'][:cut];commands=commands[cut:]
with tempfile.TemporaryDirectory(prefix='tekt-power-tests-',dir='/dev/shm') as tmp:
 env=dict(os.environ,TMPDIR=tmp,TYPST_VANILLA=str(Path('lab/typst-original/target/release/typst').resolve()));env['PATH']='/tmp/tekt-linter-none-target/debug:'+env['PATH']
 for label,cmd,expected in commands:
  item=dict(label=label,command=cmd,start=now());print(json.dumps(item),flush=True);local=dict(env)
  if label=='rustdoc':local['RUSTDOCFLAGS']='-D warnings'
  with (out/f'validation-{label}.log').open('w') as f:r=subprocess.run(cmd,stdout=f,stderr=subprocess.STDOUT,env=local)
  item.update(exit=r.returncode,end=now());record['commands'].append(item);record.update(end=now(),sources_unchanged=all(sha(p)==h for p,h in hashes.items()));(out/'validation-result.json').write_text(json.dumps(record,indent=2)+'\n');print(json.dumps(item),flush=True);assert record['sources_unchanged'] and r.returncode==expected,item
record['complete']=True;(out/'validation-result.json').write_text(json.dumps(record,indent=2)+'\n');print('VALIDATION PASS',flush=True)
