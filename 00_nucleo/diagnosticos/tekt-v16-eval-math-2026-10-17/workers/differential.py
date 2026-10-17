from pathlib import Path
import json,collections,difflib,gzip,subprocess,tempfile
out=Path(__file__).resolve().parents[1];scope=json.load(open(out/'scope.json'));b=json.load(gzip.open(out/'before.json.gz'));source=scope['sources'][0];old=b['files'][source];new=Path(source).read_text();other_line_maps={p:{i+1:j+1 for block in difflib.SequenceMatcher(None,b['files'][p].splitlines(),Path(p).read_text().splitlines(),autojunk=False).get_matching_blocks() for i,j in zip(range(block.a,block.a+block.size),range(block.b,block.b+block.size))} for p in scope['sources'] if p!=source};line_map={i+1:j+1 for block in difflib.SequenceMatcher(None,old.splitlines(),new.splitlines(),autojunk=False).get_matching_blocks() for i,j in zip(range(block.a,block.a+block.size),range(block.b,block.b+block.size))}
with tempfile.TemporaryDirectory(prefix='tekt-eval-map-') as t:
 p=Path(t)/'before.rs';p.write_text(old);before_nodes=json.loads(subprocess.check_output(['/tmp/tekt-eval-math-latent',str(p),'matches'],text=True));after_nodes=json.loads(subprocess.check_output(['/tmp/tekt-eval-math-latent',source,'matches'],text=True))
aux=[n for n in after_nodes if n['scrutinee']=='prime_content'];assert len(aux)==1;nodes=[n for n in after_nodes if n['scrutinee']!='prime_content'];assert len(nodes)==len(before_nodes);position_map={}
for a,z in zip(before_nodes,nodes):
 assert len(a['arms'])==len(z['arms'])
 for aa,zz in zip(a['arms'],z['arms']):position_map[(aa['line'],aa['column'])]=(zz['line'],zz['column'])
def rows(file):return json.load(open(file))['runs'][0]['results']
def loc(r):
 l=r['locations'][0]['physicalLocation'];return l['artifactLocation']['uri'].removeprefix('./'),l['region']['startLine'],l['region']['startColumn']
def key(r,line=None,column=None):
 z=json.loads(json.dumps(r));reg=z['locations'][0]['physicalLocation']['region']
 if line is not None:reg['startLine']=line
 if column is not None:reg['startColumn']=column
 return json.dumps(z,sort_keys=True,ensure_ascii=False)
selected={r['line'] for r in scope['targets'] if r['status']=='emitted-warning'};before=rows(out/'baseline-architecture.log');after=rows(out/'validation-architecture.log');expected=[];removed=[];removed_depth=[]
for r in before:
 p,n,c=loc(r)
 if p==source and r['ruleId']=='V16' and n in selected:removed.append(r);continue
 if p==source and r['ruleId']=='V20' and '`MathAttachSlot::Present(Content::MathPrimes(elem))`' in r['message']['text']:removed_depth.append(r);continue
 if p==source:
  if (n,c) in position_map:nn,cc=position_map[(n,c)]
  else:assert n in line_map,(r['ruleId'],n,c);nn,cc=line_map[n],c
  expected.append(key(r,nn,cc))
 elif p in other_line_maps:
  assert n in other_line_maps[p],(p,n);expected.append(key(r,other_line_maps[p][n],c))
 else:expected.append(key(r))
missing=collections.Counter(expected)-collections.Counter(key(r) for r in after);added=collections.Counter(key(r) for r in after)-collections.Counter(expected);assert not missing,list(missing.items())[:3];assert len(removed)==52 and len(removed_depth)==1;assert sum(added.values())==71
additions=[]
for k,n in added.items():
 r=json.loads(k);p,line,col=loc(r);assert n==1 and r['ruleId']=='V19' and r['level']=='note' and p==source;additions.append(r)
expected_positions=[]
for target in scope['targets']:
 idx=[i for i,n in enumerate(before_nodes) if n['line']==target['match_line'] and n['end_line']==target['match_end_line']];assert len(idx)==1;idx=idx[0];arm=[i for i,a in enumerate(before_nodes[idx]['arms']) if a['pattern']=='_'];assert len(arm)==1;expected_positions.append((nodes[idx]['arms'][arm[0]]['line'],nodes[idx]['arms'][arm[0]]['column']))
expected_positions.append((aux[0]['arms'][1]['line'],aux[0]['arms'][1]['column']));assert collections.Counter((loc(r)[1],loc(r)[2]) for r in additions)==collections.Counter(expected_positions)
counts=lambda rs:dict(collections.Counter(r['ruleId']+':'+r['level'] for r in rs));record={'before':counts(before),'after':counts(after),'removed_V16':52,'removed_V20':removed_depth,'new_V19':additions,'all_other_diagnostics_preserved':True,'position_mapping':'AST match/arm ordinals, excluding exactly the auxiliary prime_content match; unchanged line blocks for other locations'};assert record['after']=={'V16:error':170,'V19:note':989,'V20:note':870};(out/'architecture-delta.json').write_text(json.dumps(record,ensure_ascii=False,indent=2)+'\n');remaining=[r for r in after if r['ruleId']=='V16'];assert not any(loc(r)[0]==source for r in remaining);(out/'remaining-v16.json').write_text(json.dumps(remaining,ensure_ascii=False,indent=2)+'\n');print(json.dumps({'V16':170,'V19':989,'V20':870,'other_diagnostics_preserved':True}),flush=True)
