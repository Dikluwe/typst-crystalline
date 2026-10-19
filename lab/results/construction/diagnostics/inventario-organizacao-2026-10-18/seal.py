from pathlib import Path
import gzip,json,hashlib,datetime,subprocess,re,os
p=Path(__file__).resolve().parent;root=p.parents[2]
def sha(path):return hashlib.sha256(path.read_bytes()).hexdigest()
def load(name):return json.loads(gzip.decompress((p/name).read_bytes()))
prod=load('productive-state.json.gz');docs=load('documents.json.gz');base=load('git-before.json.gz')
assert all(sha(root/x['path'])==x['sha256'] for x in prod)
assert all(hashlib.sha256((root/x['path']).read_text(errors='replace').encode()).hexdigest()==x['sha256'] for x in docs)
assert all(os.path.lexists(root/f) for f in base['tracked']+base['untracked'])
for file in p.glob('*.md'):
 for link in re.findall(r'\[[^\]]*\]\(([^)]+)\)',file.read_text()):
  if link.startswith(('http://','https://','#')):continue
  assert (file.parent/link.split('#',1)[0]).exists(),(file,link)
subprocess.run(['git','diff','--check'],check=True)
v=load('verificacao.json.gz')
v.update(final_utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),historical_documents_checked=len(docs),historical_document_changes=[],resources={f:json.loads((p/f).read_text()) for f in ['coleta-resources.json','analise-resources.json','verificacao-resources.json']},final_inventory_document_hashes={f.name:sha(f) for f in p.glob('*.md')})
(p/'verificacao.json.gz').write_bytes(gzip.compress(json.dumps(v,ensure_ascii=False,indent=2).encode()))
print(json.dumps(dict(productive_unchanged=len(prod),historical_documents_unchanged=len(docs),no_missing_git_paths=True,links='OK',diff_check='OK')))
