# Experimento 5 — snapshots por compilação

Resultado: os controles de captura/compartilhamento passaram, mas **a variante eager de bytes próprios por compilação não mostrou ganho marginal sobre a retenção já existente modelada**. Fez o mesmo número de provas e aumentou leitura, retenção e RSS na fixture. Isso não rejeita toda forma de snapshot, especialmente captura lazy; rejeita recomendar esta cópia completa como otimização com base nesta rodada. Nenhuma otimização foi integrada.

## Método e contrato experimental

Skill tekt-materializacao-segregada: A implementou modelo/bancada; B escreveu testes independentes a partir da mesma especificação sem consultar A; C integrou, compilou, executou e aplicou mutantes. Rust 1.92.0, std-only, rustc --edition=2021 --test para testes e rustc -O para bancada. Todos fontes temporários, builds, executáveis, fixtures, logs e mutantes em /dev/shm/tekt-e5-20261104; TMPDIR também na RAM. Não houve Cargo/dependências externas ou recompilação do produto.

capture faz duas observações completas e ordenadas, exige igualdade exata de configuração, sources, fontes, identidade Unix de oito campos, bytes e associações de face/família. Diferença/erro/recusa de orçamento retorna None, sem truncar. Snapshot contém bytes próprios e epoch, sem cache global. Epoch identifica a captura, não prova frescor.

Compilation mantém um único Arc por objeto e serializa inicialização com Mutex. Primeira captura bem-sucedida é compartilhada pelos consumidores; falha não é retida e pode ser tentada novamente. Novo objeto Compilation recaptura mesmo com a mesma epoch. Alterações posteriores à captura não mudam bytes próprios de Worlds antigos; tampouco obrigam consumidores do mesmo objeto a ver um novo estado. A fronteira observada é uma compilação nova.

Orçamento soma bytes de config, sources e fontes, com checked_add; não limita estruturas, nomes ou Arcs externos. Aplica-se à captura inicial, não a uma mudança de orçamento passada por consumidor após o Arc já instalado. Retenção de muitos Worlds ativos requer orçamento global próprio. Duas observações iguais não garantem atomicidade de varredura do filesystem ou ausência de mudança depois da última leitura.

## Correção e adversários

39 testes independentes passaram: duas observações, todas dimensões de identidade e conteúdo, ordem/duplicatas, configuração, sources, associação de faces, mudanças no par catálogo/payload, orçamento e recusa sem truncar, falha/reparo sem negativo persistente, objetos isolados, mesmoepoch recapturando, old Arc e oito consumidores concorrentes. Overflow de soma é tratado no código, mas não foi produzido por alocações impossíveis nos testes; usize::MAX foi exercitado com dados pequenos.

21 mutantes compilados foram rejeitados, 21/21: omitir configuração, bytes/path de sources e fontes, índice/família de face, ordem de fontes/sources e cada um dos oito campos Unix; omitir segunda observação ou orçamento; retirar retenção por compilação; usar cache global entre compilações. Nenhum sobrevivente. Isso testa os erros escolhidos, não certifica uma prova formal de filesystem.

A bancada real usa um diretório plano na RAM com NotoSans-Regular.ttf e DejaVuSans.ttf do repositório, main.typ e config. Em cada passe enumera os paths ordenados, observa lstat/stat da raiz e entradas, e lê configuração/source. A variante owned lê também todos os bytes de fontes. A prova lexical/identidades entra no conteúdo de config do modelo para comparação completa. Não usa fontdb/fontconfig nem parser: family/index são descritores opacos de fixture, não FontBook real.

Seis mudanças reais entre os passes foram recusadas: configuração, source, adição, remoção, substituição com mtime restaurado e symlink retarget. Um sétimo caso injetou erro retornando None. O mesmoepoch em uma captura nova reconheceu bytes alterados após uma captura antiga, que permaneceu imutável. Oito threads sincronizadas por barreira receberam o mesmo Arc e executaram somente duas observações no total. Sem sleeps para simular corridas; os writers agem no limite controlado entre os passes, não representam todos os interleavings durante uma varredura.

## Benefício marginal e baseline

O baseline respeita o lifecycle atual: FontCatalogSession retém par catálogo/slots por Arc entre Worlds com prova igual; cada World tem aquisição OnceLock, e o recurso usado é carregado uma vez por slot. Em três compilações estáveis, três Worlds ativos compartilham um catálogo e uma carga de bytes da fonte usada. Não se compara contra uma implementação fictícia que revalida por consumidor.

O candidato também captura uma vez por compilação, mas copia todas as fontes em cada uma das três compilações. Foram três rodadas intercaladas por carga (ordem alternada), com 1/10/100 consumidores por compilação e afinidade CPU0. Cada linha abaixo agrega três compilações, com três Worlds mantidos ativos. Tempos são medianas descritivas de três amostras, incluem igualdade/clones/assertions da bancada; não são timings de CLI/watch ou do compilador completo.

|Consumidores por compilação|Variante|Tempo mediano ms|Capturas/passes|Metadata|Reads|Bytes lidos|Bytes fontes retidos|
|---:|---|---:|---|---:|---:|---:|---:|
|1|current-shape|0.285964|3/6|60|13|556384|556216|
|1|owned-snapshot|2.160441|3/6|60|24|7895784|3947808|
|10|current-shape|0.314788|3/6|60|13|556384|556216|
|10|owned-snapshot|2.148318|3/6|60|24|7895784|3947808|
|100|current-shape|0.296886|3/6|60|13|556384|556216|
|100|owned-snapshot|2.131656|3/6|60|24|7895784|3947808|

O baseline teve uma carga lazy do recurso usado no total, inclusive através das três compilações; o candidato manteve três catálogos/snapshots distintos e todas as fontes. Nenhum dos dois multiplicou captura/prova quando os consumidores passaram de um para100. Portanto o compartilhamento intracompilação do snapshot não elimina trabalho adicional neste baseline: essa retenção já existe.

RSS pico em processos separados, três processos por variante, com as mesmas três rodadas e cargas, pulando adversários para não contaminar o baseline com o caminho owned:

|Variante|RSS KiB nas três execuções|Mediana KiB|
|---|---|---:|
|current-shape|3192, 3104, 3188|3188|
|owned-snapshot|7412, 7456, 7328|7412|

O RSS agrega buffers temporários, runtime, snapshots e demais estruturas do processo; não é contador exato dos payloads. O orçamento de captura não limita esse pico. Estes dados são específicos das duas fontes e três Worlds ativos. Se Worlds antigos forem liberados antes, retenção muda; um snapshot lazy pode ter outro perfil, que não foi implementado aqui. Não se extrapola um speedup ou uma regressão percentual do produto.

## Limites e conclusão aplicável

A prova da fixture enumera o diretório por passe. Ela **não replica** a prova rápida produtiva com inventário retido nem toda configuração/raízes/negativos/fontconfig/FS suportados. O baseline é um modelo de lifecycle do produto, não execução do FontCatalogSession real nem equivalência de suas syscalls. Bytes de recurso no baseline são Vec lazy para contabilização, enquanto o produto pode usar mmap; nenhuma conclusão de imutabilidade do mmap foi obtida. SourceSession/CST e eval/layout não foram modelados.

O resultado positivo é de coerência e compartilhamento do objeto capturado sob as observações controladas. Não valida seleção, cobertura, shaping, render, diagnósticos ou todas as entradas externas de uma compilação real. Fontes inválidas/reparadas são tratadas como falha/reparo do observador no modelo; não se executou parser de fontes inválidas nesta rodada.

Para performance, a hipótese eager testada fica desfavorável: preservou o número de verificações e pagou cópia/retenção de recursos não utilizados. Uma epoch isolada não permite retirar validação entre compilações, e reaproveitar provas entre fases não foi autorizado. O L0 de font_session já exige par catálogo/slots, uma geração futura, Worlds antigos válidos e reconhecimento na próxima recompilação; nenhuma dessas regras foi alterada.

Não integrar esta variante como otimização. Um estudo futuro de snapshot lazy precisaria apontar consultas realmente repetidas além de OnceLock/Arc existentes, definir a autoridade de captura e o orçamento de Worlds ativos, preservar proof/postcheck e medir o compilador real. A captura de bytes próprios pode ter utilidade para um contrato de isolamento, mas isso seria uma decisão de produto/arquitetura própria, não um ganho presumido nem uma autorização desta bancada.

## Proveniência e preservação

HEAD 3c6b0ce7b7632cc74b004cea6681e4746e63d28d, working tree não commitado. Registro inicial UTC 2026-11-04T18:40:15.348824+00:00; conferência final UTC 2026-11-04T18:46:39.171286+00:00. Os 2413 arquivos registrados de fontes/L0/diagnósticos permanecem byte a byte iguais. Única adição persistente: este diagnóstico. Todo material experimental é temporário na RAM e removido no encerramento conforme o skill.

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
