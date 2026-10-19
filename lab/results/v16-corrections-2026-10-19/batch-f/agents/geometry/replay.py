#!/usr/bin/env python3
"""Replay frozen geometry corpus with explicit case-specific expectations; reject unknown change."""
import argparse,datetime,gzip,hashlib,json,pathlib,re,subprocess,sys
ROOT=pathlib.Path('/repos/Antigravity/typst-crystalline');HERE=pathlib.Path(__file__).resolve().parent
sha=lambda p:hashlib.sha256(pathlib.Path(p).read_bytes()).hexdigest()
utc=lambda:datetime.datetime.now(datetime.timezone.utc).isoformat()
git=lambda *a:subprocess.check_output(['git',*a],cwd=ROOT,text=True)
def read(path):
 path=pathlib.Path(path)
 with (gzip.open(path,'rt') if path.suffix=='.gz' else path.open()) as f:return json.load(f)
def observation(result,case):
 stderr=result['stderr']
 if 'source' in case:
  source=pathlib.Path(result['argv'][2])
  spellings=[str(source)]
  if source.is_absolute() and source.is_relative_to(ROOT):spellings.append(str(source.relative_to(ROOT)))
  for spelling in spellings:stderr=stderr.replace(spelling,'<probe-document>')
 value={'returncode':result['returncode'],'stdout':result['stdout'],'stderr':stderr}
 # Query metadata is a language value; preserve exact diagnostics and typed JSON.
 if 'source' in case and result['returncode']==0:
  value['value']=json.loads(value.pop('stdout'))
 return value
def typed(value):
 if isinstance(value,dict):return ('dict',tuple(sorted((k,typed(v)) for k,v in value.items())))
 if isinstance(value,list):return ('list',tuple(map(typed,value)))
 return (type(value).__name__,value)
def eq(a,b):return typed(a)==typed(b)
def expected(case,all_eval):
 name=case['name'];old=observation(case['results']['product'],case);oracle=observation(case['results']['oracle'],case)
 if name.startswith('circle-'):return 'preserve-deferred-circle',old,'Whole V16-065 owner restored after measured identity regression; retain all baseline malformed-radius and geometry obligations, not acceptance of those semantics. See circle-deferred/deferral.json.'
 if 'source' in case:return 'preserve',old,'Preserve frozen Figure/Auto debt or absolute geometry observation, including diagnostic text.'
 # Every expression here was frozen in the baseline manifest. These partitions
 # are explicit expected input classes, not blanket acceptance of any new error.
 expr=case['expression'];m=re.match(r'\{ let x = (\w+)\((\w+): (.*?)(?:, \[X\])?\); repr\(x\) \}',expr)
 if not m:return 'preserve',old,'Preserve frozen Figure lexical observation.'
 native,arg,value=m.groups()
 malformed={'none','true','"bad"','"auto"','1fr','()','(1pt, true, 2pt)','(auto, 1fr, 2pt)'}
 repair=False
 if native in {'grid','table'}:
  repair=value in {'none','true','"bad"','0','-1','(1pt, true, 2pt)'}
 elif native=='circle':repair=value in malformed | {'20%','20% + 2pt'}
 elif native=='place':repair=value in malformed | {'auto'}
 elif native=='line' and arg=='dx':
  if value in malformed | {'auto'}:
   # The legacy dx surface has no upstream counterpart. Derive only the source
   # rendering from frozen upstream length diagnostics for the identical literal,
   # then use the existing Tekt typed-extension contract's expected length text.
   sibling=next(c for c in all_eval if c['name'].startswith('line-length-') and c['expression']==expr.replace('dx:','length:'))
   target=observation(sibling['results']['oracle'],sibling)
   target['stderr']=target['stderr'].replace('expected relative length,','expected length,').replace(sibling['expression'],expr)
   target['stderr']=re.sub(r'(<input-expression>:1:)(\d+)',lambda m:m[1]+str(int(m[2])-4),target['stderr'])
   lines=target['stderr'].splitlines(keepends=True)
   for i,line in enumerate(lines):
    if '│' in line and '^' in line:
     prefix,rest=line.split('│',1);assert rest.startswith('    ');lines[i]=prefix+'│'+rest[4:]
   target['stderr']=''.join(lines)
   return 'repair-local-legacy-type',target,'Local dx typed extension rejects malformed input; expected diagnostic differs from upstream unsupported-dx. Exact value-origin source rendering derived from frozen same-literal length diagnostic.'
 if repair:return 'repair-oracle-diagnostic',oracle,'Malformed supplied input now rejects with complete frozen oracle diagnostic and source span.'
 return 'preserve',old,'Preserve valid input, explicit legacy extension, typed dimensional debt, or previously reported polar diagnostic.'
def main():
 p=argparse.ArgumentParser();p.add_argument('--output',required=True);p.add_argument('--validation-state',required=True);p.add_argument('--product',default='lab/target/product/debug/typst');a=p.parse_args()
 out=ROOT/a.output
 if out.exists():p.error('output exists; preserve all prior attempts')
 statepath=ROOT/a.validation_state;state=read(statepath);inputs=state.get('inputs')
 if not isinstance(inputs,dict) or not inputs:p.error('validation-state has no frozen inputs')
 if any(not (ROOT/path).is_file() or sha(ROOT/path)!=digest for path,digest in inputs.items()):p.error('source inputs differ from build state')
 manifestpath=HERE/'replay-manifest.json';manifest=read(manifestpath)
 receipts=[]
 for item in manifest['receipts']:
  path=HERE/item['file']
  if sha(path)!=item['sha256']:p.error('baseline receipt changed: '+str(path))
  receipts.append(read(path))
 policiespath=HERE/manifest['expected_policies']['file']
 if sha(policiespath)!=manifest['expected_policies']['sha256']:p.error('frozen expectations changed')
 policies=read(policiespath)['cases'];policies={item['name']:item for item in policies}
 deferralpath=HERE/manifest['deferral_receipt']['file']
 if sha(deferralpath)!=manifest['deferral_receipt']['sha256']:p.error('circle deferral receipt changed')
 cases=[c for r in receipts for c in r['cases']]
 if set(policies)!={c['name'] for c in cases}:p.error('expectation corpus membership differs')
 if len(cases)!=182 or len({c['name'] for c in cases})!=182:p.error('wrong frozen corpus membership')
 binaries={'product':ROOT/a.product,'oracle':ROOT/'lab/typst-original/target/release/typst'}
 if sha(binaries['oracle'])!='9b2373155e9c05bbb0daf5755bf4be88a11e64bd6b4f8b376648fa27303373b9':p.error('wrong oracle binary')
 out.mkdir(parents=True);(out/'sources').mkdir()
 record={'utc_start':utc(),'head':git('rev-parse','HEAD').strip(),'diff_head_stat':git('diff','HEAD','--stat'),'runner_sha256':sha(__file__),'manifest_sha256':sha(manifestpath),'validation_state':{'path':str(statepath),'sha256':sha(statepath)},'binaries':{k:{'path':str(v),'sha256':sha(v)} for k,v in binaries.items()},'cases':[]}
 for case in cases:
  policy_record=policies[case['name']];policy,target,reason=policy_record['policy'],policy_record['expected_observation'],policy_record['reason'];item={'name':case['name'],'policy':policy,'reason':reason,'expected':target,'utc_start':utc(),'results':{}}
  if 'source' in case:
   source=out/'sources'/(case['name']+'.typ');source.write_text(case['source']);item.update(source=case['source'],source_sha256=sha(source))
  else:item['expression']=case['expression']
  for label,binary in binaries.items():
   argv=list(case['results'][label]['argv']);argv[0]=str(binary)
   if 'source' in case:argv[2]=str(source)
   r=subprocess.run(argv,cwd=ROOT,capture_output=True,text=True,timeout=30)
   item['results'][label]={'argv':argv,'returncode':r.returncode,'stdout':r.stdout,'stderr':r.stderr}
  actual=observation(item['results']['product'],case);oracle=observation(item['results']['oracle'],case)
  item.update(observation=actual,oracle_preserved=eq(oracle,observation(case['results']['oracle'],case)),expected_matched=eq(actual,target),matches_oracle=eq(actual,oracle),unchanged=eq(actual,observation(case['results']['product'],case)))
  item['pass']=item['oracle_preserved'] and item['expected_matched'];record['cases'].append(item)
  print(case['name'],policy,'PASS' if item['pass'] else 'FAIL',flush=True)
  (out/'receipt.json').write_text(json.dumps(record,indent=2))
 record.update(utc_end=utc(),diff_head_stat_after=git('diff','HEAD','--stat'),changed_inputs=[path for path,digest in inputs.items() if not (ROOT/path).is_file() or sha(ROOT/path)!=digest],binary_sha256_after={k:sha(v) for k,v in binaries.items()})
 record['passed']=all(c['pass'] for c in record['cases']) and not record['changed_inputs'] and all(record['binaries'][k]['sha256']==v for k,v in record['binary_sha256_after'].items()) and sha(statepath)==record['validation_state']['sha256']
 record['counts']={'cases':len(cases),'passed':sum(c['pass'] for c in record['cases']),'oracle_matches':sum(c['matches_oracle'] for c in record['cases'])}
 (out/'receipt.json').write_text(json.dumps(record,indent=2));print(json.dumps({'passed':record['passed'],'counts':record['counts']}));return 0 if record['passed'] else 1
if __name__=='__main__':sys.exit(main())
