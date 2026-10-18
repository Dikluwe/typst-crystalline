from pathlib import Path
import json,gzip,subprocess,tarfile,hashlib,re,datetime
root=Path.cwd();out=Path(__file__).resolve().parent;base=root/'00_nucleo/diagnosticos'
def load(folder,name):return json.loads(gzip.decompress((base/folder/name).read_bytes()))
def productive(p):return p.startswith('00_nucleo/prompts/') or p.endswith('.rs') and p.startswith(('01_core/','02_shell/','03_infra/','04_wiring/'))
def texts(folder,name):
 d=load(folder,name);data=d.get('files',d)
 return {f:(s['text'] if isinstance(s,dict) else s) for f,s in data.items() if productive(f)}
def hashtext(s):return hashlib.sha256(s.encode()).hexdigest()
def reverse_patch(state,folder):
 raw=gzip.decompress((base/folder/'change.patch.gz').read_bytes()).decode()
 lines=raw.splitlines(keepends=True);i=0;result=dict(state)
 while i<len(lines):
  if not lines[i].startswith('--- '):i+=1;continue
  old=lines[i][4:].strip().split('\t')[0];i+=1;assert lines[i].startswith('+++ ')
  new=lines[i][4:].strip().split('\t')[0];i+=1
  old=old.split('/',1)[1] if old!='/dev/null' else old
  new=new.split('/',1)[1] if new!='/dev/null' else new
  path=new if new!='/dev/null' else old
  hunks=[]
  while i<len(lines) and not lines[i].startswith('--- '):
   if not lines[i].startswith('@@ '):i+=1;continue
   match=re.match(r'@@ -(\d+)(?:,(\d+))? \+(\d+)(?:,(\d+))? @@',lines[i]);assert match
   os_,oc,ns,nc=[int(x) if x is not None else 1 for x in match.groups()];i+=1;records=[]
   while i<len(lines) and not lines[i].startswith(('@@ ','--- ')):
    line=lines[i]
    if line.startswith('\\ No newline'):
     assert records;sign,value=records[-1];records[-1]=(sign,value.removesuffix('\n'))
    elif line[:1] in [' ','+','-']:records.append((line[0],line[1:]))
    else:break
    i+=1
   oldlines=[s for k,s in records if k in [' ','-']];newlines=[s for k,s in records if k in [' ','+']]
   assert len(oldlines)==oc and len(newlines)==nc,(path,oc,nc,len(oldlines),len(newlines))
   hunks.append((ns,nc,oldlines,newlines))
  if not productive(path):continue
  current=result.get(path,'').splitlines(keepends=True)
  for ns,nc,oldlines,newlines in reversed(hunks):
   at=ns-1 if nc else ns
   assert current[at:at+nc]==newlines,('patch mismatch',folder,path,ns)
   current[at:at+nc]=oldlines
  text=''.join(current)
  if not text:result.pop(path,None)
  else:result[path]=text
 return result
states={}
states['r3-textual']=texts('r3-t1-materializacao-2026-10-18','candidate10-sources.json.gz')
states['mixed-metrics']=texts('r3-t1-materializacao-2026-10-18','before-sources.json.gz')
states['hanging']=texts('mixed-style-metrics-2026-10-18','before-sources.json.gz')
states['margins']=texts('bibliography-hanging-2026-10-18','before-sources.json.gz')
states['structure']=texts('bibliography-margins-2026-10-18','before-sources.json.gz')
states['indent']=texts('bibliography-structure-2026-10-18','before-sources.json.gz')
states['csl']=texts('bibliography-indent-2026-10-18','before-sources.json.gz')
states['r3-markup']=texts('bibliography-a2-2026-10-18','before-sources.json.gz')
states['r2']=texts('sequence-r3-2026-10-18','before-sources.json.gz')
states['r1']=reverse_patch(states['r2'],'sequence-constructor-2026-10-17')
states['stack']=reverse_patch(states['r1'],'sequence-reflection-2026-10-17')
states['a1']=reverse_patch(states['stack'],'state-stack-2026-10-17')
# Compare reconstructed states against sealed historical hashes, not just patches.
checks=[]
for key,folder in [('a1','lote-a1-callbacks-reflexao-seletores-2026-10-17'),('stack','state-stack-2026-10-17'),('r1','sequence-reflection-2026-10-17'),('r2','sequence-constructor-2026-10-17')]:
 d=load(folder,'complete-freeze.json.gz');expected=d.get('sources',d.get('files',{}));tested=0
 for f,h in expected.items():
  if not productive(f):continue
  if isinstance(h,dict):h=h.get('sha256')
  assert f in states[key] and hashtext(states[key][f])==h,('sealed checkpoint mismatch',key,f)
  tested+=1
 checks.append(dict(stage=key,reference=folder+'/complete-freeze.json.gz',files=tested))
# Load original source contents directly from HEAD, with no working-tree reset.
proc=subprocess.Popen(['git','archive','--format=tar','HEAD','00_nucleo/prompts','01_core','02_shell','03_infra','04_wiring'],stdout=subprocess.PIPE)
original={};modes={}
with tarfile.open(fileobj=proc.stdout,mode='r|') as tf:
 for item in tf:
  if item.isfile() and productive(item.name):original[item.name]=tf.extractfile(item).read().decode();modes[item.name]='100755' if item.mode&0o111 else '100644'
while proc.stdout.read(1024*1024):pass
assert proc.wait()==0
spec=[('a1','Corrige callbacks State, suplementos Heading, reflexão e seletores','lote-a1-callbacks-reflexao-seletores-2026-10-17'),('stack','Corrige concatenação e consumo de pilha nas travessias','state-stack-2026-10-17'),('r1','Corrige reflexão de Sequence e conteúdo vazio','sequence-reflection-2026-10-17'),('r2','Implementa construção, seleção e realização causal de Sequence','sequence-constructor-2026-10-17'),('r3-markup','Preserva filhos de markup, slots consumidos e prefixos antes de 1fr','sequence-r3-2026-10-18'),('csl','Preserva estilo CSL sob receitas e carriers bibliográficos','bibliography-a2-2026-10-18'),('indent','Corrige fronteiras de bibliografia e validação de hanging-indent','bibliography-indent-2026-10-18'),('structure','Preserva estrutura CSL, colunas e continuação bibliográfica','bibliography-structure-2026-10-18'),('margins','Corrige prioridade de margens de blocos e títulos','bibliography-margins-2026-10-18'),('hanging','Implementa recuo francês causal e origens de decorações','bibliography-hanging-2026-10-18'),('mixed-metrics','Corrige métricas de faces mistas e admissão regional','mixed-style-metrics-2026-10-18'),('r3-textual','Corrige realização textual, precedência e produtos de show','r3-t1-materializacao-2026-10-18')]
stages=[];prior=original
for key,title,folder in spec:
 target=states[key];changes={f:target.get(f) for f in sorted(set(prior)|set(target)) if prior.get(f)!=target.get(f)}
 assert changes,(key,'empty stage')
 stages.append(dict(key=key,title=title,diagnostic=folder,source_changes=changes,hashes={f:hashtext(s) for f,s in target.items()}))
 prior=target
assert all((root/f).read_text()==s for f,s in states['r3-textual'].items())
head=subprocess.check_output(['git','rev-parse','HEAD'],text=True).strip()
plan=dict(utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),base_head=head,modes=modes,stages=stages,historical_checkpoint_checks=checks,final_hashes={f:hashtext(s) for f,s in prior.items()},working_tree_resets=False)
(out/'commit-plan.json.gz').write_bytes(gzip.compress(json.dumps(plan,ensure_ascii=False).encode()))
print(json.dumps(dict(base=head,stages=[dict(key=s['key'],changed_files=len(s['source_changes'])) for s in stages],checkpoint_checks=checks),ensure_ascii=False))
