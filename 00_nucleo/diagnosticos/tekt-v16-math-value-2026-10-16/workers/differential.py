from pathlib import Path
import json,collections,difflib,copy,re,datetime
out=Path('00_nucleo/diagnosticos/tekt-v16-math-value-2026-10-16');scope=json.load(open(out/'scope.json'));baseline=json.load(open(out/'baseline-result.json'))
old=json.loads((out/'baseline-architecture.log').read_text())['runs'][0]['results'];new=json.loads((out/'validation-architecture.log').read_text())['runs'][0]['results']
removed=[]
for target in scope['targets']:
 found=[x for x in old if x['ruleId']=='V16' and x['locations'][0]['physicalLocation']['artifactLocation']['uri'].removeprefix('./')==target['source'] and x['locations'][0]['physicalLocation']['region']['startLine']==target['line']]
 assert len(found)==1,target
 removed.extend(found)
line_maps={}
for source in scope['sources']:
 prev=baseline['files'][source].splitlines();current=Path(source).read_text().splitlines();mapping={}
 for a,b,size in difflib.SequenceMatcher(a=prev,b=current,autojunk=False).get_matching_blocks():
  for offset in range(size):mapping[a+offset+1]=b+offset+1
 line_maps[source]=mapping
expected=[]
for row in old:
 if row in removed:continue
 item=copy.deepcopy(row)
 for location in item['locations']:
  physical=location['physicalLocation'];path=physical['artifactLocation']['uri'].removeprefix('./')
  if path in line_maps:
   region=physical['region']
   for field in ['startLine','endLine']:
    if field in region:
     assert region[field] in line_maps[path],(row,field)
     region[field]=line_maps[path][region[field]]
 expected.append(item)
key=lambda x:json.dumps(x,sort_keys=True)
old_counter=collections.Counter(map(key,expected));new_counter=collections.Counter(map(key,new))
assert not old_counter-new_counter,'a previous diagnostic was removed or altered'
added=[json.loads(x) for x in (new_counter-old_counter).elements()]
assert len(added)==4 and all(x['ruleId']=='V19' and x['level']=='note' for x in added),added
classified=[]
for target in scope['targets']:
 text=Path(target['source']).read_text();start=list(re.finditer(r'^(?:pub(?:\([^)]*\))? )?fn '+target['function']+r'\(',text,re.M));assert len(start)==1
 first=text[:start[0].start()].count('\n')+1
 after=re.search(r'^\s*(?:pub(?:\([^)]*\))? )?fn |^#\[cfg\(test\)\]',text[start[0].end():],re.M)
 end=text[:start[0].end()+after.start()].count('\n')+1 if after else len(text.splitlines())+1
 found=[x for x in added if x['locations'][0]['physicalLocation']['artifactLocation']['uri'].removeprefix('./')==target['source'] and first<=x['locations'][0]['physicalLocation']['region']['startLine']<end]
 assert len(found)==1,target
 assert 'Alternativas escritas nos grupos: '+str(len(target['complement']))+';' in found[0]['message']['text']
 classified.append({'id':target['id'],'source':target['source'],'function':target['function'],'group':found[0],'decision':'preservar V19 informativo: todas as alternativas têm o mesmo resultado e efeito atuais; não é prova de cobertura de cada alternativa'})
counts=lambda rows:dict(collections.Counter(x['ruleId']+':'+x['level'] for x in rows))
assert counts(old)=={'V16:error':244,'V19:note':888,'V20:note':871}
assert counts(new)=={'V16:error':240,'V19:note':892,'V20:note':871}
record={'utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'old_counts':counts(old),'new_counts':counts(new),'removed_selected_v16':removed,'new_v19_equivalent_groups':classified,'other_diagnostics_preserved_including_mapped_locations':True,'line_maps':line_maps,'global_gate_still_fails':True}
(out/'architecture-delta.json').write_text(json.dumps(record,indent=2)+'\n');(out/'remaining-v16.json').write_text(json.dumps([x for x in new if x['ruleId']=='V16'],indent=2)+'\n')
print(json.dumps({'V16':'244 -> 240','V19':'888 -> 892','other_diagnostics':'preserved with exact line mapping'}),flush=True)
