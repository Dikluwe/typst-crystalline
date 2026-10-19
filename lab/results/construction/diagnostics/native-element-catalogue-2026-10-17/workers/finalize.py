from pathlib import Path
import gzip,json,hashlib,datetime,collections,difflib,subprocess
out=Path(__file__).resolve().parents[1];root=Path.cwd();sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest();now=lambda:datetime.datetime.now(datetime.timezone.utc).isoformat();b=json.load(gzip.open(out/'before.json.gz'));v=json.load(open(out/'validation-result.json'));assert v['complete'] and v['sources_unchanged'];hashes={p:sha(p) for p in b['hashes']};assert hashes==v['source_hashes'];scope=json.load(open(out/'scope.json'));changed=[p for p,h in b['hashes'].items() if hashes[p]!=h];assert set(changed)==set(scope['sources'])|set(scope['owners'].values());workspace=json.load(open(out/'workspace/workspace-result.json'));assert workspace['exit']==0 and workspace['failed']==0 and workspace['passed']==8852 and workspace['ignored']==7
obs=json.load(open(out/'observables-result.json'));arch=json.load(open(out/'architecture-delta.json'));assert arch['before']==arch['after'] and arch['no_new_errors_or_warnings'];schema=json.load(open(out/'batch-schema.json'));audit=json.load(open(out/'audit-result.json'))
resources=[]
for p in sorted(out.glob('*-resources.json')):
 r=json.load(open(p));assert r['memory_before']['memory.max']==str(12*1024**3) and r['memory_before']['memory.swap.max']=='0';assert r['memory_before']['memory.high']==str(10*1024**3);resources.append(dict(file=p.name,record=r))
peak=max(int(r['record']['memory_after']['memory.peak']) for r in resources);events=collections.Counter()
for row in resources:
 for line in row['record']['memory_after']['memory.events'].splitlines():
  k,n=line.split();events[k]+=int(n)
assert all(events[k]==0 for k in ['max','oom','oom_kill']) and peak<=12*1024**3
packages={}
for name,expected in [('text-size-length-2026-10-17','771f5fd3d36545eb4b4fb7f0c0f2c91061470d1f010fc9a887b001a636d8feba'),('table-row-groups-2026-10-17','a47fe97272fd6a5bd96022be959ba277a317a74086f5e5f4a569bba008f4fae9')]:
 folder=Path('00_nucleo/materialization')/name;manifest=folder/'SHA256SUMS';assert sha(manifest)==expected
 for line in manifest.read_text().splitlines():
  h,p=line.split(maxsplit=1);p=p.lstrip('*');candidate=folder/p
  if not candidate.exists():candidate=Path(p)
  assert candidate.exists() and sha(candidate)==h,(name,p)
 packages[name]=expected
state=dict(utc=now(),head=b['head'],working_tree='não commitado',status=subprocess.check_output(['git','status','--porcelain=v1'],text=True),diffstat=subprocess.check_output(['git','diff','HEAD','--stat'],text=True),source_hashes=hashes,changed_since_previous=changed,batch_schema=schema,audit=audit,scope=scope,tests=workspace,observables=obs,architecture=arch,peak_bytes=peak,memory_events=dict(events),resources=resources,public_api_changes=False,global_goal='active; this batch only is complete',proposal_manifests_preserved=packages)
with gzip.open(out/'final-state.json.gz','wt') as f:json.dump(state,f)
for p in scope['sources']:
 (out/'scoped.patch').write_text(''.join(difflib.unified_diff(b['files'][p].splitlines(True),Path(p).read_text().splitlines(True),fromfile='before/'+p,tofile='after/'+p)))
rows='\n'.join('| `'+r['expression']+'` | '+(', '.join('`'+f+'`' for f in r['fields']) or 'nenhum')+' | '+str(r['locatable']).lower()+' |' for r in schema['cases'])
report=f'''# Relatório — catálogo de elementos nativos, lote 1

Etapa concluída: quinze identidades nativas existentes agora têm catálogo de campos e capacidade de locatabilidade reconhecidos por callback tipado. A correção alcança `.where()` e a admissão de `figure(kind: ...)`, incluindo aliases. TS1/equality e os demais consumers foram preservados. O objetivo global continua ativo.

## Proveniência

HEAD `{b['head']}`, working tree não commitado. Coleta anterior: `{b['utc']}`; fechamento: `{state['utc']}`. `before.json.gz` e `final-state.json.gz` registram estado completo, diff HEAD --stat e hashes de {len(hashes)} arquivos protegidos. Apenas o consumer `element_where.rs` e seu Prompt L0 proprietário mudaram neste lote. `scoped.patch` preserva a diferença sobre a árvore anterior, sem confundir alterações preexistentes do projeto com este trabalho.

Oráculo vanilla pinado: `9f2b6e8715237cb086899a42873660fe744622e8`, executável SHA-256 `7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea`. Binários Tekt antes/depois, UTC, comandos completos e todas as saídas estão em `before-measurement.json.gz`/`after-measurement.json.gz`; fontes primárias e blocos literais com hashes/linhas estão em `batch-schema.json`.

## Medição e correção

A auditoria percorreu o registry efetivo em oito perfis de features e mediu 3956 casos de funções. Encontrou 57 caminhos com omissão de identidade em where e kind, correspondendo a 420 recusas por operação ao somar os perfis. São caminhos/aliases, não 57 implementações distintas. A enumeração não certifica todos os fields associados a Type, nem funções de usuário ou suporte de layout.

O lote corrige quinze callbacks, reduzindo as rotas omitidas de 57 para 41: dezesseis rotas do pin mais o alias global Tekt `op` acompanham suas funções efetivas. O reconhecimento usa fn_addr_eq com ABI própria; math.op permanece NativeWithEngine, com seis argumentos. Nome de Func, PartialEq e execução do constructor não são usados como prova de identidade. Page e funções homônimas permaneceram como controles. Não houve mudança de API pública Rust, fase, default de produto, commit, push ou release.

| Binding principal | Campos selecionáveis | Locatable |
|---|---|---|
{rows}

O L0 foi atualizado antes dos testes/código, por fluxo contínuo ADR-0127 de tabela/paridade. Não foram criados novos bindings. Campos desconhecidos seguem rejeitados com nome público; label continua permitido. Closures/With e callbacks homônimos não ganham identidade.

## Verificação

- RED funcional: quinze testes falharam por metadados ausentes e um controle passou. Uma primeira tentativa teve erro de construção de Label na fixture; foi corrigida e arquivada como `red-harness-*`, sem ser contada como RED funcional.
- GREEN focado: dezesseis testes passaram. Suíte workspace completa: {workspace['passed']} passaram, {workspace['failed']} falharam, {workspace['ignored']} ignorados, com fontes congeladas.
- Matriz antes/depois: {obs['changed_observables']} observáveis Tekt mudaram exclusivamente no conjunto autorizado; {obs['pin_matching_language_values']} reparos têm valores de linguagem iguais ao pin. Os demais {obs['preserved_observables']} observáveis, incluindo todas as saídas do oráculo, permaneceram idênticos.
- Lote detalhado: 110 casos bilaterais. {obs['batch_pin_matching_language_values']} resultados de linguagem coincidem com o pin; quinze controles de counter permanecem divergentes e quinze erros de campo desconhecido têm mensagem correta com âncora ainda divergente. Não são declarados fechados.
- O arquivo de entrada da sonda é vazio. O Tekt já emitia o aviso exato `ficheiro vazio: sem conteúdo` em {obs['existing_empty_fixture_warning_controls']} controles bem-sucedidos anteriores. Os novos sucessos mantêm esse aviso, ausente no pin. O verificador exige esses bytes exatos e registra a diferença; não declara igualdade integral de stderr nem descarta avisos arbitrários.
- Build, fmt, Clippy workspace/all-targets com warnings/unfulfilled_lint_expectations negados, Rustdoc com warnings negados e quatro índices HTML, V1/V5/V15/V26 e linhagem reversa passaram. V15/V26 foram validados antes do resselo restrito ao par proprietário.
- O gate global continua bloqueado pelos 117 V16 anteriores; V19=1076 notes, V20=873 notes. Todos os diagnósticos foram preservados após mapear deslocamentos de linhas; não houve novo erro/aviso. Esta etapa de paridade não é uma redução de V16 nem uma aprovação do gate global.

## RAM

Todos os comandos pesados foram sequenciais em cgroup com MemoryMax=12 GiB, MemoryHigh=10 GiB, MemorySwapMax=0 e Cargo jobs1. Maior memory.peak: {peak} bytes ({peak/1024**3:.3f} GiB). Eventos agregados: `{dict(events)}`. Sem atingir MemoryMax/OOM/oom_kill. O pico inclui memória de processos e cache contabilizados pelo cgroup; não é RSS isolado.

## O que continua aberto

Há 41 rotas restantes na matriz: 26 de matemática e 15 de formatos/HTML/PDF, listadas em [follow-ups.md]({out}/follow-ups.md). Formatos raiz compartilham callback e precisam de identidade própria antes de corrigir metadados. Counter e bare selector ainda têm admissão separada; capacidades declaradas aqui não comprovam índice, síntese ou render. As entradas históricas da tabela também não receberam certificação universal neste lote.

FG1, RM1, SY1 e a proposta RG1 conservam suas fronteiras. Os manifestos originais TS1 aprovado e RG1 pendente permanecem íntegros. Nenhuma nova aprovação foi inferida. Evidências, escopo e testes estão preservados no diretório deste relatório e em `SHA256SUMS`; o estado exato de fechamento está em `final-state.json.gz`.
'''
(out/'relatorio.md').write_text(report)
exclude={'SHA256SUMS','manifest.json','delivery-check.json','finalize.log','finalize-resources.json'};files=[p for p in sorted(out.rglob('*')) if p.is_file() and p.name not in exclude];manifest=''.join(sha(p)+'  '+str(p.relative_to(out))+'\n' for p in files);(out/'SHA256SUMS').write_text(manifest);m=dict(utc=now(),files=len(files),manifest_sha256=sha(out/'SHA256SUMS'),source_files=len(hashes),source_changes=changed,peak_bytes=peak);(out/'manifest.json').write_text(json.dumps(m,indent=2)+'\n');print(json.dumps(m),flush=True)
