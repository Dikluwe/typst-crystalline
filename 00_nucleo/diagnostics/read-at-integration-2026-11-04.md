# Integração de read_at na aquisição seletiva de fontes

**Resultado:** backend posicional integrado ao produto, com L0 atualizado antes do código. Na bancada pareada, os 70 pares tiveram menor CPU; no texto pequeno, o ganho mediano pareado foi 3,789 ms na CLI e 5,592 ms no startup do watch. As verificações de frescor, limites e fallback foram preservadas. Não há ganho medido nem alteração de garantia no watch quente.

## Escopo e validação

O owner [selective.md](../prompts/infra/fontdb/selective.md) continua exclusivo de [selective.rs](../../03_infra/src/fontdb/selective.rs). A mudança é interna sob ADR-0127: o File Unix usa FileExt::read_at com cursor lógico privado; nenhuma assinatura pública, modo, fase ou dependência mudou. O projetor genérico, diretórios sfnt/TTC, caps, metadados e cobertura permanecem; os guards de handle/path continuam depois da derivação. SharedFile/Binary e plataformas não elegíveis conservam os caminhos existentes.

Skill Tekt com A implementando código, B escrevendo testes sem consultar A e C integrando/executando. Os 27 testes novos persistem inline no mesmo consumer. Os 22 testes de transporte e os 27 testes de transporte/projeção passaram também em wrappers extraídos do código efetivo. Os oito mutantes compilaram e foram rejeitados em execução: offset errado, avanço pelo solicitado, avanço em erro, seek ineficaz, EOF fabricado, perda de retry Interrupted, retorno excessivo aceito e chamada ao backend em leitura vazia.

No crate real, `cargo test --offline --locked --release -p typst-infra --lib fontdb:: -- --test-threads=1` passou com 58 testes, um helper ignorado na execução direta e 1.590 filtrados. O helper é invocado pelo teste pai. O filtro `fonts::` passou com 71 testes e 1.578 filtrados. Não se afirma execução da suíte completa do workspace. Build release do binário e `crystalline-lint .` completo passaram: zero erros/avisos; somente notas informativas (inclusive três V20 em padrões dos testes novos). Linhagem e selos foram conferidos nos dois sentidos, sem resealar owners alheios.

## Pipeline pareado

Sete rodadas AB/BA alternadas, cinco fixtures e dois cenários: 140 processos, 70 pares. CLI em processos novos; watch encerrado após startup. Mesmos arquivos/path por par, fontes do host, flags `--root <fixtures> --pdf-tagged=false`, RAYON_NUM_THREADS=1, SOURCE_DATE_EPOCH=0, TZ=UTC, afinidade CPU 0, stack de 64 MiB. Variáveis TYPST_/CRYSTALLINE_/E7_ removidas no runtime. Sem instrumentação, build concorrente ou limpeza do page cache.

CLI: wall por communicate(timeout), CPU por rusage dos filhos, RSS de pico por /usr/bin/time. Watch: wall até publicação de PDF completo, CPU pela soma de schedstat dos threads após 80 ms de estabilização, RSS residente nessa amostragem (VmHWM registrado separadamente). Portanto CPU do watch inclui estabilização, e RSS dos dois cenários não é a mesma modalidade.

Fixtures: texto simples; import com 2.000 bindings; import com acumulação em 10.000 iterações; 100 parágrafos; outline com 12 seções. Todas usam página 400×600 pt.

Valores em ms; ganho = mediana das diferenças pareadas baseline menos integrado. A diferença das medianas pode ser outra.

|Fixture|Cenário|CPU baseline|CPU integrado|Ganho CPU pareado|Ganho wall pareado|CPU menor|Dispersão CPU ≤3% nos dois|
|---|---|---:|---:|---:|---:|---:|---|
|plain|cli|95.395|91.606|3.789|3.679|7/7|sim|
|plain|watch_startup|139.345|133.483|5.592|5.290|7/7|sim|
|import_syntax|cli|96.544|92.219|4.143|4.149|7/7|sim|
|import_syntax|watch_startup|139.884|135.099|4.785|5.459|7/7|não|
|import_compute|cli|96.996|93.091|3.864|3.847|7/7|sim|
|import_compute|watch_startup|139.826|137.449|1.683|3.423|7/7|não|
|layout|cli|116.288|112.636|3.468|3.657|7/7|sim|
|layout|watch_startup|161.655|156.531|5.328|5.477|7/7|não|
|outline|cli|214.159|209.784|4.407|4.816|7/7|sim|
|outline|watch_startup|256.659|251.158|6.298|6.100|7/7|sim|

A dispersão é (máximo−mínimo)/mediana. Sete dos dez grupos de CPU satisfazem ≤3% em ambos os binários; import_syntax, import_compute e layout no watch não satisfazem. No texto pequeno, wall da CLI também satisfaz; wall do watch integrado tem dispersão de 3,83%, portanto sua redução observada não autoriza igual precisão. Não se deduz porcentagem universal de aceleração.

|Fixture|RSS pico CLI baseline / integrado MiB|RSS residente watch baseline / integrado MiB|
|---|---:|---:|
|plain|24.04 / 24.76|25.12 / 25.13|
|import_syntax|26.04 / 25.92|27.74 / 27.80|
|import_compute|24.65 / 24.74|26.34 / 26.02|
|layout|26.62 / 26.73|26.51 / 26.96|
|outline|50.56 / 50.75|47.17 / 47.07|

Não há evidência de ganho de memória; no texto pequeno, o RSS de pico CLI observado subiu cerca de 0,72 MiB. As diferenças entre processos não provam equivalência exata nem um limite global de memória. O adaptador não introduz retenção/caches entre aquisições.

Texto integral, páginas/coordenadas extraídas e nomes de fontes coincidiram em todos os 70 pares. A primeira rodada completa (dez pares, todas as páginas) também teve raster idêntico a 72 dpi. O inventário completo `fonts --variants` coincidiu. Três casos de erro (parêntese incompleto, import inexistente e assert falso) tiveram exit code/stdout/stderr iguais. Um par adicional de processos watch recompilou uma edição Alpha→Beta com texto, geometria e fontes finais iguais. Esses controles não certificam toda a linguagem ou plataformas não exercitadas.

## Mecanismo e referência

Strace separado dos tempos de benchmark, no caso pequeno CLI:

|Syscall|Baseline|Integrado|
|---|---:|---:|
|lseek|11180|0|
|read|11299|119|
|pread64|2|11182|
|statx|4678|4678|
|openat|2334|2334|

As 11.180 leituras seletivas passaram a posicionais e dispensaram os respectivos seeks físicos, mantendo contagens de metadata e open. A redução observada continua restrita à aquisição inicial.

Controle adicional com oracle ratificado 64c5e8c02fe0394d913ead36e42ef8c1334d2107, tree 483bd6f1d8ac8810db47b65336d402647955fd17, SHA-256 22897f93c7c2c8589cf5913b0ce735f1c01ba711c70f55c7a350f7c4b96c0b34, conferido nesta execução; receita em [vanilla-resync](../../lab/results/construction/diagnostics/vanilla-resync-2026-10-26/relatorio.md). Três execuções CLI plain separadas, posteriores à bancada pareada, deram mediana CPU 126.333 ms e wall 128.979 ms. Não são pares intercalados, nem um limite de aceleração universal. Texto e raster a 72 dpi coincidiram com ambos os produtos. O controle exato de bbox/nome bruto não coincidiu: vanilla yMin/yMax diferem em 0,00001 pt e acrescenta sufixo PDF -Identity-H a LibertinusSerif-Regular; baseline e integrado coincidem entre si. Isso é diferença preexistente de representação/extração, não uma regressão demonstrada da integração ou certificação global de paridade.

## Proveniência

HEAD 3c6b0ce7b7632cc74b004cea6681e4746e63d28d, working tree não commitado. Estado recebido registrado em 2026-11-04T22:21:24.297579+00:00. Integração de código/testes/selos na cópia de build em 2026-11-04T22:27:49.001515+00:00. Medições pareadas de 2026-11-04T22:37:15.516994+00:00 até 2026-11-04T22:37:52.167280+00:00. Nenhum fonte mudou durante as medições; comparação final de hashes confirmou que todos os arquivos recebidos foram preservados byte a byte, exceto o consumer selective e seu L0, alterados deliberadamente. O diagnóstico atual é adição.

Baseline SHA-256 `94850b6fbae01ea6df190cc2691d50c040616e8a04062d2a2d9358e9d3910446`; integrado `902cfde0215fe0a4c5e4e9713820a1eae1b339e7aaa0d8005439849c2fdaa84c`. Cópia do workspace na RAM, mesmo path de build, substituindo apenas selective após congelar o baseline; targets separados com cache copiado, validado/reutilizado por Cargo. Builds offline/locked/release, Rust 1.92.0, jobs=1, CARGO_INCREMENTAL=0, TYPST_COMMIT_SHA fixado no HEAD, configuração .cargo preservada, unidades com MemoryHigh=8G/MemoryMax=12G/MemorySwapMax=0. Fixtures auxiliares include_str copiadas sem alteração para a RAM. Toda compilação/cache/mutante/log descartável ficou em /dev/shm/tekt-readat-integration-20261104, removido após a verificação. Nenhum commit, instalação em disco ou alteração fora do escopo.

<details><summary>Lista exata de alterações rastreadas na medição: git diff HEAD --stat</summary>

```text
 00_nucleo/prompts/core/compiler/eval/bindings/field_access.md                 |  19 ++++-
 00_nucleo/prompts/core/compiler/eval/bindings/field_access/content_methods.md |  51 +++++++++++++-
 00_nucleo/prompts/core/compiler/eval/bindings/field_access/fields.md          |  30 +++++++-
 00_nucleo/prompts/core/compiler/eval/observation/elements.md                  |  63 ++++++++++++++++-
 00_nucleo/prompts/core/compiler/eval/repr/content.md                          |  30 +++++++-
 00_nucleo/prompts/core/compiler/eval/rules/set.md                             |  41 ++++++++++-
 00_nucleo/prompts/core/compiler/eval/rules/set/text.md                        |  42 ++++++++++-
 00_nucleo/prompts/core/compiler/eval/tests.md                                 |  29 +++++++-
 00_nucleo/prompts/core/compiler/introspect.md                                 |  19 ++++-
 00_nucleo/prompts/core/compiler/introspect/convergence.md                     |  19 ++++-
 00_nucleo/prompts/core/compiler/introspect/figure_demands.md                  |  19 ++++-
 00_nucleo/prompts/core/compiler/introspect/materialize_time.md                |  32 ++++++++-
 00_nucleo/prompts/core/compiler/introspect/page_running.md                    |  23 +++++-
 00_nucleo/prompts/core/compiler/introspect/walk.md                            |  16 ++++-
 00_nucleo/prompts/core/compiler/layout/bidi_origin.md                         |  34 ++++++++-
 00_nucleo/prompts/core/compiler/layout/block.md                               |  34 ++++++++-
 00_nucleo/prompts/core/compiler/layout/checkpoint.md                          |  61 +++++++++++++++-
 00_nucleo/prompts/core/compiler/layout/columns.md                             |  60 ++++++++++++++--
 00_nucleo/prompts/core/compiler/layout/columns/flow.md                        |  18 ++++-
 00_nucleo/prompts/core/compiler/layout/columns/segmented.md                   |  34 ++++++++-
 00_nucleo/prompts/core/compiler/layout/columns/transitions.md                 |  16 ++++-
 00_nucleo/prompts/core/compiler/layout/cursor.md                              |  40 ++++++++++-
 00_nucleo/prompts/core/compiler/layout/equation.md                            |  34 ++++++++-
 00_nucleo/prompts/core/compiler/layout/figure.md                              |  32 ++++++++-
 00_nucleo/prompts/core/compiler/layout/flush.md                               |  19 ++++-
 00_nucleo/prompts/core/compiler/layout/footnote.md                            |  39 ++++++++++-
 00_nucleo/prompts/core/compiler/layout/footnote_flush.md                      |  24 ++++++-
 00_nucleo/prompts/core/compiler/layout/footnote_fragments.md                  |  31 +++++++-
 00_nucleo/prompts/core/compiler/layout/grid/cell_emission.md                  |  95 ++++++++++++++++++++++++-
 00_nucleo/prompts/core/compiler/layout/grid/tracks.md                         |  81 ++++++++++++++++++++-
 00_nucleo/prompts/core/compiler/layout/helpers.md                             |  10 ++-
 00_nucleo/prompts/core/compiler/layout/image.md                               |  58 ++++++++++++++-
 00_nucleo/prompts/core/compiler/layout/measurement.md                         |  19 ++++-
 00_nucleo/prompts/core/compiler/layout/mod.md                                 |  34 ++++++++-
 00_nucleo/prompts/core/compiler/layout/page_run.md                            |  34 ++++++++-
 00_nucleo/prompts/core/compiler/layout/page_running_requests.md               |  53 +++++++++++++-
 00_nucleo/prompts/core/compiler/layout/paragraph.md                           |  34 ++++++++-
 00_nucleo/prompts/core/compiler/layout/paragraph_indent.md                    |  34 ++++++++-
 00_nucleo/prompts/core/compiler/layout/place.md                               |  32 ++++++++-
 00_nucleo/prompts/core/compiler/layout/placement.md                           |  34 ++++++++-
 00_nucleo/prompts/core/compiler/layout/references.md                          |  48 ++++++++++++-
 00_nucleo/prompts/core/compiler/layout/region_progress.md                     |  42 ++++++++++-
 00_nucleo/prompts/core/compiler/layout/repeat.md                              |  34 ++++++++-
 00_nucleo/prompts/core/compiler/layout/set_page.md                            |  34 ++++++++-
 00_nucleo/prompts/core/compiler/layout/shape.md                               |  37 +++++++++-
 00_nucleo/prompts/core/compiler/layout/slicing.md                             |  10 ++-
 00_nucleo/prompts/core/compiler/layout/stack.md                               |  69 +++++++++++++++++-
 00_nucleo/prompts/core/compiler/layout/sub_frame.md                           |  17 ++++-
 00_nucleo/prompts/core/compiler/layout/text/style.md                          |  34 ++++++++-
 00_nucleo/prompts/core/compiler/layout/tiling.md                              |  37 +++++++++-
 00_nucleo/prompts/core/compiler/math/layout/delimited.md                      |  80 ++++++++++++++++++++-
 00_nucleo/prompts/core/compiler/math/layout/external_bridge.md                |  34 ++++++++-
 00_nucleo/prompts/core/compiler/math/layout/mod.md                            |  81 ++++++++++++++++++++-
 00_nucleo/prompts/core/compiler/math/layout/op.md                             |  52 +++++++++++++-
 00_nucleo/prompts/core/compiler/realize/figure.md                             |  32 ++++++++-
 00_nucleo/prompts/core/compiler/realize/figure_kind.md                        |  19 ++++-
 00_nucleo/prompts/core/compiler/stdlib/figure_image.md                        |  87 ++++++++++++++++++++++-
 00_nucleo/prompts/core/compiler/stdlib/tests/figure_image.md                  |  65 ++++++++++++++++-
 00_nucleo/prompts/core/compiler/stdlib/text/constructor.md                    |  42 ++++++++++-
 00_nucleo/prompts/core/compiler/stdlib/visualize.md                           |  37 +++++++++-
 00_nucleo/prompts/core/entities/content/construction.md                       |  70 ++++++++++++++++++-
 00_nucleo/prompts/core/entities/content/fields.md                             |  30 +++++++-
 00_nucleo/prompts/core/entities/content/tests/traversal.md                    |  19 ++++-
 00_nucleo/prompts/core/entities/content/traversal.md                          |  20 +++++-
 00_nucleo/prompts/core/entities/elements/figure.md                            |  20 +++++-
 00_nucleo/prompts/core/entities/elements/image.md                             |  64 ++++++++++++++++-
 00_nucleo/prompts/core/entities/image_format.md                               |  26 ++++++-
 00_nucleo/prompts/core/entities/image_sizer.md                                |  26 ++++++-
 00_nucleo/prompts/core/entities/layout_types.md                               |  31 +++++++-
 00_nucleo/prompts/core/entities/mod.md                                        |  24 ++++++-
 00_nucleo/prompts/core/entities/tiling.md                                     |  37 +++++++++-
 00_nucleo/prompts/infra/export/builder.md                                     |  15 ++++
 00_nucleo/prompts/infra/export/builder/resources.md                           |  15 +++-
 00_nucleo/prompts/infra/export/images.md                                      |  31 +++++++-
 00_nucleo/prompts/infra/export/images/png.md                                  |  20 +++++-
 00_nucleo/prompts/infra/export/images/scan.md                                 |  31 +++++++-
 00_nucleo/prompts/infra/export/render.md                                      |  36 +++++++++-
 00_nucleo/prompts/infra/export/stream/draw.md                                 |  10 ++-
 00_nucleo/prompts/infra/export/svg/images.md                                  |  19 ++++-
 00_nucleo/prompts/infra/export/svg/tests.md                                   |  10 ++-
 00_nucleo/prompts/infra/export/tests/mod.md                                   |  27 ++++++-
 00_nucleo/prompts/infra/fontdb/selective.md                                   |  10 ++-
 00_nucleo/prompts/infra/lib.md                                                |  17 ++++-
 00_nucleo/prompts/infra/pipeline/context_stabilization.md                     |  19 ++++-
 00_nucleo/prompts/infra/pipeline/paged.md                                     |  17 ++++-
 00_nucleo/prompts/infra/pipeline/tests.md                                     | 125 ++++++++++++++++++++++++++++++++-
 00_nucleo/prompts/infra/shaper/bidi.md                                        |  37 +++++++++-
 00_nucleo/prompts/infra/shaper/shaping.md                                     |  35 +++++++++-
 00_nucleo/prompts/infra/tests/svg_composite_graph.md                          |  10 ++-
 00_nucleo/prompts/shell/cli.md                                                |  19 ++++-
 01_core/src/compiler/eval/bindings/field_access.rs                            |   4 +-
 01_core/src/compiler/eval/bindings/field_access/content_methods.rs            |  21 ++++--
 01_core/src/compiler/eval/bindings/field_access/fields.rs                     |   6 +-
 01_core/src/compiler/eval/observation/elements.rs                             |  34 ++++++---
 01_core/src/compiler/eval/repr/content.rs                                     |  12 +++-
 01_core/src/compiler/eval/rules/set.rs                                        |  54 +++++++++++++-
 01_core/src/compiler/eval/rules/set/text.rs                                   |  42 ++++++++++-
 01_core/src/compiler/eval/tests.rs                                            |   4 +-
 01_core/src/compiler/introspect.rs                                            |   4 +-
 01_core/src/compiler/introspect/convergence.rs                                |   4 +-
 01_core/src/compiler/introspect/figure_demands.rs                             |   4 +-
 01_core/src/compiler/introspect/materialize_time.rs                           |  34 ++++++++-
 01_core/src/compiler/introspect/page_running.rs                               |  68 +++++++++++++++---
 01_core/src/compiler/introspect/walk.rs                                       |  69 +++++++++++++++++-
 01_core/src/compiler/layout/bidi_origin.rs                                    |   7 +-
 01_core/src/compiler/layout/block.rs                                          |   9 +--
 01_core/src/compiler/layout/checkpoint.rs                                     |  17 ++++-
 01_core/src/compiler/layout/columns.rs                                        |  32 +++++----
 01_core/src/compiler/layout/columns/flow.rs                                   |  58 ++++++++++++++-
 01_core/src/compiler/layout/columns/segmented.rs                              |  30 +++-----
 01_core/src/compiler/layout/columns/transitions.rs                            |  72 ++++++++++++++++++-
 01_core/src/compiler/layout/cursor.rs                                         |  75 +++++++++++++++++++-
 01_core/src/compiler/layout/equation.rs                                       |   9 +--
 01_core/src/compiler/layout/figure.rs                                         | 240 ++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++--
 01_core/src/compiler/layout/flush.rs                                          |   3 +-
 01_core/src/compiler/layout/footnote.rs                                       | 171 +++++++++++++++++++++++++++++++++++++++++++--
 01_core/src/compiler/layout/footnote_flush.rs                                 |  71 ++++++++++++++++++-
 01_core/src/compiler/layout/footnote_fragments.rs                             |  99 ++++++++++++++++++++++++--
 01_core/src/compiler/layout/grid/cell_emission.rs                             | 167 +++++++++++++++++++++++++++++++++++++++++---
 01_core/src/compiler/layout/grid/tracks.rs                                    | 114 ++++++++++++++++++++++++++++--
 01_core/src/compiler/layout/helpers.rs                                        |   5 +-
 01_core/src/compiler/layout/image.rs                                          |  80 ++++++++++++++++++---
 01_core/src/compiler/layout/measurement.rs                                    |   4 +-
 01_core/src/compiler/layout/mod.rs                                            |   6 +-
 01_core/src/compiler/layout/page_run.rs                                       |  16 +----
 01_core/src/compiler/layout/page_running_requests.rs                          |  79 ++++++++++++++++++++-
 01_core/src/compiler/layout/paragraph.rs                                      |  78 ++++++++++++++++++---
 01_core/src/compiler/layout/paragraph_indent.rs                               |   9 +--
 01_core/src/compiler/layout/place.rs                                          |  67 ++++++++++++++++--
 01_core/src/compiler/layout/placement.rs                                      |   9 +--
 01_core/src/compiler/layout/references.rs                                     | 141 ++++++++++++++++++++++++++++++++++++-
 01_core/src/compiler/layout/region_progress.rs                                |  86 +++++++++++++++++++++--
 01_core/src/compiler/layout/repeat.rs                                         |   7 +-
 01_core/src/compiler/layout/set_page.rs                                       |  16 +----
 01_core/src/compiler/layout/shape.rs                                          |   7 +-
 01_core/src/compiler/layout/slicing.rs                                        |   4 +-
 01_core/src/compiler/layout/stack.rs                                          |  17 +++--
 01_core/src/compiler/layout/sub_frame.rs                                      |  57 ++++++++++++++-
 01_core/src/compiler/layout/text/style.rs                                     |  11 ++-
 01_core/src/compiler/layout/tiling.rs                                         |  12 +++-
 01_core/src/compiler/math/layout/delimited.rs                                 | 211 ++++++++++++++++++++++++++++++++++++++++++++++++++++++-
 01_core/src/compiler/math/layout/external_bridge.rs                           |  10 ++-
 01_core/src/compiler/math/layout/mod.rs                                       |  10 ++-
 01_core/src/compiler/math/layout/op.rs                                        |  91 +++++++++++++++++++++++-
 01_core/src/compiler/realize/figure.rs                                        |  76 ++++++++++++++++++--
 01_core/src/compiler/realize/figure_kind.rs                                   |   6 +-
 01_core/src/compiler/stdlib/figure_image.rs                                   | 114 ++++++++++++++++++++++++------
 01_core/src/compiler/stdlib/tests/figure_image.rs                             | 282 +++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++--
 01_core/src/compiler/stdlib/text/constructor.rs                               |  20 +++++-
 01_core/src/compiler/stdlib/visualize.rs                                      |   7 +-
 01_core/src/entities/content/construction.rs                                  |   9 ++-
 01_core/src/entities/content/fields.rs                                        |   4 +-
 01_core/src/entities/content/tests/traversal.rs                               |   4 +-
 01_core/src/entities/content/traversal.rs                                     |   4 +-
 01_core/src/entities/elements/figure.rs                                       |  79 ++++++++++++++++++++-
 01_core/src/entities/elements/image.rs                                        |  21 ++++--
 01_core/src/entities/image_format.rs                                          |   9 ++-
 01_core/src/entities/image_sizer.rs                                           |  13 +++-
 01_core/src/entities/layout_types.rs                                          |   4 +-
 01_core/src/entities/mod.rs                                                   |   4 +-
 01_core/src/entities/tiling.rs                                                |   7 +-
 02_shell/src/cli.rs                                                           |   4 +-
 03_infra/Cargo.toml                                                           |   7 ++
 03_infra/src/export/builder.rs                                                |   2 +-
 03_infra/src/export/builder/resources.rs                                      |   7 +-
 03_infra/src/export/images.rs                                                 |   7 +-
 03_infra/src/export/images/png.rs                                             |  93 +++++++++++++++++++++++-
 03_infra/src/export/images/scan.rs                                            |  27 ++++++-
 03_infra/src/export/render.rs                                                 |  58 ++++++++++++++-
 03_infra/src/export/stream/draw.rs                                            |   3 +-
 03_infra/src/export/svg/images.rs                                             |   8 ++-
 03_infra/src/export/svg/tests.rs                                              |   4 +-
 03_infra/src/export/tests/mod.rs                                              |  15 +++-
 03_infra/src/fontdb/selective.rs                                              | 534 ++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++--
 03_infra/src/lib.rs                                                           |   4 +-
 03_infra/src/pipeline/context_stabilization.rs                                |   4 +-
 03_infra/src/pipeline/paged.rs                                                |   4 +-
 03_infra/src/pipeline/tests.rs                                                | 593 +++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++-
 03_infra/src/shaper/bidi.rs                                                   |  78 ++++++++++++++++++++-
 03_infra/src/shaper/shaping.rs                                                |  14 ++--
 03_infra/tests/svg_composite_graph.rs                                         |   3 +-
 Cargo.lock                                                                    | 275 ++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++---
 182 files changed, 7719 insertions(+), 443 deletions(-)
```

</details>

<details><summary>Entradas exatas e identidade das fixtures</summary>

Os arquivos principais contêm estes bytes (literais Python; todas as variantes receberam o mesmo path por fixture):

```python
plain = '#set page(width:400pt,height:600pt)\nAlpha text.\n'
import_syntax = '#set page(width:400pt,height:600pt)\n#import "syntax.typ": value\nAlpha #value\n'
import_compute = '#set page(width:400pt,height:600pt)\n#import "compute.typ": value\nAlpha #value\n'
layout = '#set page(width:400pt,height:600pt)\nAlpha\n#for i in range(100) [Paragraph #i with several words to lay out across lines.\n\n]'
outline = '#set page(width:400pt,height:600pt)\n#outline()\nAlpha\n#for i in range(12) [= Heading #i\n#lorem(30)\n#pagebreak()]'
syntax_import = "\n".join(f"#let v{i} = {i}" for i in range(2000)) + "\n#let value = v1999\n"
compute_import = '#let value = { let n=0; for i in range(10000) { n+=i }; n }\n'
```

SHA-256 das entradas:

```text
compute.typ 3079e3d57fa6dfcce404f8109786c7eeee374778e444cd542485e554ee4141b3
import_compute.typ 45c74b0f8f406c3a222292e001b32d46bedafc779a888e312d71774ec4ef9322
import_syntax.typ 1f8c2c8f4344a448ac082ac5eb8d405729e8a752c7e0ef1cb8bb8e4658f052b4
layout.typ f49606d6e7e73908233e75122ed85a190f0d58d58b4c95187c57df7eff9a4ec7
outline.typ 01778284428f92b5dc0ba432f765964b878d480565f1e3fbf78f41810ea8913d
plain.typ ef13162f076a48788dbe4fb2f1f0c391800f0dd10aab97fb471dd7662216b645
syntax.typ 8966a2efa39075703f907eedbcde8bdc16f6f2923ae462b3ac09a60bb0cac8f2
```

</details>

<details><summary>Amostras de performance: CPU ms / wall ms / RSS bytes, em ordem de rodada</summary>

RSS = pico na CLI e residente no watch. Cada linha contém as sete amostras, sem descarte.

```text
plain cli baseline: 95.432000/96.809302/25206784; 96.196000/98.395163/25866240; 95.541000/97.512366/25169920; 95.377000/97.342398/25178112; 95.395000/97.239175/25305088; 95.302000/97.392154/24899584; 95.256000/97.230882/25202688
plain cli candidate: 91.818000/93.438363/25964544; 91.074000/93.346531/26075136; 92.585000/94.702483/25313280; 92.015000/93.969487/26075136; 91.606000/93.560610/25296896; 91.147000/93.167643/26021888; 91.090000/93.059772/25214976
plain watch_startup baseline: 138.140152/140.442856/27107328; 139.725998/142.241292/26337280; 141.196354/143.341640/26136576; 138.827731/142.365471/26583040; 139.524840/143.413509/26796032; 139.344931/142.433755/26316800; 138.245779/141.017562/26120192
plain watch_startup candidate: 132.329677/135.009778/27193344; 133.482609/136.951578/27099136; 135.645256/138.347849/26345472; 133.236098/135.112835/26206208; 136.089629/140.209286/26349568; 132.547124/134.961840/26275840; 135.037726/138.179437/26673152
import_syntax cli baseline: 95.589000/97.241363/27308032; 97.025000/99.028272/27049984; 96.471000/98.547700/27303936; 97.148000/99.118345/27127808; 96.156000/98.114900/27697152; 96.917000/98.908384/27164672; 96.544000/98.479270/27684864
import_syntax cli candidate: 91.655000/93.211014/27807744; 92.219000/94.030295/27471872; 92.776000/94.832726/27181056; 91.802000/93.737739/27045888; 92.680000/94.633135/27971584; 92.052000/93.985870/27049984; 92.401000/94.330470/27074560
import_syntax watch_startup baseline: 139.883692/142.657408/29491200; 142.315292/146.459542/29462528; 137.577446/141.235468/28876800; 140.459972/143.964595/29089792; 136.936685/139.352986/29085696; 140.138060/144.161721/29396992; 137.966647/141.016728/28758016
import_syntax watch_startup candidate: 135.098950/137.198864/28778496; 134.305552/137.080738/29507584; 136.658886/140.099623/29609984; 133.447334/137.042563/28782592; 136.233159/140.064100/28848128; 133.936772/137.277962/29560832; 137.557509/140.550161/29155328
import_compute cli baseline: 96.767000/98.415759/25804800; 98.915000/101.090688/25677824; 96.101000/98.023964/25718784; 97.357000/99.454315/25845760; 96.996000/98.949859/26390528; 98.634000/100.804797/26079232; 96.704000/98.599213/26087424
import_compute cli candidate: 92.903000/94.568495/25956352; 93.091000/95.237273/25944064; 92.717000/94.738126/25956352; 93.417000/95.355619/25800704; 93.396000/95.447644/25649152; 92.125000/94.164534/25804800; 93.853000/95.842721/25968640
import_compute watch_startup baseline: 138.394391/140.390212/27611136; 140.867728/144.045620/27181056; 139.348086/143.428749/27639808; 142.614503/146.547620/27152384; 139.826223/143.427142/27885568; 140.954159/144.111395/27615232; 139.132944/143.157763/27918336
import_compute watch_startup candidate: 137.449340/139.468470/27201536; 136.393927/138.967684/27279360; 137.665131/140.005644/27283456; 134.566702/137.057479/27389952; 138.925707/142.470594/27209728; 134.426852/137.030386/28004352; 138.434082/142.092177/27979776
layout cli baseline: 115.409000/116.946681/28512256; 116.484000/118.479212/27787264; 114.956000/116.833217/28024832; 116.288000/118.269225/28483584; 116.994000/119.118262/27021312; 117.403000/119.626998/27918336; 116.097000/118.166082/27742208
layout cli candidate: 112.065000/113.838415/28536832; 112.157000/114.070252/26890240; 113.351000/115.339938/28680192; 110.497000/112.720433/28286976; 113.526000/115.627800/27996160; 113.339000/115.443354/27021312; 112.636000/114.509549/28028928
layout watch_startup baseline: 162.493814/166.067991/28352512; 160.957527/164.870723/27680768; 180.838403/184.579834/28315648; 161.654782/165.123911/27799552; 160.859082/163.963406/27549696; 161.910341/164.975325/27627520; 160.712775/164.519284/28192768
layout watch_startup candidate: 157.909700/160.590892/28266496; 155.167550/158.189942/27684864; 160.129024/162.903731/28286976; 156.326933/159.963540/27922432; 156.531058/160.657942/28340224; 155.435003/157.749993/28319744; 158.087062/160.699052/27660288
outline cli baseline: 213.908000/215.587767/53149696; 214.602000/216.675442/53051392; 213.499000/215.480077/52957184; 214.159000/216.076328/52994048; 214.091000/216.664917/52998144; 215.218000/217.262613/53362688; 217.576000/219.365307/53014528
outline cli candidate: 209.746000/211.746525/53219328; 209.361000/211.456231/53035008; 210.302000/212.472030/53194752; 212.382000/214.418172/53174272; 209.684000/211.848733/53571584; 209.784000/211.697291/53465088; 211.241000/213.259110/53288960
outline watch_startup baseline: 256.658545/260.335400/48607232; 257.301010/260.437768/49684480; 257.693336/262.011446/48525312; 255.379958/258.506732/49459200; 251.880599/255.470936/54808576; 258.433613/262.615013/49823744; 254.630353/257.520194/47824896
outline watch_startup candidate: 251.291933/255.026858/49319936; 250.175810/253.503853/49373184; 250.841649/255.911404/49983488; 251.171046/255.031432/49602560; 251.157729/254.103623/49352704; 252.135414/255.523031/49238016; 248.205394/251.369263/49229824
oracle plain cli 100: 126.333000/128.978990/30846976
oracle plain cli 101: 126.262000/128.788271/31125504
oracle plain cli 102: 126.856000/129.285296/31248384
```

</details>

O experimento anterior permanece histórico em [read-at-pipeline](read-at-pipeline-2026-11-04.md): ele não foi reescrito como se já tivesse integrado o produto.
