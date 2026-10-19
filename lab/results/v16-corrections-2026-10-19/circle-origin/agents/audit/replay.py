#!/usr/bin/env python3
"""Strict append-only Circle corpus recorder/replayer; preserve each case's complete observation."""
import argparse,datetime,gzip,hashlib,json,pathlib,subprocess,sys
R=pathlib.Path('/repos/Antigravity/typst-crystalline'); H=pathlib.Path(__file__).resolve().parent
sha=lambda p:hashlib.sha256(pathlib.Path(p).read_bytes()).hexdigest()
utc=lambda:datetime.datetime.now(datetime.timezone.utc).isoformat()
git=lambda *a:subprocess.check_output(['git',*a],cwd=R,text=True)
def read(p):
 p=pathlib.Path(p)
 with (gzip.open(p,'rt') if p.suffix=='.gz' else p.open()) as f:return json.load(f)
def obs(result,c):
 stderr=result['stderr']
 if 'source' in c:
  p=pathlib.Path(result['argv'][2]);stderr=stderr.replace(str(p),'<probe-document>')
  if p.is_relative_to(R):stderr=stderr.replace(str(p.relative_to(R)),'<probe-document>')
 o={k:result[k] for k in ('returncode','stdout')};o['stderr']=stderr
 if 'source' in c and o['returncode']==0:o['value']=json.loads(o.pop('stdout'))
 return o
def typed(v):
 if isinstance(v,dict):return ('dict',tuple(sorted((k,typed(x)) for k,x in v.items())))
 if isinstance(v,list):return ('list',tuple(map(typed,v)))
 return (type(v).__name__,v)
def main():
 p=argparse.ArgumentParser();p.add_argument('--baseline',action='store_true');p.add_argument('--product',required=True);p.add_argument('--build-receipt',required=True);p.add_argument('--output',required=True);p.add_argument('--validation-state');a=p.parse_args()
 out=R/a.output
 if out.exists():p.error('output exists: every attempt must be retained')
 manifest=H/('cases-draft.json' if a.baseline else 'cases.json');m=read(manifest);br=R/a.build_receipt;build=read(br)
 assert len(m['cases'])==m['count']==224 and len({c['name'] for c in m['cases']})==224
 binaries={'product':R/a.product,'oracle':R/'lab/typst-original/target/release/typst'}
 assert sha(binaries['oracle'])=='9b2373155e9c05bbb0daf5755bf4be88a11e64bd6b4f8b376648fa27303373b9'
 frozen_inputs=read(R/a.validation_state)['inputs'] if a.validation_state else None
 if not a.baseline:assert frozen_inputs,'accepted replay needs frozen build inputs'
 if frozen_inputs:assert all((R/p).is_file() and sha(R/p)==s for p,s in frozen_inputs.items())
 # Receipt schema varies; the exact executable SHA must occur in its frozen JSON.
 assert sha(binaries['product']) in json.dumps(build),'build receipt does not bind product bytes'
 out.mkdir(parents=True);(out/'sources').mkdir()
 rec={'utc_start':utc(),'head':git('rev-parse','HEAD').strip(),'diff_head_stat':git('diff','HEAD','--stat'),'source_state_note':'Baseline executable is bound to original frozen build inputs independently of current working-tree edits.' if a.baseline else 'Product bound to frozen current inputs.','runner_sha256':sha(__file__),'manifest':{'path':str(manifest.relative_to(R)),'sha256':sha(manifest)},'build_receipt':{'path':str(br.relative_to(R)),'sha256':sha(br)},'binaries':{k:{'path':str(v),'sha256':sha(v)} for k,v in binaries.items()},'cases':[]}
 if a.validation_state:rec['validation_state']={'path':a.validation_state,'sha256':sha(R/a.validation_state)}
 for c in m['cases']:
  item={k:c[k] for k in ('name','expression','source') if k in c};item.update(utc_start=utc(),results={})
  if 'source' in c:
   src=out/'sources'/(c['name']+'.typ');src.write_text(c['source']);item['source_sha256']=sha(src)
  for label,binary in binaries.items():
   argv=[str(binary),'query',str(src),'<probe>','--field','value'] if 'source' in c else [str(binary),'eval',c['expression']]
   run=subprocess.run(argv,cwd=R,capture_output=True,text=True,timeout=30);item['results'][label]={'argv':argv,'returncode':run.returncode,'stdout':run.stdout,'stderr':run.stderr}
  item['observations']={k:obs(v,c) for k,v in item['results'].items()}
  if not a.baseline:item['passed']=typed(item['observations']['product'])==typed(c['expected']) and typed(item['observations']['oracle'])==typed(c['oracle'])
  rec['cases'].append(item);(out/'receipt.json').write_text(json.dumps(rec,indent=2)+'\n')
 rec.update(utc_end=utc(),diff_head_stat_after=git('diff','HEAD','--stat'),binary_sha256_after={k:sha(v) for k,v in binaries.items()},changed_inputs=[p for p,s in (frozen_inputs or {}).items() if not (R/p).is_file() or sha(R/p)!=s])
 rec['passed']=(not a.validation_state or sha(R/a.validation_state)==rec['validation_state']['sha256']) and not rec['changed_inputs'] and all(rec['binary_sha256_after'][k]==v['sha256'] for k,v in rec['binaries'].items()) and (a.baseline or all(c['passed'] for c in rec['cases'])) and sha(manifest)==rec['manifest']['sha256'] and sha(br)==rec['build_receipt']['sha256']
 rec['counts']={'cases':len(rec['cases']),'passed':sum(c.get('passed',True) for c in rec['cases'])};(out/'receipt.json').write_text(json.dumps(rec,indent=2)+'\n');print(json.dumps({k:rec[k] for k in ('passed','counts')}));return 0 if rec['passed'] else 1
if __name__=='__main__':sys.exit(main())
