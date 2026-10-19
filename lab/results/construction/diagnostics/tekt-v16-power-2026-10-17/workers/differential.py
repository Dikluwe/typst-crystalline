from pathlib import Path
import json,gzip,collections,difflib
out=Path(__file__).resolve().parents[1];b=json.load(gzip.open(out/'before.json.gz'));source=b['source'];a=b['files'][source].splitlines();z=Path(source).read_text().splitlines();mapping={i+1:j+1 for block in difflib.SequenceMatcher(None,a,z,autojunk=False).get_matching_blocks() for i,j in zip(range(block.a,block.a+block.size),range(block.b,block.b+block.size))}
old=json.load(open('00_nucleo/diagnosticos/tekt-v16-table-cells-2026-10-17/validation-architecture.log'))['runs'][0]['results'];new=json.load(open(out/'validation-architecture.log'))['runs'][0]['results']
def loc(r):
 p=r['locations'][0]['physicalLocation'];return p['artifactLocation']['uri'].removeprefix('./'),p['region']['startLine']
def key(r):return json.dumps(r,sort_keys=True,ensure_ascii=False)
expected=[];removed=[]
for r in old:
 p,n=loc(r)
 if p==source and r['ruleId']=='V16':removed.append(r);continue
 if p==source:
  r=json.loads(json.dumps(r));assert n in mapping,(r['ruleId'],n);r['locations'][0]['physicalLocation']['region']['startLine']=mapping[n]
 expected.append(key(r))
assert len(removed)==3
missing=collections.Counter(expected)-collections.Counter(map(key,new));added=collections.Counter(map(key,new))-collections.Counter(expected);assert not missing,missing
assert sum(added.values())==1,added
for s,n in added.items():
 r=json.loads(s);assert n==1 and r['ruleId']=='V19' and r['level']=='note' and loc(r)[0]==source
assert not any(r['ruleId']=='V16' and loc(r)[0]==source for r in new)
counts=lambda rs:dict(collections.Counter(r['ruleId']+':'+r['level'] for r in rs));record=dict(before=counts(old),after=counts(new),removed=removed,added=[json.loads(s) for s in added],all_other_diagnostics_preserved=True);(out/'architecture-delta.json').write_text(json.dumps(record,indent=2)+'\n');print(record['after'])
