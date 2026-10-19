from pathlib import Path
import collections,json,gzip,copy,difflib
out=Path(__file__).resolve().parents[1];scope=json.load(open(out/'scope.json'));old=json.load(open('00_nucleo/diagnosticos/tekt-v16-figure-casts-2026-10-17/validation-architecture.log'))['runs'][0]['results'];new=json.load(open(out/'validation-architecture.log'))['runs'][0]['results'];uri=lambda r:r['locations'][0]['physicalLocation']['artifactLocation']['uri'].removeprefix('./');key=lambda r:json.dumps(r,sort_keys=True,ensure_ascii=False);count=lambda rs:dict(collections.Counter(r['ruleId']+':'+r['level'] for r in rs));affected=set(scope['sources']);b=json.load(gzip.open(out/'before.json.gz'));maps={}
for p in affected:
 olds=b['files'][p].splitlines();news=Path(p).read_text().splitlines();maps[p]={i+1:j+1 for m in difflib.SequenceMatcher(None,olds,news).get_matching_blocks() for i,j in zip(range(m.a,m.a+m.size),range(m.b,m.b+m.size))}
expected=[]
for row in old:
 x=copy.deepcopy(row);p=uri(row)
 if p in affected:
  location=x['locations'][0]['physicalLocation'];line=location['region']['startLine'];assert line in maps[p],row;location['region']['startLine']=maps[p][line]
 expected.append(x)
assert collections.Counter(map(key,expected))==collections.Counter(map(key,new)),'Architecture diagnostics changed after mapped line shifts'
assert not [r for r in new if r['level'] in ['error','warning'] and r['ruleId']!='V16']
r=dict(before=count(old),after=count(new),all_diagnostics_preserved_after_line_mapping=True,no_new_errors_or_warnings=True,scope=scope['sources']);(out/'architecture-delta.json').write_text(json.dumps(r,indent=2)+'\n');print(r['after'])
