# Fontes: reúso do parse de configuração com prova fresca

**Resultado:** a bancada reduziu trabalho puro de parse sem retirar as observações de filesystem. O ganho foi pequeno: ConfigKey completo economizou 0,747 ms CPU por chamada em mediana pareada; na repetição de watch pequeno, os três blocos economizaram em média 0,678–1,605 ms por edição (mediana dos deltas de bloco 0,937 ms). A dispersão do candidato ainda excedeu 3%. Nenhuma otimização nova foi integrada ao produto; read_at permanece como entregue.

## O que foi testado

Fork descartável de fontconfig-parser 0.5.8 em RAM, preservando merge_config/include originais literalmente. O método adicional cacheia apenas Vec<ConfigPart> de análises integralmente válidas, por texto integral exato. Canonicalização, leitura de bytes, metadata, listagem/file_type/sort de includes, ausência/erros e resolução lexical de caminhos/HOME/XDG/CWD continuam frescos. Configuração final e erros não são cacheados. O fallback de erro reexecuta o iterador sobre a mesma String já lida, preservando partes aplicadas antes do erro, sem releitura.

Na cópia de pipeline, somente FastKey Linux recebe um Mutex<ParseCache> por geração; igualdade continua config+roots. Poison recusa o caminho rápido. config_key_cached preserva o corpo atual de config_key, alterando apenas o dispatch de merge. Mantém ConfigKey completo (dirs, aliases, bytes de arquivos, env e cwd), ProbeKey, GenerationKey, identidade Unix, sequência de verificação e fallback. CLI segue o método original de fontdb. O cache tem limites de 256 entradas/1 MiB de bytes-fonte, não um limite medido de heap total de ConfigPart.

Os L0 [generation](../prompts/infra/fontdb/generation.md), [font_session](../prompts/infra/world/font_session.md) e o Nucleus watch-font-generation foram consultados. O L0 atual exige reparse no hot path; este estudo é quarentena e não altera esse contrato. Uma integração futura requer L0 antes do código e uma forma mantível de loader/cache; o fork não é uma dependência produtiva aprovada.

## Configuração completa isolada

Sete pares AB/BA, 500 construções e comparações integrais por processo após um controle fresco e uma chamada inicial. Inclui o corpo atual de config_key: parser/merge, aliases, nova leitura dos 55 arquivos (113.288 bytes), env/cwd e comparação completa. Não inclui prova de diretórios e stamps dos arquivos de fontes. Não mede apenas lookup.

CPU mediana por chamada: 2.345842 → 1.593814 ms; ganho pareado 0.746880 ms. Wall por chamada: 2.330802 → 1.577585 ms; ganho pareado 0.747479 ms. CPU menor em 7/7 pares; dispersões das médias por processo 2.13% e 2.06%.

Cada validação quente teve 53 hits de partes puras e quatro análises XML efetivas em caminhos inválidos não cacheáveis (eager+fallback). Retenção: 53 entradas e 111.223 bytes-fonte, além dos objetos parseados. RSS pico mediano do processo: 3280 → 3392 KiB. Primeira chamada após controle fresco: 2.510 → 2.739 ms wall; é custo descritivo de primeira chamada, não cold page cache nem medida de startup CLI.

## Pipeline real em cópia RAM

Sete rodadas, cinco fixtures, CLI e watch com startup/samebytes/edição pequena/estrutural: 350 observações e 175 pares. Cada processo watch executa startup e três recompilações. AB/BA alternado por rodada, mesmas entradas/path por par, RAYON_NUM_THREADS=1, SOURCE_DATE_EPOCH=0, TZ=UTC, NO_COLOR=1, afinidade CPU0 e stack64MiB; flags --root e --pdf-tagged=false. Variáveis TYPST_/CRYSTALLINE_/E7_ removidas no runtime. Sem build concorrente, sonda ou limpeza de page cache.

CPU CLI por rusage dos filhos, incluindo wrapper time; wall por communicate(timeout). Watch CPU por /proc/schedstat dos threads antes da edição e após publicação+80ms; startup conta desde o processo novo. Wall termina em PDF publicado completo, sem os 80ms posteriores. RSS CLI é pico; watch é residente na amostragem (VmHWM separado). Esses observadores incluem trabalho de runtime, não CPU exclusiva de uma função.

Valores ms; ganhos = mediana das diferenças pareadas baseline−candidato. Não são necessariamente a diferença das medianas.

|Fixture|Modo|CPU base|CPU cand.|Ganho CPU pareado|Ganho wall pareado|CPU menor|Dispersão CPU ≤3% ambos|
|---|---|---:|---:|---:|---:|---:|---|
|plain|cli|92.615|92.495|0.068|-0.028|4/7|não|
|plain|startup|133.646|132.974|0.179|-0.072|4/7|sim|
|plain|samebytes|18.781|18.312|0.192|0.267|6/7|não|
|plain|small|19.168|17.363|1.659|0.839|7/7|não|
|plain|structural|19.133|18.819|0.497|1.368|7/7|não|
|import_syntax|cli|93.697|95.261|-1.564|-1.975|3/7|não|
|import_syntax|startup|133.757|133.562|0.203|0.213|4/7|sim|
|import_syntax|samebytes|23.492|22.450|1.042|0.700|6/7|não|
|import_syntax|small|23.061|21.997|0.959|1.977|6/7|não|
|import_syntax|structural|23.431|22.835|0.453|0.104|6/7|não|
|import_compute|cli|94.191|93.896|-0.091|-0.597|3/7|não|
|import_compute|startup|135.989|135.544|1.939|2.168|5/7|sim|
|import_compute|samebytes|25.616|24.402|1.555|1.679|5/7|não|
|import_compute|small|24.828|23.872|1.312|1.129|6/7|não|
|import_compute|structural|25.703|24.684|0.915|1.471|7/7|não|
|layout|cli|113.551|115.535|-2.063|-2.242|3/7|não|
|layout|startup|155.237|155.191|0.047|0.644|4/7|sim|
|layout|samebytes|37.604|36.904|0.548|0.884|5/7|não|
|layout|small|37.563|37.298|0.292|0.246|5/7|não|
|layout|structural|38.177|36.610|1.873|0.953|6/7|não|
|outline|cli|214.418|210.787|3.631|3.317|5/7|não|
|outline|startup|253.569|251.863|0.765|-0.288|4/7|não|
|outline|samebytes|128.542|127.693|1.150|0.711|5/7|não|
|outline|small|126.853|127.785|0.296|-0.391|4/7|não|
|outline|structural|128.978|128.133|1.839|2.253|5/7|não|

Somente 4/25 grupos de CPU satisfizeram (máximo−mínimo)/mediana ≤3% em ambos os lados. Nos ciclos quentes houve CPU menor em 86/105 pares e delta mediano positivo em todos os 15 grupos, mas nenhum desses grupos teve dispersão ≤3% nos dois lados. CLI não usa esse cache: seus sinais mistos, inclusive negativos, não demonstram ganho nem encerram ausência de regressão de custo. Não se atribui a oscilação a uma causa não medida, nem se descartam amostras.

|Fixture|RSS CLI pico base/cand. MiB|RSS startup residente base/cand. MiB|RSS edição pequena residente base/cand. MiB|
|---|---:|---:|---:|
|plain|24.49/24.54|25.22/25.81|25.02/25.43|
|import_syntax|25.98/26.07|27.59/27.62|28.46/28.55|
|import_compute|24.77/24.79|26.70/26.29|28.42/28.01|
|layout|26.73/26.19|26.75/26.82|26.91/27.18|
|outline|50.94/50.94|47.18/47.39|50.91/47.44|

RSS variou por fixture/processo; no texto pequeno, o residente aumentou 604 KiB no startup e 416 KiB na edição pequena. Reduções vistas em outras cargas não demonstram que o cache economize memória. Não há limite global de heap inferido dessas amostras.

## Repetição focada para a dispersão

Pré-definida após observar o ruído da bancada ampla: três pares de processos, cinco recompilações de warmup e 40 edições medidas por processo (240 observações medidas). Texto/página iguais, comentários numerados alterados em cada edição; comparação AB/BA e mesmo observador de CPU/wall. Warmups excluídos por desenho, nenhuma observação medida descartada. Unidade da tabela é média das 40 edições por processo, não comparação com as medianas da bancada ampla.

|Par|CPU média base|CPU média cand.|Economia média CPU|Economia média wall|
|---:|---:|---:|---:|---:|
|1|18.930475|17.993849|0.936626|0.892095|
|2|19.083424|17.478840|1.604584|1.509107|
|3|18.757444|18.079866|0.677577|0.636903|

Mediana das economias médias pareadas: 0.936626 ms CPU e 0.892095 ms wall. Dispersão dos blocos: 1.72% base e 3.34% candidato. Três blocos favoráveis sustentam benefício observado neste caso, mas não precisão uniforme abaixo de 3% ou aceleração universal.

## Correção e controles

Skill tekt-materializacao-segregada: A implementou somente código; B escreveu testes sem consultar A; C integrou/executou. Passaram 39 testes independentes do parser/cache e 13 da conexão; os 13 também passaram dentro do crate real. Grupos fontdb:: e fonts::: 71+71 testes aprovados; um helper ignorado na execução direta é invocado pelo pai. Não foi executada a suíte inteira do workspace.

Dez mutantes do cache e cinco da conexão foram rejeitados em execução após compilar. Dois sobreviventes iniciais mostraram lacunas reais de testes: limite de bytes mascarava limite de entradas; e faltava assert exato da contagem eager+fallback. B acrescentou controles ortogonais já previstos na spec; as soluções não precisaram mudar. A suíte ampliada e os mutantes correspondentes passaram pelo RED→GREEN.

Controle separado de strace: 3.008 assinaturas ordenadas de solicitações FS coincidiram no host (três construções de ConfigKey com controle/primeira/quente). Em fixture com erro parcial de include, alteração, criação, remoção e reparo, outras 337 coincidiram. Comparação usa operação, path, follow/flags, tamanho solicitado de read e status/quantidade retornada; normaliza somente PID de /proc/self/maps e não exige bytes de structs de metadata/endereços. Counts iguais isolados não substituem a comparação ordenada.

Os 175 pares do pipeline coincidiram em texto integral, páginas/bboxes e nomes de fontes. Dez pares (primeira rodada, startup+edição pequena em cinco fixtures) tiveram raster idêntico de todas as páginas a72dpi. Inventários completos fonts --variants coincidiram. A repetição focada manteve texto/geometry/fontes iguais entre os seis processos. São controles exercitados no Linux, não certificação de toda a linguagem/plataformas.

## Decisão e próximo limite

Manter read_at produtivo e o restante do produto como recebido. Este candidato elimina parse puro repetido, preserva os controles exercitados e mostra economia próxima de1ms no watch pequeno. Não resolve o custo principal das consultas de metadata; não autoriza reduzir observações ou frescor, nem usar cache nativo Fontconfig como autoridade. A maior parcela de fontes continua fora desse parse.

Não promover este protótipo nesta rodada: o conjunto não fecha benefício líquido repetível abaixo de3%, o caminho CLI apresenta sinais mistos, e o fork/duplicação do merger ainda não é uma forma produtiva mantível. Integração futura exigiria L0 próprio atualizado antes do código, preservação do loader/error policy e orçamento de retenção avaliado para partes parseadas, com revalidação no pipeline. A economia isolada não justifica uma promessa de remover os cerca de16ms históricos de prova quente.

Próxima investigação prioritária: custo efetivo das observações de metadata com os mesmos campos, ordem e follow/nofollow. Para a CLI, medir a duplicação de trabalho entre a descoberta fontdb e a aquisição posterior: fontdb0.23.0 lib.rs:233–272 já abre/mapeia/parseia as faces; fontdb.rs:49–78 depois chama selective/acquire para metadata+coverage. Isso é evidência de dois contatos, não ganho demonstrado ou autorização de reestruturar o loader. Qualquer bancada permanece em quarentena e deve respeitar os contratos atuais antes de propor integração.

## Proveniência e amostras

HEAD 3c6b0ce7b7632cc74b004cea6681e4746e63d28d, uncommitted working tree. Captura inicial 2026-11-05T00:15:16.272566+00:00. Pipeline pareado de 2026-11-05T00:42:25.886286+00:00 até 2026-11-05T00:43:46.825839+00:00. A rodada isolada terminou em 2026-11-05T00:25:47.074597+00:00; a repetição focada terminou em 2026-11-05T00:51:21.881453+00:00. Os binários ficaram congelados nessas medições. Todos os arquivos recebidos, incluindo read_at e L0, permaneceram byte a byte idênticos; apenas este diagnóstico e as amostras de domínio são adições. A lista exata de alterações rastreadas recebidas está abaixo.

Baseline SHA-256 `902cfde0215fe0a4c5e4e9713820a1eae1b339e7aaa0d8005439849c2fdaa84c` (idêntico ao produto integrado na rodada read_at); candidato `e3d795512cf30cf8c824e4f512a0f432e69dccdf496fb3b48f748398f885c067`. Fontconfig-parser0.5.8 e roxmltree0.20.0; no pipeline só mudou a origem do parser, com todas as versões do lock preservadas. A bancada isolada usa serde_json apenas para emitir resultados fora dos relógios internos, com lock local gerado offline.

Rust1.92.0, offline/release, um job, targets separados na RAM, CARGO_INCREMENTAL=0, TYPST_COMMIT_SHA fixado no HEAD e .cargo/config do produto preservado no pipeline. Unidades MemoryHigh8G/MemoryMax12G/MemorySwapMax0. Build candidato teve um aviso esperado de config_key original não usado, mantido como referência de testes; não houve supressão. O workspace produtivo, dependências, prompts e selos não mudaram. Builds/logs/forks/mutantes/caches descartáveis em /dev/shm/tekt-font-config-20261104, removidos ao final. Sem commit.

[Amostras completas](../../lab/results/construction/diagnostics/font-config-parse-2026-11-04/amostras.csv): config_key contém CPU do processo para500 chamadas e wall interno dessas chamadas (repetitions=500); primeira chamada registra sówall. Pipeline e focus têm uma observação por linha (repetitions=1). RSS é pico na CLI/processo isolado, residente no watch, não amostrado por edição no focus. O CSV permanece como artefato local no diretório de resultados; segue a regra de ignore existente de lab e não foi forçado no índice Git.

<details><summary>Entradas reproduzíveis</summary>

Página prefixada por #set page(width:400pt,height:600pt). Corpos principais:

```python
plain = 'Alpha text.\n'
import_syntax = '#import "syntax.typ": value\nAlpha #value\n'
import_compute = '#import "compute.typ": value\nAlpha #value\n'
layout = 'Alpha\n#for i in range(100) [Paragraph #i with several words to lay out across lines.\n\n]'
outline = '#outline()\nAlpha\n#for i in range(12) [= Heading #i\n#lorem(30)\n#pagebreak()]'
syntax_import = "\n".join(f"#let v{i} = {i}" for i in range(2000)) + "\n#let value = v1999\n"
compute_import = '#let value = { let n=0; for i in range(10000){n+=i}; n }\n'
```

Edição pequena: substituir Alpha por Beta. Estrutural: mesma substituição e adicionar \n\nGamma extra paragraph.\n. Focus: Alpha text. constante, comentário final // revision i; i=-5..-1 no warmup e0..39 medidos. Os paths são compartilhados entre variantes, sob o diretório exclusivo RAM.

SHA-256 das entradas de cada revisão no pipeline:

```text
plain cli ef13162f076a48788dbe4fb2f1f0c391800f0dd10aab97fb471dd7662216b645
plain small dd008c1dcebe9ba72dba68628205cf9e8974e66092d7deb64005376dc66a6cde
plain structural 560738747894f89fc7ec30dee99a89bd21b999996af9dbf1b5c1b39ccbc2c814
import_syntax cli 1f8c2c8f4344a448ac082ac5eb8d405729e8a752c7e0ef1cb8bb8e4658f052b4
import_syntax small b2877ab7bf2138dc195307e247e89c968caad55992ef4532c4227174983d248b
import_syntax structural 645fa0831b8053f2894fcc8c7b243bae6367bea276f55ae52e2d7ab26b2605a7
import_compute cli 45c74b0f8f406c3a222292e001b32d46bedafc779a888e312d71774ec4ef9322
import_compute small 24724992767e55b09d4bd870f2e34727d8e5bc8baeb56de2484be89ec73817b4
import_compute structural 6e59bb0cfe81023d2b31bb5cd0833e6eef25454515ea740ad3ef4fa2ea0aec66
layout cli f49606d6e7e73908233e75122ed85a190f0d58d58b4c95187c57df7eff9a4ec7
layout small 92c348190656ecd9fdd39736930e146df12a6150fb1e2e98bc0b552e4421cd18
layout structural b6ce0818ba1077459cc40037a1ea595373548d77999bcde51e27c868bdd42df8
outline cli 01778284428f92b5dc0ba432f765964b878d480565f1e3fbf78f41810ea8913d
outline small 38c096f97507341b8d17c481a6ea4162820a201925e4af5480cafa332124d282
outline structural f3466ac0d7a32a21eebe7503445f7012f88a98f05a558d552cf552bacba573b0
```

</details>

<details><summary>git diff HEAD --stat=240 --stat-name-width=200 do estado recebido</summary>

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
