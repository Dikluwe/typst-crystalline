from pathlib import Path
import os,gzip,json,hashlib,subprocess,datetime,re
p=Path(__file__).resolve().parent;root=p.parents[2]
def load(name):return json.loads(gzip.decompress((p/name).read_bytes()))
def sha(path):
 h=hashlib.sha256()
 with path.open('rb') as f:
  for b in iter(lambda:f.read(1024*1024),b''):h.update(b)
 return h.hexdigest()
def git(*args):return subprocess.check_output(['git',*args],text=True)
base=load('git-before.json.gz');prod=load('productive-state.json.gz')
changed=[x['path'] for x in prod if sha(root/x['path'])!=x['sha256']]
assert not changed,changed
missing=[f for f in base['tracked']+base['untracked'] if not os.path.lexists(root/f)]
assert not missing,missing
now=set(git('diff','HEAD','--name-only').splitlines())
assert now==set(base['modified'])|{'README.md'},now.symmetric_difference(set(base['modified'])|{'README.md'})
original=git('show','HEAD:README.md');entry='- [Inventário consolidado, entregas, pendências e limpeza proposta — 18/10/2026](00_nucleo/diagnosticos/inventario-organizacao-2026-10-18/README.md)\n'
assert (root/'README.md').read_text().replace(entry,'',1)==original
files={}
# Preserve exact documents behind this inventory and compact copies of final
# receipts otherwise ignored by Git. This is not a backup of every output.
folders=['lote-a1-callbacks-reflexao-seletores-2026-10-17','state-stack-2026-10-17','sequence-reflection-2026-10-17','sequence-constructor-2026-10-17','sequence-r3-2026-10-18','bibliography-a2-2026-10-18','bibliography-structure-2026-10-18','bibliography-margins-2026-10-18','bibliography-hanging-2026-10-18','bibliography-indent-2026-10-18','mixed-style-metrics-2026-10-18','r3-t1-materializacao-2026-10-18','show-returns-2026-10-18','clippy-aridade-final-2026-10-16','clippy-locais-2026-10-16','clippy-enums-publicos-2026-10-16','tekt-v12-2026-10-16','text-size-length-2026-10-17','text-styled-equality-2026-10-17','vanilla-resync-2026-10-17','tekt-v16-none-parser-2026-10-16']
paths=[]
for folder in folders:
 for name in ['relatorio.md','proximos.md','escopo-geral.md']:
  f=root/'00_nucleo/diagnosticos'/folder/name
  if f.exists():paths.append(f)
for f in ['README.md','CLAUDE.md','Cargo.toml','.gitignore','00_nucleo/diagnosticos/.gitignore','tools/ci/README.md','.github/workflows/ci.yml','.github/workflows/release.yml','00_nucleo/diagnosticos/paridade-novo-vanilla-2026-10-17/inventario.md','00_nucleo/materialization/table-row-groups-2026-10-17/review.md','00_nucleo/materialization/textual-realization-2026-10-18/approval-receipt.json','00_nucleo/diagnosticos/auditoria-producao-tekt-2026-10-13.md','00_nucleo/diagnosticos/correcoes-producao-2026-10-13.md','00_nucleo/diagnosticos/linhagem-ci-release-2026-10-13.md']:paths.append(root/f)
r3=root/'00_nucleo/diagnosticos/r3-t1-materializacao-2026-10-18'
for name in ['closure-candidate10-resources.json','suite-summary-candidate10.json','audit-candidate10.json','manifest-encerramento.json','architecture-candidate10.log','clippy-candidate10.log','build-candidate10.log','rustdoc-candidate10.log','reverse-candidate10.log']:paths.append(r3/name)
paths += [p/'indice-documentos.csv',p/'espaco-diretorios.csv',p/'coleta-resources.json',p/'analise-resources.json']
for path in paths:
 files[str(path.relative_to(root))]=dict(sha256=sha(path),text=path.read_text())
(p/'documentos-consultados.json.gz').write_bytes(gzip.compress(json.dumps(dict(utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),head=base['head'],files=files,scope='Documentos, índices e recibos consultados; não é backup integral dos outputs'),ensure_ascii=False).encode()))
links=[]
for f in [p/'README.md',p/'estado-e-pendencias.md',p/'limpeza-e-arquivamento.md']:
 for target in re.findall(r'\[[^\]]*\]\(([^)]+)\)',f.read_text()):
  if target.startswith(('http://','https://','#')):continue
  target=target.split('#',1)[0]
  dest=(f.parent/target).resolve()
  links.append(dict(document=f.name,target=target,exists=dest.exists()))
assert all(x['exists'] for x in links),[x for x in links if not x['exists']]
subprocess.run(['git','diff','--check'],check=True)
verify=dict(utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),head=base['head'],branch=base['branch'],productive_files_checked=len(prod),productive_changes=changed,missing_git_paths=missing,changed_tracked_since_inventory=['README.md'],root_readme_change='one additive index link',local_links=links,status=git('status','--short'),diffstat=git('diff','HEAD','--stat'),no_deletion_or_move=True,no_build_or_test_rerun=True,validation_reused_from='r3-t1-materializacao-2026-10-18/candidate10-sources.json.gz')
(p/'verificacao.json.gz').write_bytes(gzip.compress(json.dumps(verify,ensure_ascii=False,indent=2).encode()))
print(json.dumps(dict(productive_files_checked=len(prod),productive_changes=changed,missing_git_paths=missing,local_links_checked=len(links),result='OK'),ensure_ascii=False))
