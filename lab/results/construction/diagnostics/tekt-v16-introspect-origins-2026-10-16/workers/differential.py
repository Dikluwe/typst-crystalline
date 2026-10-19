from pathlib import Path
import json,collections,difflib,copy,re,datetime
out=Path('00_nucleo/diagnosticos/tekt-v16-introspect-origins-2026-10-16');scope=json.load(open(out/'scope.json'));baseline=json.load(open(out/'baseline-result.json'))
old=json.loads((out/'baseline-architecture.log').read_text())['runs'][0]['results'];new=json.loads((out/'validation-architecture.log').read_text())['runs'][0]['results'];removed=[]
for target in scope['targets']:
 if target['kind']!='emitted-v16':continue
 found=[x for x in old if x['ruleId']=='V16' and x['locations'][0]['physicalLocation']['artifactLocation']['uri'].removeprefix('./')==target['source'] and x['locations'][0]['physicalLocation']['region']['startLine']==target['line']];assert len(found)==1,target;removed.extend(found)
assert len(removed)==1
line_maps={}
for source in scope['sources']:
 mapping={}
 for a,b,size in difflib.SequenceMatcher(a=baseline['files'][source].splitlines(),b=Path(source).read_text().splitlines(),autojunk=False).get_matching_blocks():
  for offset in range(size):mapping[a+offset+1]=b+offset+1
 line_maps[source]=mapping
expected=[]
for row in old:
 if row in removed:continue
 item=copy.deepcopy(row)
 for loc in item['locations']:
  physical=loc['physicalLocation'];path=physical['artifactLocation']['uri'].removeprefix('./')
  if path in line_maps:
   for field in ['startLine','endLine']:
    if field in physical['region']:
     line=physical['region'][field];assert line in line_maps[path],(row,field);physical['region'][field]=line_maps[path][line]
 expected.append(item)
key=lambda row:json.dumps(row,sort_keys=True);a=collections.Counter(map(key,expected));b=collections.Counter(map(key,new));assert not a-b,'previous diagnostic changed'
added=[json.loads(x) for x in (b-a).elements()];assert len(added)==8 and all(x['ruleId']=='V19' and x['level']=='note' for x in added),added
alternatives={'I1':[38],'I2':[6,114],'I3':[118],'I4':[115],'I5':[5,109],'I6':[99]};classified=[]
for target in scope['targets']:
 text=Path(target['source']).read_text();starts=list(re.finditer(r'^(?:pub(?:\([^)]*\))? )?fn '+target['function']+r'\(',text,re.M));assert len(starts)==1
 first=text[:starts[0].start()].count('\n')+1;after=re.search(r'^\s*(?:pub(?:\([^)]*\))? )?fn |^#\[cfg\(test\)\]',text[starts[0].end():],re.M);end=text[:starts[0].end()+after.start()].count('\n')+1 if after else len(text.splitlines())+1
 notes=[x for x in added if x['locations'][0]['physicalLocation']['artifactLocation']['uri'].removeprefix('./')==target['source'] and first<=x['locations'][0]['physicalLocation']['region']['startLine']<end]
 counts=[int(re.search(r'Alternativas escritas nos grupos: (\d+);',x['message']['text']).group(1)) for x in notes]
 assert sorted(counts)==sorted(alternatives[target['id']]),(target,counts)
 classified.append({**target,'notes':notes,'alternatives_written':counts,'decision':'V19 informativo preservado por consequência uniforme explícita no L0; não é prova de cobertura de cada alternativa'})
counts=lambda rows:dict(collections.Counter(x['ruleId']+':'+x['level'] for x in rows));assert counts(old)=={'V16:error':240,'V19:note':892,'V20:note':871};assert counts(new)=={'V16:error':239,'V19:note':900,'V20:note':871}
record={'utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'old_counts':counts(old),'new_counts':counts(new),'removed_selected_v16':removed,'new_v19_equivalent_groups':classified,'other_diagnostics_preserved_including_mapped_locations':True,'line_maps':line_maps,'global_gate_still_fails':True}
(out/'architecture-delta.json').write_text(json.dumps(record,indent=2)+'\n');(out/'remaining-v16.json').write_text(json.dumps([x for x in new if x['ruleId']=='V16'],indent=2)+'\n');print(json.dumps({'V16':'240 -> 239','V19':'892 -> 900','V20':'871 preserved','latent_decisions_protected':5}),flush=True)
