from pathlib import Path
import collections,difflib,gzip,json,subprocess
out=Path(__file__).resolve().parents[1];b=json.load(gzip.open(out/'before.json.gz'));e=json.load(gzip.open(out/'extended-before.json.gz'));scope=json.load(open(out/'scope.json'));maps={}
for source in scope['sources']:
 old=(b['files'].get(source) or e['files'][source]).splitlines();new=Path(source).read_text().splitlines();maps[source]={i+1:j+1 for block in difflib.SequenceMatcher(None,old,new,autojunk=False).get_matching_blocks() for i,j in zip(range(block.a,block.a+block.size),range(block.b,block.b+block.size))}
old=json.load(open('00_nucleo/diagnosticos/tekt-v16-power-2026-10-17/validation-architecture.log'))['runs'][0]['results'];new=json.load(open(out/'validation-architecture.log'))['runs'][0]['results']
def loc(r):
 p=r['locations'][0]['physicalLocation'];return p['artifactLocation']['uri'].removeprefix('./'),p['region']['startLine'],p['region']['startColumn']
def key(r):return json.dumps(r,sort_keys=True,ensure_ascii=False)
remove={('01_core/src/compiler/layout/paragraph.rs',344,'V16'),('01_core/src/compiler/layout/paragraph.rs',354,'V16'),('01_core/src/compiler/layout/paragraph.rs',363,'V16'),('01_core/src/compiler/layout/paragraph.rs',350,'V19'),('01_core/src/compiler/stdlib/structural/par.rs',153,'V19')};expected=[];removed=[]
for row in old:
 p,n,c=loc(row)
 if (p,n,row['ruleId']) in remove:removed.append(row);continue
 r=json.loads(json.dumps(row))
 if p in maps:
  assert n in maps[p],(r['ruleId'],p,n,c);r['locations'][0]['physicalLocation']['region']['startLine']=maps[p][n]
 expected.append(key(r))
assert len(removed)==5
missing=collections.Counter(expected)-collections.Counter(map(key,new));added=collections.Counter(map(key,new))-collections.Counter(expected);assert not missing,missing;assert sum(added.values())==7,added
notes=[json.loads(s) for s in added];assert all(n==1 for n in added.values());assert all(r['ruleId']=='V19' and r['level']=='note' for r in notes)
prescribed=[]
for source in scope['sources']:
 nodes=json.loads(subprocess.check_output(['/tmp/tekt-table-cells-latent',source,'matches'],text=True))
 for node in nodes:
  if node['function'] not in ['paragraph_body','is_paragraph','is_empty_body','empty_text_style','native_par']:continue
  prescribed += [(source,a['line'],a['column']) for a in node['arms'] if '|' in a['pattern'].split(' if ',1)[0]]
assert collections.Counter(map(loc,notes))==collections.Counter(prescribed),(list(map(loc,notes)),prescribed)
counts=lambda rows:dict(collections.Counter(r['ruleId']+':'+r['level'] for r in rows));record=dict(before=counts(old),after=counts(new),removed=removed,added=notes,all_other_diagnostics_preserved=True);assert record['after']=={'V16:error':125,'V19:note':1065,'V20:note':867},record['after'];(out/'architecture-delta.json').write_text(json.dumps(record,indent=2)+'\n');print(record['after'])
