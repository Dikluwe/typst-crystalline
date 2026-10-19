import copy,datetime,hashlib,json,pathlib,sys
H=pathlib.Path(__file__).resolve().parent;R=pathlib.Path('/repos/Antigravity/typst-crystalline');sys.path.insert(0,str(H));from replay import typed
sha=lambda p:hashlib.sha256(p.read_bytes()).hexdigest()
m=json.loads((H/'cases-draft.json').read_text());b=json.loads((H/'baseline-224-final/receipt.json').read_text());assert b['passed'];observations={c['name']:c['observations'] for c in b['cases']};deltas=[]
assert not (H/'cases.json').exists()
for c in m['cases']:
 o=observations[c['name']];target=copy.deepcopy(o['product']);rule=c['expected_rule']
 if 'prior_policy' in c and not c['name'].startswith('circle-'):
  assert typed(target)==typed(c['prior_policy']['expected_observation']),c['name']
 if rule=='approved-oracle-diagnostic':target=copy.deepcopy(o['oracle']);assert target['returncode']==1
 elif rule=='approved-oracle-value-retain-local-diagnostics':
  assert target['returncode']==o['oracle']['returncode']==0;target['value']=copy.deepcopy(o['oracle']['value'])
 elif rule=='approved-boolean':target={'returncode':0,'stdout':str(c['expected_boolean']).lower()+'\n','stderr':''}
 else:assert rule=='preserve-fresh-baseline'
 if 'asserted_baseline_boolean' in c:assert target=={'returncode':0,'stdout':str(c['asserted_baseline_boolean']).lower()+'\n','stderr':''},c['name']
 c['expected']=target;c['oracle']=o['oracle'];c['baseline']=o['product']
 if typed(target)!=typed(o['product']):deltas.append(c['name'])
m['baseline_receipt']={'path':str((H/'baseline-224-final/receipt.json').relative_to(R)),'sha256':sha(H/'baseline-224-final/receipt.json')};m['approved_changed_cases']=deltas;m['frozen_utc']=datetime.datetime.now(datetime.timezone.utc).isoformat();(H/'cases.json').write_text(json.dumps(m,indent=2)+'\n');print(json.dumps({'cases':len(m['cases']),'changes':len(deltas),'manifest_sha256':sha(H/'cases.json')}))
