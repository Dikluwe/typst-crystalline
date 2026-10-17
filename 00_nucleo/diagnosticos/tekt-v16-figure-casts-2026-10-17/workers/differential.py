from pathlib import Path
import collections,gzip,json,copy,difflib
out=Path(__file__).resolve().parents[1];scope=json.load(open(out/'scope.json'));old=json.load(open('00_nucleo/diagnosticos/tekt-v16-catalogos-2026-10-17/validation-architecture.log'))['runs'][0]['results'];new=json.load(open(out/'validation-architecture.log'))['runs'][0]['results'];uri=lambda r:r['locations'][0]['physicalLocation']['artifactLocation']['uri'].removeprefix('./');key=lambda r:json.dumps(r,sort_keys=True,ensure_ascii=False);count=lambda rs:dict(collections.Counter(r['ruleId']+':'+r['level'] for r in rs));affected=set(scope['sources']);a=[x for x in old if uri(x) not in affected];z=[x for x in new if uri(x) not in affected];assert collections.Counter(map(key,a))==collections.Counter(map(key,z)),'Unrelated diagnostics changed'
assert not [r for r in new if r['level'] in ['error','warning'] and r['ruleId']!='V16']
a=[r for r in old if uri(r) in affected];z=[r for r in new if uri(r) in affected];assert collections.Counter(x['ruleId'] for x in a)['V16']==15
assert len([x for x in z if x['ruleId']=='V16'])==12;assert len([x for x in old if x['ruleId']=='V16'])-len([x for x in new if x['ruleId']=='V16'])==3
b=json.load(gzip.open(out/'working-baseline.json.gz'));maps={};expected=[];removed=[]
for row in [x for x in old if x['ruleId']=='V16']:
 p=uri(row);x=copy.deepcopy(row)
 if p not in affected:expected.append(x);continue
 if p not in maps:
  olds=b['files'][p].splitlines();news=Path(p).read_text().splitlines();maps[p]={i+1:j+1 for m in difflib.SequenceMatcher(None,olds,news).get_matching_blocks() for i,j in zip(range(m.a,m.a+m.size),range(m.b,m.b+m.size))}
 location=x['locations'][0]['physicalLocation'];line=location['region']['startLine']
 if line in maps[p]:location['region']['startLine']=maps[p][line];expected.append(x)
 else:removed.append(x)
assert collections.Counter(uri(x) for x in removed)==collections.Counter({'01_core/src/compiler/stdlib/figure_image.rs':1,'01_core/src/compiler/stdlib/layout/casting.rs':2}),removed
assert collections.Counter(map(key,expected))==collections.Counter(map(key,[x for x in new if x['ruleId']=='V16'])),'Unexpected V16 delta after line mapping'
r=dict(before=count(old),after=count(new),unrelated_diagnostics_identical=True,no_new_errors_or_warnings=True,all_remaining_v16_preserved_after_line_mapping=True,removed_v16=removed,selected_before=a,selected_after=z,scope=scope['sources']);(out/'architecture-delta.json').write_text(json.dumps(r,indent=2)+'\n');print(r['after'])
