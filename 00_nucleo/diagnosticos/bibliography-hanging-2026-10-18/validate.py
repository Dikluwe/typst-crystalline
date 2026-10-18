from pathlib import Path
import json,gzip,datetime,subprocess,hashlib,os,tempfile,sys,re
p=Path(__file__).resolve().parent
pin='/tmp/tekt-linter-none-target/debug/crystalline-lint'
os.environ['PATH']=str(Path(pin).parent)+':'+os.environ['PATH']
os.environ['TYPST_VANILLA']=str(Path.cwd()/'lab/typst-original/target/release/typst')
os.environ.setdefault('CRYSTALLINE_PDF_FIXED_EPOCH','1792281600')
now=lambda:datetime.datetime.now(datetime.timezone.utc).isoformat()
def sha(f):
 with Path(f).open('rb') as s:return hashlib.file_digest(s,'sha256').hexdigest()
results=[];mode=sys.argv[1]
def run(label,cmd,allowed=(0,)):
 print(label,flush=True);start=now()
 with (p/(label+'.log')).open('w') as log:z=subprocess.run(cmd,stdout=log,stderr=subprocess.STDOUT)
 results.append(dict(label=label,command=cmd,start=start,end=now(),exit=z.returncode));(p/(mode+'-results.json')).write_text(json.dumps(results,indent=2)+'\n')
 assert z.returncode in allowed,(label,z.returncode)
if mode=='measure':
 run('architecture-pre',[pin,'.','--format','sarif'],(0,1))
 run('build',['cargo','build','--offline','--locked','--workspace'])
 for script,label in [('probe.py','after'),('extra.py','extra-after'),('boundary.py','boundary-after'),('repeat.py','repeat-after'),('controls.py','controls-after'),('controls_extra.py','controls-extra-after'),('deco-probe.py','deco-after')]:run(label,['python3',str(p/script),label])
 run('deco-empty-after',['python3',str(p/'deco-empty-probe.py')])
 for script in ['audit.py','causal-audit.py','controls-audit.py','deco-audit.py']:run(script[:-3],['python3',str(p/script)])
 freeze=json.loads(gzip.decompress((p/'freeze.json.gz').read_bytes()))
 assert all(sha(f)==h for f,h in freeze['sources'].items())
else:
 freeze=json.loads(gzip.decompress((p/'freeze.json.gz').read_bytes()))
 assert all(sha(f)==h for f,h in freeze['sources'].items())
 run('clippy',['cargo','clippy','--offline','--locked','--workspace','--all-targets','--','-D','warnings','-D','unfulfilled_lint_expectations'])
 with tempfile.TemporaryDirectory(prefix='tekt-hanging-',dir='/dev/shm',ignore_cleanup_errors=True) as temp:
  os.environ['TMPDIR']=temp
  (p/'harness-config.json').write_text(json.dumps(dict(utc=now(),TMPDIR=temp,TYPST_VANILLA=os.environ['TYPST_VANILLA'],oracle_sha256=sha(os.environ['TYPST_VANILLA'])),indent=2)+'\n')
  run('workspace',['cargo','test','--offline','--locked','--workspace','--lib','--bins','--tests','--no-fail-fast','--','--test-threads=1'])
  run('doctest',['cargo','test','--offline','--locked','--workspace','--doc','--no-fail-fast','--','--test-threads=1'])
 os.environ['RUSTDOCFLAGS']='-D warnings';run('rustdoc',['cargo','doc','--offline','--locked','--workspace','--no-deps'])
 run('fmt',['cargo','fmt','--all','--check'])
 run('lineage',[pin,'.','--checks','v1,v5,v15,v26','--fail-on','warning'])
 run('reverse',['python3','tools/ci/check_reverse_lineage.py'])
 run('architecture',[pin,'.','--format','sarif'],(0,1))
 run('diff',['git','diff','--check'])
 assert all(sha(f)==h for f,h in freeze['sources'].items())
 rows=[dict(status=a,passed=int(b),failed=int(c),ignored=int(d)) for a,b,c,d in re.findall(r'^test result: (ok|FAILED)\. (\d+) passed; (\d+) failed; (\d+) ignored;',(p/'workspace.log').read_text(),re.M)]
 assert rows and all(x['failed']==0 for x in rows)
 (p/'suite-summary.json').write_text(json.dumps(dict(utc=now(),suite=rows,passed=sum(x['passed'] for x in rows),failed=sum(x['failed'] for x in rows),ignored=sum(x['ignored'] for x in rows),sources_count=len(freeze['sources']),binary_sha256=sha('target/debug/typst')),indent=2)+'\n')
