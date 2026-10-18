from pathlib import Path
import os,json,gzip,subprocess,sys,tempfile,datetime,re,hashlib
p=Path(__file__).resolve().parent;r=p.parents[2];pin='/tmp/tekt-linter-none-target/debug/crystalline-lint'
os.environ['PATH']=str(Path(pin).parent)+':'+os.environ['PATH'];os.environ['TYPST_VANILLA']=str(r/'lab/typst-original/target/release/typst');os.environ['CRYSTALLINE_PDF_FIXED_EPOCH']='1792281600'
freeze=json.loads(gzip.decompress((p/'candidate-sources.json.gz').read_bytes()))
def check():assert all(Path(f).read_text()==s for f,s in freeze['files'].items())
def run(label,cmd,allowed=(0,)):
 print(label,flush=True)
 with (p/(label+'.log')).open('w') as out:z=subprocess.run(cmd,stdout=out,stderr=subprocess.STDOUT)
 if z.returncode not in allowed:print((p/(label+'.log')).read_text()[-12000:]);sys.exit(z.returncode)
check()
with tempfile.TemporaryDirectory(prefix='tekt-r3-',dir='/dev/shm',ignore_cleanup_errors=True) as temp:
 os.environ['TMPDIR']=temp
 (p/'harness-config.json').write_text(json.dumps(dict(utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),TMPDIR=temp,TYPST_VANILLA=os.environ['TYPST_VANILLA'],fixed_epoch=os.environ['CRYSTALLINE_PDF_FIXED_EPOCH']),indent=2)+'\n')
 run('workspace-rest',['cargo','test','--offline','--locked','--workspace','--exclude','typst-core','--exclude','typst-wiring','--lib','--bins','--tests','--no-fail-fast','--','--test-threads=1'])
 os.environ.pop('CRYSTALLINE_PDF_FIXED_EPOCH')
 run('wiring',['cargo','test','--offline','--locked','-p','typst-wiring','--bins','--tests','--no-fail-fast','--','--test-threads=1'])
 run('doctest',['cargo','test','--offline','--locked','--workspace','--doc','--no-fail-fast','--','--test-threads=1'])
os.environ['RUSTDOCFLAGS']='-D warnings';run('rustdoc',['cargo','doc','--offline','--locked','--workspace','--no-deps'])
run('fmt-final',['cargo','fmt','--all','--check']);run('lineage-final',[pin,'.','--checks','v1,v5,v15,v26','--fail-on','warning']);run('reverse',['python3','tools/ci/check_reverse_lineage.py']);run('architecture',[pin,'.','--format','sarif'],(0,1));run('diff-final',['git','diff','--check']);check()
rows=[]
for log in ['core-candidate7.log','workspace-rest.log','wiring.log','doctest.log']:
 rows.extend(dict(log=log,status=a,passed=int(b),failed=int(c),ignored=int(d)) for a,b,c,d in re.findall(r'^test result: (ok|FAILED)\. (\d+) passed; (\d+) failed; (\d+) ignored;', (p/log).read_text(),re.M))
assert rows and all(row['status']=='ok' and row['failed']==0 for row in rows)
(p/'suite-summary.json').write_text(json.dumps(dict(utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),rows=rows,passed=sum(row['passed'] for row in rows),ignored=sum(row['ignored'] for row in rows),failed=0),indent=2)+'\n')
