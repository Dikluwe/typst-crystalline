from pathlib import Path
import json,collections,datetime,gzip,hashlib,difflib,copy
out=Path(__file__).resolve().parents[1];prior=out.parent/'native-format-catalogue-2026-10-17';b=json.load(gzip.open(out/'before.json.gz'));scope=json.load(open(out/'scope.json'));old=json.load(open(prior/'validation-architecture.log'))['runs'][0]['results'];new=json.load(open(out/'validation-architecture.log'))['runs'][0]['results'];maps={}
assert json.load(gzip.open(prior/'final-state.json.gz'))['source_hashes']==b['hashes']
for p in scope['sources']:
 a=[line.lstrip() for line in b['files'][p].splitlines()];z=[line.lstrip() for line in Path(p).read_text().splitlines()];maps[p]={i+1:j+1 for m in difflib.SequenceMatcher(None,a,z).get_matching_blocks() for i,j in zip(range(m.a,m.a+m.size),range(m.b,m.b+m.size))}
expected=[]
for r in old:
 row=copy.deepcopy(r);loc=row['locations'][0]['physicalLocation'];p=loc['artifactLocation']['uri'].removeprefix('./')
 if p in maps:
  line=loc['region']['startLine'];assert line in maps[p],row;loc['region']['startLine']=maps[p][line]
 expected.append(row)
key=lambda r:json.dumps(r,sort_keys=True,ensure_ascii=False);a=collections.Counter(map(key,expected));z=collections.Counter(map(key,new));assert a==z,dict(removed=list((a-z).items()),added=list((z-a).items()));counts=collections.Counter(r['ruleId']+':'+r['level'] for r in new)
(out/'architecture-delta.json').write_text(json.dumps(dict(utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),baseline_sha256=hashlib.sha256((prior/'validation-architecture.log').read_bytes()).hexdigest(),baseline_source_hashes_verified=True,exact_mapped_diagnostics_preserved=True,counts=dict(counts)),indent=2)+'\n');print('Architecture preserved',counts)
