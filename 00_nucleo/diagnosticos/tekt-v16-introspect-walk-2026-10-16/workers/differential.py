from pathlib import Path
import json,collections,re,difflib,copy,gzip,datetime
out=Path('00_nucleo/diagnosticos/tekt-v16-introspect-walk-2026-10-16');scope=json.load(open(out/'scope.json'));old_text=json.load(gzip.open(out/'tests-and-l0.json.gz'))
results=lambda p:[r for run in json.load(open(p))['runs'] for r in run['results']]
old=results(out/'baseline-architecture.log');new=results(out/'validation-architecture.log');counts=lambda rows:dict(collections.Counter(r['ruleId']+':'+r['level'] for r in rows));assert counts(old)==scope['initial_counts'];assert counts(new)=={'V16:error':233,'V19:note':907,'V20:note':871},counts(new)
source=lambda r:r['locations'][0]['physicalLocation']['artifactLocation']['uri'].removeprefix('./');line=lambda r:r['locations'][0]['physicalLocation']['region']['startLine']
removed=[r for r in old if r['ruleId']=='V16' and source(r)==scope['sources'][0]];assert len(removed)==6;assert sorted(line(r) for r in removed)==sorted(t['line'] for t in scope['targets'])
newgroups=sorted([r for r in new if r['ruleId']=='V19' and source(r)==scope['sources'][0] and int(re.search(r'grupos: (\d+);',r['message']['text']).group(1))>100],key=line);assert len(newgroups)==6
for r,target in zip(newgroups,scope['targets']):
 names=re.findall(r'Content::(\w+)',r['message']['text']);expected=[n for n in scope['variants'] if n not in target['excluded']];assert names==expected,(target['id'],names);assert int(re.search(r'grupos: (\d+);',r['message']['text']).group(1))==target['group_size']
aux=[r for r in new if r['ruleId']=='V19' and source(r)==scope['additional_group']['source'] and 'grupos: 118;' in r['message']['text']];assert len(aux)==1
assert re.findall(r'Content::(\w+)',aux[0]['message']['text'])==[n for n in scope['variants'] if n not in scope['additional_group']['excluded']]
old=[copy.deepcopy(r) for r in old if r not in removed];new=[r for r in new if r not in newgroups+aux];changed=[]
for r in old:
 if r['ruleId']!='V19':continue
 if source(r)=='01_core/src/compiler/introspect/extract_payload.rs' and 'grupos: 99;' in r['message']['text']:
  original=copy.deepcopy(r);r['message']['text']=r['message']['text'].replace('grupos: 99;','grupos: 98;').replace('\n        | Content::OutlineEntry(_)','');changed.append({'before':original,'expected':copy.deepcopy(r)})
 elif source(r)=='01_core/src/compiler/layout/realized.rs' and line(r) in [16,22]:
  original=copy.deepcopy(r)
  if line(r)==16:r['message']['text']=r['message']['text'].replace('grupos: 6;','grupos: 7;').replace('Content::Outline(_)\n','Content::Outline(_)\n        | Content::OutlineEntry(_)\n',1)
  else:r['message']['text']=r['message']['text'].replace('grupos: 114;','grupos: 113;').replace('\n        | Content::OutlineEntry(_)','')
  changed.append({'before':original,'expected':copy.deepcopy(r)})
assert len(changed)==3,len(changed)
maps={}
for p in scope['sources']:
 blocks=difflib.SequenceMatcher(a=old_text[p].splitlines(),b=Path(p).read_text().splitlines(),autojunk=False).get_matching_blocks();maps[p]={i+1:j+1 for a,b,n in blocks for i,j in zip(range(a,a+n),range(b,b+n))}
for r in old:
 p=source(r)
 if p not in maps:continue
 for loc in r.get('locations',[]):
  region=loc['physicalLocation']['region'];old_line=region['startLine'];assert old_line in maps[p],(p,r['ruleId'],old_line);region['startLine']=maps[p][old_line]
  if 'endLine' in region:region['endLine']=maps[p][region['endLine']]
serial=lambda rows:collections.Counter(json.dumps(r,sort_keys=True) for r in rows)
a,b=serial(old),serial(new)
if a!=b:
 (out/'unexpected-delta.json').write_text(json.dumps({'missing':[json.loads(x) for x in (a-b).elements()],'new':[json.loads(x) for x in (b-a).elements()]},indent=2));raise AssertionError('unexpected SARIF differential')
record={'utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'old_counts':scope['initial_counts'],'new_counts':{'V16:error':233,'V19:note':907,'V20:note':871},'removed_v16':removed,'new_v19_groups':newgroups+aux,'prescribed_v19_changes':changed,'other_diagnostics_preserved_including_mapped_locations':True};(out/'architecture-delta.json').write_text(json.dumps(record,indent=2)+'\n');remaining=[r for r in results(out/'validation-architecture.log') if r['ruleId']=='V16'];(out/'remaining-v16.json').write_text(json.dumps(remaining,indent=2)+'\n');print('SARIF differential PASS: six V16 removed, seven V19 groups and three prescribed message updates, no other changes',flush=True)
