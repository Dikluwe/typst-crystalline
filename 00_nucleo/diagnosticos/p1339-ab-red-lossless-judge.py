"""Record D evidence using the unchanged sealed predicates; no compiler execution."""
import collections, datetime, hashlib, importlib.util, json, subprocess
from pathlib import Path
D=Path(__file__).resolve().parent
def read(p):return json.loads(Path(p).read_text())
def sha(p):return hashlib.sha256(Path(p).read_bytes()).hexdigest()
def pin(p):return {'path':str(Path(p).resolve()),'sha256':sha(p)}
def main():
 sealpath=D/'p1339-seal.json';assert sha(sealpath)=='35f00c4b9e15a010692017f5083ea4f451f4104a3730022136972e151ac0a8ee'
 seal=read(sealpath);rawpath=D/'p1339-ab-red-lossless-runs.json';raw=read(rawpath)
 selection=read(raw['selection']['path']);assert pin(raw['selection']['path'])==raw['selection']
 manifestpath=D/'p1339-ab-red-lossless-manifest.json';manifest=read(manifestpath)
 for item in manifest['inputs']:assert pin(item['path'])==item,('D input drift',item['path'])
 assert raw['adapter']==pin(D/'p1339-ab-red-lossless-executor.py') and raw['seal']==pin(sealpath) and raw['phase']=='red'
 assert raw['processes']==len(raw['rows'])==2706 and raw['integrity_before']['valid'] and raw['integrity_after']['valid']
 for item in [*seal['active_oracles'].values(),seal['active_public_predicate'],seal['active_causal_derived']]:assert pin(item['path'])==item
 positive=read(seal['active_oracles']['positive']['path']);opaque=read(seal['active_oracles']['opaque']['path'])
 spec=importlib.util.spec_from_file_location('p1339_sealed_public',seal['active_public_predicate']['path']);predicate=importlib.util.module_from_spec(spec);spec.loader.exec_module(predicate)
 focalpath=D/'p1339-ab-batch2-focal-runs.json';focal=read(focalpath);derived=read(seal['active_causal_derived']['path']);assert pin(focalpath)==derived['raw']
 ids=selection['ids'];discovery=selection['discovery_ids'];controls=selection['control_ids'];oracles={x['id']:x for x in positive['oracles']}
 judged=predicate.check(raw,'calibration',ids,focal,derived)
 failures=[{'kind':'own_product_or_control_difference','row':x} for x in judged if x['classification']!='Preserved' or x.get('differences')]
 red_rows=[];vanilla_rows=[]
 for index,r in enumerate(raw['rows']):
  if r['id'] not in discovery:continue
  x=predicate.evaluate(r,'candidate',positive,opaque,focal,derived)
  x.update({k:r[k] for k in ['product','profile','order']});x['obligation_ids']=oracles[r['id']]['obligation_ids'];x['raw_row_index']=index
  if r['product']=='baseline':
   red_rows.append(x)
   if not (r['execution']=='Observed' and r['unknown_reason'] is None and r['exit']==1 and x['classification']=='Violated' and x.get('differences')):failures.append({'kind':'not_genuine_absence_RED','row':x})
  else:
   vanilla_rows.append(x)
   if not (r['execution']=='Observed' and r['exit']==0 and x['classification']=='Preserved'):failures.append({'kind':'vanilla_discovery_not_green','row':x})
 assert len(judged)==2706 and len(red_rows)==len(vanilla_rows)==120
 control_rows=[x for x in judged if x['id'] in controls];assert len(control_rows)==2466
 counts={'IDs':124,'baseline_RED_observed_Violated':sum(x['classification']=='Violated' for x in red_rows),'vanilla_discovery_Preserved':sum(x['classification']=='Preserved' for x in vanilla_rows),'control_cells':2466,'controls_Preserved':sum(x['classification']=='Preserved' for x in control_rows),'mandatory_Unknown':sum(r['execution']!='Observed' for r in raw['rows'])}
 incidentpath=D/'p1339-ab-red-executor-call.json';incident=read(incidentpath)
 data={'schema':'p1339-independent-D-RED-v1','at':datetime.datetime.now(datetime.timezone.utc).isoformat(),'author':'/root/p1319_tests','regime':'executado sem atestacao de isolamento','seal':pin(sealpath),'authority_manifest':seal['authority_manifest'],'contract':seal['contract'],'discrimination':seal['discrimination'],
 'status':'GENUINE_BASELINE_RED_WITH_GREEN_CONTROLS' if not failures else 'RED_GATE_FAILED','independent_verifier_status':'PENDING_REVIEW','before':raw['before'],'after':raw['after'],'manifest':pin(manifestpath),'raw':pin(rawpath),'selection':raw['selection'],'adapter':raw['adapter'],'predicate':seal['active_public_predicate'],'derived':seal['active_causal_derived'],
 'processes':2706,'raw_wall_seconds':raw['wall_seconds'],'sum_child_seconds':sum(r['seconds'] for r in raw['rows']),'raw_execution_counts':dict(collections.Counter(r['execution'] for r in raw['rows'])),
 'sealed_integrity':{'raw_pointer':'integrity_before / integrity_after','before_seconds':raw['integrity_before']['seconds'],'after_seconds':raw['integrity_after']['seconds'],'before_valid':raw['integrity_before']['valid'],'after_valid':raw['integrity_after']['valid']},
 'preserved_preexecution_incident':{'receipt':pin(incidentpath),'Typst_processes':0,'seconds':incident['seconds'],'cause':'Frozen comma-split CLI rejected exact IDs containing commas before task creation. The lossless adapter changes transport only; not a product retry or RED.'},
 'discovery_ids':discovery,'control_ids':controls,'own_product_classifications':judged,'baseline_RED_against_sealed_F_reference':red_rows,'vanilla_discovery_green':vanilla_rows,'counts':counts,'failures':failures,'genuine_RED':not failures,
 'future_ledger':{'state':'NotDue_in_D','definitions':seal['phase_F_NotDue_ledger'],'execution_D':0,'credit_D':0,'not_RED':'No missing private API compile error is used as evidence.'},
 'no_new_expectations':'Same124 IDs, original plan case objects, sealed predicates and references. Direct evaluate(candidate) applies sealed F reference to actual baseline discovery rows without relabeling products.',
 'implementation_permission':'This evidence requires independent verifier review. No candidate inspected or implementation authorized by this author.','recorder':pin(__file__)}
 output=D/'p1339-red.json';assert not output.exists()
 body=json.dumps(data,indent=2,ensure_ascii=True)+'\n';patch='*** Begin Patch\n*** Add File: '+str(output)+'\n'+''.join('+'+line+'\n' for line in body.split('\n')[:-1])+'*** End Patch\n'
 subprocess.run(['apply_patch'],input=patch,text=True,capture_output=True,check=True)
 print(json.dumps({'receipt':pin(output),'raw':pin(rawpath),'status':data['status'],'counts':counts,'failures':len(failures)}),flush=True)
if __name__=='__main__':main()
