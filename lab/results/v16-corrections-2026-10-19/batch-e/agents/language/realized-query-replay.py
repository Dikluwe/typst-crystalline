#!/usr/bin/env python3
"""Replay frozen realized Equation query corpus; require every observation to match oracle."""
import argparse,datetime,gzip,hashlib,json,os,pathlib,subprocess,sys
import yaml
HERE=pathlib.Path(__file__).resolve().parent
ROOT=HERE.parents[5]
ORACLE_SHA='9b2373155e9c05bbb0daf5755bf4be88a11e64bd6b4f8b376648fa27303373b9'
UTC=lambda:datetime.datetime.now(datetime.timezone.utc).isoformat()
def sha(path):return hashlib.sha256(pathlib.Path(path).read_bytes()).hexdigest()
def read_json(path):
 path=pathlib.Path(path)
 with (gzip.open(path,'rt') if path.suffix=='.gz' else path.open()) as f:return json.load(f)
def git(*args):return subprocess.check_output(['git',*args],cwd=ROOT,text=True)
def full(path):
 path=pathlib.Path(path)
 return path if path.is_absolute() else ROOT/path
def canonical(text,result,case):
 # Only substitute the exact source filename of this query; no diagnostic text,
 # line/column, warning, whitespace, or error-channel normalization.
 if case.get('command')=='query':
  path=pathlib.Path(result['argv'][2])
  for spelling in [str(path),os.path.relpath(path,ROOT)]:text=text.replace(spelling,'<probe-document>')
 return text

def observation(result,case):
 if result.get('timeout'):return {'timeout':True}
 out=canonical(result['stdout'],result,case)
 stderr=canonical(result['stderr'],result,case)
 if result['returncode']==0:
  parser=yaml.safe_load if case.get('format')=='yaml' else json.loads
  try:value=parser(out)
  except Exception as error:return {'returncode':0,'parse_error':str(error),'stdout':out,'stderr':stderr}
  return {'returncode':0,'value':value,'stderr':stderr}
 return {'returncode':result['returncode'],'stdout':out,'stderr':stderr}
def digest(value):return hashlib.sha256(json.dumps(value,sort_keys=True,ensure_ascii=False,separators=(',',':')).encode()).hexdigest()
def run(argv):
 started=UTC()
 try:
  p=subprocess.run(argv,cwd=ROOT,capture_output=True,text=True,timeout=60)
  result={'returncode':p.returncode,'stdout':p.stdout,'stderr':p.stderr}
 except subprocess.TimeoutExpired as e:
  def dec(x):return x.decode(errors='replace') if isinstance(x,bytes) else (x or '')
  result={'timeout':True,'stdout':dec(e.stdout),'stderr':dec(e.stderr)}
 return dict(result,argv=argv,cwd=str(ROOT),utc_start=started,utc_end=UTC())

def main():
 ap=argparse.ArgumentParser(description=__doc__)
 ap.add_argument('--output',required=True,help='New evidence directory; must not exist')
 ap.add_argument('--validation-state',required=True,help='Parent-owned candidate build input receipt JSON/JSON.gz')
 ap.add_argument('--product',default='lab/target/product/debug/typst')
 ap.add_argument('--oracle',default='lab/typst-original/target/release/typst')
 args=ap.parse_args();output=full(args.output);state_path=full(args.validation_state)
 if output.exists():ap.error('output already exists; baseline and prior replay evidence cannot be overwritten')
 state=read_json(state_path);inputs=state.get('inputs')
 if not isinstance(inputs,dict) or not inputs:ap.error('validation-state requires a nonempty inputs path→SHA256 map')
 mismatches=[p for p,h in inputs.items() if not full(p).is_file() or sha(full(p))!=h]
 if mismatches:ap.error('candidate sources differ from build receipt: '+', '.join(mismatches))
 product,oracle=full(args.product),full(args.oracle)
 if sha(oracle)!=ORACLE_SHA:ap.error('oracle binary differs from ratified pin')
 manifest_path=HERE/'realized-query-manifest.json';manifest=read_json(manifest_path);corpus=[]
 for entry in manifest['receipts']:
  path=HERE/entry['file']
  if sha(path)!=entry['sha256']:ap.error('baseline receipt modified: '+entry['file'])
  baseline=read_json(path)
  for case in baseline['cases']:
   case=dict(case,command='query',format='json')
   corpus.append((entry['file'],case))
 output.mkdir(parents=True);(output/'sources').mkdir()
 record={'utc_start':UTC(),'state':'uncommitted working tree' if git('diff','HEAD','--stat').strip() else 'committed source tree','head':git('rev-parse','HEAD').strip(),'diff_head_stat':git('diff','HEAD','--stat'),'status':git('status','--short'),'runner_sha256':sha(__file__),'baseline_manifest':{'path':str(manifest_path),'sha256':sha(manifest_path)},'validation_state':{'path':str(state_path),'sha256':sha(state_path),'receipt':state},'binaries':{k:{'path':str(p),'sha256':sha(p)} for k,p in [('product',product),('oracle',oracle)]},'oracle_pin':{'commit':'bd5ebec285505889e8ff1d4f68804ab147a699f3','tree':'cac836c07e4dec2a89219370428292393a3216b5','recipe':'lab/results/construction/diagnostics/vanilla-resync-2026-10-17/relatorio.md'},'acceptance':'Every observation must match the unchanged pinned oracle; no partial repair exceptions. Array order, labels, defaults and empty selections remain exact semantic values.','classification':'PRESERVED: same parsed value and diagnostics as baseline product; REPAIRED: matches pinned oracle; REGRESSION: neither. JSON/YAML map formatting is not compared. Error stdout/stderr and success stderr remain exact except exact query source-path substitution.','cases':[]}
 frozen_binaries={k:v['sha256'] for k,v in record['binaries'].items()}
 for index,(receipt_name,case) in enumerate(corpus):
  key=receipt_name+'::'+case['name'];item={k:v for k,v in case.items() if k not in ['results','parsed_equal','utc_start','head','diff_head_stat']}
  item.update(id=key,baseline_receipt=receipt_name,utc_start=UTC(),head=git('rev-parse','HEAD').strip(),diff_head_stat=git('diff','HEAD','--stat'),results={})
  for label,binary in [('product',product),('oracle',oracle)]:
   original=case['results'][label]['argv'];argv=list(original);argv[0]=str(binary)
   if case.get('command')=='query':
    src=output/'sources'/f'{index:03d}-{case["name"]}.typ';src.write_text(case['source']);argv[2]=str(src)
    item['source_sha256']=sha(src)
   else:item['source_sha256']=hashlib.sha256(case['expression'].encode()).hexdigest()
   item['results'][label]=run(argv)
  candidate=observation(item['results']['product'],case);fresh_oracle=observation(item['results']['oracle'],case)
  old_product=observation(case['results']['product'],case);old_oracle=observation(case['results']['oracle'],case)
  item['oracle_preserved']=fresh_oracle==old_oracle
  item['classification']='PRESERVED' if candidate==old_product else 'REPAIRED' if candidate==fresh_oracle else 'REGRESSION'
  item['candidate_observation_sha256']=digest(candidate)
  item['observations']={'baseline_product':old_product,'baseline_oracle':old_oracle,'candidate_product':candidate,'fresh_oracle':fresh_oracle}
  item['private_marker_leak']='equation.constructor' in item['results']['product']['stdout']
  item['matches_oracle']=candidate==fresh_oracle
  item['pass']=item['oracle_preserved'] and item['matches_oracle'] and candidate.get('returncode')==0 and 'parse_error' not in candidate and not item['private_marker_leak'] and not candidate.get('timeout')
  record['cases'].append(item)
  print(f'{index+1}/{len(corpus)} {key}: {item["classification"]} oracle_preserved={item["oracle_preserved"]} pass={item["pass"]}',flush=True)
  (output/'receipt.json').write_text(json.dumps(record,indent=2))
 record['utc_end']=UTC();record['diff_head_stat_after']=git('diff','HEAD','--stat');record['head_after']=git('rev-parse','HEAD').strip()
 record['changed_build_inputs']=[p for p,h in inputs.items() if not full(p).is_file() or sha(full(p))!=h]
 record['binary_sha256_after']={k:sha(p) for k,p in [('product',product),('oracle',oracle)]}
 record['validation_state_sha256_after']=sha(state_path)
 record['passed']=all(c['pass'] for c in record['cases']) and not record['changed_build_inputs'] and frozen_binaries==record['binary_sha256_after'] and record['validation_state']['sha256']==record['validation_state_sha256_after'] and record['head']==record['head_after']
 record['counts']={k:sum(c['classification']==k for c in record['cases']) for k in ['PRESERVED','REPAIRED','REGRESSION']}
 (output/'receipt.json').write_text(json.dumps(record,indent=2));print(json.dumps({'passed':record['passed'],'counts':record['counts']}));return 0 if record['passed'] else 1
if __name__=='__main__':sys.exit(main())
