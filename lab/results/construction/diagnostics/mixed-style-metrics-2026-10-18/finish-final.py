from pathlib import Path
import json,gzip,datetime,subprocess,hashlib,os,tempfile,re
p=Path(__file__).resolve().parent
pin='/tmp/tekt-linter-none-target/debug/crystalline-lint'
os.environ['PATH']=str(Path(pin).parent)+':'+os.environ['PATH']
os.environ['TYPST_VANILLA']=str(Path.cwd()/'lab/typst-original/target/release/typst')
os.environ.pop('CRYSTALLINE_PDF_FIXED_EPOCH',None)
now=lambda:datetime.datetime.now(datetime.timezone.utc).isoformat()
def sha(f):
 with Path(f).open('rb') as s:return hashlib.file_digest(s,'sha256').hexdigest()
freeze=json.loads(gzip.decompress((p/'freeze.json.gz').read_bytes()));assert all(sha(f)==h for f,h in freeze['sources'].items())
prior=json.loads((p/'gates-results.json').read_text());assert [(r['label'],r['exit']) for r in prior]==[('build-recheck',0),('clippy',0),('workspace',0),('wiring',101)]
assert (p/'wiring.log').read_text().strip()=='error: no library targets found in package `typst-wiring`'
(p/'wiring.log').rename(p/'wiring-invalid-invocation.log')
results=[]
def run(label,cmd,allowed=(0,)):
 print(label,flush=True);start=now()
 with (p/(label+'.log')).open('w') as log:z=subprocess.run(cmd,stdout=log,stderr=subprocess.STDOUT)
 results.append(dict(label=label,command=cmd,start=start,end=now(),exit=z.returncode));(p/'finish-results.json').write_text(json.dumps(results,indent=2)+'\n');assert z.returncode in allowed,(label,z.returncode)
with tempfile.TemporaryDirectory(prefix='tekt-mixed-wiring-',dir='/dev/shm',ignore_cleanup_errors=True) as temp:
 os.environ['TMPDIR']=temp
 (p/'finish-harness-config.json').write_text(json.dumps(dict(utc=now(),TMPDIR=temp,TYPST_VANILLA=os.environ['TYPST_VANILLA'],fixed_epoch=None,reused_successes=prior[:-1],scope='Only the invalid package invocation is corrected; completed workspace/Clippy/build are reused on the same frozen sources.'),indent=2)+'\n')
 run('wiring',['cargo','test','--offline','--locked','-p','typst-wiring','--bins','--tests','--no-fail-fast','--','--test-threads=1'])
 run('doctest',['cargo','test','--offline','--locked','--workspace','--doc','--no-fail-fast','--','--test-threads=1'])
os.environ['RUSTDOCFLAGS']='-D warnings';run('rustdoc',['cargo','doc','--offline','--locked','--workspace','--no-deps'])
run('fmt',['cargo','fmt','--all','--check'])
run('lineage',[pin,'.','--checks','v1,v5,v15,v26','--fail-on','warning'])
run('reverse',['python3','tools/ci/check_reverse_lineage.py'])
run('architecture',[pin,'.','--format','sarif'],(0,1))
run('diff',['git','diff','--check'])
assert all(sha(f)==h for f,h in freeze['sources'].items())
rows=[dict(status=a,passed=int(b),failed=int(c),ignored=int(d)) for a,b,c,d in re.findall(r'^test result: (ok|FAILED)\. (\d+) passed; (\d+) failed; (\d+) ignored;','\n'.join((p/f).read_text() for f in ['workspace.log','wiring.log']),re.M)]
assert rows and all(x['failed']==0 for x in rows)
(p/'suite-summary.json').write_text(json.dumps(dict(utc=now(),suite=rows,passed=sum(x['passed'] for x in rows),failed=sum(x['failed'] for x in rows),ignored=sum(x['ignored'] for x in rows),sources_count=len(freeze['sources']),binary_sha256=sha('target/debug/typst'),source_freeze='freeze.json.gz',workspace_record='gates-results.json',wiring_record='finish-results.json'),indent=2)+'\n')
