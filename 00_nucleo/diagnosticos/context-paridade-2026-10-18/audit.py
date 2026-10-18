from pathlib import Path
import json,gzip,re,collections,datetime,hashlib,subprocess
p=Path(__file__).resolve().parent;r=p.parents[2]
read=lambda n:json.loads(gzip.decompress((p/n).read_bytes()))
before=read('before-sources.json.gz')['files'];after=read('text-final-sources.json.gz')['files']
assert all(Path(f).read_text()==s for f,s in after.items())
old=json.loads((p.parent/'r3-t1-materializacao-2026-10-18/architecture-candidate10.log').read_text())['runs'][0]['results']
new=json.loads((p/'architecture-text-final.log').read_text())['runs'][0]['results']
def path(row):return row['locations'][0]['physicalLocation']['artifactLocation']['uri'].removeprefix('file://').removeprefix(str(r)+'/').removeprefix('./')
def key(row,sources):
 f=path(row);line=row['locations'][0]['physicalLocation']['region']['startLine'];prefix='\n'.join(sources.get(f,'').splitlines()[:line]);names=re.findall(r'^\s*(?:pub(?:\([^)]*\))?\s+)?(?:async\s+)?fn\s+(\w+)\s*[<(]',prefix,re.M);name=names[-1] if names else '<module>'
 if f=='01_core/src/compiler/realize/mod.rs' and name=='context_recipe':name='realize_math_insertions'
 return row['ruleId'],row['level'],f,name,row['message']['text']
a=collections.Counter(key(x,before) for x in old if x['level'] in ['warning','error']);b=collections.Counter(key(x,after) for x in new if x['level'] in ['warning','error'])
changed=[f for f,s in after.items() if before.get(f)!=s]
pattern=re.compile(r'^\s*pub\s+(?:async\s+)?fn\s+\w+[^{};]*\{',re.M)
surface=[]
for f in changed:
 if f.endswith('.rs'):
  normalize=lambda s:[' '.join(x.split()) for x in pattern.findall(s)]
  if normalize(before[f])!=normalize(after[f]):surface.append(f)
d=dict(utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),head=subprocess.check_output(['git','rev-parse','HEAD'],text=True).strip(),diffstat=subprocess.check_output(['git','diff','HEAD','--stat'],text=True),before=dict(collections.Counter(x['ruleId']+':'+x['level'] for x in old)),after=dict(collections.Counter(x['ruleId']+':'+x['level'] for x in new)),added=[dict(key=k,count=v) for k,v in (b-a).items()],removed=[dict(key=k,count=v) for k,v in (a-b).items()],public_signature_changes=surface,changed=changed,notes='Baseline SARIF is historical candidate10, whose committed productive source is this batch baseline. Existing shallow context-recipe lookup moved unchanged from realize_math_insertions to context_recipe; normalize only that enclosing function for the blocking-diagnostic comparison. New fields/types additionally reviewed in diff; regex is supplementary.')
(p/'audit.json').write_text(json.dumps(d,indent=2)+'\n');print(json.dumps({k:d[k] for k in ['before','after','added','removed','public_signature_changes']},indent=2))
assert not d['added'];assert not surface
