from pathlib import Path
import json,gzip,datetime,subprocess,hashlib,re,os,tempfile
p=Path(__file__).resolve().parent;r=Path.cwd()
now=lambda:datetime.datetime.now(datetime.timezone.utc).isoformat()
def sha(f):
 with Path(f).open('rb') as s:return hashlib.file_digest(s,'sha256').hexdigest()
freeze=json.loads(gzip.decompress((p/'freeze.json.gz').read_bytes()))
assert all(sha(f)==h for f,h in freeze['sources'].items())
def suites(text):
 rows=[]
 for m in re.finditer(r'^\s*Running ([^\n]+)\n(.*?)(?=^\s*Running |\Z)',text,re.M|re.S):
  result=re.search(r'^test result: (ok|FAILED)\. (\d+) passed; (\d+) failed; (\d+) ignored;',m[2],re.M)
  if result:
   a,b,c,d=result.groups();rows.append(dict(target=m[1],status=a,passed=int(b),failed=int(c),ignored=int(d)))
 return rows
old=suites((p/'workspace.log').read_text());bad=[x for x in old if x['failed']]
assert len(bad)==5 and sum(x['failed'] for x in bad)==40,bad
names=['p1293_contract','p1465_format_html','p1467_format_pdf_tagged_pretty','p1468_format_pdf_pages','p1469_format_pdf_standard']
assert all(any('tests/'+n+'.rs' in x['target'] for n in names) for x in bad)
text=(p/'workspace.log').read_text();assert text.count('TMPDIR must be explicit')==34 and text.count('set TYPST_VANILLA to the independently verified oracle')==6
oracle=r/'lab/typst-original/target/release/typst';assert sha(oracle)=='9b2373155e9c05bbb0daf5755bf4be88a11e64bd6b4f8b376648fa27303373b9'
os.environ['TYPST_VANILLA']=str(oracle)
results=[]
def run(label,cmd):
 start=now();print(label,flush=True)
 with (p/(label+'.log')).open('w') as log:z=subprocess.run(cmd,stdout=log,stderr=subprocess.STDOUT)
 results.append(dict(label=label,command=cmd,start=start,end=now(),exit=z.returncode));(p/'finish-results.json').write_text(json.dumps(results,indent=2)+'\n')
 assert z.returncode in ((0,1) if label=='architecture' else (0,)),(label,z.returncode)
with tempfile.TemporaryDirectory(prefix='tekt-a2-margin-tests-',dir='/dev/shm',ignore_cleanup_errors=True) as temp:
 os.environ['TMPDIR']=temp
 (p/'harness-config.json').write_text(json.dumps({'utc':now(),'TYPST_VANILLA':str(oracle),'oracle_sha256':sha(oracle),'TMPDIR':temp,'temporary_directory_created':True,'source_freeze':'freeze.json.gz','correction':'Runtime harness inputs only; no productive/test assertion changes.'},indent=2)+'\n')
 command=['cargo','test','--offline','--locked','-p','typst-wiring']
 for name in names:command.extend(['--test',name])
 command.extend(['--no-fail-fast','--','--test-threads=1']);run('workspace-retry',command)
 new=suites((p/'workspace-retry.log').read_text());assert len(new)==5 and sum(x['passed'] for x in new)==46 and all(x['failed']==0 for x in new),new
 replacements={x['target']:x for x in new};assert set(replacements)=={x['target'] for x in bad}
 effective=[replacements.get(x['target'],x) for x in old];assert sum(x['passed'] for x in effective)==9011 and all(x['failed']==0 for x in effective)
 run('doctest',['cargo','test','--offline','--locked','--workspace','--doc','--no-fail-fast','--','--test-threads=1'])
 os.environ['RUSTDOCFLAGS']='-D warnings';run('rustdoc',['cargo','doc','--offline','--locked','--workspace','--no-deps'])
 run('fmt',['cargo','fmt','--all','--check'])
 run('lineage',['crystalline-lint','.','--checks','v1,v5,v15,v26','--fail-on','warning'])
 run('reverse',['python3','tools/ci/check_reverse_lineage.py'])
 run('architecture',['crystalline-lint','.','--format','sarif'])
 run('lint-diff',['python3',str(p/'lint-diff.py'),'architecture.log'])
 run('diff',['git','diff','--check'])
assert all(sha(f)==h for f,h in freeze['sources'].items())
summary={'utc':now(),'suite':effective,'passed':sum(x['passed'] for x in effective),'failed':sum(x['failed'] for x in effective),'ignored':sum(x['ignored'] for x in effective),'initial_environment_failures':40,'initial_targets_failed':5,'reverified_targets':5,'retry_tests_passed':46,'unchanged_successful_tests_reused':sum(x['passed'] for x in old if not x['failed']),'source_proof':'Exact freeze unchanged across original broad suite and affected-target retry. No test expectations or production changed.','binary_sha256':sha('target/debug/typst'),'sources_count':len(freeze['sources']),'sources_unchanged':True,'broad_results':'results.json','final_results':results}
(p/'summary.json').write_text(json.dumps(summary,indent=2)+'\n');print(json.dumps({k:v for k,v in summary.items() if k not in ['suite','final_results']}),flush=True)
