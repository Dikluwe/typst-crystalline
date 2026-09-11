# P1339 — revisão da dependência contextual

## Regime e proveniência

Revisor `/root/p1339_context_dependency_review`, investigação somente leitura
das entradas, com escrita exclusiva deste diagnóstico. Skill
`/home/dikluwe/.codex/skills/tekt-materializacao-segregada/SKILL.md` e ambas as
referências de papéis/capacidades e artefatos/gates lidas. A busca normativa
por segregação em `00_nucleo/adr` não encontrou ADR específica; ADR-0127 lida.
Regime: executado sem atestação de isolamento, por disciplina declarada em
filesystem compartilhado. Não constitui contrato, oráculo independente, selo,
aprovação de escopo nem veredito de implementação.

Entradas: fontes baseline de `03_infra/src/pipeline.rs`, `compiler/eval` e
`compiler/introspect`; L0 `infra/pipeline.md` e `compiler/eval.md`; runner,
relatório e recibo `p1339-where-counter-runtime-probe`; aprovação
`p1339-where-counter-phase-approval.json`. O coordenador acrescentou durante
a revisão `p1339-context-dependency-probe.py` e seu recibo. Não foram lidos
outros L0 modificados nem patch candidato. Não foi lido conteúdo de
`materialization/` ou `context/`; referências de paths presentes nos
inventários de proveniência não foram seguidas.

Baseline: `2f42d64253547734564513a1159ee6b584c1c4b4`, working tree não commitado.
Em `2026-09-10T01:42:32.987744131Z`, `git diff --exit-code HEAD --` sobre
as fontes e os dois L0 permitidos terminou com código zero: essas entradas
continuam idênticas ao HEAD. O `git diff HEAD --stat` e o inventário exato
da árvore na medição bilateral estão no recibo abaixo; as modificações
produtivas examinadas são nulas. Este revisor não compilou nem executou
fixtures; analisou as execuções registradas pelo coordenador.

SHA-256 das entradas principais:

| Entrada | SHA-256 |
|---|---|
| `prompts/infra/pipeline.md` | `ce6da4f623a0270869606bdf42d07dae0283f3f9e5a593258609b219424e63a8` |
| `prompts/compiler/eval.md` | `80412483c8353a736f5c11842511cdef6c5e854728281a780c0183d6b73875cf` |
| `03_infra/src/pipeline.rs` | `72f9c080b55ca295e5b51ae45acf527c76921cd06211a64c2c43fdd010af6088` |
| `01_core/src/compiler/eval/mod.rs` | `115a34e8aa41b4ec5a55b0cec5d05927216dc6a4a6fcb98d2fd2c1edc98d262c` |
| `01_core/src/compiler/introspect.rs` | `09029233b75b358953024023dd11b060544890ac791b3151129afd7687a3066e` |
| `p1339-where-counter-runtime-probe-runs.json` | `178a8637df7eb75d21131a46ad68005b9f7e35670644c9cb9b7eeb93031d5ae3` |
| `p1339-where-counter-phase-approval.json` | `daad3fedc3c97be3a7aad950f837fa8702e92a9551e5f363e3a54ed21187445e` |
| `p1339-context-dependency-probe.py` | `7fd025bc7ef8befe84d8417408dac392c5a9258b2ec24ef0e5b664d08a33d6b0` |
| `p1339-context-dependency-probe-runs.json` | `4da38b499916ed3d5946bb16c904f2eaa268a4ec2f788cc0149927f0d83260d1` |

Hashes identificam arquivos integrais, não atestam a normalização de linhagem
do linter. Não se executou resselo ou gate arquitetural nesta investigação.

## Medição anterior à conclusão

O recibo bilateral executou `compile` real entre
`2026-09-10T01:41:58.226479+00:00` e `01:42:00.368356+00:00`, no HEAD e
working tree acima. Preserva fontes, hashes, argv, cwd e stderr completos.
Vanilla ratificado `a51e02804`: `/usr/local/bin/typst`, SHA-256
`7b4f40c56d6fa95082ebcfd893e275d418ebcaed1b97b62785f78284c63ff7b8`.
Cristalino: `/tmp/p1338-target.vlNAmp/release/typst`, SHA-256
`f7c8085f8453ecb6b10841b698092d41e7fbec64d9d46f9341b9b9b538bfefa1`.
A revisão verifica a identidade registrada do executável, sem alegar novo
build ou atestação independente da sua construção.

| Fixture | Vanilla | Cristalino baseline |
|---|---|---|
| `plain_set_control`: update(12) não contextual, chave string | Sucesso | Sucesso |
| `context_set_forward`: update(12) gerado em context, get posterior | Sucesso | `assertion failed` |
| `context_set_final_before`: final antes do produtor contextual | Sucesso | `assertion failed` |
| `context_filtered_set`: heading.where(level: 1), Set contextual | Sucesso | `assertion failed` |
| `context_filtered_func`: mesmo filtro, Func contextual | Sucesso | `assertion failed` |
| `stable_error_control`: assert dependente seguido de panic | `panicked with: stable-error` | `assertion failed` |

O probe runtime vanilla anterior mediu separadamente
`context_generated_update_assert12` e `stable_one_page_context`: o update
funcional contextual, seguido da leitura/assert, chega ao valor `(12,)`.
O controle de página final igual a uma não mede quantas passagens internas
foram necessárias. O mesmo recibo mostra que callback sem consulta não é
executada, enquanto get anterior a callback inválida do mesmo counter observa
o erro posterior. Callback de update não recebe contexto para ler outro
counter, inclusive quando o elemento update foi criado dentro de context.

Na fonte baseline:

- `eval/mod.rs:1404-1427` cria uma closure para `Expr::Contextual`; a expressão
  do update ainda não executou. O produto imediato é `Content::ContextBlock`.
- `introspect.rs:1058-1065` registra somente o ID/Location do bloco;
  `:1930-1933` trata o bloco como terminal. Logo, a primeira introspecção não
  contém o update que só existirá ao chamar essa closure.
- `pipeline.rs:244-249` avalia cada bloco com um novo `EvalContext` e clone
  do mesmo `intr` recebido. `:278` guarda o resultado em `resolved`, mas
  `:281` só substitui a árvore depois de concluir o conjunto. Um produtor
  executado antes do consumidor não atualiza o snapshot do consumidor.
- `pipeline.rs:271-277` propaga o erro da closure imediatamente por `?`.
  A reintrospecção em `:309-310` não acontece se qualquer bloco falhar;
  `:658-662` encerra a compilação. O runtime de counters em `:665-666` e o
  primeiro layout em `:690` ficam inalcançáveis para esse documento.
- `introspect.rs:1037-1039` ignora Func no walk sem runtime; `:415` chama
  o pós-processador global quando há runtime. `from_tags.rs:94-135` percorre
  as callbacks e propaga erros. Isto é outra dimensão, distinta da ausência
  do próprio elemento update na primeira expansão.
- `pipeline.rs:721-743` já reexpande a árvore original usando o snapshot
  anterior e reintrospecta entre layouts. Contudo, `:754-757` encerra o ciclo
  apenas por igualdade de quantidade de páginas. Não testa que as entradas
  contextuais ou resultados realizados usados na última expansão continuam
  válidos perante o snapshot que ela própria produziu.
- `introspect/fixpoint.rs:83` também aborta imediatamente se `eval_step`
  falha; `:111-130` usa hash de tags para terminar. Envolver o helper atual
  nesse loop, sem mudar o tratamento de tentativas e invalidação, não remove
  o bloqueio inicial.

## Conclusão e limites

Apenas resolver `CounterUpdate::Func` sob demanda em get/final/at/display
**não basta** para o caso perguntado. Mesmo um resolver correto para todos
os eventos presentes não pode encontrar um update ainda ausente da árvore
introspectada. O assert falha antes da passagem que incorporaria o produtor.
A falha medida com Set e chave string demonstra que o problema não depende
da nova chave filtrada nem da execução de Func. O consumidor de final antes
do produtor refuta a solução limitada a ordenar blocos de frente para trás.

É inferência causal apoiada pela fonte e pelos controles. Uma leitura que
observasse updates gerados por blocos irmãos no mesmo snapshot inicial, sem
nova expansão/introspecção ou dependência explícita, refutaria a descrição;
o baseline não apresenta tal caminho. Não se afirma que todas as falhas
filtradas tenham só esta causa: o resolver e a identidade dos filtros ainda
precisam dos seus próprios contratos.

Sucesso/erro do programa, valores consultados e diagnóstico final são
observáveis da linguagem. Quantidade de tentativas, ordem interna dos maps
ou igualdade Rust de `Content` não são critérios de paridade. Igualdade de
páginas, sozinha, não certifica a estabilidade desses observáveis.

## Menor desenho compatível a concretizar em L0

O L0 vigente oferece o lugar para a correção: expansão entre introspecção
e layout (`infra/pipeline.md:175`), reintrospecção da árvore realizada
(`:426`), preservação do marcador contextual e partida da árvore original
em cada iteração (`:515`), e convergência por páginas **mais** snapshots,
objetos e conteúdo realizado (`:503-504`). `compiler/eval.md:49-54` conserva
as árvores pré-show e pós-show distintas. Portanto, a menor direção mantém
a produção de `ContextBlock` no eval e a estabilização na fronteira já
existente em L3. A proposta é condicional, não instrução para implementar:

1. Uma tentativa privada de expansão recolhe por bloco resultado e
   diagnósticos. Continua a tentar os irmãos após erro de um bloco, mantendo
   cada leitura no snapshot daquela tentativa. Não reconhece mensagens
   `assertion failed` nem inspeciona sintaxe para adivinhar produtores.
2. Incorpora os resultados dos blocos que tiveram sucesso numa árvore
   provisória, conservando marcadores dos pendentes, e reintrospecta para
   produzir o próximo snapshot. A próxima tentativa parte novamente da
   árvore original; nunca acumula updates ou conserva um resultado anterior
   como sucesso atual de um bloco que passou a falhar.
3. Reavalia consumidores contra o novo snapshot. No caso focal, uma
   tentativa torna o update disponível e uma tentativa posterior permite
   ao get demandar sua sequência; o assert passa se esse resolver produzir
   o valor esperado. A mesma forma cobre final anterior ao produtor.
4. Só conclui sucesso quando resultados e dependências efetivamente
   observadas permanecem válidos, com nenhum erro pendente. Estabilidade
   deve incluir a contribuição contextual e os dados consultados de
   counters/queries/pages; não apenas total de páginas ou presença de tags.
   Um erro de tentativa invalidada pode ser descartado; erro que persiste
   na tentativa estável deve sair com seus spans e traces. No controle
   medido, o diagnóstico final deve ser `stable-error`, nunca sucesso nem
   o assert do snapshot inicial. Warnings também precisam de política por
   tentativa para não duplicar avisos de trabalho descartado.
5. Um teto finito impede repetição ilimitada. Invalidação sem estabilização
   não permite exportar conteúdo provisório como se fosse final. Para
   dependência de páginas, a invalidação contextual integra a condição de
   repetição já existente após layout; a contagem de páginas não encerra
   sozinha esse trabalho.

São dados privados possíveis: árvore original, snapshot anterior, resultados
e erros por ID de bloco e informação de validação. As assinaturas existentes
`expand_context_blocks(...) -> SourceResult<Content>` e
`expand_context_blocks_and_reintrospect(...) -> SourceResult<(Content,
TagIntrospector)>` podem permanecer fachadas. Isto evita pressupor novo
campo público em `EvalContext`, callback de L3 dentro de entidade L1 ou
reentrada layout→eval. A suficiência dos carriers existentes é inferência:
identidade estável dos blocos, exportação das dependências necessárias e
comparação normativa ainda precisam ser concretizadas e verificadas pelos
owners autorizados. Esta revisão não leu os demais L0 para prometer que
novos carriers seriam dispensáveis.

Resposta explícita sobre compatibilidade: é possível descrever essa
orquestração com as assinaturas públicas existentes, mas **não foi
demonstrado um desenho que preserve simultaneamente essas assinaturas, os
L0 do snapshot sem alterações e o escopo seletivo já aprovado**. A proposta
genérica acima é uma direção para a dependência, não materialização
autorizada. Aplicá-la indiscriminadamente também corrigiria os controles
de chave string e alteraria caminhos legados que a aprovação mandou
preservar. Esses controles localizam a causa; não autorizam corrigir esses
caminhos neste lote.

Para restringir a mudança aos consumidores aprovados, a decisão precisa
de informação semântica da dependência realmente observada. Não pode
usar string de erro, inspeção do AST ou varredura de nomes de métodos para
decidir quais tentativas repetir. Esta revisão não identificou no snapshot
um protocolo explícito L1→L3 de invalidação seletiva que satisfaça isso.
Se a forma concreta exigir tal dado por novo campo/carrier público,
retorno ou método público, trata-se de decisão de contrato ainda não
especificada/aprovada. Não se promete compatibilidade total baseada apenas
na possibilidade de manter as duas fachadas de expansão.

Não é suficiente repetir só o bloco que falhou sem incorporar produtores,
repetir com o mesmo snapshot, ignorar erros até o teto ou aplicar o update
como mutação lateral quando a chamada retorna conteúdo. Esse último atalho
quebraria a distinção entre construir e inserir um elemento update. Executar
globalmente toda Func tampouco resolve a obrigação de demanda: o controle
de callback nunca consultada exige preservar sua não execução.

## Fronteira de autorização

`p1339-where-counter-phase-approval.json` autoriza resolução sob demanda
contextual dos contadores filtrados, preservando assinaturas e chaves
legadas, sem execução global antecipada; não dispensa nucleação dos owners
restantes, contrato/oráculos, mutantes, selo ou RED independente. Não contém
`infra/pipeline.md` nem `compiler/eval.md` na lista de L0 aprovados. A revisão
não converte essa aprovação numa autorização irrestrita para estabilização.

Corrigir a condição de parada para cumprir `infra/pipeline.md:503-504`,
dentro das mesmas fases e APIs, é candidato a reparo interno/paridade em
fluxo contínuo, sujeito à preservação efetiva do escopo aprovado. Contudo,
o protocolo de tentativas parciais, invalidação de
diagnósticos e tratamento de não convergência não está explicitado no L0
lido: precisa de redação normativa anterior ao código e classificação
concreta pelo coordenador. Compatibilidade arquitetural não equivale a L0
já suficiente. Este diagnóstico não decide que tal ampliação já foi aprovada.

ADR-0127 exige gate para nova assinatura pública, novo campo público/carrier
de entidade, mudança de trait, quebra de compatibilidade ou deslocamento
de fase/ordem. Exemplos aqui: expor API de tentativa no lugar de SourceResult;
transportar resolver de contexto por novo campo público; mover execução de
context/show para eval inicial; introspectar como default a árvore pós-show
para corrigir conjuntamente o achado P1037; executar callbacks dentro do
layout. Essas ampliações não se tornam implícitas por a implementação ser
pequena. O desenho mínimo deve primeiro mostrar quais delas consegue evitar.

Estado entregue: bloqueio causal demonstrado no baseline; direção mínima
e fronteiras normativas identificadas. Código, L0, contrato e autorização
permanecem intactos por esta revisão. Não há alegação de GREEN ou fechamento
da paridade contextual.
