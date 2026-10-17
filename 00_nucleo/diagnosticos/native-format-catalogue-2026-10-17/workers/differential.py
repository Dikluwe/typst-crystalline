from pathlib import Path
import collections,json,gzip,copy,difflib,hashlib
out=Path(__file__).resolve().parents[1];prior=Path('00_nucleo/diagnosticos/native-math-catalogue-2026-10-17');scope=json.load(open(out/'scope.json'));b=json.load(gzip.open(out/'before.json.gz'));assert json.load(gzip.open(prior/'final-state.json.gz'))['source_hashes']==b['hashes'];old=json.load(open(prior/'validation-architecture.log'))['runs'][0]['results'];new=json.load(open(out/'validation-architecture.log'))['runs'][0]['results'];uri=lambda r:r['locations'][0]['physicalLocation']['artifactLocation']['uri'].removeprefix('./');key=lambda r:json.dumps(r,sort_keys=True,ensure_ascii=False);count=lambda rs:dict(collections.Counter(r['ruleId']+':'+r['level'] for r in rs));maps={}
for p in scope['sources']:
 a=b['files'][p].splitlines();z=Path(p).read_text().splitlines();maps[p]={i+1:j+1 for m in difflib.SequenceMatcher(None,a,z).get_matching_blocks() for i,j in zip(range(m.a,m.a+m.size),range(m.b,m.b+m.size))}
expected=[];unmapped=[]
for r in old:
 row=copy.deepcopy(r);p=uri(row)
 if p in maps:
  loc=row['locations'][0]['physicalLocation'];line=loc['region']['startLine']
  if line not in maps[p]:unmapped.append(row);continue
  loc['region']['startLine']=maps[p][line]
 expected.append(row)
a=collections.Counter(map(key,expected));z=collections.Counter(map(key,new));removed=[json.loads(k) for k in (a-z).elements()];added=[json.loads(k) for k in (z-a).elements()];(out/'differential-review.json').write_text(json.dumps(dict(unmapped=unmapped,removed=removed,added=added),indent=2)+'\n');assert not unmapped,'Existing diagnostic line unmapped';updated=[]
for previous in removed:
 assert previous['ruleId']=='V19' and previous['level']=='note'
 matches=[r for r in added if r['ruleId']=='V19' and r['level']=='note' and r['locations']==previous['locations']]
 assert len(matches)==1,previous
 current=matches[0]
 pattern_re=r'Alternativas escritas nos grupos: (\d+); guard: ([^;]+); padrão: `([\s\S]*)`\.'
 import re
 a=re.search(pattern_re,previous['message']['text']);z=re.search(pattern_re,current['message']['text']);assert a and z
 assert int(z[1])==int(a[1])+1 and a[2]==z[2] and previous['message']['text'][:a.start()]==current['message']['text'][:z.start()]
 without_format=re.sub(r'\|\s*FuncRepr::Format\(\s*_\s*,\s*_\s*\)\s*','',z[3])
 assert re.sub(r'\s+','',without_format)==re.sub(r'\s+','',a[3]),(a[3],z[3])
 updated.append(dict(source=uri(current),line=current['locations'][0]['physicalLocation']['region']['startLine'],old_pattern=a[3],new_pattern=z[3],reason='Only explicit Format alternative added; no existing alternative removed',tests=['seven-representation identity matrix','native_format_catalogue_projection_groups_cover_all_representations','native_format_catalogue_observation_kind_and_payload_axes']))
 added.remove(current)
allowed={'01_core/src/entities/func.rs','01_core/src/compiler/eval/call_dispatch/apply.rs','01_core/src/compiler/eval/bindings/field_access/identity.rs'};notes=[]
for row in added:
 p=uri(row);assert row['ruleId']=='V19' and row['level']=='note' and p in allowed,row;line=row['locations'][0]['physicalLocation']['region']['startLine'];text=Path(p).read_text();offset=sum(len(x) for x in text.splitlines(True)[:line-1]);pattern=text[offset:text.index('=>',offset)];assert 'FuncRepr::Native(' in pattern and 'FuncRepr::Format(' in pattern and '|' in pattern,(p,line,pattern);notes.append(dict(source=p,line=line,pattern=pattern.strip(),source_sha256=hashlib.sha256(Path(p).read_bytes()).hexdigest(),justification='Same NativeFunc payload and ABI, both representations exercised',tests=['native_format_catalogue_public_native_projections','native_format_catalogue_observation_witness','144 bilateral field/call cases','456 bilateral name/alias/namespace controls']))
assert not [r for r in new if r['level'] in ['error','warning'] and r['ruleId']!='V16'];result=dict(baseline=str(prior),baseline_source_hashes_verified=True,before=count(old),after=count(new),all_existing_errors_and_warnings_preserved=True,existing_notes_preserved_except_explicit_format_extensions=True,no_new_errors_or_warnings=True,added_v19_notes=notes,updated_v19_notes=updated);(out/'architecture-delta.json').write_text(json.dumps(result,indent=2)+'\n');print(result['after']);print('new V19 grouped Native/Format',len(notes))
