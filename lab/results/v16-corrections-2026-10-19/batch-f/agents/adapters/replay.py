#!/usr/bin/env python3
"""Strictly replay the frozen adapter corpus against predeclared exact outcomes."""
import argparse,datetime,gzip,hashlib,json,pathlib,subprocess,sys
HERE=pathlib.Path(__file__).resolve().parent
ROOT=HERE.parents[5]
ORACLE_SHA='9b2373155e9c05bbb0daf5755bf4be88a11e64bd6b4f8b376648fa27303373b9'
UTC=lambda:datetime.datetime.now(datetime.timezone.utc).isoformat()
def sha(path):return hashlib.sha256(pathlib.Path(path).read_bytes()).hexdigest()
def canonical(value):return json.dumps(value,sort_keys=True,ensure_ascii=False,separators=(',',':'))
def same(a,b):return canonical(a)==canonical(b)
def read(path):
 with (gzip.open(path,'rt') if str(path).endswith('.gz') else open(path)) as f:return json.load(f)
def git(*args):return subprocess.check_output(['git',*args],cwd=ROOT,text=True)
def full(path):
 p=pathlib.Path(path);return p if p.is_absolute() else ROOT/p
def observe(raw):
 if raw.get('timeout'):return {'timeout':True}
 result={k:raw[k] for k in ['returncode','stderr']}
 if raw['returncode']==0:
  try:result['value']=json.loads(raw['stdout'])
  except (ValueError,TypeError):result.update(parse_error=True,stdout=raw['stdout'])
 else:result['stdout']=raw['stdout']
 return result

def invoke(argv):
 started=UTC()
 try:
  p=subprocess.run(argv,cwd=ROOT,capture_output=True,text=True,timeout=30)
  r=dict(returncode=p.returncode,stdout=p.stdout,stderr=p.stderr)
 except subprocess.TimeoutExpired as e:
  def decode(x):return x.decode(errors='replace') if isinstance(x,bytes) else (x or '')
  r=dict(timeout=True,stdout=decode(e.stdout),stderr=decode(e.stderr))
 return dict(r,argv=argv,cwd=str(ROOT),utc_start=started,utc_end=UTC())

def main():
 ap=argparse.ArgumentParser(description=__doc__)
 ap.add_argument('--output',required=True)
 ap.add_argument('--validation-state',required=True)
 ap.add_argument('--build-receipt',required=True)
 ap.add_argument('--product',default='lab/target/product/debug/typst')
 ap.add_argument('--oracle',default='lab/typst-original/target/release/typst')
 args=ap.parse_args();out=full(args.output)
 if out.exists():ap.error('output exists; evidence is append-only')
 manifest_path=HERE/'replay-manifest.json';manifest=read(manifest_path)
 corpus=[]
 for entry in manifest['baseline_receipts']:
  p=HERE/entry['path']
  if sha(p)!=entry['sha256']:ap.error('baseline receipt changed: '+str(p))
  receipt=read(p)
  for case in receipt['cases']:corpus.append((entry['path']+'::'+case['name'],case))
 policy=manifest['cases']
 if len(policy)!=len(corpus) or set(policy)!={key for key,_ in corpus}:ap.error('exact case membership mismatch')
 for entry in manifest['supporting_evidence']:
  p=full(entry['path'])
  if sha(p)!=entry['sha256']:ap.error('supporting evidence changed: '+str(p))
 for fixture in manifest['fixtures']:
  p=full(fixture['path'])
  if fixture.get('absent'):
   if p.exists():ap.error('negative fixture exists: '+str(p))
  elif sha(p)!=fixture['sha256']:ap.error('fixture changed: '+str(p))
 state_path=full(args.validation_state);build_path=full(args.build_receipt);state=read(state_path)
 inputs=state.get('inputs')
 if not isinstance(inputs,dict) or not inputs:ap.error('validation-state must contain nonempty inputs map')
 changed=[p for p,h in inputs.items() if not full(p).is_file() or sha(full(p))!=h]
 if changed:ap.error('build inputs differ: '+str(changed))
 binaries={'product':full(args.product),'oracle':full(args.oracle)}
 if sha(binaries['oracle'])!=ORACLE_SHA:ap.error('oracle is not pinned executable')
 out.mkdir(parents=True)
 r=dict(utc_start=UTC(),head=git('rev-parse','HEAD').strip(),status=git('status','--short'),diff_head_stat=git('diff','HEAD','--stat'),runner_sha256=sha(__file__),manifest=dict(path=str(manifest_path),sha256=sha(manifest_path)),validation_state=dict(path=str(state_path),sha256=sha(state_path)),build_receipt=dict(path=str(build_path),sha256=sha(build_path)),binaries={k:dict(path=str(p),sha256=sha(p)) for k,p in binaries.items()},oracle_pin=manifest['oracle_pin'],acceptance='Exact frozen case membership, unchanged oracle, and exactly one declared expected outcome per case. Baseline parity debt is retained explicitly; no generic acceptance of changed observations.',cases=[])
 for key,case in corpus:
  expectation=policy[key]
  if expectation['expression_sha256']!=hashlib.sha256(case['expression'].encode()).hexdigest():ap.error('source mismatch: '+key)
  if expectation['baseline_observation']!=observe(case['results']['product']):ap.error('baseline observation mismatch: '+key)
  item=dict(id=key,expression=case['expression'],expression_sha256=expectation['expression_sha256'],policy=expectation['policy'],reason=expectation['reason'],results={})
  for label,binary in binaries.items():
   argv=[str(binary),*case['results'][label]['argv'][1:]]
   if argv[1:]!=['eval',case['expression'],'--format','json']:ap.error('unexpected argv shape: '+key)
   item['results'][label]=invoke(argv)
  candidate=observe(item['results']['product']);oracle=observe(item['results']['oracle']);old=observe(case['results']['product']);old_oracle=observe(case['results']['oracle'])
  item.update(observations=dict(baseline_product=old,baseline_oracle=old_oracle,candidate_product=candidate,fresh_oracle=oracle,required=expectation['expected_observation']),oracle_preserved=same(oracle,old_oracle),required_outcome_met=same(candidate,expectation['expected_observation']),matches_oracle=same(candidate,oracle),classification='PRESERVED' if same(candidate,old) else 'REPAIRED' if same(candidate,oracle) else 'CHANGED_WITH_RESIDUAL')
  item['passed']=item['oracle_preserved'] and item['required_outcome_met']
  r['cases'].append(item);(out/'receipt.json').write_text(json.dumps(r,indent=2));print(key,item['classification'],'pass='+str(item['passed']),flush=True)
 r.update(utc_end=UTC(),head_after=git('rev-parse','HEAD').strip(),status_after=git('status','--short'),diff_head_stat_after=git('diff','HEAD','--stat'),changed_build_inputs=[p for p,h in inputs.items() if not full(p).is_file() or sha(full(p))!=h],binary_sha256_after={k:sha(p) for k,p in binaries.items()},manifest_sha256_after=sha(manifest_path),validation_state_sha256_after=sha(state_path),build_receipt_sha256_after=sha(build_path))
 r['counts']={name:sum(c['classification']==name for c in r['cases']) for name in ['PRESERVED','REPAIRED','CHANGED_WITH_RESIDUAL']}
 r['counts']['oracle_matches']=sum(c['matches_oracle'] for c in r['cases']);r['counts']['failed']=sum(not c['passed'] for c in r['cases'])
 r['passed']=all(c['passed'] for c in r['cases']) and not r['changed_build_inputs'] and all(r['binaries'][k]['sha256']==v for k,v in r['binary_sha256_after'].items()) and r['head']==r['head_after'] and r['manifest']['sha256']==r['manifest_sha256_after'] and r['validation_state']['sha256']==r['validation_state_sha256_after'] and r['build_receipt']['sha256']==r['build_receipt_sha256_after']
 r['full_parity']=r['passed'] and all(c['matches_oracle'] for c in r['cases'])
 (out/'receipt.json').write_text(json.dumps(r,indent=2));print(json.dumps(dict(passed=r['passed'],full_parity=r['full_parity'],counts=r['counts'])));return 0 if r['passed'] else 1
if __name__=='__main__':sys.exit(main())
