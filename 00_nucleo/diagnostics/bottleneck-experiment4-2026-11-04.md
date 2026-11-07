# Experimento 4 — fronteira de memoização para imports

Resultado: **memoização funcionou nos controles de uma entrada isolada com imports puros sob World congelado e chave completa**, incluindo replay da projeção de diagnósticos. Layout foi executado e analisado separadamente, sem cache. Nenhuma otimização, fonte produtiva ou L0 foi alterado.

## Fronteira e método

Skill tekt-materializacao-segregada: A escreveu modelo/bancadas; B escreveu 37 testes independentes a partir da mesma especificação sem consultar A; C integrou, compilou, executou e aplicou mutantes. Todos fontes, caches, targets, executáveis, logs e mutantes descartáveis em /dev/shm/tekt-e4-20261104. Rust 1.92.0, Cargo offline/release, um job, MemoryMax=12GiB/MemoryHigh=8GiB/MemorySwapMax=0; core compilado em 5m54s e bancada em 32s. Dependências do lockfile atual, comemo 0.4.0. Primeira resolução offline sem lock escolheu zerocopy-derive indisponível; repetição com lock do repositório passou.

O modelo std usa chave exata e cache por instância com limite por quantidade de entradas e bypass para chamadas não elegíveis. A chave inclui namespace World, root, lista ordenada de paths/bytes de todos os arquivos do snapshot, inputs ordenados, features, target, full_error, namespace host, época de fontes e route. Não descobre dependências automaticamente; assume o World completamente capturado e estável. Datas, plugins reais, fontes mutáveis e qualquer capacidade externa não capturada ficam fora do caso elegível.

A bancada real liga o core produtivo somente para leitura e chama a API pública eval_with_full_error_target_and_features em FrozenWorld sem filesystem. main.typ importa expensive.typ, que importa leaf.typ e calcula um loop de 10.000 iterações. A função #[comemo::memoize] recebe a chave completa e retorna **uma projeção escalar do binding result e vetores de diagnósticos formatados**, não um Module reutilizável. A fronteira engloba a avaliação da entrada isolada com seus imports; **não é um patch/cache do eval_imported_file privado**, nem reúso de closures/Engine/caller scopes.

Source/CST retidos por pool (FileId,texto), sem cache negativo. O contador parse_ns mede Source::new completo, incluindo suas operações auxiliares, não parser exclusivo. Pool global de bancada e cache comemo entram no RSS; não são políticas de produto. Key contém o snapshot inteiro, inclusive arquivos não consultados, portanto é conservadora e pode invalidar mais que o necessário. Não se mediu custo de obter esse snapshot a partir do filesystem.

Chamadas frescas e com memo são comparadas na ordem fresh→primeira memo→segunda memo, em três rodadas com evict(0) entre elas; sem randomização, poucas amostras. Source é pré-aquecido em controles fora das linhas temporais. Contadores separam execução real do wrapper, chamadas include_source e construções Source. Exige igualdade completa do Record e zero execução do wrapper na segunda chamada memo. Formatação Debug de diagnósticos é um controle interno para conferir campos, spans, ordem e multiplicidade; não é critério universal de paridade de linguagem.

## Resultados de imports reais

45 comparações fresh/primeira memo/segunda memo, 135 registros temporais. Todas passaram. Valores esperados do controle puro: 49.995.001; alteração transitiva: 49.995.002; alteração de sys.inputs: 49.995.003. Em hits quentes: zero execução de eval e zero include_source.

|Caso|Fresh mediana ms|Primeira memo ms|Segunda memo µs|Execuções na primeira memo, por rodada|Erros/avisos retornados|
|---|---:|---:|---:|---|---|
|pure-import|6.843295|6.942661|0.561|1,1,1|0/0|
|transitive-edit|6.779556|6.923535|0.521|1,1,1|0/0|
|inputs-edit|6.812728|6.921131|0.581|1,1,1|0/0|
|missing-transitive|1.507488|1.476479|0.491|1,1,1|1/0|
|repaired-transitive|6.713372|0.000601|0.411|0,0,0|0/0|
|warning-replay|6.881537|6.993586|0.752|1,1,1|0/1|
|cycle-error|2.191571|2.192091|0.581|1,1,1|1/0|
|syntax-error|1.484835|1.495986|0.561|1,1,1|1/0|
|foreign-world|6.681783|6.951789|0.802|1,1,1|0/0|
|host-namespace|6.840279|6.976545|0.711|1,1,1|0/0|
|font-epoch|6.776950|6.998346|0.531|1,1,1|0/0|
|route-context|6.917894|6.971075|0.612|1,1,1|0/0|
|full-error|6.761031|6.946108|0.511|1,1,1|0/0|
|html-target|6.796157|6.965163|0.551|1,1,1|0/0|
|feature-mask|6.836743|6.913046|0.521|1,1,1|0/0|

A primeira memo de repaired-transitive já é hit: restaura exatamente a chave original ainda retida. Não chamá-la de miss frio. Mudanças de World/host/font_epoch/route são namespaces adicionais que forçam miss; esses casos não executam um plugin ou fonte real e route permanece raiz no avaliador da bancada. HTML/features são passados à API real, mas o corpo puro não usa operações específicas desses targets.

Erros de arquivo ausente, ciclo e sintaxe, e um aviso this import has no effect, foram devolvidos em cada demanda conforme fresh. O wrapper armazena e retorna a projeção dos diagnósticos: **não implementa replay de SourceDiagnostic dentro do Sink do chamador produtivo**. A integração ainda precisaria preservar sink, traces, spans, rota e efeitos sem supressão ou duplicação.

O processo completo registrou 108 execuções reais, 225 include_source, sete construções de Source e 261.521 ns de Source::new. Inclui warmups e avaliações auxiliares de expectativas além dos 135 registros. RSS pico 14748 KiB para o processo completo, não um orçamento de cache provado. O controle puro fresh fez duas resoluções de import, enquanto o hit fez zero. O custo do hash/lookup da pequena chave RAM está dentro dos tempos memo; aquisição/validação do World externo está excluída. Não há intervalo de confiança ou alegação de speedup CLI/watch.

## Testes independentes e mutantes

37 testes passaram: cada dimensão da chave, root/import/transitivo alterado, aparecimento/remoção de arquivo, ordens distintas, vazio versus ausência de input, erro corrigido, elegibilidade, replay de ordem/duplicatas de diagnósticos, clone retornado alterado sem afetar cache, quotas e isolamento.

17 mutantes compilados foram rejeitados, 17/17: omitir cada um dos dez campos da Key; ignorar transitivo; ignorar elegibilidade no hit ou na inserção; omitir warnings ou errors no replay; ignorar quota; contar hit como execução. Essa verificação é do modelo de contrato, não mutation testing de todo comemo/core. O cache real tem retenção/eviction próprios de comemo; os testes do limite de entradas do modelo não certificam um limite equivalente no comemo.

## Layout separado

O primeiro controle font_size=10 versus40 gerou sete páginas em ambos os casos e falhou na hipótese. Leitura confirmou que Layouter::new recebe _font_size e inicializa o estilo por TextStyle::from(StyleChain::default_chain) em layout/mod.rs:973–994; variar esse argumento não era um estímulo efetivo para a fixture. Nenhuma expectativa foi afrouxada.

Controle corrigido: mesmo Content, mesmo estilo, 4.000 palavras; um FontMetrics delega as métricas a FixedMetrics e varia somente advance por fator um ou três, com contagem de chamadas. Resultado final:

```text
advance_scale=1 pages=7 layout_ns=16459178 words=4000 advance_calls=12000 no_memo=true
advance_scale=3 pages=22 layout_ns=9707919 words=4000 advance_calls=12000 no_memo=true
```

Cada saída preservou as mesmas 4.000 palavras e sua ordem, com sete versus22 páginas e 12.000 chamadas reais de advance em cada execução. Não houve cache ou import nesse executável. Mostra uma dependência efetiva de layout independente da chave pura de eval, **não certifica suficiência de qualquer chave de layout**. Não se compararam velocidades dos dois layouts; as durações isoladas não têm controle de aquecimento. Não se reaproveitou resultado entre os seis candidatos de outline do experimento 1.

O fonte de convergence.rs também recebe introspector, métricas, sizer, tamanho, callbacks e seed; o caminho planejado mantém transações de running-content por tentativa. Imports memoizados não autorizam memoizar candidatos por Content apenas. Regiões, estilos, fontes, introspecção, locator, callbacks e rollback exigiriam contrato e experimento próprios.

## Decisão e lacunas

A fronteira pura e congelada é candidata para estudo adicional: evitar avaliação repetida de imports caros pode ser material nesse domínio isolado. Para aproximar do produto, primeiro identificar uma unidade de retorno que preserve Module/bindings/closures e diagnósticos; rastrear todas as consultas de World ou congelar entradas de fato; validar mutações transitivas, host/inputs/features/target, erros e rota no caller real; medir aquisição, validação, memória e replay incluídos em CLI/watch. Contar atributos, eviction ou chamadas não substitui hits genuínos.

Este experimento não demonstra que Engine.world atual está integralmente rastreado por comemo, não cria TrackedWorld nem move fases. A função privada de import preserva isolamento e sink conforme L0 modules.md; ela não foi modificada. ADR-0030/L1/globals e o L0 da unidade continuam exigindo decisão específica antes de qualquer integração. Sem ganho de produto presumido, sem cache de layout, sem otimização integrada.

## Proveniência e preservação

HEAD 3c6b0ce7b7632cc74b004cea6681e4746e63d28d, working tree não commitado. Registro inicial UTC 2026-11-04T18:23:54.457128+00:00; conferência final UTC 2026-11-04T18:38:02.397578+00:00. Os 2412 arquivos registrados de fontes/L0/diagnósticos permaneceram byte a byte iguais. A única adição persistente é este diagnóstico. Material experimental é descartável na RAM, removido no encerramento conforme o skill.

Uma repetição do build de layout iniciada de outro diretório herdou configuração diferente e começou recompilar dependências; foi interrompida. A repetição no mesmo diretório/configuração RAM do build inicial passou, e o resultado corrigido só foi aceito após conferir a saída nova. Nenhuma configuração do repositório foi alterada.

<details><summary>git diff HEAD --stat=200 exato do estado medido</summary>

```text
 00_nucleo/prompts/core/compiler/eval/bindings/field_access.md                 |  19 +++-
 00_nucleo/prompts/core/compiler/eval/bindings/field_access/content_methods.md |  51 +++++++++-
 00_nucleo/prompts/core/compiler/eval/bindings/field_access/fields.md          |  30 +++++-
 00_nucleo/prompts/core/compiler/eval/observation/elements.md                  |  63 ++++++++++++-
 00_nucleo/prompts/core/compiler/eval/repr/content.md                          |  30 +++++-
 00_nucleo/prompts/core/compiler/eval/rules/set.md                             |  41 +++++++-
 00_nucleo/prompts/core/compiler/eval/rules/set/text.md                        |  42 ++++++++-
 00_nucleo/prompts/core/compiler/eval/tests.md                                 |  29 +++++-
 00_nucleo/prompts/core/compiler/introspect.md                                 |  19 +++-
 00_nucleo/prompts/core/compiler/introspect/convergence.md                     |  19 +++-
 00_nucleo/prompts/core/compiler/introspect/figure_demands.md                  |  19 +++-
 00_nucleo/prompts/core/compiler/introspect/materialize_time.md                |  32 ++++++-
 00_nucleo/prompts/core/compiler/introspect/page_running.md                    |  23 ++++-
 00_nucleo/prompts/core/compiler/introspect/walk.md                            |  16 +++-
 00_nucleo/prompts/core/compiler/layout/bidi_origin.md                         |  34 ++++++-
 00_nucleo/prompts/core/compiler/layout/block.md                               |  34 ++++++-
 00_nucleo/prompts/core/compiler/layout/checkpoint.md                          |  61 +++++++++++-
 00_nucleo/prompts/core/compiler/layout/columns.md                             |  60 +++++++++++-
 00_nucleo/prompts/core/compiler/layout/columns/flow.md                        |  18 +++-
 00_nucleo/prompts/core/compiler/layout/columns/segmented.md                   |  34 ++++++-
 00_nucleo/prompts/core/compiler/layout/columns/transitions.md                 |  16 +++-
 00_nucleo/prompts/core/compiler/layout/cursor.md                              |  40 +++++++-
 00_nucleo/prompts/core/compiler/layout/equation.md                            |  34 ++++++-
 00_nucleo/prompts/core/compiler/layout/figure.md                              |  32 ++++++-
 00_nucleo/prompts/core/compiler/layout/flush.md                               |  19 +++-
 00_nucleo/prompts/core/compiler/layout/footnote.md                            |  39 +++++++-
 00_nucleo/prompts/core/compiler/layout/footnote_flush.md                      |  24 ++++-
 00_nucleo/prompts/core/compiler/layout/footnote_fragments.md                  |  31 +++++-
 00_nucleo/prompts/core/compiler/layout/grid/cell_emission.md                  |  95 ++++++++++++++++++-
 00_nucleo/prompts/core/compiler/layout/grid/tracks.md                         |  81 +++++++++++++++-
 00_nucleo/prompts/core/compiler/layout/helpers.md                             |  10 +-
 00_nucleo/prompts/core/compiler/layout/image.md                               |  58 +++++++++++-
 00_nucleo/prompts/core/compiler/layout/measurement.md                         |  19 +++-
 00_nucleo/prompts/core/compiler/layout/mod.md                                 |  34 ++++++-
 00_nucleo/prompts/core/compiler/layout/page_run.md                            |  34 ++++++-
 00_nucleo/prompts/core/compiler/layout/page_running_requests.md               |  53 ++++++++++-
 00_nucleo/prompts/core/compiler/layout/paragraph.md                           |  34 ++++++-
 00_nucleo/prompts/core/compiler/layout/paragraph_indent.md                    |  34 ++++++-
 00_nucleo/prompts/core/compiler/layout/place.md                               |  32 ++++++-
 00_nucleo/prompts/core/compiler/layout/placement.md                           |  34 ++++++-
 00_nucleo/prompts/core/compiler/layout/references.md                          |  48 +++++++++-
 00_nucleo/prompts/core/compiler/layout/region_progress.md                     |  42 ++++++++-
 00_nucleo/prompts/core/compiler/layout/repeat.md                              |  34 ++++++-
 00_nucleo/prompts/core/compiler/layout/set_page.md                            |  34 ++++++-
 00_nucleo/prompts/core/compiler/layout/shape.md                               |  37 +++++++-
 00_nucleo/prompts/core/compiler/layout/slicing.md                             |  10 +-
 00_nucleo/prompts/core/compiler/layout/stack.md                               |  69 +++++++++++++-
 00_nucleo/prompts/core/compiler/layout/sub_frame.md                           |  17 +++-
 00_nucleo/prompts/core/compiler/layout/text/style.md                          |  34 ++++++-
 00_nucleo/prompts/core/compiler/layout/tiling.md                              |  37 +++++++-
 00_nucleo/prompts/core/compiler/math/layout/delimited.md                      |  80 +++++++++++++++-
 00_nucleo/prompts/core/compiler/math/layout/external_bridge.md                |  34 ++++++-
 00_nucleo/prompts/core/compiler/math/layout/mod.md                            |  81 +++++++++++++++-
 00_nucleo/prompts/core/compiler/math/layout/op.md                             |  52 +++++++++-
 00_nucleo/prompts/core/compiler/realize/figure.md                             |  32 ++++++-
 00_nucleo/prompts/core/compiler/realize/figure_kind.md                        |  19 +++-
 00_nucleo/prompts/core/compiler/stdlib/figure_image.md                        |  87 ++++++++++++++++-
 00_nucleo/prompts/core/compiler/stdlib/tests/figure_image.md                  |  65 ++++++++++++-
 00_nucleo/prompts/core/compiler/stdlib/text/constructor.md                    |  42 ++++++++-
 00_nucleo/prompts/core/compiler/stdlib/visualize.md                           |  37 +++++++-
 00_nucleo/prompts/core/entities/content/construction.md                       |  70 +++++++++++++-
 00_nucleo/prompts/core/entities/content/fields.md                             |  30 +++++-
 00_nucleo/prompts/core/entities/content/tests/traversal.md                    |  19 +++-
 00_nucleo/prompts/core/entities/content/traversal.md                          |  20 +++-
 00_nucleo/prompts/core/entities/elements/figure.md                            |  20 +++-
 00_nucleo/prompts/core/entities/elements/image.md                             |  64 ++++++++++++-
 00_nucleo/prompts/core/entities/image_format.md                               |  26 ++++-
 00_nucleo/prompts/core/entities/image_sizer.md                                |  26 ++++-
 00_nucleo/prompts/core/entities/layout_types.md                               |  31 +++++-
 00_nucleo/prompts/core/entities/mod.md                                        |  24 ++++-
 00_nucleo/prompts/core/entities/tiling.md                                     |  37 +++++++-
 00_nucleo/prompts/infra/export/builder.md                                     |  15 +++
 00_nucleo/prompts/infra/export/builder/resources.md                           |  15 ++-
 00_nucleo/prompts/infra/export/images.md                                      |  31 +++++-
 00_nucleo/prompts/infra/export/images/png.md                                  |  20 +++-
 00_nucleo/prompts/infra/export/images/scan.md                                 |  31 +++++-
 00_nucleo/prompts/infra/export/render.md                                      |  36 ++++++-
 00_nucleo/prompts/infra/export/stream/draw.md                                 |  10 +-
 00_nucleo/prompts/infra/export/svg/images.md                                  |  19 +++-
 00_nucleo/prompts/infra/export/svg/tests.md                                   |  10 +-
 00_nucleo/prompts/infra/export/tests/mod.md                                   |  27 +++++-
 00_nucleo/prompts/infra/lib.md                                                |  17 +++-
 00_nucleo/prompts/infra/pipeline/context_stabilization.md                     |  19 +++-
 00_nucleo/prompts/infra/pipeline/paged.md                                     |  17 +++-
 00_nucleo/prompts/infra/pipeline/tests.md                                     | 125 +++++++++++++++++++++++-
 00_nucleo/prompts/infra/shaper/bidi.md                                        |  37 +++++++-
 00_nucleo/prompts/infra/shaper/shaping.md                                     |  35 ++++++-
 00_nucleo/prompts/infra/tests/svg_composite_graph.md                          |  10 +-
 00_nucleo/prompts/shell/cli.md                                                |  19 +++-
 01_core/src/compiler/eval/bindings/field_access.rs                            |   4 +-
 01_core/src/compiler/eval/bindings/field_access/content_methods.rs            |  21 ++++-
 01_core/src/compiler/eval/bindings/field_access/fields.rs                     |   6 +-
 01_core/src/compiler/eval/observation/elements.rs                             |  34 ++++---
 01_core/src/compiler/eval/repr/content.rs                                     |  12 ++-
 01_core/src/compiler/eval/rules/set.rs                                        |  54 ++++++++++-
 01_core/src/compiler/eval/rules/set/text.rs                                   |  42 ++++++++-
 01_core/src/compiler/eval/tests.rs                                            |   4 +-
 01_core/src/compiler/introspect.rs                                            |   4 +-
 01_core/src/compiler/introspect/convergence.rs                                |   4 +-
 01_core/src/compiler/introspect/figure_demands.rs                             |   4 +-
 01_core/src/compiler/introspect/materialize_time.rs                           |  34 ++++++-
 01_core/src/compiler/introspect/page_running.rs                               |  68 +++++++++++--
 01_core/src/compiler/introspect/walk.rs                                       |  69 +++++++++++++-
 01_core/src/compiler/layout/bidi_origin.rs                                    |   7 +-
 01_core/src/compiler/layout/block.rs                                          |   9 +-
 01_core/src/compiler/layout/checkpoint.rs                                     |  17 +++-
 01_core/src/compiler/layout/columns.rs                                        |  32 ++++---
 01_core/src/compiler/layout/columns/flow.rs                                   |  58 +++++++++++-
 01_core/src/compiler/layout/columns/segmented.rs                              |  30 ++----
 01_core/src/compiler/layout/columns/transitions.rs                            |  72 +++++++++++++-
 01_core/src/compiler/layout/cursor.rs                                         |  75 ++++++++++++++-
 01_core/src/compiler/layout/equation.rs                                       |   9 +-
 01_core/src/compiler/layout/figure.rs                                         | 240 ++++++++++++++++++++++++++++++++++++++++++++--
 01_core/src/compiler/layout/flush.rs                                          |   3 +-
 01_core/src/compiler/layout/footnote.rs                                       | 171 ++++++++++++++++++++++++++++++++-
 01_core/src/compiler/layout/footnote_flush.rs                                 |  71 +++++++++++++-
 01_core/src/compiler/layout/footnote_fragments.rs                             |  99 +++++++++++++++++--
 01_core/src/compiler/layout/grid/cell_emission.rs                             | 167 +++++++++++++++++++++++++++++---
 01_core/src/compiler/layout/grid/tracks.rs                                    | 114 ++++++++++++++++++++--
 01_core/src/compiler/layout/helpers.rs                                        |   5 +-
 01_core/src/compiler/layout/image.rs                                          |  80 +++++++++++++---
 01_core/src/compiler/layout/measurement.rs                                    |   4 +-
 01_core/src/compiler/layout/mod.rs                                            |   6 +-
 01_core/src/compiler/layout/page_run.rs                                       |  16 +---
 01_core/src/compiler/layout/page_running_requests.rs                          |  79 +++++++++++++++-
 01_core/src/compiler/layout/paragraph.rs                                      |  78 +++++++++++++--
 01_core/src/compiler/layout/paragraph_indent.rs                               |   9 +-
 01_core/src/compiler/layout/place.rs                                          |  67 +++++++++++--
 01_core/src/compiler/layout/placement.rs                                      |   9 +-
 01_core/src/compiler/layout/references.rs                                     | 141 ++++++++++++++++++++++++++-
 01_core/src/compiler/layout/region_progress.rs                                |  86 ++++++++++++++++-
 01_core/src/compiler/layout/repeat.rs                                         |   7 +-
 01_core/src/compiler/layout/set_page.rs                                       |  16 +---
 01_core/src/compiler/layout/shape.rs                                          |   7 +-
 01_core/src/compiler/layout/slicing.rs                                        |   4 +-
 01_core/src/compiler/layout/stack.rs                                          |  17 +++-
 01_core/src/compiler/layout/sub_frame.rs                                      |  57 ++++++++++-
 01_core/src/compiler/layout/text/style.rs                                     |  11 ++-
 01_core/src/compiler/layout/tiling.rs                                         |  12 ++-
 01_core/src/compiler/math/layout/delimited.rs                                 | 211 ++++++++++++++++++++++++++++++++++++++++-
 01_core/src/compiler/math/layout/external_bridge.rs                           |  10 +-
 01_core/src/compiler/math/layout/mod.rs                                       |  10 +-
 01_core/src/compiler/math/layout/op.rs                                        |  91 +++++++++++++++++-
 01_core/src/compiler/realize/figure.rs                                        |  76 ++++++++++++++-
 01_core/src/compiler/realize/figure_kind.rs                                   |   6 +-
 01_core/src/compiler/stdlib/figure_image.rs                                   | 114 +++++++++++++++++-----
 01_core/src/compiler/stdlib/tests/figure_image.rs                             | 282 +++++++++++++++++++++++++++++++++++++++++++++++++++++-
 01_core/src/compiler/stdlib/text/constructor.rs                               |  20 +++-
 01_core/src/compiler/stdlib/visualize.rs                                      |   7 +-
 01_core/src/entities/content/construction.rs                                  |   9 +-
 01_core/src/entities/content/fields.rs                                        |   4 +-
 01_core/src/entities/content/tests/traversal.rs                               |   4 +-
 01_core/src/entities/content/traversal.rs                                     |   4 +-
 01_core/src/entities/elements/figure.rs                                       |  79 +++++++++++++++-
 01_core/src/entities/elements/image.rs                                        |  21 ++++-
 01_core/src/entities/image_format.rs                                          |   9 +-
 01_core/src/entities/image_sizer.rs                                           |  13 ++-
 01_core/src/entities/layout_types.rs                                          |   4 +-
 01_core/src/entities/mod.rs                                                   |   4 +-
 01_core/src/entities/tiling.rs                                                |   7 +-
 02_shell/src/cli.rs                                                           |   4 +-
 03_infra/Cargo.toml                                                           |   7 ++
 03_infra/src/export/builder.rs                                                |   2 +-
 03_infra/src/export/builder/resources.rs                                      |   7 +-
 03_infra/src/export/images.rs                                                 |   7 +-
 03_infra/src/export/images/png.rs                                             |  93 +++++++++++++++++-
 03_infra/src/export/images/scan.rs                                            |  27 +++++-
 03_infra/src/export/render.rs                                                 |  58 +++++++++++-
 03_infra/src/export/stream/draw.rs                                            |   3 +-
 03_infra/src/export/svg/images.rs                                             |   8 +-
 03_infra/src/export/svg/tests.rs                                              |   4 +-
 03_infra/src/export/tests/mod.rs                                              |  15 ++-
 03_infra/src/lib.rs                                                           |   4 +-
 03_infra/src/pipeline/context_stabilization.rs                                |   4 +-
 03_infra/src/pipeline/paged.rs                                                |   4 +-
 03_infra/src/pipeline/tests.rs                                                | 593 +++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++-
 03_infra/src/shaper/bidi.rs                                                   |  78 ++++++++++++++-
 03_infra/src/shaper/shaping.rs                                                |  14 +--
 03_infra/tests/svg_composite_graph.rs                                         |   3 +-
 Cargo.lock                                                                    | 275 +++++++++++++++++++++++++++++++++++++++++++++++++++--
 180 files changed, 7181 insertions(+), 437 deletions(-)
```

</details>
