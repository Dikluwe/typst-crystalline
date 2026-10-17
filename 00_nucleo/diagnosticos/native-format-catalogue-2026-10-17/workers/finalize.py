from pathlib import Path
import json,gzip,hashlib,datetime,subprocess,collections,difflib
out=Path(__file__).resolve().parents[1];sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest();now=lambda:datetime.datetime.now(datetime.timezone.utc).isoformat();b=json.load(gzip.open(out/'before.json.gz'));scope=json.load(open(out/'scope.json'));v=json.load(open(out/'validation-result.json'));assert v['complete'];hashes={p:sha(p) for p in list(b['hashes'])+scope['nuclei']};assert hashes==v['source_hashes'];changed=[p for p,h in b['hashes'].items() if h!=hashes[p]];assert set(changed)==set(scope['sources'])|set(scope['owners'].values());workspace=json.load(open(out/'workspace/workspace-result.json'));assert workspace['passed']==8900 and workspace['failed']==0 and workspace['ignored']==7 and workspace['exit']==0;obs=json.load(open(out/'observables-result.json'));arch=json.load(open(out/'architecture-delta.json'));integrity=json.load(open(out/'integrity-result.json'));assert not obs['missing_paths_after'];assert arch['no_new_errors_or_warnings'];schema=json.load(open(out/'batch-schema.json'));assert json.load(open(out/'public-contract-before.json'))==json.load(open(out/'public-contract-after.json'));resources=[];aliases=[];seen={}
for p in sorted(out.rglob('*-resources.json')):
 r=json.load(open(p));identity=json.dumps(r,sort_keys=True);relative=str(p.relative_to(out))
 if identity in seen:aliases.append(dict(file=relative,same_execution_as=seen[identity]));continue
 seen[identity]=relative;resources.append(dict(file=relative,record=r))
peak=max(int(x['record']['memory_after']['memory.peak']) for x in resources);events=collections.Counter()
for row in resources:
 r=row['record'];assert r['memory_before']['memory.max']==str(12*1024**3) and r['memory_before']['memory.high']==str(10*1024**3) and r['memory_before']['memory.swap.max']=='0'
 for line in r['memory_after']['memory.events'].splitlines():k,n=line.split();events[k]+=int(n)
assert events['max']==events['oom']==events['oom_kill']==0
packages={}
for name,expected in [('text-size-length-2026-10-17','771f5fd3d36545eb4b4fb7f0c0f2c91061470d1f010fc9a887b001a636d8feba'),('table-row-groups-2026-10-17','a47fe97272fd6a5bd96022be959ba277a317a74086f5e5f4a569bba008f4fae9')]:
 folder=Path('00_nucleo/materialization')/name;assert sha(folder/'SHA256SUMS')==expected
 for line in (folder/'SHA256SUMS').read_text().splitlines():
  digest,path=line.split(maxsplit=1);candidate=folder/path
  if not candidate.exists():candidate=Path(path)
  assert sha(candidate)==digest
 packages[name]=expected
patch=''.join(''.join(difflib.unified_diff(b['files'][p].splitlines(True),Path(p).read_text().splitlines(True),fromfile='before/'+p,tofile='after/'+p)) for p in scope['sources']);(out/'scoped.patch').write_text(patch)
state=dict(utc=now(),head=b['head'],working_tree='não commitado',status=subprocess.check_output(['git','status','--porcelain=v1'],text=True),diffstat=subprocess.check_output(['git','diff','HEAD','--stat'],text=True),source_hashes=hashes,changed_since_previous=changed,new_nuclei=scope['nuclei'],scope=scope,batch_schema=schema,tests=workspace,observables=obs,architecture=arch,integrity=integrity,public_contract_unchanged=True,peak_bytes=peak,memory_events=dict(events),resources=resources,resource_aliases=aliases,public_api_changes=False,global_goal='active; format metadata admission stage only complete',proposal_manifests_preserved=packages)
with gzip.open(out/'final-state.json.gz','wt') as f:json.dump(state,f)
unique={c['name']:c for c in schema['cases']};rows='\n'.join('| `'+c['name']+'` | '+(', '.join('`'+f+'`' for f in c['fields']) or 'nenhum')+' | '+str(c['locatable']).lower()+' |' for c in unique.values());notes=len(arch['added_v19_notes']);counts=obs['batch_counts']
follow='''# Pendências depois de formatos/HTML/PDF

A matriz de admissão do registry não tem mais rotas omitidas entre os 3956 casos de funções nos oito perfis. O escopo é type/where/figure-kind, não uma certificação geral de toda a linguagem, schema de todo Type, seleção efetiva, layout ou morfologia.

- A disponibilidade de std.html e descendentes sem a feature html já divergia e foi preservada: html-off-availability-audit.json registra type antes/depois e o oráculo. Os 24 where/kind agora aceitos nesses perfis não recebem crédito de paridade; corrigir o gate da rota std e testar aliases/tags também.
- Counter e construção de bare selectors ainda usam admissões separadas. Os quinze controles deste lote permanecem como antes; consultas/show/indexação têm seus testes e fronteiras próprios.
- Os erros de campo desconhecido ou interno têm mensagem correta, com âncoras ainda divergentes. Corrigir causalmente por ocorrência no owner de where, sem procurar valores iguais nem presumir span da chamada inteira.
- Auditar getters e matching das identidades adicionadas nos três lotes; aceitar campos em where não materializa a consulta nem o conteúdo.
- Permanecem 117 V16 anteriores. As novas notas V19 de Native/Format têm justificativa e vínculos de teste em architecture-delta.json; não são silenciadas. Outros V19/V20 seguem inventariados.
- FG1, RM1, SY1, RG1, pintura de texto e demais frentes anteriores permanecem abertas. RG1 segue pendente de aprovação; qualquer materialização futura precisa rebasear candidatos sobre TS1/equality e os demais owners vigentes, sem sobrescrever mudanças por cópia antiga.
- A indicação anterior de que FuncRepr era público no L0 estava errada. L0 e código declaram pub(crate); retificacao-func-repr.md conserva a correção com fonte/hash. Os artefatos anteriores ficam históricos.

O objetivo global continua ativo. Nenhum campo/trait/assinatura Rust externo novo, default, fase ou publicação é autorizado por este lote.
''';(out/'follow-ups.md').write_text(follow)
report=f'''# Relatório — identidade dos formatos e catálogo HTML/PDF

Etapa concluída: as quinze rotas restantes agora têm identidade de elemento reconhecida em `.where()` e `figure(kind: ...)`. Os cinco formatos usam um identificador interno fechado; HTML elem/frame e PDF attach/artifact usam seus callbacks reais. Não há nova API Rust externa. O objetivo global permanece ativo.

## Proveniência

HEAD `{b['head']}`, working tree não commitado. Baseline `{b['utc']}`, fechamento `{state['utc']}`. before.json.gz/final-state.json.gz conservam status, diff HEAD --stat, fontes afetadas e hashes de {len(hashes)} arquivos protegidos. A matriz anterior foi reutilizada apenas após igualdade exata de hashes de fontes/configuração/binário com a etapa anterior, em baseline-reuse.json. A posterior foi executada sobre a versão final.

Vanilla pinado `9f2b6e8715237cb086899a42873660fe744622e8`, binário SHA-256 `7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea`. before/after-measurement.json.gz e os lotes detalhados registram UTC, argv, binários e saídas. pinned-schema.json.gz conserva nove declarações do pin, blobs/hash/linhas e exclusões internal/external.

## Implementação e contratos

Onze pares consumer/owner atualizados; um Núcleo Tekt `selectors/format-native-identity.toml`, consumido/pinado pelos onze L0. A especificação antecedeu testes/código. `FormatElementKind` e `FuncRepr::Format(kind, NativeFunc)` são restritos ao crate. Func, NativeFunc e NativeFuncWithEngine conservam campos e assinaturas públicas; public-contract-before/after.json compara as interfaces dos onze consumers e os structs públicos de Func.

A testemunha distingue svg/png/pdf/html/bundle mesmo compartilhando callback. Nome, namespace, Eq Rust ou ponteiro comum não autenticam formato. Clones preservam identidade; uma Native comum reconstruída pelo mesmo ponteiro continua sem a capacidade. O constructor privado recebe o NativeFunc original; não há wrapper de chamada, novo binding, novo default, nova fase ou duplicação de factory. As chamadas continuam pela ABI estática existente, com o mesmo callback.

Nome, namespace, native_fn_addr, span, Debug, clone/With e Eq/Hash públicos preservam os observáveis anteriores. Eq/Hash continuam compatíveis com Native comum de mesmo nome; essa igualdade não é usada como prova de elemento. A relação observacional inclui o novo identificador e distingue Format de Native comum, sem executar. A factory global pdf/html e as projeções mantêm namespaces e membros clonados. Os quatro constructors de conteúdo permanecem Native; somente html.frame amplia visibilidade interna para reexport de metadados.

integrity-result.json prova os nove corpos de callbacks intactos e recupera as três factories anteriores removendo apenas a nova construção do carrier. Não houve commit, push, merge ou release, nem supressão de lint. Os manifestos TS1 e RG1 estão preservados.

| Identidade canônica | Campos selecionáveis | Locatable |
|---|---|---|
{rows}

HtmlElem exclui css/parent/role internos; label continua filtro comum. With não é elemento bare. As rotas globais/projetadas conservam o comportamento anterior de gates e aliases. Há uma lacuna preexistente em std.html e descendentes sem a feature html, registrada em html-off-availability-audit.json; não se certifica paridade desse gate.

## Verificação

- RED funcional: dezesseis falhas (quinze admissões e distinção observacional) e dois controles passaram. red-input.json.gz fixa fontes, owners, hashes, status/diffstat no momento anterior à execução. O gerador inicialmente encontrou um import stdlib também no módulo de teste; a busca foi restrita à função produtiva, sem resetar trabalho. Tentativa registrada em implementation-first-attempt.*.
- GREEN focado: 21 testes passaram, incluindo três guardas diretas V19 adicionadas após o RED inicial. A matriz anterior foi ampliada para as sete representações e executada também isoladamente. Suíte completa: {workspace['passed']} passaram, zero falhas, {workspace['ignored']} ignorados; fontes congeladas.
- Matriz completa: 3956 casos, oito perfis. {obs['changed_observables']} observáveis mudaram somente nas quinze rotas autorizadas; {obs['pin_matching_language_values']} valores de linguagem foram corrigidos para coincidir com o pin. Os outros {obs['preserved_observables']} observáveis permaneceram idênticos, inclusive todas as saídas do oráculo. Nenhuma omissão resta nesta matriz de admissão onde a função está disponível nos dois lados. As 24 mudanças em perfis de disponibilidade só Tekt são separadas em availability-residuals.json e não recebem crédito de paridade. A matriz parte dos bindings Tekt e não certifica que todo binding upstream exista.
- Lote detalhado: 144 casos; {counts['values']} valores de linguagem coincidem com o pin; {counts['headlines']} erros têm mensagem correta e âncora residual; {counts['counter']} controles de counter, {counts['partial']} parciais e {counts['root_calls']} rejeições de construção foram preservados. Counter/bare selector/query/show continuam frentes separadas.
- Compatibilidade: 456 casos bilaterais de nomes, repr, aliases/igualdade, partial-name e namespace em oito perfis, todos preservados exatamente nos dois binários. O aviso antigo do Tekt para a fixture vazia continua explicitamente tratado pelo verificador; não se declara stderr igual ao pin nem se descartam avisos arbitrários.
- Tamanhos medidos no RED e GREEN: `{obs['layout_before']}` bytes, sem alteração em Func, FuncRepr, NativeFunc e NativeFuncWithEngine. O layout Rust é controle de custo/API, não critério de paridade da língua.
- Build, fmt, Clippy workspace/all-targets estrito, Rustdoc -D warnings com quatro índices, V1/V5/V15/V26, reverse e diff-check passaram. Ownership/V26 precederam o resselo restrito aos onze pares.
- Erros/avisos anteriores foram preservados por mapeamento de linhas. Quatro notas V19 existentes foram estendidas somente pela alternativa Format, com a contagem correspondente; nenhuma alternativa anterior foi removida. Existem {notes} novas notas V19 nas uniões explícitas Native/Format: mesmo payload/ABI para a operação, com Native e Format exercitados nos testes de projeção/observação, nos 144 casos e nos 456 controles. Os padrões exatos, fontes/hashes e vínculos estão em architecture-delta.json. Nenhum novo erro/aviso; o gate global ainda é bloqueado pelos 117 V16 anteriores.
- candidate-1 conserva a primeira versão validada até o comparador. Antes da versão final, os L0 de três owners prescreveram guardas diretas: sete representações em identidade/hash/projeções; cinco kinds com mesmo nome/callback que panica se chamado; matriz de observação que varia kind, callback, nome e namespace separadamente. Nenhum comportamento produtivo foi alterado nessa ampliação de testes, e todos os gates e sondas foram repetidos.
- A primeira tentativa de iniciar os gates não criou processo porque a revisão automática expirou. Uma repetição reduzida foi aprovada e executou os gates; execution-review-timeout.json registra a ocorrência. Não houve conclusão de que o comando fosse inseguro.

## RAM e fronteiras

Comandos pesados sequenciais em MemoryMax12GiB, MemoryHigh10GiB, swap0 e Cargo jobs1. Pico contabilizado pelo cgroup: {peak} bytes ({peak/1024**3:.3f} GiB); eventos `{dict(events)}`. Sem atingir MemoryMax/OOM/oom_kill. Inclui cache, não somente RSS.

[Pendências]({out}/follow-ups.md). A admissão de metadados não certifica execução de seletores, síntese, getters completos, layout, morfologia, índice ou contadores. FG1/RM1/SY1/RG1 e os V16 permanecem abertos.

[Retificação sobre FuncRepr]({out}/retificacao-func-repr.md): a nota anterior que o chamava público no L0 era incorreta; a declaração já era pub(crate). Os arquivos anteriores ficam preservados como histórico. Evidências e hashes finais estão em final-state.json.gz e SHA256SUMS.
''';(out/'relatorio.md').write_text(report)
exclude={'SHA256SUMS','manifest.json','delivery-check.json','delivery-check.json.gz','finalize.log','finalize-resources.json'};files=[p for p in sorted(out.rglob('*')) if p.is_file() and p.name not in exclude];(out/'SHA256SUMS').write_text(''.join(sha(p)+'  '+str(p.relative_to(out))+'\n' for p in files));m=dict(utc=now(),files=len(files),manifest_sha256=sha(out/'SHA256SUMS'),source_files=len(hashes),source_changes=changed,new_nuclei=scope['nuclei'],peak_bytes=peak);(out/'manifest.json').write_text(json.dumps(m,indent=2)+'\n');print(json.dumps(m))
