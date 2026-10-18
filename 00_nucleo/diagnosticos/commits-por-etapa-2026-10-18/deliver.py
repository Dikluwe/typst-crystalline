from pathlib import Path
import subprocess,json,gzip,hashlib,datetime,re
root=Path.cwd();out=Path(__file__).resolve().parent
load=lambda n:json.loads(gzip.decompress((out/n).read_bytes()))
def sha(f):return hashlib.sha256(f.read_bytes()).hexdigest()
plan=load('commit-plan.json.gz');h=load('commits.json.gz');audit=load('audit.json.gz')
assert len(h['commits'])==15 and audit['strict_lineage_checkpoints']==12
assert subprocess.check_output(['git','rev-parse','HEAD'],text=True).strip()==h['commits'][-1]['commit']
assert all(sha(root/f)==d for f,d in plan['final_hashes'].items())
assert subprocess.run(['git','diff','--cached','--quiet']).returncode==0
pending=[f for f in subprocess.check_output(['git','ls-files','--others','--exclude-standard','-z'],text=True).split('\0') if f]
assert all(f.startswith(str(out.relative_to(root))+'/') for f in pending)
for doc in [out/'relatorio.md',root/'README.md']:
 for target in re.findall(r'\[[^\]]*\]\(([^)]+)\)',doc.read_text()):
  if target.startswith(('http:','https:','#','mailto:')) or re.match(r'^[A-Za-z]+:',target):continue
  # README also contains inherited reference-style/external constructs; check
  # only the local entries added by this organization.
  if doc.name=='README.md' and not target.startswith('00_nucleo/diagnosticos/'):continue
  assert (doc.parent/target.split('#',1)[0]).exists(),(str(doc),target)
manifest=dict(utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),base_head=h['base'],parent_commit=h['commits'][-1]['commit'],scope='Organization scripts, exact checkpoint plan, signed commit receipts, evidence package indexes and audit; final commit identity is Git itself.',files={f.name:sha(f) for f in sorted(out.iterdir()) if f.is_file() and f.name!='manifest.json.gz' and (f.suffix in ['.py','.md'] or f.name.endswith('.json.gz') or f.name=='.gitignore')},root_readme_sha256=sha(root/'README.md'))
(out/'manifest.json.gz').write_bytes(gzip.compress(json.dumps(manifest,ensure_ascii=False,indent=2).encode()))
paths=[f for f in subprocess.check_output(['git','ls-files','--others','--exclude-standard','-z'],text=True).split('\0') if f]
subprocess.run(['git','add','--','README.md',*paths],check=True)
subprocess.run(['git','diff','--cached','--check'],check=True)
staged=subprocess.check_output(['git','diff','--cached','--name-only'],text=True).splitlines()
assert all(f=='README.md' or f.startswith(str(out.relative_to(root))+'/') for f in staged)
message=out/'scratch/delivery-message.txt'
message.write_text('Registra commits por etapa e evidências da entrega\n\nIndexa os 15 commits anteriores e a auditoria de árvores/assinaturas, sem modificar os fontes produtivos. Inclui o plano exato de reconstrução, recibos de linhagem dos 12 checkpoints, índices dos 17 pacotes com 4529 arquivos de evidência e ferramenta de recuperação.\n\nValidação: conteúdo final Rust/L0 idêntico ao working tree original; lineage/reverse repetidos por etapa; assinaturas SSH verificadas; fontes e hashes dos pacotes conferidos. A suíte funcional continua a do freeze R3-T1 documentado (9111 aprovados, sete ignorados); não houve rebuild. Limite de 12 GiB, sem push ou release.\n')
subprocess.run(['git','-c','gc.auto=0','commit','--file',str(message)],check=True)
commit=subprocess.check_output(['git','rev-parse','HEAD'],text=True).strip()
subprocess.run(['git','-c','gpg.ssh.allowedSignersFile='+str(out/'scratch/allowed-signers'),'verify-commit',commit],check=True)
assert subprocess.check_output(['git','status','--porcelain=v1','--untracked-files=all'],text=True).strip()==''
assert all(sha(root/f)==d for f,d in plan['final_hashes'].items())
assert int(subprocess.check_output(['git','rev-list','--count',h['base']+'..HEAD'],text=True))==16
print(json.dumps(dict(final_commit=commit,commit_count=16,worktree='clean',signatures='verified',production='unchanged',push=False),ensure_ascii=False),flush=True)
