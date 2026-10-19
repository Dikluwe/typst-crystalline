from pathlib import Path
import json,gzip,hashlib,datetime,subprocess
b=Path('lab/results/v16-corrections-2026-10-19/batch-f');g=b/'structural-final-green';o=b/'agents/adapters';sha=lambda p:hashlib.sha256(p.read_bytes()).hexdigest();readgz=lambda p:json.loads(gzip.decompress(p.read_bytes()))
reports=json.loads((g/'results.json').read_text());assert [r['name'] for r in reports]==['all-domains','arity-zero','arity-many']
freeze=json.loads((g/'freeze.json').read_text());assert not freeze['changed']
original=readgz(g/'original-sources.json.gz');scope=json.loads((b/'scope.json').read_text());inputs=readgz(g/'inputs.json.gz');assert len(scope['sources'])==len(scope['owners'])==19
current_frozen={path:sha(Path(path)) for path in scope['sources']+scope['owners']};assert inputs['frozen_owners']==current_frozen
for path in scope['sources']: assert hashlib.sha256(original[path].encode()).hexdigest()==current_frozen[path]
assert all(Path(path).is_file() and sha(Path(path))==digest for path,digest in inputs['inputs'].items())
targets=json.loads((b/'structural-targets.json').read_text());expected=[t for t in targets if 'green' in t];audits=[]
for report in reports:
 name=report['name'];mut=readgz(g/(name+'-mutations.json.gz'));textmap=original|mut;errors=[]
 for line in (g/(name+'.log')).read_text().splitlines():
  try:item=json.loads(line)
  except ValueError:continue
  if item.get('reason')=='compiler-message' and item['message']['level']=='error':errors.append(item['message'])
 assert errors and all((e.get('code') or {}).get('code')=='E0004' for e in errors)
 if name=='all-domains':assert {t['id'] for t in report['targets']}=={t['id'] for t in expected}
 if name.startswith('arity'):
  source=textmap['01_core/src/compiler/stdlib/foundations/selector.rs'];start=source.index('fn parse_selector_arg(');assert '[] | [_, _, ..]' not in source[start:source.index('\n}',start)]
  keep='[_, _, ..]' if name=='arity-zero' else '[]';assert keep in source[start:source.index('\n}',start)]
 findings=[]
 for target in report['targets']:
  spec=target.get('green',target);s=textmap[target['file']];p=s.index(spec['anchor'],s.index(spec['after']));line=s[:p].count('\n')+1;bs=len(s[:p].encode());be=bs+len(spec['anchor'].encode());assert (line,bs,be)==(target['line'],target['byte_start'],target['byte_end'])
  matched=[e for e in errors if any(z['is_primary'] and z['file_name'].endswith(target['file']) and z['line_start']==line and bs<=z['byte_start']<be for z in e['spans'])]
  assert matched,(name,target['id'])
  if name=='all-domains':assert any('TektV16EvolutionProbe' in e['message'] for e in matched)
  findings.append({'id':target['id'],'file':target['file'],'source_sha256':hashlib.sha256(s.encode()).hexdigest(),'line':line,'byte_start':bs,'byte_end':be,'primary_e0004':len(matched),'messages':[e['message'] for e in matched]})
 audits.append({'name':name,'recorded_utc_start':report['utc_start'],'recorded_utc_end':report['utc_end'],'exit':report['exit'],'all_errors':len(errors),'exact_selected':findings,'bound':{str(p):sha(p) for p in [g/(name+'.log'),g/(name+'-mutations.json.gz')]}})
r={'status':'PASS independent raw compiler and mutation reparse','utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'head':subprocess.check_output(['git','rev-parse','HEAD'],text=True).strip(),'working_tree':'uncommitted working tree','diff_HEAD_stat':subprocess.check_output(['git','diff','HEAD','--stat'],text=True),'method':'Recompute selected match line/byte positions from archived original+mutant sources; require exact primary E0004 in original compiler JSON. Separate slice mutants prove zero/many arity; no enum extension substitutes for arity. No new compiler execution.','bound':{str(p):sha(p) for p in [g/'results.json',g/'freeze.json',g/'original-sources.json.gz',g/'inputs.json.gz',b/'structural-targets.json',o/'structural-baseline-063-correction.json']},'audits':audits,'final_source_owner_hashes':current_frozen,'source_owner_pairs':19,'all_archived_build_inputs_equal_current':True,'reviewer_script_sha256':sha(Path(__file__)),'exclusions':'V16-055 Figure and V16-065 circle remain deferred. No green proof or closure claim for them.'}
assert not (o/'structural-final-green-independent.json').exists();(o/'structural-final-green-independent.json').write_text(json.dumps(r,indent=2)+'\n');print(json.dumps({'status':r['status'],'selected':[len(a['exact_selected']) for a in audits],'sha256':sha(o/'structural-final-green-independent.json')}))
