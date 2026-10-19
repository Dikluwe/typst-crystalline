from pathlib import Path
import hashlib,json,re,subprocess,tomllib,datetime
root=Path('/repos/Antigravity/typst-crystalline');out=root/'lab/results/prompt-consolidation-2026-10-18';arc=root/'lab/history/prompt-consolidation-2026-10-18';manifest=json.loads((arc/'manifest.json').read_text());issues=[];changes=[]
def sha(b):return hashlib.sha256(b).hexdigest()
for entry in manifest['files']:
 rel=entry['path'];old=(arc/'records'/rel).read_bytes();new=(root/rel).read_bytes()
 if sha(old)!=entry['sha256']:issues.append([rel,'archive digest'])
 if rel.endswith('.md'):
  for label in ['Hash do Código:']:
   if re.findall('^'+re.escape(label)+'.*$',old.decode(),re.M)!=re.findall('^'+re.escape(label)+'.*$',new.decode(),re.M):issues.append([rel,'reverse code seal changed'])
  oldpins=re.findall(r'^- (00_nucleo/prompts/_nuclei/[^ ]+) sha256:[0-9a-f]{64}$',old.decode(),re.M)
  newpins=re.findall(r'^- (00_nucleo/prompts/_nuclei/[^ ]+) sha256:([0-9a-f]{64})$',new.decode(),re.M)
  if oldpins!=[a for a,b in newpins]:issues.append([rel,'nucleus edge changed'])
  for target,digest in newpins:
   nucleus=(root/target).read_bytes()
   assert not tomllib.loads(nucleus.decode()).get('dependencies'), 'Dependency framing required'
   if sha(nucleus+b'\x00TEKT-NUCLEUS-DEPS-V1\x00')!=digest:issues.append([rel,'stale nucleus pin',target])
  for target in re.findall(r'(?<![\w/])(00_nucleo/prompts/[\w/.-]+\.md)',new.decode()):
   if not (root/target).is_file():issues.append([rel,'missing owner reference',target])
  if len(re.findall(r'^```',new.decode(),re.M))%2:issues.append([rel,'fences'])
 else:
  def strip(x):
   if isinstance(x,dict):return {k:strip(v) for k,v in x.items() if k not in ['title','statement']}
   if isinstance(x,list):return [strip(v) for v in x]
   return x
  if strip(tomllib.loads(old.decode()))!=strip(tomllib.loads(new.decode())):issues.append([rel,'nucleus identity/strength/schema/graph changed'])
 changes.append({'path':rel,'original_sha256':sha(old),'final_sha256':sha(new),'changed':old!=new})
rsfiles=subprocess.check_output(['git','diff','--name-only','HEAD','--','*.rs'],cwd=root,text=True).splitlines()
for rel in rsfiles:
 old=subprocess.check_output(['git','show',manifest['source_commit']+':'+rel],cwd=root);new=(root/rel).read_bytes()
 def unseal(b):
  lines=b.splitlines(keepends=True)
  hits=[i for i,line in enumerate(lines) if re.match(rb'//! @prompt-hash [0-9a-f]+\r?\n$',line)]
  if len(hits)!=1 or hits[0]>10:raise ValueError('unexpected forward seal placement: '+rel)
  return b''.join(line for i,line in enumerate(lines) if i!=hits[0])
 if unseal(old)!=unseal(new):issues.append([rel,'Rust changed beyond leading forward seal'])
summary={'utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'checkpoint':manifest['source_commit'],'state':'uncommitted working tree; complete exact file inputs recorded','files':len(changes),'changed_documents':sum(x['changed'] for x in changes),'rust_seals_changed':len(rsfiles),'issues':issues,'inputs':changes}
(out/'final-integrity.json').write_text(json.dumps(summary,indent=2)+'\n');print(json.dumps({k:v for k,v in summary.items() if k!='inputs'},indent=2))
raise SystemExit(bool(issues))
