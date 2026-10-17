from pathlib import Path
import copy,collections,difflib,gzip,json
out=Path(__file__).resolve().parents[1];b=json.load(gzip.open(out/'before.json.gz'));scope=json.load(open(out/'scope.json'));old=json.load(open('00_nucleo/diagnosticos/text-size-length-2026-10-17/validation-architecture.log'))['runs'][0]['results'];new=json.load(open(out/'validation-architecture.log'))['runs'][0]['results'];uri=lambda r:r['locations'][0]['physicalLocation']['artifactLocation']['uri'].removeprefix('./');key=lambda r:json.dumps(r,sort_keys=True,ensure_ascii=False);count=lambda rs:dict(collections.Counter(r['ruleId']+':'+r['level'] for r in rs));affected=set(scope['sources']);a=[x for x in old if uri(x) not in affected];z=[x for x in new if uri(x) not in affected];assert collections.Counter(map(key,a))==collections.Counter(map(key,z)),'Unrelated diagnostics changed'
assert not [r for r in new if r['level'] in ['error','warning'] and r['ruleId']!='V16'], 'New error/warning'
assert collections.Counter(uri(x) for x in old if x['ruleId']=='V16')==collections.Counter(uri(x) for x in new if x['ruleId']=='V16')
expected=[];unmapped=[];maps={}
for row in old:
 p=uri(row)
 if p not in affected:expected.append(row);continue
 x=copy.deepcopy(row);location=x['locations'][0]['physicalLocation'];line=location['region']['startLine'];
 if p not in maps:
  olds=b['files'][p].splitlines();news=Path(p).read_text().splitlines();maps[p]={i+1:j+1 for m in difflib.SequenceMatcher(None,olds,news).get_matching_blocks() for i,j in zip(range(m.a,m.a+m.size),range(m.b,m.b+m.size))}
 mapping=maps[p]
 if line in mapping:location['region']['startLine']=mapping[line];expected.append(x)
 else:unmapped.append(row)
missing=collections.Counter(map(key,expected))-collections.Counter(map(key,new));added=collections.Counter(map(key,new))-collections.Counter(map(key,expected));result=dict(before=count(old),after=count(new),unrelated_diagnostics_identical=True,v16_per_consumer_preserved=True,no_new_errors_or_warnings=True,old_rows_on_changed_lines=unmapped,missing_after_line_mapping=[json.loads(k) for k in missing.elements()],added_after_line_mapping=[json.loads(k) for k in added.elements()],scope=scope['sources']);(out/'architecture-delta.json').write_text(json.dumps(result,indent=2)+'\n');print(result['after'])
