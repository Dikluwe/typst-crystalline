#!/usr/bin/env python3
"""Assess reviewed scoped repairs without changing raw parity results."""
import argparse,datetime,hashlib,json,os,pathlib,sys
import yaml
HERE=pathlib.Path(__file__).resolve().parent

def sha(path):return hashlib.sha256(pathlib.Path(path).read_bytes()).hexdigest()
def digest(value):return hashlib.sha256(json.dumps(value,sort_keys=True,ensure_ascii=False,separators=(',',':')).encode()).hexdigest()
MISSING={'$missing':True}
def differences(a,b,path=''):
 if isinstance(a,dict) and isinstance(b,dict):
  return [item for key in sorted(set(a)|set(b)) for item in differences(a.get(key,MISSING),b.get(key,MISSING),path+'/'+key)]
 if isinstance(a,list) and isinstance(b,list) and len(a)==len(b):
  return [item for index,(x,y) in enumerate(zip(a,b)) for item in differences(x,y,path+'/'+str(index))]
 return [] if a==b else [{'path':path or '/','old':a,'new':b}]
def expanded(obs,case):
 value=dict(obs)
 if case['name'].startswith('marker-encode-') and isinstance(value.get('value'),str):
  value['decoded_encoded_json']=json.loads(value['value'])
 return value

def baseline_observation(result,case):
 if result.get('timeout'):return {'timeout':True}
 def normalize(text):
  if case.get('command')=='query':
   path=pathlib.Path(result['argv'][2])
   for spelling in [str(path),os.path.relpath(path,HERE.parents[5])]:text=text.replace(spelling,'<probe-document>')
  return text
 out=normalize(result['stdout']);stderr=normalize(result['stderr'])
 if result['returncode']!=0:return {'returncode':result['returncode'],'stdout':out,'stderr':stderr}
 try:value=(yaml.safe_load if case.get('format')=='yaml' else json.loads)(out)
 except Exception as error:return {'returncode':0,'parse_error':str(error),'stdout':out,'stderr':stderr}
 return {'returncode':0,'value':value,'stderr':stderr}

def main():
 ap=argparse.ArgumentParser(description=__doc__)
 ap.add_argument('--receipt',required=True)
 ap.add_argument('--review',required=True)
 ap.add_argument('--output',required=True,help='New assessment JSON; existing files refused')
 args=ap.parse_args();receipt_path=pathlib.Path(args.receipt);review_path=pathlib.Path(args.review);output=pathlib.Path(args.output)
 if output.exists():ap.error('assessment output already exists; preserve prior evidence')
 receipt=json.loads(receipt_path.read_text());review=json.loads(review_path.read_text());errors=[]
 if not receipt.get('utc_end'):errors.append('raw replay incomplete')
 if review.get('candidate_receipt_sha256')!=sha(receipt_path):errors.append('review is not bound to this exact raw receipt')
 if review.get('candidate_binary_sha256')!=receipt['binaries']['product']['sha256']:errors.append('review candidate binary differs')
 if receipt.get('changed_build_inputs'):errors.append('build inputs changed during raw replay')
 if receipt.get('head')!=receipt.get('head_after'):errors.append('HEAD changed during raw replay')
 if receipt['validation_state']['sha256']!=receipt.get('validation_state_sha256_after'):errors.append('build receipt changed during raw replay')
 if {k:v['sha256'] for k,v in receipt['binaries'].items()}!=receipt.get('binary_sha256_after'):errors.append('binary changed during raw replay')
 manifest_path=HERE/'baseline-manifest.json';manifest=json.loads(manifest_path.read_text())
 if receipt['baseline_manifest']['sha256']!=sha(manifest_path):errors.append('frozen baseline manifest digest differs')
 frozen={};frozen_paths={}
 for entry in manifest['receipts']:
  path=HERE/entry['file']
  if pathlib.Path(entry['file']).name!=entry['file'] or not path.is_file() or sha(path)!=entry['sha256']:
   errors.append('baseline receipt path/digest invalid: '+entry['file']);continue
  original=json.loads(path.read_text());frozen_paths[str(path.resolve())]=entry
  for case in original['cases']:
   key=entry['file']+'::'+case['name']
   if key in frozen:errors.append('duplicate frozen baseline case '+key)
   frozen[key]={'case':case,'path':str(path.resolve()),'receipt_sha256':entry['sha256'],'product':baseline_observation(case['results']['product'],case),'oracle':baseline_observation(case['results']['oracle'],case)}
 raw_ids=[case['id'] for case in receipt['cases']]
 if len(raw_ids)!=len(set(raw_ids)) or set(raw_ids)!=set(frozen):errors.append('raw replay does not contain every frozen case exactly once')
 rows=review['cases'];indexed={row['id']:row for row in rows}
 if len(indexed)!=len(rows):errors.append('duplicate review case IDs')
 cases=[];by_id={};partial_count=0
 for case in receipt['cases']:
  key=case['id'];by_id[key]=case;obs=case['observations'];old=obs['baseline_product'];new=obs['candidate_product'];oracle=obs['fresh_oracle'];bad=[]
  baseline=frozen.get(key)
  if baseline is None:bad.append('case absent from independently frozen corpus')
  else:
   if old!=baseline['product'] or obs['baseline_oracle']!=baseline['oracle']:bad.append('raw baseline observations differ from hashed original source receipt')
   for field in ['expression','command','source','format','field','one']:
    if case.get(field)!=baseline['case'].get(field):bad.append('source/argument changed from frozen corpus: '+field)
  actual='PRESERVED' if new==old else 'REPAIRED' if new==oracle else 'REGRESSION'
  if actual!=case['classification']:bad.append('raw classification does not match observations')
  if oracle!=obs['baseline_oracle']:bad.append('oracle drift')
  if new.get('timeout') or oracle.get('timeout'):bad.append('timeout')
  if 'equation.constructor' in case['results']['product']['stdout']:bad.append('private marker leaked')
  if digest(new)!=case['candidate_observation_sha256']:bad.append('candidate observation digest invalid')
  review_row=indexed.get(key)
  partial=False
  if actual=='REGRESSION':
   if not review_row:bad.append('changed observation has no independent scoped review')
   else:
    if not review_row['independent_assessment'].startswith('partial measured repair;'):
     bad.append('independent review identifies an introduced regression or unresolved change')
    if review_row['candidate_observation_sha256']!=digest(new):bad.append('observation differs from exact reviewed candidate')
    if not review_row.get('reason') or not review_row.get('source_cause') or not review_row.get('independent_baseline_evidence'):bad.append('missing source cause or independent baseline justification')
    for evidence in review_row.get('independent_baseline_evidence',[]):
     if not isinstance(evidence,dict):bad.append('baseline evidence must include explicit path, digest and probe');continue
     evidence_path=pathlib.Path(evidence.get('receipt',''))
     if not evidence_path.is_absolute():evidence_path=HERE.parents[5]/evidence_path
     registered=frozen_paths.get(str(evidence_path.resolve()))
     if not registered or evidence.get('receipt_sha256')!=registered['sha256']:
      bad.append('independent evidence receipt path/digest is not frozen');continue
     evidence_key=registered['file']+'::'+str(evidence.get('probe',''))
     cited=frozen.get(evidence_key)
     if cited is None:bad.append('independent evidence probe does not exist');continue
     if evidence.get('product_observation_sha256')!=digest(cited['product']) or evidence.get('oracle_observation_sha256')!=digest(cited['oracle']):
      bad.append('independent evidence observation digests do not match named probe')
    observed_repairs=differences(expanded(old,case),expanded(new,case))
    remaining_debt=differences(expanded(new,case),expanded(oracle,case))
    if review_row.get('old_to_new_differences')!=observed_repairs:bad.append('required repaired paths/values differ from independent review')
    if review_row.get('candidate_to_oracle_differences')!=remaining_debt:bad.append('remaining residual paths/values differ from independent review')
    if not observed_repairs:bad.append('claimed partial repair changed no observation')
    partial=not bad
  partial_count+=partial
  cases.append({'id':key,'raw_classification':actual,'matches_oracle':new==oracle,'scoped_partial_repair':partial,'required_repaired_paths':review_row.get('old_to_new_differences',[]) if review_row else [],'unchanged_reviewed_residual_paths':review_row.get('candidate_to_oracle_differences',[]) if review_row else [],'passed':not bad,'errors':bad})
 # Native identity is a required correction, never excused by preserving a prior failure.
 required=[]
 for name in ['eq-native-equality','eq-alias']:
  key='receipt.json::'+name;case=by_id.get(key);observed=case['observations']['candidate_product'] if case else None
  passed=bool(observed and observed.get('returncode')==0 and observed.get('value') is True)
  required.append({'id':key,'requirement':'native/alias equation identity must be true','observed':observed,'passed':passed})
  if not passed:errors.append('required identity remains false or missing: '+key)
 for key in ['receipt.json::eq-call','receipt.json::eq-styled-call']:
  case=by_id.get(key);observed=case['observations']['candidate_product'] if case else None
  passed=bool(observed and observed.get('returncode')==0)
  required.append({'id':key,'requirement':'returned equation constructor must be callable','observed_returncode':observed.get('returncode') if observed else None,'passed':passed})
  if not passed:errors.append('required callable constructor missing: '+key)
 for name in ['marker-encode-show-set-text','marker-semantic-show-set-text-json','marker-semantic-show-set-text-yaml']:
  key='artifact-marker-receipt.json::'+name;case=by_id.get(key);observed=case['observations']['candidate_product'] if case else None
  value=observed.get('value') if observed else None
  if name.startswith('marker-encode-'):
   try:value=json.loads(value)
   except (TypeError,ValueError):value=None
  child=value.get('child') if isinstance(value,dict) else None
  passed=bool(observed and observed.get('returncode')==0 and isinstance(value,dict) and set(value)=={'func','child','styles'} and value.get('func')=='styled' and value.get('styles')=='styles(..)' and isinstance(child,dict) and child.get('func')=='equation' and 'numbering' in child and child['numbering'] is None and child.get('body')=={'func':'text','text':'x'})
  required.append({'id':key,'requirement':'lexical set text retains exact root Styled child/styles schema around equation with explicit numbering none and original body','observed':value,'passed':passed})
  if not passed:errors.append('required lexical Styled boundary missing: '+key)
 result={'utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'receipt':str(receipt_path),'receipt_sha256':sha(receipt_path),'review':str(review_path),'review_sha256':sha(review_path),'assessor_sha256':sha(__file__),'candidate_binary_sha256':receipt['binaries']['product']['sha256'],'raw_comparator_passed':receipt.get('passed'),'full_parity':all(c['matches_oracle'] for c in cases),'scoped_acceptance':not errors and all(c['passed'] for c in cases),'counts':{'cases':len(cases),'matches_oracle':sum(c['matches_oracle'] for c in cases),'reviewed_partial_repairs':partial_count,'failed_cases':sum(not c['passed'] for c in cases)},'required_corrections':required,'errors':errors,'cases':cases,'scope_statement':'Scoped acceptance preserves independently measured open obligations. Raw receipt and comparator status are unchanged; no full-parity claim follows. Observation hashes bind exact reviewed language values/errors, not a requirement for serializer byte identity.'}
 output.parent.mkdir(parents=True,exist_ok=True);output.write_text(json.dumps(result,indent=2));print(json.dumps({k:result[k] for k in ['raw_comparator_passed','full_parity','scoped_acceptance','counts','errors']}));return 0 if result['scoped_acceptance'] else 1
if __name__=='__main__':sys.exit(main())
