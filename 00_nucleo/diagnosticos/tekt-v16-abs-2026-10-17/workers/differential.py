from pathlib import Path
import copy,collections,difflib,gzip,json
out=Path(__file__).resolve().parents[1];b=json.load(gzip.open(out/'before.json.gz'));source='01_core/src/compiler/stdlib/calc/abs.rs';old_source=b['files'][source].splitlines();new_source=Path(source).read_text().splitlines();mapping={i+1:j+1 for m in difflib.SequenceMatcher(None,old_source,new_source,autojunk=False).get_matching_blocks() for i,j in zip(range(m.a,m.a+m.size),range(m.b,m.b+m.size))}
old=json.load(open('00_nucleo/diagnosticos/par-box-fronteira-2026-10-17/validation-architecture.log'))['runs'][0]['results'];new=json.load(open(out/'validation-architecture.log'))['runs'][0]['results'];expected=[];removed=[]
for row in old:
 x=copy.deepcopy(row);p=x['locations'][0]['physicalLocation'];uri=p['artifactLocation']['uri'].removeprefix('./');line=p['region']['startLine']
 if uri==source and row['ruleId']=='V16' and line==68:removed.append(row);continue
 if uri==source:assert line in mapping,(row['ruleId'],line);p['region']['startLine']=mapping[line]
 expected.append(x)
key=lambda r:json.dumps(r,sort_keys=True,ensure_ascii=False);missing=collections.Counter(map(key,expected))-collections.Counter(map(key,new));added=collections.Counter(map(key,new))-collections.Counter(map(key,expected));assert len(removed)==1 and not missing and sum(added.values())==1,(removed,missing,added)
row=json.loads(next(iter(added)));p=row['locations'][0]['physicalLocation'];assert row['ruleId']=='V19' and row['level']=='note' and p['artifactLocation']['uri'].removeprefix('./')==source;assert 'Value::None' in new_source[p['region']['startLine']-1]
counts=lambda rs:dict(collections.Counter(r['ruleId']+':'+r['level'] for r in rs));r=dict(before=counts(old),after=counts(new),removed=removed,added=[row],all_other_diagnostics_preserved=True);assert r['after']=={'V16:error':124,'V19:note':1066,'V20:note':867};(out/'architecture-delta.json').write_text(json.dumps(r,indent=2)+'\n');print(r['after'])
