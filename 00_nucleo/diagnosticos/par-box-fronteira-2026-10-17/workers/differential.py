from pathlib import Path
import json,gzip,collections,difflib,copy
out=Path(__file__).resolve().parents[1];e=json.load(gzip.open(out/'extended-before.json.gz'));scope=json.load(open(out/'scope.json'));maps={}
for p in scope['sources']:
 old=e['files'][p].splitlines();new=Path(p).read_text().splitlines();maps[p]={i+1:j+1 for b in difflib.SequenceMatcher(None,old,new,autojunk=False).get_matching_blocks() for i,j in zip(range(b.a,b.a+b.size),range(b.b,b.b+b.size))}
old=json.load(open('00_nucleo/diagnosticos/tekt-v16-paragraph-2026-10-17/validation-architecture.log'))['runs'][0]['results'];new=json.load(open(out/'validation-architecture.log'))['runs'][0]['results'];expected=[]
for r in old:
 x=copy.deepcopy(r);q=x['locations'][0]['physicalLocation'];p=q['artifactLocation']['uri'].removeprefix('./');n=q['region']['startLine']
 if p in maps:assert n in maps[p],(p,n,x['ruleId']);q['region']['startLine']=maps[p][n]
 expected.append(x)
key=lambda x:json.dumps(x,sort_keys=True,ensure_ascii=False)
missing=collections.Counter(map(key,expected))-collections.Counter(map(key,new));added=collections.Counter(map(key,new))-collections.Counter(map(key,expected));assert not missing and not added,dict(missing=missing,added=added)
counts=lambda r:dict(collections.Counter(x['ruleId']+':'+x['level'] for x in r));record=dict(before=counts(old),after=counts(new),all_diagnostics_preserved=True);assert record['after']=={'V16:error':125,'V19:note':1065,'V20:note':867};(out/'architecture-delta.json').write_text(json.dumps(record,indent=2)+'\n');print(record)
