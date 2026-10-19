# CI: auxiliares privados exclusivos de teste

Cinco adaptadores sem callers produtivos agora são compilados apenas nas
configurações de teste que os usam. Também foram retiradas duas conversões
puras descartadas e cinco bindings sem uso nos patterns de desenho. O inventário
Clippy cai de 196 para 184; não foram adicionadas
supressões, removidos testes ou alterados contratos públicos.

## Medição anterior à decisão

HEAD 06848cdcb5450f2c555626bf116bc7acd82bd82b, working tree não commitado, UTC 2026-10-14T07:31:22.050844+00:00.
before.json.gz conserva fontes/owners, hashes, status e git diff HEAD --stat.
references-before.json.gz registra a busca de todas as referências nas camadas,
imports, visibilidade e as fontes das fachadas/suítes. Os hashes de todas as
fontes do workspace foram comparados com a suíte anterior ci-simplificacoes,
confirmando que seu inventário de 196 avisos descrevia o baseline atual.

O gate before-diagnostic-gate encontrou 12 diagnósticos selecionados e saiu 1.
Esse RED é um critério de diagnóstico, não uma regressão funcional nova.
A lista dos testes de infraestrutura foi coletada antes da edição, de
2026-10-14T07:31:22.620118+00:00 até 2026-10-14T07:31:44.232674+00:00; o mesmo comando foi executado depois,
de 2026-10-14T07:33:22.865794+00:00 a 2026-10-14T07:33:30.052672+00:00. O conjunto e a ordem dos
1294 nomes permaneceram idênticos. Listagem não substitui execução:
a suíte completa foi rodada separadamente no estado final.

## Classificação e decisões por owner

- layout_bidi::split_ltr_suffixes_page: adaptador privado usado pelos testes de
  múltiplas linhas; recebe cfg(test). O caminho produtivo continua usando
  split_ltr_suffixes_line com os índices já selecionados, sem mudar reordenação.
- shaper/shaping::try_shape: fachada pub(super) sem contexto usada pela suíte;
  recebe cfg(test). try_shape_context e shaped_width continuam produtivos.
- shaper/traversal::shape_item: adaptador pub(super) da suíte recebe cfg(test).
  shape_item_inner e shape_document continuam produtivos. A menção antiga do
  L0 a shape_item público foi corrigida pelo fato observado: traversal é módulo
  privado e a fachada exporta shape_document, não shape_item. A delegação
  produtiva também é descrita pelo nome atual try_shape_context.
- pipeline/paged::compile_to_paged_document_full_error: o import ancestral já
  estava sob cfg(test), e todos os callers pertencem à suíte. A definição passa
  a ter a mesma condição; os caminhos and_features/with_introspector continuam
  produtivos. A docstring deixa de atribuir ao wrapper o uso dos backends.
- world/font_session::catalog_with: adaptador privado do módulo de testes Unix;
  recebe cfg(all(test, unix)), igual ao módulo consumidor. catalog e
  catalog_with_fast permanecem normalmente compilados. Fallback, poison,
  invalidação e concorrência continuam cobertos pelas mesmas funções de teste.
- font_metrics/caches: duas chamadas text_style_to_font_variant descartadas
  foram removidas. O helper puro só constrói dados; axis_variations_for_text_style
  já executa essa conversão internamente antes de produzir o hash de eixos.
  Os campos bold/italic/weight, variações, tamanho e math das chaves continuam
  iguais; não se retirou informação do cache nem se introduziu aliasing novo.
- export/stream/draw: patterns Image top/local deixam de criar quatro bindings
  intrinsic_width/intrinsic_height sem consumidor; o .. existente cobre esses
  campos do carrier. O desenho usa width/height resolvidos, EXIF e clip como antes.
- export/stream/text/runs: apenas o binding font no teste inicial de style.math
  foi retirado. O binding usado por multifont/lookup permanece, assim como os
  critérios de compatibilidade e divisão dos runs.

Os oito owners foram atualizados antes do código e ressellados com seus oito
consumers. A visibilidade privada/pub(super), os imports condicionais e a
compilação normal demonstram ausência de callers produtivos dos adaptadores.
Nenhuma API externamente alcançável foi removida. cfg(test) delimita suporte da
suíte; não desativa casos nem funciona como allow(dead_code). Não houve mudança
de fase, campo de entidade, default, dependência ou algoritmo de render.

## Validação final

Gate de diagnósticos após a mudança: zero das 12 ocorrências selecionadas.
Clippy completo: 196 antes, 184 depois; delta
{'dead_code': -5, 'unused_variables': -7}. A coleta é cap-lints warn para enumerar tudo, não aprovação
do CI -D warnings. Arquitetura: 0 erros,
382 warnings e 1631
notas; nenhum aviso novo.

Build, all-targets, rustdoc -D warnings, fmt, diffcheck, linhagem V1/V5/V15/V26
estrita e linhagem reversa passaram. Suíte completa entre 2026-10-14T07:33:48.599841+00:00 e
2026-10-14T07:39:09.879755+00:00: 8455 testes passados, 0 falhas,
7 ignorados. Fontes estáveis durante cada execução; workspace-pre
registra hashes de todas as fontes, comando, estado e ambiente /dev/shm.
Os fixtures que exigem vanilla usaram o pin
9f2b6e8715237cb086899a42873660fe744622e8, árvore
6ab8bb9434b1bd4d44e1a10fbf1e407b3b86a1a4, receita P1489, SHA-256 do binário
7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea.
Este lote não constitui uma nova medição de paridade completa.

## Pendências e limites

Restam 184 avisos Clippy, além dos avisos arquiteturais. Outros
casos de dead_code não foram apagados por associação: subset/gradiente e campos
de retenção ou observação precisam de análise própria. O defeito de cobertura
conic já medido continua aberto; a retirada de bindings de Image não o corrige.
As frentes funcionais e matrizes remotas CI/release continuam no objetivo geral.
A proposta pública de Raw segue aguardando confirmação ADR-0127; não foi aplicada.

Nenhum commit, push, PR, publicação ou mudança do pin de CI. evidence.json.gz
preserva fontes/owners, referências, listas, scripts, logs, diff e relatórios
com base64/SHA-256 verificado após releitura. Contagem de testes ou ausência de
novos avisos não é certificação de produção nem de paridade geral.
