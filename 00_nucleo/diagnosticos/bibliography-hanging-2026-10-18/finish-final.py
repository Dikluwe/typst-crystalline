from pathlib import Path
import json,gzip,datetime,subprocess,hashlib,re,os,tempfile,ast
p=Path(__file__).resolve().parent;r=Path.cwd();pin='/tmp/tekt-linter-none-target/debug/crystalline-lint';os.environ['PATH']=str(Path(pin).parent)+':'+os.environ['PATH'];os.environ['TYPST_VANILLA']=str(r/'lab/typst-original/target/release/typst')
now=lambda:datetime.datetime.now(datetime.timezone.utc).isoformat()
def sha(f):
 with Path(f).open('rb') as stream:return hashlib.file_digest(stream,'sha256').hexdigest()
freeze=json.loads(gzip.decompress((p/'freeze.json.gz').read_bytes()));assert all(sha(f)==h for f,h in freeze['sources'].items())
text=(p/'workspace.log').read_text();section=text[text.index('---- export::tests::p270_4_pdf_bytes_default_oklab_preserved_p268 stdout ----'):];a=bytes(ast.literal_eval(re.search(r'  left: (\[[^\n]+)',section)[1]));b=bytes(ast.literal_eval(re.search(r' right: (\[[^\n]+)',section)[1]));assert len(a)==len(b);indices=[i for i,(x,y) in enumerate(zip(a,b)) if x!=y];assert len(indices)==4
old=b'20261018100955';new=b'20261018100956';oldiso=b'2026-10-18T10:09:55Z';newiso=b'2026-10-18T10:09:56Z';assert a.replace(old,new).replace(oldiso,newiso)==b
(p/'timestamp-left.pdf').write_bytes(a);(p/'timestamp-right.pdf').write_bytes(b)
(p/'timestamp-difference.json').write_text(json.dumps(dict(utc=now(),bytes=len(a),changed_indices=indices,left_sha256=sha(p/'timestamp-left.pdf'),right_sha256=sha(p/'timestamp-right.pdf'),difference='Only four timestamp bytes, Info CreationDate/ModDate and XMP CreateDate/ModifyDate, 10:09:55Z to10:09:56Z. Replacing only these date values makes entire PDFs equal. Original assert unchanged.',source='03_infra/src/export/builder/metadata.rs:22-28',source_sha256=sha('03_infra/src/export/builder/metadata.rs'),owner='00_nucleo/prompts/infra/export/builder/metadata.md',owner_sha256=sha('00_nucleo/prompts/infra/export/builder/metadata.md')),indent=2)+'\n')
def suites(t):
 out=[]
 for m in re.finditer(r'^\s*Running ([^\n]+)\n(.*?)(?=^\s*Running |\Z)',t,re.M|re.S):
  result=re.search(r'^test result: (ok|FAILED)\. (\d+) passed; (\d+) failed; (\d+) ignored;',m[2],re.M)
  if result:
   a,b,c,d=result.groups();out.append(dict(target=m[1],status=a,passed=int(b),failed=int(c),ignored=int(d)))
 return out
original=suites(text);failed=[x for x in original if x['failed']];assert len(failed)==1 and failed[0]['failed']==1 and 'typst_infra-' in failed[0]['target'],failed
results=[]
def run(label,cmd,allowed=(0,)):
 print(label,flush=True);start=now()
 with (p/(label+'.log')).open('w') as log:z=subprocess.run(cmd,stdout=log,stderr=subprocess.STDOUT)
 results.append(dict(label=label,command=cmd,start=start,end=now(),exit=z.returncode));(p/'finish-results.json').write_text(json.dumps(results,indent=2)+'\n');assert z.returncode in allowed,(label,z.returncode)
os.environ['CRYSTALLINE_PDF_FIXED_EPOCH']=str(int(datetime.datetime(2026,10,18,tzinfo=datetime.timezone.utc).timestamp()))
with tempfile.TemporaryDirectory(prefix='tekt-hanging-final-',dir='/dev/shm',ignore_cleanup_errors=True) as temp:
 os.environ['TMPDIR']=temp
 (p/'timestamp-harness.json').write_text(json.dumps(dict(utc=now(),CRYSTALLINE_PDF_FIXED_EPOCH=os.environ['CRYSTALLINE_PDF_FIXED_EPOCH'],TMPDIR=temp,TYPST_VANILLA=os.environ['TYPST_VANILLA'],scope='Controlled existing runtime PDF clock in affected-target retry. No production, test expectation or source changed; repeat whole infra target.'),indent=2)+'\n')
 run('infra-retry',['cargo','test','--offline','--locked','-p','typst-infra','--lib','--','--test-threads=1'])
 retry=suites((p/'infra-retry.log').read_text());assert len(retry)==1 and retry[0]['failed']==0 and retry[0]['passed']==1349 and retry[0]['ignored']==2,retry
 assert retry[0]['target']==failed[0]['target']
 effective=[retry[0] if x['target']==retry[0]['target'] else x for x in original];assert sum(x['passed'] for x in effective)==9029 and all(x['failed']==0 for x in effective)
 run('doctest',['cargo','test','--offline','--locked','--workspace','--doc','--no-fail-fast','--','--test-threads=1'])
 os.environ['RUSTDOCFLAGS']='-D warnings';run('rustdoc',['cargo','doc','--offline','--locked','--workspace','--no-deps'])
 run('fmt',['cargo','fmt','--all','--check']);run('lineage',[pin,'.','--checks','v1,v5,v15,v26','--fail-on','warning']);run('reverse',['python3','tools/ci/check_reverse_lineage.py']);run('architecture',[pin,'.','--format','sarif'],(0,1));run('lint-diff',['python3',str(p/'lint-diff.py')]);run('diff',['git','diff','--check'])
assert all(sha(f)==h for f,h in freeze['sources'].items())
summary=dict(utc=now(),suite=effective,passed=sum(x['passed'] for x in effective),failed=sum(x['failed'] for x in effective),ignored=sum(x['ignored'] for x in effective),sources_count=len(freeze['sources']),binary_sha256=sha('target/debug/typst'),initial_failure='One PDF byte determinism test crossed a clock second; only four date bytes differed.',repeated_target_passed=1349,unchanged_successful_tests_reused=sum(x['passed'] for x in original if not x['failed']),proof='Identical final source freeze for initial broad suite, affected-target retry and remaining gates. Existing PDF snapshot clock controlled externally; no test weakened.')
(p/'suite-summary.json').write_text(json.dumps(summary,indent=2)+'\n');print(json.dumps({k:v for k,v in summary.items() if k!='suite'},indent=2))
