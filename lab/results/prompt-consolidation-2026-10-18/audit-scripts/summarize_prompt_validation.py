from pathlib import Path
import json,re,hashlib,datetime
root=Path('/repos/Antigravity/typst-crystalline');out=root/'lab/results/prompt-consolidation-2026-10-18';v=out/'validation'
commands=json.loads((v/'validation-commands.json').read_text());freeze=json.loads((v/'validation-freeze.json').read_text())
assert len(commands)==12 and all(c['exit']==0 for c in commands),commands
assert freeze['completed_jobs']==freeze['expected_jobs']==12 and not freeze['changed_inputs'],freeze
counts={}
for name in ['tests-domain-infra','tests-wiring','doctests']:
 p=v/(name+'.log');s=p.read_text();rows=[]
 for n,line in enumerate(s.splitlines(),1):
  m=re.match(r'^test result: (\w+)\. (\d+) passed; (\d+) failed; (\d+) ignored;',line)
  if m:rows.append({'line':n,'status':m[1],'passed':int(m[2]),'failed':int(m[3]),'ignored':int(m[4])})
 assert rows and all(r['status']=='ok' and r['failed']==0 for r in rows),name
 counts[name]={'log':str(p.relative_to(root)),'sha256':hashlib.sha256(p.read_bytes()).hexdigest(),'binaries':len(rows),'passed':sum(x['passed'] for x in rows),'failed':sum(x['failed'] for x in rows),'ignored':sum(x['ignored'] for x in rows),'summaries':rows}
python_tests=int(re.search(r'Ran (\d+) tests', (v/'python-ci-tests.log').read_text())[1])
summary={'utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'checkpoint':'ef4a08ec3f48b2eecc470d9b2582349665cb29c1','state':'Frozen uncommitted tree; exact input hashes/diff stat/status and tool identities in validation-state.json.gz','jobs_passed':len(commands),'input_count':freeze['input_count'],'changed_inputs':freeze['changed_inputs'],'python_tests_passed':python_tests,'unit_integration_passed':counts['tests-domain-infra']['passed']+counts['tests-wiring']['passed'],'unit_integration_ignored':counts['tests-domain-infra']['ignored']+counts['tests-wiring']['ignored'],'doctests_passed':counts['doctests']['passed'],'doctests_ignored':counts['doctests']['ignored'],'test_groups':counts}
(out/'validation-summary.json').write_text(json.dumps(summary,indent=2)+'\n');print(json.dumps({k:v for k,v in summary.items() if k!='test_groups'},indent=2))
