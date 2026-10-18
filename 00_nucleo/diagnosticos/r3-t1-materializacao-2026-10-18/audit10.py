from pathlib import Path
import json,gzip,re,collections,datetime,hashlib,subprocess
p=Path(__file__).resolve().parent
read=lambda n:json.loads(gzip.decompress((p/n).read_bytes()))
before=read('before-sources.json.gz')['files'];after=read('candidate10-sources.json.gz')['files'];assert all(Path(f).read_text()==s for f,s in after.items())
old=read('lint-before.json.gz')['runs'][0]['results'];new=json.loads((p/'architecture-candidate10.log').read_text())['runs'][0]['results']
def path(row):return row['locations'][0]['physicalLocation']['artifactLocation']['uri'].removeprefix('file://').removeprefix(str(Path.cwd())+'/').removeprefix('./')
def key(row,sources):
 f=path(row);line=row['locations'][0]['physicalLocation']['region']['startLine'];text=sources.get(f,'');prefix='\n'.join(text.splitlines()[:line]);names=re.findall(r'^\s*(?:pub(?:\([^)]*\))?\s+)?(?:async\s+)?fn\s+(\w+)\s*[<(]',prefix,re.M);return (row['ruleId'],row['level'],f,names[-1] if names else '<module>',row['message']['text'])
a=collections.Counter(key(row,before) for row in old if row['level'] in ['warning','error']);b=collections.Counter(key(row,after) for row in new if row['level'] in ['warning','error']);added=b-a;removed=a-b
counts=lambda rows:dict(collections.Counter(row['ruleId']+':'+row['level'] for row in rows))
changed=[f for f,s in after.items() if before.get(f)!=s]
# A supplementary source audit: public declarations/signatures, excluding crate-private helpers.
signature=re.compile(r'^\s*pub\s+(?:async\s+)?fn\s+\w+[^{};]*\{',re.M)
surface=[]
for f in changed:
 if f.endswith('.rs'):
  normalize=lambda s:[' '.join(x.split()) for x in signature.findall(s)]
  x,y=normalize(before.get(f,'')),normalize(after[f])
  if x!=y:surface.append(dict(file=f,before=x,after=y))
r=dict(utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),head=subprocess.check_output(['git','rev-parse','HEAD'],text=True).strip(),diffstat=subprocess.check_output(['git','diff','HEAD','--stat'],text=True),before=counts(old),after=counts(new),added_blocking=[dict(key=list(k),count=v) for k,v in added.items()],removed_blocking=[dict(key=list(k),count=v) for k,v in removed.items()],changed_files=changed,public_signature_differences=surface,notes='Blocking diagnostics compared by rule, severity, file, enclosing function and message so source line movement is not counted as a new defect. V19/V20 remain visible; signatures audit is supplementary to source review and existing contract tests.')
(p/'audit-candidate10.json').write_text(json.dumps(r,indent=2)+'\n');print(json.dumps({k:r[k] for k in ['before','after','added_blocking','removed_blocking','public_signature_differences']},indent=2))
assert not added,'Inspect every added blocking diagnostic'
assert not surface,'Inspect any apparent public signature change'
