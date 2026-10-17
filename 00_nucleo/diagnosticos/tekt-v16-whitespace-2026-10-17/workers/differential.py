from pathlib import Path
import collections,difflib,gzip,json
out=Path(__file__).resolve().parents[1];scope=json.load(open(out/'scope.json'));b=json.load(gzip.open(out/'before.json.gz'));probe=json.load(gzip.open(out/'probe-green-infra-input.json.gz'));old_ast=probe['AST_before'];new_ast=probe['AST_after'];maps={};positions={};nodes={};aux={}
for source in scope['sources']:
 s=Path(source).read_text();marker=s.index('\n#[cfg(test)]\nmod v16_whitespace_tests');cut=s[:marker].count('\n')+1;current=[n for n in new_ast[source] if n['line']<cut]
 if source.endswith('/space.rs'):
  a=s.index('fn space_boundary_content(');line=s[:s.index('match content {',a)].count('\n')+1;helper=[n for n in current if n['line']==line];assert len(helper)==1;aux[source]=helper[0];current=[n for n in current if n['line']!=line]
 assert len(current)==len(old_ast[source]);nodes[source]=current;positions[source]={}
 for a,z in zip(old_ast[source],current):
  assert len(a['arms'])==len(z['arms'])
  for aa,zz in zip(a['arms'],z['arms']):positions[source][(aa['line'],aa['column'])]=(zz['line'],zz['column'])
 maps[source]={i+1:j+1 for block in difflib.SequenceMatcher(None,b['files'][source].splitlines(),s.splitlines(),autojunk=False).get_matching_blocks() for i,j in zip(range(block.a,block.a+block.size),range(block.b,block.b+block.size))}
def rows(file):return json.load(open(file))['runs'][0]['results']
def loc(r):
 p=r['locations'][0]['physicalLocation'];return p['artifactLocation']['uri'].removeprefix('./'),p['region']['startLine'],p['region']['startColumn']
def key(r,line=None,column=None):
 x=json.loads(json.dumps(r));reg=x['locations'][0]['physicalLocation']['region']
 if line is not None:reg['startLine']=line;reg['startColumn']=column
 return json.dumps(x,sort_keys=True,ensure_ascii=False)
before=rows(out/'baseline-architecture.log');after=rows(out/'validation-architecture.log');selected={(r['source'],r['line']) for r in scope['targets'] if r['status']=='emitted-warning'};expected=[];removed=[]
for r in before:
 p,n,c=loc(r)
 if r['ruleId']=='V16' and (p,n) in selected:removed.append(r);continue
 if p in positions:
  if (n,c) in positions[p]:nn,cc=positions[p][(n,c)]
  else:assert n in maps[p],(r['ruleId'],p,n,c);nn,cc=maps[p][n],c
  expected.append(key(r,nn,cc))
 else:expected.append(key(r))
missing=collections.Counter(expected)-collections.Counter(key(r) for r in after);added=collections.Counter(key(r) for r in after)-collections.Counter(expected);assert not missing,list(missing.items())[:4];assert len(removed)==11;assert sum(added.values())==13
new_notes=[]
for k,n in added.items():
 r=json.loads(k);assert n==1 and r['ruleId']=='V19' and r['level']=='note';new_notes.append(r)
expected_positions=[]
for target in scope['targets']:
 source=target['source'];ids=[i for i,n in enumerate(old_ast[source]) if n['line']==target['match_line'] and n['end_line']==target['match_end_line']];assert len(ids)==1;i=ids[0];arms=[j for j,a in enumerate(old_ast[source][i]['arms']) if a['pattern']==target['pattern_source']];assert len(arms)==1;arm=nodes[source][i]['arms'][arms[0]];expected_positions.append((source,arm['line'],arm['column']))
for source,node in aux.items():arm=node['arms'][-1];expected_positions.append((source,arm['line'],arm['column']))
assert collections.Counter(loc(r) for r in new_notes)==collections.Counter(expected_positions)
counts=lambda rs:dict(collections.Counter(r['ruleId']+':'+r['level'] for r in rs));record=dict(before=counts(before),after=counts(after),removed_V16=removed,new_V19=new_notes,all_other_diagnostics_preserved=True,position_mapping='Exact fields; AST match/arm ordinals excluding the new helper and appended tests; exact unchanged line blocks elsewhere.');assert record['after']=={'V16:error':141,'V19:note':1041,'V20:note':869};(out/'architecture-delta.json').write_text(json.dumps(record,indent=2,ensure_ascii=False)+'\n');remaining=[r for r in after if r['ruleId']=='V16'];assert not any(loc(r)[0] in scope['sources'] for r in remaining);(out/'remaining-v16.json').write_text(json.dumps(remaining,indent=2,ensure_ascii=False)+'\n');print(json.dumps(record['after']),flush=True)
