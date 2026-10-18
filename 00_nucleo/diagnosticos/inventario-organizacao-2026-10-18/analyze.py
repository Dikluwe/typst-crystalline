from pathlib import Path
import gzip,json,csv,collections,hashlib,subprocess,datetime,re
p=Path(__file__).resolve().parent;root=p.parents[2]
def load(name):return json.loads(gzip.decompress((p/name).read_bytes()))
def dump(name,obj):(p/name).write_bytes(gzip.compress(json.dumps(obj,ensure_ascii=False,indent=2).encode()))
agg=collections.defaultdict(lambda:dict(files=0,logical_bytes=0,allocated_bytes=0,tracked=0,untracked=0,ignored=0))
critical=[];root_help=None
with gzip.open(p/'files.tsv.gz','rt') as inp:
 for row in csv.DictReader(inp,delimiter='\t'):
  f=row['path'];a=f.split('/');keys=[]
  if len(a)>2 and a[0]=='target':keys.append('/'.join(a[:3]))
  if len(a)>1 and a[0]=='00_nucleo' and a[1] in ['prompts','materialization','diagnosticos','adr']:keys.append('taxonomy/'+a[1])
  if '__pycache__' in a:keys.append('cache-python');keys.append('cache-python/'+a[0])
  if a[:2]==['00_nucleo','diagnosticos']:
   if 'bin' in a:keys.append('diagnostics-bin')
   if 'outputs' in a:keys.append('diagnostics-outputs')
   if 'rustdoc' in a or 'static.files' in a:keys.append('diagnostics-rustdoc')
  for key in keys:
   v=agg[key];v['files']+=1;v['logical_bytes']+=int(row['logical_bytes']);v['allocated_bytes']+=int(row['allocated_bytes']);v[row['git_state']]+=1
  if a[:3]==['00_nucleo','diagnosticos','r3-t1-materializacao-2026-10-18'] and len(a)==4 and row['git_state']=='ignored' and (a[-1].endswith('.json') or a[-1].endswith('.log')):critical.append(row)
  if f=='--help':root_help=row
state=load('git-before.json.gz');prod=load('productive-state.json.gz')
module_counts=collections.Counter(x['path'].split('/')[0] for x in prod if x['path'].endswith('.rs'))
counts=collections.Counter('nuclei' if x['path'].startswith('00_nucleo/prompts/_nuclei/') else 'L0' for x in prod if x['path'].startswith('00_nucleo/prompts/'))
mods=collections.Counter(f.split('/')[0] for f in state['modified']);new=collections.Counter(f.split('/')[0] for f in state['untracked'] if not f.startswith(str(p.relative_to(root))+'/'))
untracked_prod=[x for x in prod if x['git_state']=='untracked']
dump('analysis.json.gz',dict(utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),categories=dict(agg),productive_rust_counts=dict(module_counts),prompt_counts=dict(counts),modified_by_root=dict(mods),untracked_before_inventory_by_root=dict(new),untracked_productive=untracked_prod,ignored_closure_receipts=critical,root_help=root_help))
docs=load('documents.json.gz')
with (p/'indice-documentos.csv').open('w') as out:
 w=csv.writer(out);w.writerow(['path','titulo','git','classe','sha256'])
 for row in docs:
  a=row['path'].split('/');generated=any(x in a for x in ['outputs','bin','rustdoc','static.files'])
  w.writerow([row['path'],row['title'],row['git_state'],'gerado-ou-fixture' if generated else 'diagnostico-historico' if a[1]=='diagnosticos' else 'processo-materializacao-historico',row['sha256']])
with (p/'espaco-diretorios.csv').open('w') as out:
 w=csv.writer(out);w.writerow(['path','files','logical_bytes','allocated_bytes','tracked','untracked','ignored'])
 for f,v in load('filesystem.json.gz')['directories'].items():w.writerow([f]+[v.get(k,0) for k in ['files','logical_bytes','allocated_bytes','tracked','untracked','ignored']])
print(json.dumps(dict(categories=dict(agg),productive_rust_counts=dict(module_counts),prompt_counts=dict(counts),modified_by_root=dict(mods),untracked_before_inventory_by_root=dict(new),untracked_productive_count=len(untracked_prod),ignored_closure_receipt_count=len(critical)),ensure_ascii=False))
