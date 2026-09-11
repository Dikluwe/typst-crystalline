"""Exact public-observation predicates for independent P1339 receipts.

Reads only pinned oracle/reference artifacts and supplied raw observations.
No compiler execution, no normalization, no seal/verdict emission.
"""
import argparse, hashlib, json
from pathlib import Path
D=Path(__file__).resolve().parent
PINS={'p1339-positive-oracles.json':'482babb923e306357f1aa70490e4fef8a5a16f772a494819d50feaee6f2c6972',
      'p1339-opaque-oracles.json':'4f33c33353caee89f509fe81ed76a2f631510684645f1c9f6b82341587472ed0',
      'p1339-ab-batch1-cli-plan.json':'55a46cf8390ea7914aba115afaf17e9ca9d07cedfb94e5cf7ce8ad3c0ce1d693'}
CACHE={}
def require(x,label):
 if not x:raise AssertionError(label)
def pinned(path,sha):
 path=Path(path).resolve();require(path.parent==D,'reference outside diagnostics allowlist')
 key=(str(path),sha)
 if key not in CACHE:
  data=path.read_bytes();require(hashlib.sha256(data).hexdigest()==sha,('reference hash',str(path)))
  CACHE[key]=json.loads(data)
 return CACHE[key]
def root(name):return pinned(D/name,PINS[name])
def reference(ref):
 data=pinned(ref['artifact'],ref['sha256'])[ref['collection']][ref['row_index']]
 if ref.get('product_field'):data=data[ref['product_field']]
 return {dimension:data[field] for dimension,field in ref['fields'].items()}
def tuple_of(row,dimensions):
 return {d:row['pdf_exists' if d=='artifact_presence' else d] for d in dimensions}
def unique(raw,id,product,profile,order):
 rows=[r for r in raw['rows'] if (r['id'],r['product'],r['profile'],r['order'])==(id,product,profile,order)]
 require(len(rows)==1,('unique reference cell',id,product,profile,order));return rows[0]
def old_raw_reference(ref,id,product,profile):
 data=pinned(ref['path'],ref['sha256'])
 found=[r for r in data['rows'] if r['id']==id and r['product']==product and r['profile']==profile]
 require(len(found)==1,('old raw reference cell',id,product,profile));return tuple_of(found[0],['exit','stdout','stderr'])

def expected_for(o,row,purpose,focal=None,derived=None):
 e=o['expected_observation_or_reference_with_sha256'];profile=row['profile']
 if purpose=='calibration':product=row['product']
 elif purpose=='mutation_host':product=e.get('mutation_host_product','vanilla')
 else:product=e.get('candidate_product',e.get('candidate_reference_product'))
 if o['id']=='where-l0-empty-vs-bare' and purpose=='candidate':
  require(derived is not None and focal is not None,'causal expected receipt absent')
  rows=[r for r in derived['rows'] if r['profile']==profile and r['order']==row['order']]
  require(len(rows)==1,'unique derived expected');return rows[0]['candidate_expected']
 if 'by_product' in e:return {d:v for d,v in reference(e['by_product'][product][profile]).items() if d in e['dimensions']}
 if o['id']=='opaque-budget-finite':
  return old_raw_reference(o['raw_input_provenance']['previous_reference']['artifact'],o['id'],product,profile)
 require(o['id'].startswith('causal-') and focal is not None,'pending isolated reference absent')
 r=unique(focal,o['id'],product,profile,row['order'])
 require(r['execution']=='Observed' and r['unknown_reason'] is None,'pending isolated reference Unknown')
 return tuple_of(r,['exit','stdout','stderr'])

def evaluate(row,purpose,positive,opaque,focal=None,derived=None):
 lookup={o['id']:o for o in positive['oracles']}
 conditionals={o['id']:o for o in opaque['conditional_angle_controls']}
 if row['id'] in conditionals:
  # Preserve actual raw execution separately. Applicability was resolved before
  # this runner; normalized finite Angle is not an actual NaN receiver test.
  return {'id':row['id'],'raw_execution':row['execution'],'classification':'Unknown','reason':'conditional_angle_receiver_unactivated','positive_credit':0}
 if row['id']=='opaque-budget-workload':
  o=opaque['oracles'][0];e=o['expected_observation_or_reference_with_sha256']['expected_by_product']
  product=row['product'] if purpose=='calibration' else 'vanilla' if purpose=='mutation_host' else 'candidate'
  expected=e[product]
  if product=='vanilla':
   require(row['execution']=='Unknown' and row['unknown_reason']=='timeout','declared opacity must be raw timeout Unknown')
   require(row['timeout']==1 or row['timeout'] is True,'declared deadline provenance')
   require(tuple_of(row,['exit','stdout','stderr'])=={'exit':None,'stdout':'','stderr':''},'exact opaque raw channels')
   return {'id':row['id'],'raw_execution':'Unknown','classification':'Unknown','reason':'declared_observation_budget_opacity','positive_credit':0,'control_satisfied':True}
  require(row['execution']=='Observed' and row['unknown_reason'] is None,'known guard error must not become Unknown')
  wanted=old_raw_reference(expected['reference'],o['id'],'baseline',row['profile'])
  require(tuple_of(row,['exit','stdout','stderr'])==wanted,'known guard error exact preservation')
  return {'id':row['id'],'raw_execution':'Observed','classification':'Preserved','reason':'asymmetric_known_error_control','positive_credit':0,'control_satisfied':True}
 o=lookup[row['id']]
 if row['execution']!='Observed' or row['unknown_reason'] is not None:
  return {'id':row['id'],'raw_execution':row['execution'],'classification':'Unknown','reason':'mandatory_unknown','positive_credit':0}
 wanted=expected_for(o,row,purpose,focal,derived);actual=tuple_of(row,wanted)
 differences=[{'dimension':d,'expected':wanted[d],'actual':actual[d]} for d in wanted if actual[d]!=wanted[d]]
 effect=o.get('show_effect_by_product',{}).get('candidate' if purpose=='candidate' else row['product'],o.get('show_effect'))
 if effect:
  marked=o['show_marker'] in row['stderr']
  valid={'CallbackExecuted':marked and row['exit']==1,
    'CompileSucceededNoCallback':not marked and row['exit']==0 and row['pdf_exists'] is True,
    'OtherDiagnosticBeforeCallback':not marked and row['exit']==1}[effect]
  if not valid:differences.append({'dimension':'causal_show_effect','expected':effect,'actual':{'marker':marked,'exit':row['exit'],'pdf_exists':row['pdf_exists']}})
 return {'id':row['id'],'raw_execution':'Observed','classification':'Violated' if differences else 'Preserved','reason':'observable_difference' if differences else 'exact_designated_reference','differences':differences,'positive_credit':0 if differences else 1}

def check(raw,purpose,ids,focal=None,derived=None):
 positive=root('p1339-positive-oracles.json');opaque=root('p1339-opaque-oracles.json');plan=root('p1339-ab-batch1-cli-plan.json')
 require(raw['plan_sha256']==PINS['p1339-ab-batch1-cli-plan.json'],'raw plan hash')
 expected_ids=set(ids);by_case={c['id']:c for c in plan['cases']}
 require(expected_ids<=set(by_case),'unknown requested case')
 products=['vanilla','baseline'] if purpose=='calibration' else ['candidate'] if purpose=='candidate' else ['vanilla']
 wanted={(id,product,profile,order) for id in expected_ids for product in products for profile in by_case[id].get('profiles',['default','html','a11y','html+a11y']) for order in ['normal','repeat','reverse']}
 cells=[(r['id'],r['product'],r['profile'],r['order']) for r in raw['rows']]
 require(len(cells)==len(set(cells)) and set(cells)==wanted,'exact required matrix without duplicates/missing/extra cells')
 if focal is not None:require(focal['plan_sha256']==PINS['p1339-ab-batch1-cli-plan.json'],'focal plan hash')
 if derived is not None:
  require(derived['plan_sha256']==PINS['p1339-ab-batch1-cli-plan.json'],'derived plan hash')
  require(derived['recipe']==positive['pending_causal_recipe']['artifact'],'derived recipe identity')
  require(derived['candidate_observed'] is False,'no candidate-derived expected')
 results=[]
 for r in raw['rows']:
  require(r['binary_sha256']==raw['binaries'][r['product']]['sha256'],'raw binary binding')
  c=by_case[r['id']];src=c.get('expression')
  if src is not None:require(r['source_sha256']==hashlib.sha256(src.encode()).hexdigest(),'source identity')
  x=evaluate(r,purpose,positive,opaque,focal,derived)
  x.update({k:r[k] for k in ['product','profile','order']});results.append(x)
 for id,product,profile in {(r['id'],r['product'],r['profile']) for r in raw['rows']}:
  rs=[r for r in raw['rows'] if (r['id'],r['product'],r['profile'])==(id,product,profile)]
  stable=[(r['execution'],r['unknown_reason'],r['exit'],r['stdout'],r['stderr'],r['pdf_exists']) for r in rs]
  require(stable[0]==stable[1]==stable[2],('nondeterministic observed dimensions',id,product,profile))
 return results

if __name__=='__main__':
 p=argparse.ArgumentParser();p.add_argument('--raw',required=True);p.add_argument('--purpose',choices=['calibration','candidate','mutation_host'],required=True)
 p.add_argument('--ids',required=True);p.add_argument('--focal');p.add_argument('--focal-sha256');p.add_argument('--derived');p.add_argument('--derived-sha256');a=p.parse_args()
 raw=json.loads(Path(a.raw).read_text())
 focal=pinned(a.focal,a.focal_sha256) if a.focal else None
 derived=pinned(a.derived,a.derived_sha256) if a.derived else None
 if derived is not None:require(derived['raw']=={'path':str(Path(a.focal).resolve()),'sha256':a.focal_sha256},'derived raw identity')
 for result in check(raw,a.purpose,a.ids.split(','),focal,derived):print(json.dumps(result,ensure_ascii=True))
