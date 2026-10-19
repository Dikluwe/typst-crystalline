from pathlib import Path
import json,gzip,hashlib,datetime,subprocess,collections,difflib
out=Path(__file__).resolve().parents[1];sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest();now=lambda:datetime.datetime.now(datetime.timezone.utc).isoformat();b=json.load(gzip.open(out/'before.json.gz'));scope=json.load(open(out/'scope.json'));validation=json.load(open(out/'validation-result.json'));assert validation['complete'];hashes={p:sha(p) for p in b['hashes']};hashes.update({p:sha(p) for p in scope['nuclei']});assert validation['source_hashes']==hashes;changed=[p for p,h in b['hashes'].items() if hashes[p]!=h];assert set(changed)==set(scope['sources'])|set(scope['owners'].values());workspace=json.load(open(out/'workspace/workspace-result.json'));assert workspace['passed']==8879 and workspace['failed']==0 and workspace['ignored']==7 and workspace['exit']==0;obs=json.load(open(out/'observables-result.json'));arch=json.load(open(out/'architecture-delta.json'));integrity=json.load(open(out/'integrity-result.json'));schema=json.load(open(out/'batch-schema.json'));assert arch['before']==arch['after'];resources=[];resource_aliases=[];seen={}
for p in sorted(out.rglob('*-resources.json')):
 record=json.load(open(p));identity=json.dumps(record,sort_keys=True);relative=str(p.relative_to(out))
 if identity in seen:resource_aliases.append(dict(file=relative,same_execution_as=seen[identity]));continue
 seen[identity]=relative;resources.append(dict(file=relative,record=record))
peak=max(int(x['record']['memory_after']['memory.peak']) for x in resources);events=collections.Counter()
for row in resources:
 r=row['record'];assert r['memory_before']['memory.max']==str(12*1024**3) and r['memory_before']['memory.high']==str(10*1024**3) and r['memory_before']['memory.swap.max']=='0'
 for line in r['memory_after']['memory.events'].splitlines():
  key,n=line.split();events[key]+=int(n)
assert all(events[k]==0 for k in ['max','oom','oom_kill'])
packages={}
for name,expected in [('text-size-length-2026-10-17','771f5fd3d36545eb4b4fb7f0c0f2c91061470d1f010fc9a887b001a636d8feba'),('table-row-groups-2026-10-17','a47fe97272fd6a5bd96022be959ba277a317a74086f5e5f4a569bba008f4fae9')]:
 folder=Path('00_nucleo/materialization')/name;assert sha(folder/'SHA256SUMS')==expected
 for line in (folder/'SHA256SUMS').read_text().splitlines():
  digest,path=line.split(maxsplit=1);candidate=folder/path
  if not candidate.exists():candidate=Path(path)
  assert sha(candidate)==digest
 packages[name]=expected
patch=''.join(''.join(difflib.unified_diff(b['files'][p].splitlines(True),Path(p).read_text().splitlines(True),fromfile='before/'+p,tofile='after/'+p)) for p in scope['sources']);(out/'scoped.patch').write_text(patch)
state=dict(utc=now(),head=b['head'],working_tree='não commitado',status=subprocess.check_output(['git','status','--porcelain=v1'],text=True),diffstat=subprocess.check_output(['git','diff','HEAD','--stat'],text=True),source_hashes=hashes,changed_since_previous=changed,new_nuclei=scope['nuclei'],scope=scope,batch_schema=schema,tests=workspace,observables=obs,architecture=arch,integrity=integrity,peak_bytes=peak,memory_events=dict(events),resources=resources,resource_aliases=resource_aliases,public_api_changes=False,global_goal='active; this metadata batch complete',proposal_manifests_preserved=packages)
with gzip.open(out/'final-state.json.gz','wt') as f:json.dump(state,f)
rows='\n'.join('| `'+c['expression']+'` | '+', '.join('`'+f+'`' for f in c['fields'])+' | '+(', '.join('`'+f+'`' for f in c['excluded']) or 'nenhum')+' |' for c in schema['cases']);remaining=obs['missing_paths_after'];assert len(remaining)==15 and all(not p.startswith('std.math.') for p in remaining)
follow='''# Pendências depois do catálogo matemático

A matriz registra oito perfis e 3956 casos de funções do registry efetivo; não certifica todos os campos associados a Type ou comportamento de render/query/show. before.json.gz e final-state.json.gz fixam HEAD, árvore não commitada, diffstat e hashes; as coletas bilaterais conservam todos os comandos e saídas.

## Rotas ainda omitidas

'''+ '\n'.join('- `'+p+'`' for p in remaining)+'''

## Próximas correções

- Auditoria preparatória em next-format-source-evidence.json.gz e next-format-pinned-schema.json.gz conserva fontes, hashes e nove declarações de schemas do pin, sem decidir arquitetura. O L0 histórico de FuncRepr o mostra público enquanto o código atual é pub(crate): conferir o contrato antes de assumir que ampliar a representação dispensa gate.
- Formatos raiz compartilham native_format no Tekt; reconhecer somente pelo ponteiro atual ou pelo nome criaria identidades falsas. Medir e definir a identidade interna no L0 antes de materializar. HTML/PDF e subfunções exigem schemas próprios e aliases efetivos preservados.
- Counter/bare selector têm admissões separadas, inclusive listas antigas de funções. As 26 sondas de counter continuam divergentes; não confundir metadados com suporte de contador/indexação.
- Os 29 negativos de campo desconhecido/internal/external têm mensagem correta, mas a âncora do diagnóstico ainda diverge do pin. São 26 desconhecidos mais dois campos internos de frac e gap external de mat.
- Revisar campos/capacidades das entradas históricas; as 26 funções aqui corrigidas não certificam toda a tabela. Helpers sqrt/abs/norm/round continuam funções não-elemento.
- Permanecem os 117 V16 anteriores. V19/V20 seguem notes preservadas; nenhuma supressão nova foi adicionada.
- FG1/RM1/SY1/RG1 e demais frentes anteriores continuam abertas. TS1/equality foram preservados; RG1 continua pendente de ratificação. Nenhuma API pública ou publicação é autorizada por este lote.

Correções internas de tabela/paridade seguem ADR-0127 em fluxo contínuo. Mudança pública, fase, default ou compatibilidade requer a proposta concreta correspondente. O objetivo global continua ativo.
''';(out/'follow-ups.md').write_text(follow)
report=f'''# Relatório — catálogo matemático nativo

Etapa concluída: as 26 rotas matemáticas omitidas agora são reconhecidas como funções de elemento em `.where()` e na admissão de `figure(kind: ...)`. O catálogo mantém os campos selecionáveis e a não locatabilidade definidos no pin. O objetivo global permanece ativo.

## Proveniência e medição

HEAD `{b['head']}`, working tree não commitado. Baseline `{b['utc']}`, fechamento `{state['utc']}`. before.json.gz/final-state.json.gz conservam status, lista exata de alterações via diff HEAD --stat e hashes de {len(hashes)} arquivos protegidos. Vanilla pinado `9f2b6e8715237cb086899a42873660fe744622e8`, executável SHA-256 `7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea`.

A matriz anterior foi reutilizada da etapa precedente somente após provar igualdade exata dos hashes de fontes/configuração e do binário Tekt atual, em baseline-reuse.json. É a medição anterior real, com seu UTC original, não nova execução presumida. A matriz posterior foi executada novamente: 3956 casos nos oito perfis, cobrindo type/where/kind. before-batch.json.gz e after-batch.json.gz registram 227 casos adicionais de campos, exclusões, label, counter e aplicação parcial. batch-schema.json conserva fonte primária do pin, blob SHA-256, file:line, bloco literal, atributos e ABI real de cada callback.

## Implementação e L0

Cinco pares consumer/owner foram atualizados; mais um Núcleo Tekt compartilhado `selectors/math-native-bridges.toml`, pinado pelos cinco L0. O L0 e o Núcleo precederam testes/código. A tabela central recebeu 26 entradas por fn_addr_eq tipada, sem invocar o constructor ou discriminar pelo nome. Todas são não locatáveis. As três fachadas transportam 23 callbacks existentes por reexport direto restrito a crate::compiler; accent/cancel/mat já tinham pontes. O alias interno de class preserva o reexport público de teste preexistente. Apenas scripts/limits ampliam visibilidade interna de pub(super) para pub(in crate::compiler), com corpos e assinaturas preservados.

integrity-result.json prova que os hubs mudaram somente por reexports internos, os corpos de scripts/limits são preservados, a montagem runtime de math não mudou e toda a lógica/testes produtivos anteriores de element_where permanece fora das 26 novas entradas. Não há API Rust externa nova, mudança de fase/default, binding novo, commit/push/release ou supressão de lint. TS1/equality, demais consumers e os manifestos TS1/RG1 permanecem íntegros.

| Binding | Campos selecionáveis | Campos excluídos |
|---|---|---|
{rows}

`mat(gap:)` continua parâmetro de construção válido sob seu contrato; gap external não é campo de seleção. Frac não expõe controles internos de deparenthesização. Math underline/overline conservam identidade própria, diferente das decorações globais. Sqrt/abs/norm/round não viram elementos por produzirem conteúdo.

## Validação

- RED funcional: 26 testes falharam por identidade ausente; um controle passou. A primeira tentativa teve erro de inferência de lifetimes na fixture de ponteiros; foi corrigida e arquivada em red-harness-*, sem ser contada como RED funcional.
- O primeiro verificador de integridade esperava chaves também em imports unitários, mas rustfmt as remove. O parser foi corrigido para as duas formas, preservando a exigência do conjunto exato de 23 callbacks; a tentativa e a retomada com fontes congeladas estão arquivadas em integrity-checker-first-attempt.
- Clippy detectou um reexport novo após o módulo inline de testes do hub estrutural. A tentativa candidate-1 conserva fontes, hashes e logs. O L0 foi atualizado antes de mover as pontes para antes dos testes; todos os gates e a matriz foram repetidos sobre as fontes finais, sem supressão do lint.
- O comparador diferencial inicialmente reutilizou o caminho SARIF de uma etapa mais antiga. Foi corrigido para a etapa imediatamente anterior, com igualdade obrigatória entre seus hashes finais de fontes e o baseline desta etapa. O comparador completo confirmou zero diferenças após mapear as linhas; tentativa preservada em differential-baseline-first-attempt. Nenhum código mudou nessa correção do verificador.
- GREEN: 27 testes focados passaram. Suíte completa: {workspace['passed']} passaram, zero falhas, {workspace['ignored']} ignorados, sem mutação das fontes durante a execução.
- Matriz completa: {obs['changed_observables']} observáveis mudaram só nas identidades autorizadas e nos aliases globais Tekt accent/cancel; {obs['pin_matching_language_values']} valores de linguagem passaram a coincidir com o pin. Os demais {obs['preserved_observables']} observáveis ficaram idênticos, incluindo todas as saídas do vanilla.
- As rotas omitidas caíram de 41 para 15; nenhuma rota matemática continua omitida nesta matriz de admissão. Isso não é certificado geral de morfologia/layout/query/show.
- Dos 227 casos detalhados, {obs['batch_pin_matching_language_values']} têm valores de linguagem iguais ao pin; {obs['unknown_and_excluded_headlines_match']} erros desconhecidos/excluídos têm mensagem correta mas âncora residual; {obs['counter_controls_preserved']} controles de counter e {obs['partial_controls_preserved']} de aplicação parcial foram preservados. Counter permanece divergente e aberto.
- A entrada das sondas é vazia. O aviso Tekt `ficheiro vazio: sem conteúdo` já existia em {obs['existing_empty_fixture_warning_controls']} controles anteriores e continua nos novos sucessos; o verificador exige seus bytes exatos. Não é declarado stderr igual ao pin nem são descartados outros avisos.
- Build, fmt, Clippy workspace/all-targets estrito (-D warnings e unfulfilled_lint_expectations), Rustdoc estrito com quatro índices, V1/V5/V15/V26, linhagem reversa e diff-check passaram. V15/V26 foram verificados antes do resselo limitado aos cinco owners.
- Diagnóstico global preservado após deslocamentos de linha: 117 V16 errors, 1076 V19 notes, 873 V20 notes. Nenhum novo erro/aviso. O gate global continua bloqueado pelos V16 anteriores; este lote não os reduz nem os silencia.

## RAM e evidências

Comandos pesados sequenciais, MemoryMax12GiB, MemoryHigh10GiB, swap0, Cargo jobs1. Pico do cgroup: {peak} bytes ({peak/1024**3:.3f} GiB). Eventos agregados `{dict(events)}`; sem atingir MemoryMax, OOM ou oom_kill. Inclui cache contabilizado pelo cgroup, não apenas RSS.

[Pendências e próximos lotes]({out}/follow-ups.md). Fontes exatas, escopo, patch, scripts, logs, testes, hashes e medições estão neste diretório; final-state.json.gz e SHA256SUMS fixam o fechamento. O Núcleo tem um papel distinto dos cinco Prompts L0 proprietários e não é alvo direto de @prompt.
''';(out/'relatorio.md').write_text(report)
exclude={'SHA256SUMS','manifest.json','delivery-check.json','delivery-check.json.gz','finalize.log','finalize-resources.json'};files=[p for p in sorted(out.rglob('*')) if p.is_file() and p.name not in exclude];(out/'SHA256SUMS').write_text(''.join(sha(p)+'  '+str(p.relative_to(out))+'\n' for p in files));m=dict(utc=now(),files=len(files),manifest_sha256=sha(out/'SHA256SUMS'),source_files=len(hashes),source_changes=changed,new_nuclei=scope['nuclei'],peak_bytes=peak);(out/'manifest.json').write_text(json.dumps(m,indent=2)+'\n');print(json.dumps(m))
