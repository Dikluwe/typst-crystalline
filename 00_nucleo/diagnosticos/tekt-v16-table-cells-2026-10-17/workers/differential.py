from pathlib import Path
import collections,difflib,gzip,json,subprocess,re
out=Path(__file__).resolve().parents[1];s=json.load(open(out/'scope.json'));b=json.load(gzip.open(out/'before.json.gz'));maps={}
for source in s['sources']:
 a=b['files'][source].splitlines();z=Path(source).read_text().splitlines();maps[source]={i+1:j+1 for block in difflib.SequenceMatcher(None,a,z,autojunk=False).get_matching_blocks() for i,j in zip(range(block.a,block.a+block.size),range(block.b,block.b+block.size))}
def rows(path):return json.load(open(path))['runs'][0]['results']
def loc(r):
 l=r['locations'][0]['physicalLocation'];return l['artifactLocation']['uri'].removeprefix('./'),l['region']['startLine'],l['region']['startColumn']
def key(r,line=None):
 x=json.loads(json.dumps(r))
 if line is not None:x['locations'][0]['physicalLocation']['region']['startLine']=line
 return json.dumps(x,sort_keys=True,ensure_ascii=False)
before=rows(out/'baseline-architecture.log');after=rows(out/'validation-architecture.log');selected={(t['source'],t['line']) for t in s['targets'] if t['status']=='emitted-warning'};snapshots={(t['source'],t['match_line']+5) for t in s['targets'] if t['mode']=='snapshot'};expected=[];removed=[]
for r in before:
 p,n,c=loc(r)
 if r['ruleId']=='V16' and (p,n) in selected:removed.append(r);continue
 if r['ruleId']=='V20' and (p,n) in snapshots:removed.append(r);continue
 if p in maps:assert n in maps[p],(r['ruleId'],p,n,c);expected.append(key(r,maps[p][n]))
 else:expected.append(key(r))
missing=collections.Counter(expected)-collections.Counter(map(key,after));added=collections.Counter(map(key,after))-collections.Counter(expected)
assert not missing,list(missing.items())[:6];assert len(removed)==12;assert sum(added.values())==18,(len(added),list(added)[:3])
notes=[]
for k,n in added.items():
 r=json.loads(k);assert n==1 and r['ruleId']=='V19' and r['level']=='note';notes.append(r)
# Each added note belongs to exactly one prescribed exhaustive match in the proof.
proof=json.load(open(out/'probe-green-result.json'));by_position={(t['source'],t['line']):t for t in proof['targets']};positions=set(by_position);by_id={t['id']:t for t in s['targets']};actual=[]
norm=lambda p:re.sub(r'\s+','',p)
for source in s['sources']:
 nodes=json.loads(subprocess.check_output(['/tmp/tekt-table-cells-latent',source,'matches'],text=True))
 for n in nodes:
  if (source,n['line']) in positions:
   target=by_position[source,n['line']]
   if target['id'] in by_id:arms=[a for a in n['arms'] if norm(a['pattern'])==norm(by_id[target['id']]['replacement'])]
   else:arms=[a for a in n['arms'] if ' | ' in a['pattern']]
   assert len(arms)==1,(source,n);actual.append((source,arms[0]['line'],arms[0]['column']))
assert collections.Counter(map(loc,notes))==collections.Counter(actual)
counts=lambda rs:dict(collections.Counter(r['ruleId']+':'+r['level'] for r in rs));record=dict(before=counts(before),after=counts(after),removed=removed,new_V19=notes,all_other_diagnostics_preserved=True,mapping='Full SARIF fields; exact unchanged line blocks; only ten selected V16 and two snapshot V20 removed. New V19 matched to 18 prescribed/proven AST matches.')
assert record['after']=={'V16:error':131,'V19:note':1059,'V20:note':867},record['after'];(out/'architecture-delta.json').write_text(json.dumps(record,indent=2,ensure_ascii=False)+'\n');remaining=[r for r in after if r['ruleId']=='V16'];assert not any(loc(r)[0] in s['sources'] for r in remaining);(out/'remaining-v16.json').write_text(json.dumps(remaining,indent=2,ensure_ascii=False)+'\n');print(json.dumps(record['after']),flush=True)
