# P1340 — fronteiras do binding passivo L3

Executor `/root/p1339_observation_design`, papel exclusivamente de
implementador do binding; executado sem atestação de isolamento. Skill
`tekt-materializacao-segregada` e ambas as referências lidas. Não é autoria
de oráculo, auditoria aceita, selo ou veredito.

Em `2026-09-10T14:13:38Z`, HEAD
`2f42d64253547734564513a1159ee6b584c1c4b4`, working tree não commitado e
concorrente. A instrução posterior do coordenador manda interromper o adapter
e registrar somente as interfaces/lacunas medidas. **Não foi criado o arquivo
Rust do observer, nenhum stub e nenhum hook produtivo por este executor.**
Não foram executados builds nem matrizes nesta subtarefa. Este diagnóstico
e seu recibo JSON companheiro são as únicas escritas novas desta subtarefa.

## Entradas e medições anteriores à disposição

Selo recebido e verificado:
`p1340-verifier-seal-r3.json`, SHA-256
`82ceda880adef9a6ba2f0330f71badcfea084af64168510501cb4c4a939ea9f4`.
Delegação `p1340-pipeline-observer-delegation.json`, SHA-256
`781dca3f165a5e99ad32da2c326bb2ac1895dfec56d843eb6016caa03dd0d3e7`.

As referências abaixo são paths relativos ao repositório, medidos no
snapshot acima; linhas de arquivos concorrentes não substituem seus hashes.

### Portas presentes

- `01_core/src/compiler/eval/mod.rs:5850-5864`: requests na ordem original,
  replays append-only, operação, argumentos, resultado integral, chain,
  contexto, phase e relação Same/Different/Unproven realmente calculada.
  Um cursor antes/depois da chamada real separa seu suffix sem inventar
  replays nem inferir a relação do bool final. Reflection e Diagnostics
  não devem ser rotulados como ValidationCompleted de um candidato.
- `eval/mod.rs:5643-5658,5664-5677`: projeção de diagnóstico e identidade
  Func/capture/body-span. `entities/source.rs:183-195` resolve raw Span na
  Source original para coordenadas; spans desconhecidos não autorizam
  substituição por offset da fixture.
- `eval/mod.rs:5740`: projeção integral dos eventos CounterRegistry na
  ordem de `actions()`, inclusive `key:null` para evento automático real.
  Não filtrar esse evento como se fosse ausência de observação.

### Lacuna demonstrada: identidade da função efetivamente invocada

`eval/mod.rs:5870-5873` armazena somente `(phase, ContextCallbackKind)`.
Não guarda a Func invocada. `introspect/from_tags.rs:76-81` dispõe da
função real no fold e chama `apply_func`; `stdlib/state.rs:116-117` também
registra só a categoria antes da invocação real. Há outros callsites de
`apply_func` em `from_tags.rs:130,174,251,357,418,473` e
`stdlib/counter.rs:563`; sua inclusão/exclusão no grafo alcançável precisa
de auditoria, não de uma lista vazia presumida como completa.

O DTO exige `FunctionInvoked { func_id, origin }`, e
`SW-LC-invocation-origin-completeness` exige cobertura de todas as invocações
alcançáveis. O port de Func individual não resolve a falta: é preciso
receber a **Func do callsite real**, conservar sua vida útil e a origem da
chamada. Extrair uma Func de uma request não demonstra que ela foi chamada.
As escritas nesses callsites não pertencem à delegação deste executor.

### Lacuna demonstrada: Span do evento de counter

`entities/counter_registry.rs:21-24` conserva location/key/action, sem Span.
`entities/elements/counter_update.rs:23-25` conserva key/action, sem Span.
`stdlib/counter.rs:121-148` constrói o conteúdo sem receber Span; uma vez
nesse carrier, a L3 não consegue recuperar o span exato da expressão que
construiu o update. O Span de uma leitura de counter não é o Span do update.

O DTO `CounterEvent` exige Span e producer_generation_id. A cadeia real
`candidate.parent_locations` / `context_block_locations` pode fornecer
proveniência contextual em casos representados, mas não recupera a origem
lexical perdida nem prova a origem estática de todo evento automático.
Um binding completo precisa observar a origem antes dessa perda no callsite
de construção e conservá-la até a emissão do evento; isso não existe nas
portas lidas. Não substituir por source.root, primeiro offset da fixture,
posição do counter consumidor, Debug, ou uma seleção por resultado esperado.

### Conflito de features encaminhado, não corrigido pelo binding

`p1340-contract-correction-predicate-v2-r3.py:77-86` exige features do
perfil em **todo** BodyStarted, sem excluir descoberta (`attempt=0`).
No baseline `git show HEAD:03_infra/src/pipeline.rs:245-278`, a expansão
cria EvalContext::new e não propaga features. O L0
`prompts/infra/pipeline/context_stabilization.md:101-110` conserva recursos
da tentativa e proíbe reparar propagação legada fora do recorte.
O candidato lido `context_stabilization.rs:131-132` propagava features
também na descoberta; o coordenador anunciou sua correção e encaminhou o
conflito contratual ao verificador no limite de revisões.

O adapter não decide qual obrigação suceder nem falsifica `resources.features`
usando o perfil recebido. Sua projeção teria de mostrar os valores reais
do corpo/request/replay, mesmo se isso refutar o predicado congelado.

## Interfaces discutidas — não implementadas

Foram enviadas ao coordenador assinaturas para os boundaries reais:
post_eval, session_started, attempt_started, body_started/body_completed,
candidate_built, validation_started/validation_completed,
contribution_retained, generation_invalidated,
execution_discarded/execution_published, terminal_before_decision,
compilation_returned e exporter_dispatched. Todas seriam cfg-only no
submódulo descendente da mesma Session, nunca outro coordenador.

Requisitos mecânicos levantados na conversa, sem materialização aqui:

- identidade de Execution alocada antes do corpo e conservada no objeto,
  com execuções retiradas retidas até o fim, para evitar reutilização de
  endereço e associar output/requests/sink à execução original;
- snapshots reais em Arc, conservados entre CandidateBuilt e o próximo
  AttemptStarted; não gerar IDs de snapshot por contador do collector;
- candidate_built recebe também Content/documento reais; transferência
  causal Dk até retorno precisa de hook no retorno comum. A hipótese de
  usar backing de `doc.pages` exige prova de vida útil/move e não é aceita
  neste recibo; não basta presumir que último candidato é documento final;
- captura dos erros originais e dos resultados de corpos selecionados
  antes do branch terminal, não clone do resultado final já decidido;
- inserção somente das sentinelas normadas nos sinks reais e hooks de
  descarte/publicação nas arestas reais. O collector não deve publicar,
  escolher retenção nem sintetizar disposição pelo resultado desejado;
- contador de exportação no dispatch real da fachada de exportação, e não
  inferido de `Ok(document)`; caso de topologia impossível deve chamar a
  operação terminal real pelo estado privado comprovado definido em R2.

As matrizes lifecycle e terminal continuam **não vinculadas por este
executor**. Nenhum `binding_audit=accepted_real_product_path` foi escrito.
Propostas de assinaturas não satisfazem nenhum gate runtime ou testemunha
estrutural. A continuidade do adapter requer resolver explicitamente as
portas/capacidades e o conflito apontado; não uma mudança clandestina dos
predicados ou das expectativas.

## Pins de entradas e fontes medidas

```text
4d9426025cbc986b79793658e99a20203216089eb9951d9ece759cb962027f8c  p1340-contract-binding-r2.md
1a23c05af0ed187b233fb9f8afe1198f9d7712a327619bee32bbd24c06bd414c  p1340-contract-correction-predicate-v2-r3.py
873a408dc935e3d3c543301a0019a8f5d4a26d887d95f8affca4095a85e8e5ad  p1340-contract-correction-predicate-r3.py
d8e07c060162976895e54ebf55dc12e5b51e5234aa173f09f234f82a2cc38f1b  p1339-mutant-closed-state-lifecycle-collector.rs
6d8c257997631133fb1a7a55816daaa50e1633c129fafd17f6d1ddeb378ca541  p1339-ab-batch1-retention-lifecycle-typed-r3.json
b4146639cfc5f34f8f0cbf158b13dd4c79e1922bc1c385e3cff6e6dffb11ce5b  01_core/src/compiler/eval/mod.rs
1c73dd0a10e078fb17d531304bb43b3fef2c4c280789c19c06f20c92506daed0  01_core/src/entities/elements/counter_update.rs
4f2d6292229a32f0a4c74d48fde3e95162e61920e4960af86f172a478e5ef81d  01_core/src/entities/counter_registry.rs
7e195d6717a21a9d5cba46a7a675f3853550794d3d3eaaa0c23543ba998ebeef  03_infra/src/pipeline/context_stabilization.rs
```

Nomes sem diretório nessa lista são relativos a `00_nucleo/diagnosticos/`.
Os hashes de fontes concorrentes identificam a leitura, não uma versão
final congelada ou aprovação da implementação.
