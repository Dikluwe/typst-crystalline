#!/usr/bin/env python3
"""Prepare a fresh scoped review only for independently identified prior paths."""
import argparse,copy,datetime,hashlib,importlib.util,json,pathlib
HERE=pathlib.Path(__file__).resolve().parent
spec=importlib.util.spec_from_file_location('scoped_assessor',HERE/'assess.py');assess=importlib.util.module_from_spec(spec);spec.loader.exec_module(assess)

def main():
 ap=argparse.ArgumentParser(description=__doc__);ap.add_argument('--receipt',required=True);ap.add_argument('--output',required=True);args=ap.parse_args();rp=pathlib.Path(args.receipt);op=pathlib.Path(args.output)
 if op.exists():ap.error('review output exists; preserve earlier review')
 r=json.loads(rp.read_text())
 if not r.get('utc_end'):ap.error('replay incomplete')
 previous_path=HERE/'candidate-replay-review-bound.json';previous=json.loads(previous_path.read_text());prior={c['id']:c for c in previous['cases']}
 corpus={}
 manifest=json.loads((HERE/'baseline-manifest.json').read_text())
 for item in manifest['receipts']:
  path=HERE/item['file'];assert assess.sha(path)==item['sha256']
  for case in json.loads(path.read_text())['cases']:corpus[item['file']+'::'+case['name']]=(path,case)
 def cite(key):
  path,case=corpus[key]
  return {'receipt':str(path),'receipt_sha256':assess.sha(path),'probe':case['name'],'product_observation_sha256':assess.digest(assess.baseline_observation(case['results']['product'],case)),'oracle_observation_sha256':assess.digest(assess.baseline_observation(case['results']['oracle'],case))}
 rows=[];unreviewed=[]
 for case in r['cases']:
  if case['classification']!='REGRESSION':continue
  key=case['id'];old_review=prior.get(key);obs=case['observations'];new=obs['candidate_product'];oracle=obs['fresh_oracle']
  row=copy.deepcopy(old_review) if old_review else {'id':key}
  if old_review and old_review['independent_assessment'].startswith('partial measured repair;') and old_review['candidate_observation_sha256']==assess.digest(new):
   row['fresh_review_basis']='Exact same language observation as independently reviewed first-candidate partial repair; fresh oracle and raw baseline checked by separate assessor.'
  elif key in ['artifact-marker-receipt.json::marker-encode-show-set-text','artifact-marker-receipt.json::marker-semantic-show-set-text-json','artifact-marker-receipt.json::marker-semantic-show-set-text-yaml']:
   a=json.loads(new.get('value','null')) if 'encode-' in key else new.get('value');b=json.loads(oracle.get('value','null')) if 'encode-' in key else oracle.get('value')
   expected=copy.deepcopy(b)
   if not isinstance(expected,dict) or not isinstance(expected.get('child'),dict):ap.error('oracle Styled contract missing for '+key)
   expected['child']['block']=False
   if new.get('returncode')!=0 or new.get('stderr')!=oracle.get('stderr') or a!=expected:ap.error('set-text candidate differs beyond independently measured omitted block debt: '+key)
   if set(a)!={'func','child','styles'} or a['func']!='styled' or a['styles']!='styles(..)' or a['child'].get('func')!='equation' or 'numbering' not in a['child'] or a['child']['numbering'] is not None:ap.error('ordinary Styled boundary still missing: '+key)
   row.update(independent_assessment='partial measured repair; existing debt remains',reason='Lexical set text now preserves its real outer Styled child/styles boundary, while marked inner Equation exposes identity and explicit numbering none. The only remaining decoded value difference is child.block=false for omitted block, independently established before this batch.',recommendation='Scoped repair only; omitted Equation block presence remains open.',source_cause=['01_core/src/compiler/eval/expression_dispatch.rs:style_code_tail','01_core/src/compiler/stdlib/loading/encoding.rs:textual_content_fields','01_core/src/compiler/eval/bindings/field_access/content_methods.rs:content_field Equation block'],independent_baseline_evidence=[cite(key),cite('supplement-receipt.json::eq-block-unset'),cite('receipt.json::eq-omitted')],fresh_review_basis='Previously introduced lexical-style loss is now positively required to match the oracle outer structure; residual is checked to be exactly the already measured omitted block value.')
  else:
   unreviewed.append(key);row.update(independent_assessment='unreviewed candidate change; fail closed',reason='Candidate observation is not the exact prior independently reviewed partial repair and is not the independently traced set-text correction.',recommendation='Independent source and language review required before scoped acceptance.',source_cause=[],independent_baseline_evidence=[])
  row['candidate_observation_sha256']=assess.digest(new);row['old_to_new_differences']=assess.differences(assess.expanded(obs['baseline_product'],case),assess.expanded(new,case));row['candidate_to_oracle_differences']=assess.differences(assess.expanded(new,case),assess.expanded(oracle,case));rows.append(row)
 review={'utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'candidate_receipt':str(rp),'candidate_receipt_sha256':assess.sha(rp),'candidate_binary_sha256':r['binaries']['product']['sha256'],'prior_independent_review':str(previous_path),'prior_review_sha256':assess.sha(previous_path),'review_routine_sha256':assess.sha(__file__),'cases':rows,'unreviewed_changes':unreviewed,'scope':'Fresh review preserves raw failure/global parity false. Prior partial repair observations must be identical; newly repaired lexical-style path is constrained to exact oracle outer structure plus established omitted-block debt. Unknown changes fail closed.'};op.parent.mkdir(parents=True,exist_ok=True);op.write_text(json.dumps(review,indent=2));print(json.dumps({'reviewed':len(rows),'unreviewed':unreviewed}));return 0 if not unreviewed else 1
if __name__=='__main__':raise SystemExit(main())
