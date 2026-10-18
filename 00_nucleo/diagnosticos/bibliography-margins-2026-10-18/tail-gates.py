from pathlib import Path
import json,gzip,datetime,subprocess,hashlib,re
p=Path(__file__).resolve().parent;r=Path.cwd();now=lambda:datetime.datetime.now(datetime.timezone.utc).isoformat()
def sha(f):
 with Path(f).open('rb') as s:return hashlib.file_digest(s,'sha256').hexdigest()
freeze=json.loads(gzip.decompress((p/'freeze.json.gz').read_bytes()));base=json.loads(gzip.decompress((p/'before-sources.json.gz').read_bytes()));assert all(sha(f)==h for f,h in freeze['sources'].items())
pin='/tmp/tekt-linter-none-target/debug/crystalline-lint';assert sha(pin)==base['binaries'][pin]=='e2e51f3074cee2d0179bfa89f5b12884728f3d2d8f3113589dd095ef1d9eadb2'
global_report=json.loads(gzip.decompress((p/'lint-diff-global.json.gz').read_bytes()));rows=[]
for x in global_report['added']:
 name=x['file'].removeprefix('./');text=Path(name).read_text();assert text==base['files'][name],name
 prompt=re.search(r'@prompt (\S+)',text)
 rows.append(dict(x,source_sha256=sha(name),unchanged_from_before=True,L0=prompt[1] if prompt else None))
(p/'global-linter-findings.json').write_text(json.dumps({'utc':now(),'CI_pin':'12797073972e375d9ab078cf6750c8b30b71e2b9','pinned_binary':pin,'pinned_binary_sha256':sha(pin),'global_binary':'/home/dikluwe/.cargo/bin/crystalline-lint','global_binary_sha256':sha('/home/dikluwe/.cargo/bin/crystalline-lint'),'global_source_revision_unknown':True,'source_freeze':'freeze.json.gz','findings':rows,'scope':'Nine extra findings in unchanged consumers, observed with a different linter binary. Preserve for investigation; do not silently dismiss or update CI.'},indent=2,ensure_ascii=False)+'\n')
results=[]
def run(label,cmd,accepted=(0,)):
 start=now();print(label,flush=True)
 with (p/(label+'.log')).open('w') as log:z=subprocess.run(cmd,stdout=log,stderr=subprocess.STDOUT)
 results.append(dict(label=label,command=cmd,start=start,end=now(),exit=z.returncode));(p/'tail-results.json').write_text(json.dumps(results,indent=2)+'\n');assert z.returncode in accepted,(label,z.returncode)
run('pinned-lineage',[pin,'.','--checks','v1,v5,v15,v26','--fail-on','warning'])
run('reverse',['python3','tools/ci/check_reverse_lineage.py'])
run('architecture',[pin,'.','--format','sarif'],accepted=(0,1))
run('lint-diff',['python3',str(p/'lint-diff.py'),'architecture.log'])
run('diff',['git','diff','--check'])
def suites(text):
 rows=[]
 for m in re.finditer(r'^\s*Running ([^\n]+)\n(.*?)(?=^\s*Running |\Z)',text,re.M|re.S):
  result=re.search(r'^test result: (ok|FAILED)\. (\d+) passed; (\d+) failed; (\d+) ignored;',m[2],re.M)
  if result:
   a,b,c,d=result.groups();rows.append(dict(target=m[1],status=a,passed=int(b),failed=int(c),ignored=int(d)))
 return rows
old=suites((p/'workspace.log').read_text());new=suites((p/'workspace-retry.log').read_text());bad=[x for x in old if x['failed']];replacements={x['target']:x for x in new}
assert len(bad)==len(new)==5 and sum(x['failed'] for x in bad)==40 and sum(x['passed'] for x in new)==46
assert set(replacements)=={x['target'] for x in bad} and all(x['failed']==0 for x in new)
effective=[replacements.get(x['target'],x) for x in old];assert sum(x['passed'] for x in effective)==9011 and all(x['failed']==0 for x in effective)
previous=json.loads((p/'finish-results-first.json').read_text());assert all(next(x for x in previous if x['label']==n)['exit']==0 for n in ['workspace-retry','doctest','rustdoc','fmt','lineage','reverse'])
assert all(sha(f)==h for f,h in freeze['sources'].items())
summary={'utc':now(),'suite':effective,'passed':sum(x['passed'] for x in effective),'failed':sum(x['failed'] for x in effective),'ignored':sum(x['ignored'] for x in effective),'initial_environment_failures':40,'initial_targets_failed':5,'reverified_targets':5,'retry_tests_passed':46,'unchanged_successful_tests_reused':sum(x['passed'] for x in old if not x['failed']),'source_proof':'Exact freeze unchanged across all cohorts; no expectation/product changes.','binary_sha256':sha('target/debug/typst'),'sources_count':len(freeze['sources']),'sources_unchanged':True,'broad_results':'results.json','retry_results':'finish-results-first.json','final_results':results,'linter_pin':'12797073972e375d9ab078cf6750c8b30b71e2b9','global_linter_additional_findings':'global-linter-findings.json'}
(p/'summary.json').write_text(json.dumps(summary,indent=2)+'\n');print(json.dumps({k:v for k,v in summary.items() if k not in ['suite','final_results']}),flush=True)
