from pathlib import Path
import subprocess,json,gzip,hashlib,shutil,datetime,os,re
root=Path.cwd();out=Path(__file__).resolve().parent;scratch=out/'scratch';scratch.mkdir(exist_ok=True)
def git(*args,**kwargs):return subprocess.check_output(['git',*args],**kwargs)
def sha(path):return hashlib.sha256(path.read_bytes()).hexdigest()
plan=json.loads(gzip.decompress((out/'commit-plan.json.gz').read_bytes()))
checks=json.loads(gzip.decompress((out/'checkpoint-validation.json.gz').read_bytes()));assert len(checks)==12
previous=json.loads(gzip.decompress((out/'commits.json.gz').read_bytes())) if (out/'commits.json.gz').exists() else dict(commits=[])
assert git('rev-parse','HEAD',text=True).strip()==(previous['commits'][-1]['commit'] if previous['commits'] else plan['base_head'])
if not previous['commits']:assert subprocess.run(['git','diff','--cached','--quiet']).returncode==0
assert all(sha(root/f)==h for f,h in plan['final_hashes'].items())
index_path=Path(git('rev-parse','--git-path','index',text=True).strip());
if not (scratch/'index-before').exists():shutil.copy2(index_path,scratch/'index-before')
# Verify signatures against the configured public signing key without changing
# the user's global/repository configuration or private key material.
keypath=Path(git('config','--get','user.signingkey',text=True).strip())
public=keypath.read_text().split();assert public[0].startswith(('ssh-','ecdsa-'))
email=git('config','--get','user.email',text=True).strip();allowed=scratch/'allowed-signers'
allowed.write_text(email+' '+public[0]+' '+public[1]+'\n')
other=git('ls-files','--others','--exclude-standard','-z',text=True).split('\0');other=[f for f in other if f]+git('diff','--cached','--name-only',text=True).splitlines()
productive=lambda f:f.startswith('00_nucleo/prompts/') or f.endswith('.rs') and f.startswith(('01_core/','02_shell/','03_infra/','04_wiring/'))
mapping={s['diagnostic']:s['key'] for s in plan['stages']}
mapping.update({'paridade-novo-vanilla-2026-10-17':'parity-audit','textual-realization-2026-10-18':'r3-textual','show-returns-2026-10-18':'r3-textual','inventario-organizacao-2026-10-18':'inventory','limpeza-caches-2026-10-18':'cleanup','commits-por-etapa-2026-10-18':'delivery'})
materials={'state-update-init-2026-10-17':'a1','state-reader-runtime-2026-10-17':'a1','heading-supplement-runtime-2026-10-17':'a1','sequence-realization-2026-10-17':'r2','hanging-indent-2026-10-18':'hanging','textual-realization-2026-10-18':'r3-textual'}
documents={}
for f in other:
 if productive(f):continue
 parts=f.split('/')
 if f.endswith('/inventario-organizacao-2026-10-18/limpeza-executada.md'):key='cleanup'
 elif parts[:2]==['00_nucleo','diagnosticos']:key=mapping[parts[2]]
 elif parts[:2]==['00_nucleo','materialization']:key=materials[parts[2]]
 else:raise AssertionError(('unassigned path',f))
 documents.setdefault(key,[]).append(f)
sequence=[dict(key='parity-audit',title='Registra inventário de paridade contra o novo vanilla',source_changes={},diagnostic='paridade-novo-vanilla-2026-10-17')]+plan['stages']+[dict(key='inventory',title='Consolida inventário, entregas e pendências do projeto',source_changes={},diagnostic='inventario-organizacao-2026-10-18'),dict(key='cleanup',title='Registra limpeza de caches e retenção verificável de artefatos',source_changes={},diagnostic='limpeza-caches-2026-10-18')]
receipts=previous['commits'];parent=receipts[-1]['commit'] if receipts else plan['base_head']
for stage in sequence[len(receipts) :]:
 assert git('rev-parse','HEAD',text=True).strip()==parent
 assert all(sha(root/f)==h for f,h in plan['final_hashes'].items())
 entries=[]
 for path,text in stage['source_changes'].items():
  if text is None:entries.append('0 '+'0'*40+'\t'+path+'\0');continue
  blob=git('hash-object','-w','--stdin',input=text.encode()).decode().strip()
  mode=plan['modes'].get(path,'100755' if (root/path).stat().st_mode&0o111 else '100644')
  entries.append(mode+' '+blob+'\t'+path+'\0')
 if entries:subprocess.run(['git','update-index','-z','--index-info'],input=''.join(entries).encode(),check=True)
 paths=documents.get(stage['key'],[])
 for start in range(0,len(paths),120):subprocess.run(['git','add','--',*paths[start:start+120]],check=True)
 subprocess.run(['git','diff','--cached','--check','--','00_nucleo/prompts','01_core','02_shell','03_infra','04_wiring','README.md'],check=True)
 zwhite=subprocess.run(['git','diff','--cached','--check'],text=True,capture_output=True)
 archive_whitespace=[]
 if zwhite.returncode:
  for line in zwhite.stdout.splitlines():
   if line.startswith('+'):
    assert archive_whitespace and archive_whitespace[-1]['diagnostic'].endswith('trailing whitespace.')
    continue
   match=re.match(r'^(00_nucleo/(?:diagnosticos|materialization)/.+):(\d+): (new blank line at EOF|trailing whitespace)\.$',line)
   assert match,('unexpected whitespace diagnostic',line)
   f=match.group(1);folder=root/Path(f).parts[0]/Path(f).parts[1]/Path(f).parts[2]
   digest=sha(root/f);pinned=False;provenance=None
   if (folder/'manifest.json').exists():
    manifest=json.loads((folder/'manifest.json').read_text());rows=manifest.get('files',[])
    if isinstance(rows,list):pinned=any(row.get('path')==f and row.get('sha256')==digest for row in rows)
    elif isinstance(rows,dict):pinned=rows.get(f)==digest or rows.get(str(Path(f).relative_to(folder.relative_to(root))))==digest
    if pinned:
     assert sha(folder/'manifest.json') in (folder/'manifest.sha256').read_text()
     provenance=str((folder/'manifest.json').relative_to(root))
   if not pinned and (folder/'manifest.sha256').exists():
    local=str(Path(f).relative_to(folder.relative_to(root)))
    pinned=any(row.split() and row.split()[0]==digest and row.split()[-1] in [f,local] for row in (folder/'manifest.sha256').read_text().splitlines())
    if pinned:provenance=str((folder/'manifest.sha256').relative_to(root))
   assert pinned,('unsealed archival whitespace requires review',f)
   archive_whitespace.append(dict(diagnostic=line,path=f,sha256=digest,sealed_by=provenance,reason='Exact historical probe/spec bytes verified against the existing manifest; productive source whitespace check remains strict.'))
 staged=git('diff','--cached','--name-only',text=True).splitlines();assert staged
 body=stage['title']+'\n\nEtapa: '+stage['key']+'. Relatório e evidências: 00_nucleo/diagnosticos/'+stage['diagnostic']+'/\n\n'
 if stage['source_changes']:body+='Estado de código reconstruído dos snapshots e patches históricos, com hashes conferidos. Linhagem V1/V5/V15/V26 e selos reversos revalidados em commit-lineage.json.gz. A suíte funcional permanece a do freeze documentado; não houve novo build nesta organização.\n\n'
 body+='Inclui arquivos novos do lote, fontes das sondas e evidências textuais compactadas. Binários e saídas grandes preservados localmente com proveniência; sem push ou release.\n'
 message=scratch/'message.txt';message.write_text(body)
 with (out/(stage['key']+'-commit.log')).open('w') as log:subprocess.run(['git','commit','--file',str(message)],stdout=log,stderr=subprocess.STDOUT,check=True)
 commit=git('rev-parse','HEAD',text=True).strip();tree=git('rev-parse','HEAD^{tree}',text=True).strip()
 z=subprocess.run(['git','-c','gpg.ssh.allowedSignersFile='+str(allowed),'verify-commit',commit],text=True,capture_output=True);assert z.returncode==0,(commit,z.stderr)
 receipt=dict(key=stage['key'],title=stage['title'],commit=commit,parent=parent,tree=tree,files=staged,signature_verification=z.stdout+z.stderr,archive_whitespace=archive_whitespace,utc=datetime.datetime.now(datetime.timezone.utc).isoformat())
 receipts.append(receipt);parent=commit
 (out/'commits.json.gz').write_bytes(gzip.compress(json.dumps(dict(base=plan['base_head'],commits=receipts,delivery_commit_pending=True),ensure_ascii=False,indent=2).encode()))
 print(stage['key'],commit[:12],len(staged),'files; signed',flush=True)
# All productive content in HEAD must exactly equal the original final working tree.
for f,h in plan['final_hashes'].items():assert hashlib.sha256(git('show','HEAD:'+f)).hexdigest()==h,f
assert git('diff','--name-only',text=True).splitlines()==['README.md']
remaining=[f for f in git('ls-files','--others','--exclude-standard','-z',text=True).split('\0') if f]
assert all(f.startswith(str(out.relative_to(root))+'/') for f in remaining),remaining
print('Prepared delivery commit; only README link changes and orchestration evidence remain',flush=True)
