from pathlib import Path
import json,gzip,datetime,subprocess,hashlib,os,tempfile,sys,re,shutil
p=Path(__file__).resolve().parent
pin='/tmp/tekt-linter-none-target/debug/crystalline-lint'
os.environ['PATH']=str(Path(pin).parent)+':'+os.environ['PATH']
os.environ['TYPST_VANILLA']=str(Path.cwd()/'lab/typst-original/target/release/typst')
os.environ['CRYSTALLINE_PDF_FIXED_EPOCH']='1792281600'
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
 assert not (p/'outputs-candidate1').exists()
 shutil.copytree(p/'outputs',p/'outputs-candidate1')
 before=json.loads(gzip.decompress((p/'before-sources.json.gz').read_bytes()))
 sources={f:sha(f) for f in before['files']}
 changed={f:Path(f).read_text() for f,s in before['files'].items() if hashlib.sha256(s.encode()).hexdigest()!=sources[f]}
 (p/'freeze.json.gz').write_bytes(gzip.compress(json.dumps(dict(utc=now(),head=subprocess.check_output(['git','rev-parse','HEAD'],text=True).strip(),diffstat=subprocess.check_output(['git','diff','HEAD','--stat'],text=True),sources=sources,changed_texts=changed,oracle_sha256=sha(os.environ['TYPST_VANILLA']),linter_sha256=sha(pin)),indent=2).encode()))
 run('architecture-pre',[pin,'.','--format','sarif'],(0,1))
 run('build-final',['cargo','build','--offline','--locked','--workspace'])
 for script,label in [('probe.py','after'),('bib-probe.py','bib-after'),('regions.py','regions-after'),('corpus.py','corpus-after'),('corpus_extra.py','corpus-extra-after'),('corpus_boundary.py','corpus-boundary-after'),('corpus_repeat.py','corpus-repeat-after'),('controls.py','controls-after'),('controls_extra.py','controls-extra-after'),('deco-probe.py','deco-after')]:run(label,['python3',str(p/script),label])
 run('deco-empty-after',['python3',str(p/'deco-empty-probe.py')])
 assert all(sha(f)==h for f,h in sources.items())
else:
 freeze=json.loads(gzip.decompress((p/'freeze.json.gz').read_bytes()))
 assert all(sha(f)==h for f,h in freeze['sources'].items())
 run('build-recheck',['cargo','build','--offline','--locked','--workspace'])
 assert sha('target/debug/typst')==json.loads(gzip.decompress((p/'after.json.gz').read_bytes()))['binaries']['tekt']['sha256']
 run('clippy',['cargo','clippy','--offline','--locked','--workspace','--all-targets','--','-D','warnings','-D','unfulfilled_lint_expectations'])
 with tempfile.TemporaryDirectory(prefix='tekt-mixed-',dir='/dev/shm',ignore_cleanup_errors=True) as temp:
  os.environ['TMPDIR']=temp
  (p/'harness-config.json').write_text(json.dumps(dict(utc=now(),TMPDIR=temp,TYPST_VANILLA=os.environ['TYPST_VANILLA'],oracle_sha256=sha(os.environ['TYPST_VANILLA']),fixed_epoch=os.environ['CRYSTALLINE_PDF_FIXED_EPOCH']),indent=2)+'\n')
  run('workspace',['cargo','test','--offline','--locked','--workspace','--exclude','typst-wiring','--lib','--bins','--tests','--no-fail-fast','--','--test-threads=1'])
  os.environ.pop('CRYSTALLINE_PDF_FIXED_EPOCH')
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
 (p/'suite-summary.json').write_text(json.dumps(dict(utc=now(),suite=rows,passed=sum(x['passed'] for x in rows),failed=sum(x['failed'] for x in rows),ignored=sum(x['ignored'] for x in rows),sources_count=len(freeze['sources']),binary_sha256=sha('target/debug/typst')),indent=2)+'\n')
