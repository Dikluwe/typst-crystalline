# Validação de read_at no pipeline de laboratório

**Resultado:** o candidato preservou os controles funcionais e apresentou redução de CPU na CLI e no startup do watch nesta bancada. No texto pequeno, o ganho pareado mediano foi4,037 ms CPU na CLI e4,586 ms no startup. Ele elimina seeks físicos da aquisição seletiva; não altera ou reduz a prova de frescor dos ciclos quentes. Nenhuma otimização foi integrada ao produto.

## Cópia de laboratório e mudança executada

Skill tekt-materializacao-segregada: A escreveu a implementação/runner; B escreveu testes independentes sem consultar A; C integrou, compilou, executou e verificou resultados. Código experimental, testes, fixtures, executáveis, caches e mutantes ficaram em /dev/shm/tekt-readat-pipeline-20261104. A cópia de workspace é quarentena descartável, não worktree produtivo nem import de lab para as camadas ativas.

Somente selective.rs mudou na cópia: módulo privado Unix PositionalReader e dispatch da aquisição File para ele. O projetor genérico Read+Seek/read_exact, caps, reconstrução sfnt/TTC, FontInfo, coverage e fallback não mudaram. A verificação mecânica mostrou que remover adaptador/dispatch recupera o original normalizado e que o adaptador testado standalone coincide com o usado no pipeline.

O backend File usa FileExt::read_at com cursor privado; avança pelo número realmente retornado, preserva posição em erro e permite read_exact repetir Interrupted e completar leituras curtas. EOF prematuro não vira bytes zero nem sucesso. Arithmetic de cursor/seek permanece checked. SeekFrom::End usa o tamanho capturado; o projetor utiliza apenas Start, portanto não se generaliza esse contrato para consumidores que esperam tamanho externo atualizado durante a leitura.

Os guards continuam no lugar original: antes da projeção e depois de FontInfo e coverage, comparando handle e caminho. Nenhum cache de identidade/bytes entre faces, recurso retido, novo flag, dependência, unsafe ou mudança de contrato público foi introduzido. Não-Unix continua recusando esse fast path; SharedFile/Binary seguem os caminhos anteriores.

## Validação funcional no código real

21 testes de transporte independentes passaram standalone e foram incluídos temporariamente dentro do crate real da cópia para usar o adaptador efetivo. Cobrem leituras parciais, Interrupted repetido, EOF, outros erros, cursor, overflow, seeks, dois leitores e independência do cursor físico do File. Seis mutantes preparados foram rejeitados: offset errado, avançar pelo solicitado em vez do lido, avançar em erro, seek não atualizar cursor, EOF fabricando sucesso e Interrupted convertido em erro permanente.

No crate candidato, grupo fontdb:: passou com52 testes (incluindo os21 novos), um helper ignorado na execução direta e1.590 filtrados. O helper de configuração é marcado para ser invocado pelo teste pai, não foi promovido como um teste independente executado. O grupo fonts:: passou com71 testes. Esses grupos cobrem a área alterada: projeção, limites/falhas, aquisição seletiva, metadata/cobertura, fallback após recusa, retenção de SharedFile e emparelhamento slots/book. Não se executou a suíte inteira do workspace ou lint produtivo para promover uma alteração que continua em lab.

A primeira compilação de testes não encontrou16 fixtures lab referenciadas em18 include_str existentes; copiar esses inputs para a RAM resolveu o ambiente, sem alterar testes. Os testes independentes foram removidos da cópia de selective.rs após execução, recuperando exatamente a fonte candidata do executável medido; a verificação de diff foi repetida.

Os70 pares de pipeline passaram em texto integral, páginas/posições extraídas e nomes de fontes. Dez pares representativos (primeira rodada, cinco fixtures, CLI e startup watch) também tiveram rasterização idêntica de todas as páginas a72 dpi. O inventário CLI completo de fontes/variantes coincidiu. Isso valida os controles exercitados, não certifica toda a linguagem/corpus/plataformas.

Os PDFs não foram byte a byte iguais nos70 pares; isso não foi gate de paridade. No controle simples, metadados visíveis e tamanho coincidiram, enquanto /ID diferiu. Não se atribui toda diferença binária a uma causa não auditada; texto/geometry/fontes e raster acima são controles próprios.

## Medição pareada de CLI e startup do watch

Sete rodadas alternando ordem baseline/candidato em cada par, cinco fixtures, CLI em processos novos e watch novo por startup:140 execuções e70 pares. Mesma entrada/path por par, mesmas fontes/configuração do host e flags --pdf-tagged=false; afinidadeCPU0, RAYON_NUM_THREADS=1 e SOURCE_DATE_EPOCH=0. Sem instrumentação, sem ablação de fontes, sem build concorrente e sem limpar page cache.

CLI wall por communicate(timeout), evitando quantização do wait(timeout); CPU por diferença de rusage dos filhos, incluindo wrapper time. Watch wall até publicação de PDF com EOF; CPU por soma /proc/schedstat dos threads após80 ms de estabilização. Startup inclui aquisição de fontes; não é o tempo exibido de compilação nem o ciclo quente. RSS CLI é pico e RSS watch é residente na amostragem após startup.

Medianas e deltas pareados, em ms. Ganho positivo significa baseline menos candidato; a mediana das diferenças pareadas não precisa coincidir com diferença das medianas de cada binário.

|Fixture|Cenário|CPU baseline|CPU candidato|Ganho CPU pareado|Ganho wall pareado|Pares com menor CPU|Dispersão CPU dentro de3% nos dois grupos|
|---|---|---:|---:|---:|---:|---|---|
|plain|cli|96.611|92.173|4.037|3.993|7/7|sim|
|plain|watch_startup|138.720|134.338|4.586|4.325|7/7|não|
|import_syntax|cli|97.131|93.324|4.369|4.326|7/7|sim|
|import_syntax|watch_startup|139.095|135.204|4.648|3.859|7/7|não|
|import_compute|cli|97.529|93.279|3.390|3.549|7/7|sim|
|import_compute|watch_startup|140.242|137.120|2.508|3.737|7/7|sim|
|layout|cli|118.340|113.749|4.848|4.806|7/7|não|
|layout|watch_startup|159.383|156.520|3.512|2.586|7/7|não|
|outline|cli|217.624|214.803|1.893|1.964|7/7|sim|
|outline|watch_startup|257.578|253.651|3.323|2.311|6/7|sim|

CPU foi menor no candidato em35/35 pares CLI e34/35 pares startup watch. O controle independente validou a integridade dos140 registros,70 pares, hashes/entradas/PIDs e unidades de memória; seus13 testes sintéticos passaram. Não se combinaram fixtures em uma porcentagem geral de ganho.

A dispersão descritiva (máximo−mínimo)/mediana ficou até3% nos dois lados em6 dos10 grupos de CPU. Portanto há redução observada consistente em direção, mas não precisão uniforme para afirmar uma aceleração exata universal. Os grupos instáveis permanecem na tabela; não foram descartados para melhorar o resultado. A regra de repetibilidade do L0 continua gate de uma promoção futura com reivindicações precisas.

|Fixture|RSS CLI baseline / candidato MiB|RSS residente startup baseline / candidato MiB|
|---|---:|---:|
|plain|24.17 / 24.16|25.86 / 25.36|
|import_syntax|25.79 / 25.91|27.50 / 27.17|
|import_compute|24.84 / 24.93|25.93 / 26.07|
|layout|25.96 / 26.25|26.50 / 26.84|
|outline|50.67 / 50.77|47.21 / 47.33|

Não apareceu mudança grande de memória nessas cargas; pequenas diferenças de RSS entre processos não demonstram equivalência exata de consumo nem limite global. Sem cache novo ou retenção além da aquisição, este candidato não deve ser apresentado como solução de memória.

## Confirmação do mecanismo por syscall

Strace separado dos benchmarks, no texto pequeno da CLI, com o mesmo arquivo de entrada. Usaram-se contagens, não os tempos perturbados do trace.

|Syscall|Baseline|Candidato|
|---|---:|---:|
|lseek|11.180|0|
|read|11.299|119|
|pread64|2|11.182|
|statx|4.678|4.678|
|openat|2.334|2.334|

As11.180 leituras seletivas passaram a ser posicionais, eliminando os respectivos seeks; não se retiraram as observações de metadados ou aberturas. O ganho se aplica à aquisição fresca da CLI/startup watch. O experimento não mede melhoria do watch quente, e não há mecanismo aqui que dispense sua validação.

## O que deve entrar nos prompts

**Para a bancada:** a especificação compartilhada aos agentes é coordenação tática; medições e conclusão pertencem a diagnostics. Não criar um L0 produtivo para esse experimento descartável nem um owner órfão em prompts. O produto permaneceu byte a byte igual, então não há hash produtivo a resealar nesta rodada.

**Para uma futura integração:** atualizar primeiro o owner existente [infra/fontdb/selective.md](../prompts/infra/fontdb/selective.md), preservando a relação1:1 com [selective.rs](../../03_infra/src/fontdb/selective.rs). O L0 já define Read+Seek/read_exact lógico e guardas; deve passar a especificar também o backend concreto posicional, as falhas e os controles adicionados. Não criar um novo prompt, Nucleus ou ADR apenas por essa troca interna. Parent fontdb, coverage e geração de fontes não mudam de responsabilidade.

Delta proposto para revisão futura, **não aplicado ao L0 ativo**:

> Para aquisição regular Unix, usar um backend privado de leitura posicional no File, expondo cursor lógico Read+Seek apenas ao projetor. Read avança pelo número efetivamente retornado; erro/Interrupted não avançam o cursor. read_exact mantém a política de retry de Interrupted, completa leituras parciais e recusa EOF prematuro ou outro erro. Arithmetic de offset/cursor é checked. O projetor genérico e seus testes de falha continuam disponíveis. Não depender do cursor físico do File nem ampliar o uso de SeekEnd sob tamanho externo mutável.
>
> Preservar exatamente limites de alocação/leitura, validação de offsets/índices, diretório e duplicatas sfnt/TTC, tratamento post, FontInfo e cobertura; não reter File, bytes ou identidade entre aquisições. Os guards de handle/path continuam após FontInfo/coverage, sem antecipação ou supressão. None continua decline seguido do fallback do parent; não descartar fonte por falha do backend. Não alterar SharedFile/Binary, política final de mmap ou plataformas não elegíveis.
>
> Exigir controles de leituras curtas, Interrupted repetido, EOF/erro e offsets/cursor, além dos testes de projeção e fallback existentes. Medir CLI e startup do watch contra baseline pareado com o custo completo e relatar dispersão/RSS; um ganho de projetor isolado não encerra a aceitação produtiva.

A classificação prevista é interna em ADR-0127: mesmo acquire privado, mesmas assinaturas públicas, mesmo modo padrão/semântica e mesma fase. A troca de syscall por si só não obriga um gate de novo contrato público; a integração futura ainda deve começar por L0 atualizado, linhagem/selos válidos e revalidação correspondente. Se o escopo mudar contrato, default, fase ou compatibilidade, reclassificar antes de escrever código produtivo. O consentimento desta rodada foi para laboratório, não para promoção.

## Decisão e próximos limites

O candidato está validado nos controles de pipeline desta rodada e mostrou ganho pequeno na aquisição inicial. É razoável levá-lo a um pacote futuro de integração com L0 e validação final, mantendo ressalvas de repetibilidade nos grupos indicados. Esta conclusão não escolhe reduzir a garantia de frescor, não autoriza cache por face nem muda o watch quente. Os próximos passos de produto não foram executados.

## Proveniência e preservação

HEAD 3c6b0ce7b7632cc74b004cea6681e4746e63d28d, working tree não commitado. Registro de preservação UTC 2026-11-04T21:49:31.663398+00:00; medições 2026-11-04T22:06:22.776923+00:00 até 2026-11-04T22:06:59.459509+00:00; conferência final 2026-11-04T22:13:02.012685+00:00. Os 2421 arquivos registrados de fontes/L0/diagnósticos/configuração permaneceram byte a byte iguais antes desta adição.

Baseline RAM SHA-256 `aa7689fb5e6f5628ef4d5870686dd1ff3a42fb3c1904449568904e1662113c81`; candidato `503380881fb1e9adad666816b2fe03de5b224e12c9c2160535b708c6c2ab98cc`. Baseline já disponível na bancada foi reaproveitado após verificar o SHA idêntico às rodadas do mesmo estado. Uma recompilação redundante foi interrompida. Candidato compilado offline/locked/release Rust1.92 na RAM em3m22s; testes reais em4m36s após completar fixtures. Targets separados, jobs1, MemoryHigh8G/Max12G/SwapMax0. Apenas dependências pré-compiladas foram copiadas entre targets; nenhuma fonte produtiva mudou.

O resultado persistente é este diagnóstico; os prompts ativos não foram editados. Código candidato, testes experimentais, fixtures/copias, logs, mutantes e builds serão removidos da RAM no encerramento conforme o skill. Reproduzir a bancada exigirá reconstruí-la a partir do escopo acima, sem promover artefatos descartáveis a ferramentas do produto.

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
