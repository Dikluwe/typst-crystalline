# Fontes na CLI e no watch e candidato de leitura posicional

**Resultado:** nas fixtures medidas, o Tekt já foi mais rápido que o vanilla na CLI pequena, mas gastou mais CPU nos ciclos watch. A prova quente continua dominada por metadados e configuração. O candidato mais delimitado desta rodada é usar leitura posicional na aquisição seletiva da CLI, mantendo bytes, limites e rechecagens. Ele passou na bancada isolada; não foi integrado nem medido dentro do pipeline. Não foi encontrado um candidato de ganho grande no watch que preserve a garantia atual.

## Comparação com o vanilla e contrafactual sem fontes do sistema

Baseline Tekt compilado do working tree atual na RAM; oracle vanilla ratificado em 64c5e8c02fe0394d913ead36e42ef8c1334d2107, árvore 483bd6f1d8ac8810db47b65336d402647955fd17. SHA do oracle confirmado e três blobs CLI world/watch/fonts comparados ao pin. Não se usou o release antigo do produto como estado atual.

A opção --ignore-system-fonts existe no vanilla compile, mas a CLI produtiva do Tekt atual a aceita apenas na listagem de fontes. A primeira tentativa detectou isso e foi interrompida. O contrafactual Tekt abaixo usa exclusivamente a cópia RAM com E7_NO_SYSTEM=1: omite descoberta/prova de fontes do sistema e conserva ordem de composição, fontes embutidas e de projeto. Não é uma opção adicionada ao produto nem sua implementação é candidata de integração. O baseline com sistema usa o binário original recém-compilado, não essa ablação.

Três rodadas, cinco fixtures anteriores, ordem dos binários intercalada; CLI em processos novos e watch com startup, mesmos bytes, edição pequena e estrutura. Afinidade em um CPU, RAYON_NUM_THREADS=1, SOURCE_DATE_EPOCH=0 e --pdf-tagged=false nos dois binários, configurações de fontes do host iguais. Não se limpou page cache. CLI por communicate(timeout), evitando polling quantizado de wait(timeout); watch latência por publicação PDF e CPU por /proc/schedstat. CPU e latência são medidas diferentes, e a CPU watch inclui a janela de estabilização do observador após publicação. RSS CLI é pico; watch residente é separado.

300 registros comparáveis concluídos, todos com texto da revisão esperada e publicação observada. Controle adicional da primeira rodada: em cada variante, as cinco fixtures mantiveram palavras/posições extraídas, dimensões de página e nomes de fontes PDF entre os modos com/sem sistema. Isso é controle da ablação nessas fixtures latinas; não certifica seleção/fallback universal ou paridade geométrica entre produtos. Ignorar fontes altera o conjunto disponível e não define piso universal para documentos que dependem delas.

Medianas em ms. I é contrafactual sem sistema; no Tekt é experimental, no vanilla é a opção nativa.

|Fixture|Cenário|Tekt sistema CPU / wall|Tekt I CPU / wall|Vanilla sistema CPU / wall|Vanilla I CPU / wall|
|---|---|---:|---:|---:|---:|
|plain|cli/startup|96.736 / 98.548|9.822 / 10.648|125.161 / 127.129|10.751 / 11.594|
|plain|watch/startup|138.291 / 141.109|7.233 / 8.430|128.371 / 131.373|8.620 / 10.480|
|plain|watch/samebytes|16.827 / 72.787|3.315 / 58.843|0.623 / 101.943|0.976 / 102.145|
|plain|watch/small|19.002 / 72.885|3.473 / 58.707|1.280 / 102.107|1.541 / 102.061|
|import_syntax|cli/startup|97.105 / 98.706|16.651 / 17.509|128.794 / 130.946|13.657 / 14.488|
|import_syntax|watch/startup|137.923 / 140.603|13.867 / 16.570|125.249 / 128.387|11.497 / 13.051|
|import_syntax|watch/samebytes|21.143 / 77.004|9.913 / 66.875|1.228 / 102.170|0.990 / 102.650|
|import_syntax|watch/small|23.178 / 78.780|9.248 / 64.928|1.088 / 101.526|1.742 / 102.186|
|import_compute|cli/startup|97.942 / 99.467|16.985 / 17.732|127.135 / 129.473|13.637 / 14.487|
|import_compute|watch/startup|140.518 / 143.116|14.537 / 16.707|133.350 / 136.613|11.365 / 12.567|
|import_compute|watch/samebytes|23.791 / 81.703|12.153 / 66.554|0.911 / 102.035|0.954 / 101.933|
|import_compute|watch/small|22.464 / 78.917|12.124 / 67.052|1.182 / 101.936|1.509 / 103.249|
|layout|cli/startup|117.291 / 118.999|29.100 / 29.920|129.928 / 132.165|15.587 / 16.408|
|layout|watch/startup|160.750 / 163.275|27.006 / 29.185|127.333 / 130.690|13.611 / 14.732|
|layout|watch/samebytes|35.168 / 83.110|25.570 / 70.679|1.995 / 103.752|1.928 / 104.109|
|layout|watch/small|37.702 / 86.295|25.368 / 72.787|4.089 / 105.529|4.292 / 104.472|
|outline|cli/startup|218.455 / 220.377|130.223 / 131.158|133.817 / 135.994|18.672 / 19.529|
|outline|watch/startup|258.509 / 261.144|131.311 / 133.425|130.946 / 134.715|16.751 / 18.873|
|outline|watch/samebytes|130.630 / 230.295|113.761 / 215.653|2.823 / 103.966|2.659 / 103.283|
|outline|watch/small|128.457 / 218.752|112.606 / 213.458|5.449 / 106.387|3.979 / 104.730|

No texto pequeno: CLI 96,736 ms CPU Tekt versus125,161 ms vanilla. No watch samebytes:16,827 ms versus0,623 ms CPU, mas latência72,787 ms versus101,943 ms. A diferença de debounce/publicação impede equiparar CPU à responsividade. Startup vanilla inclui aqui a varredura que seu contador exibido de compilação exclui. Outline permanece uma classe distinta: a ablação de fontes não elimina seu alto custo de eval/layout.

O contrafactual permite ver a escala do trabalho de fontes no cenário, mas não fornece uma economia implementável por simples subtração de medianas. A comparação com vanilla tampouco prova que toda diferença seja uma única função, porque os compiladores têm pipelines/caches próprios.

## Decomposição agregada de fontes

A sonda E7 está somente na cópia RAM e descarrega uma linha JSON por compilação. Tempos são spans wall monotônicos agregados, com filhos subtraídos no exclusivo; não são CPU exclusiva universal. Contadores locais e globais são mantidos em escopos distintos. Não se somam pais/filhos nem medianas para construir um total. Fontdb e parser de configuração externos têm envelopes, sem decomposição interna completa.

Mais300 registros off/on foram coletados com e sem sistema; a validação independente incluiu300 comparações e150 registros off/on com sistema. Perfil e comparação de produtos foram executados separadamente, sem build concorrente. Todos os dumps emitidos terminaram com pilha vazia.

Texto pequeno com sistema: medianas dos spans ligados na CLI e watch samebytes, três amostras.

|Caminho|Fase|Chamadas|Wall inclusivo ms|Wall exclusivo ms|
|---|---|---:|---:|---:|
|cli|cli.fresh_catalog|1|92.093586|0.127437|
|cli|cli.fontdb_discovery|1|35.381555|35.381555|
|cli|cli.catalog_from_database|1|56.153585|0.363476|
|cli|cli.selective_acquire|1112|55.668028|3.168829|
|cli|cli.selective_project|1112|33.454239|7.418982|
|cli|cli.selective_read|11180|16.046159|16.046159|
|cli|cli.selective_seek|11180|4.474512|4.474512|
|cli|cli.selective_open|1112|4.336442|4.336442|
|cli|cli.selective_handle_stat|2224|2.264183|2.264183|
|cli|cli.selective_path_stat|1112|2.930952|2.930952|
|cli|cli.font_info|1129|2.007438|1.358739|
|cli|cli.info_face_parse|1129|0.644691|0.644691|
|cli|cli.selective_coverage_face_parse|1112|0.252553|0.252553|
|cli|cli.coverage|1112|7.696365|7.696365|
|cli|cli.combine_catalog|1|0.165306|0.165306|
|watch|watch.session_catalog|1|15.579562|0.000420|
|watch|watch.config_key|1|3.448377|0.080810|
|watch|watch.config_parse|1|2.696483|2.696483|
|watch|watch.config_read|55|0.423346|0.423346|
|watch|watch.config_aliases|1|0.194430|0.194430|
|watch|watch.probe_verify|1|8.831102|0.171562|
|watch|watch.generation_verify|1|3.264495|0.109219|
|watch|watch.lstat|1345|4.307792|4.307792|
|watch|watch.stat|2456|7.055688|7.055688|
|watch|watch.config_probe_lstat|2|0.013154|0.013154|
|watch|watch.path_clone|1345|0.071370|0.071370|
|watch|watch.session_compare|1|15.579101|0.001583|

CLI:1.112 faces de sistema; 11.180 requisições seletivas e51.122.918 bytes solicitados pelo projetor. Não é todo o I/O do processo; descoberta fontdb, mmap e parser interno têm custos próprios. Metadados/cobertura foram adquiridos pelo caminho seletivo nas amostras; não houve chamada medida ao fallback de aquisição. Uma contagem strace separada do baseline confirmou11.180 lseek,11.299 read,4.678 statx e2.334 openat, mais dois pread64 de outras operações. Tempos de strace não entram nos benchmarks.

Watch quente:1.345 lstat de ProbeKey +2.456 stat de ProbeKey/GenerationKey +dois probes de configuração =3.803 consultas nos wrappers instrumentados. A configuração executou55 leituras explícitas adicionais além do trabalho interno de seu parser. Não houve nova aquisição seletiva de fonte nos ciclos quentes medidos. Resource.font_get permaneceu cached. As rechecagens não foram removidas ou agrupadas.

Copiar caminhos consumiu cerca de0,071 ms wall instrumentado por ciclo pequeno; a comparação local da sessão foi muito menor. Não sustenta atacar16 ms de custo com essa micro-otimização. O segundo parse de Face para coverage ficou em cerca de0,253 ms por aquisição CLI completa, também pequeno diante de descoberta/leitura/projeção.

Overhead observado (CPU off/on, ms), separado dos tempos da tabela de fases:

|Fixture|CLI off / on|Watch samebytes off / on|
|---|---:|---:|
|plain|95.949 / 101.072|19.496 / 19.299|
|import_syntax|96.324 / 101.722|23.637 / 24.261|
|import_compute|98.150 / 104.650|24.245 / 24.340|
|layout|118.832 / 123.753|37.486 / 39.742|
|outline|246.464 / 222.992|130.878 / 132.000|

A CLI pequena teve cerca de5 ms adicionais com sonda; outline apresentou variação de sinal oposto. Portanto os spans pequenos não são custos exatos do produto sem sonda, nem se demonstra overhead uniforme. As amostras não atendem uma alegação geral de precisão abaixo de3%; nenhum percentual preciso de aceleração é promovido. O escopo seguro do candidato vem também da contagem de operações e dos testes, não apenas dos nanossegundos.

## Candidato delimitado para a aquisição CLI

**Leitura posicional no File concreto de selective::acquire**, sob o projetor e os guards atuais. Hoje cada Reader::get faz seek(Start(offset)) seguido de read_exact. O adaptador de bancada mantém um cursor privado e atende Read usando FileExt::read_at, mantendo os seeks lógicos sem executar lseek físico. O projetor original foi extraído para a bancada, não reescrito para aceitar outro formato.

Escopo de segurança: mesmas regiões, mesmos comprimentos, mesmos buffers projetados, ordem de registros e duplicatas; mesmos limites4.096 tabelas,8 MiB de buffer e16 MiB cumulativos pedidos; mesmo tratamento de índices/sfnt/TTC e recusa em formatos inválidos. Abrir o arquivo e as duas verificações de handle mais a de caminho permanecem. A bancada isolada não executa FontInfo/coverage entre projeção e guard final; em eventual integração, as verificações devem permanecer após essas etapas como no acquire atual, sem antecipação de barreiras. Nenhum cache de identidade, bytes ou resultado entre faces, arquivos ou compilações. O offset do File é interno a essa aquisição e não é consumido depois pelo caller; o projetor usa apenas SeekFrom::Start. Não generalizar o adaptador para qualquer uso de SeekEnd diante de arquivo mutável.

A bancada comparou sete fixtures reais e dez faces do TTC NotoSansCJK do host, três rodadas com ordem alternada:102 operações, buffers exatos iguais. Por variante/rodada foram190 leituras e2.465.448 bytes, com os mesmos seeks lógicos. Baseline executou190 seeks físicos; candidato zero, com190 leituras posicionais. Guardas em cada projeção bem-sucedida:2handle metadata +1path metadata.

|Rodada|Baseline projeção total ms|Posicional projeção total ms|
|---:|---:|---:|
|0|1.794014|1.604662|
|1|1.869162|1.341963|
|2|1.270598|1.461052|

Os tempos são do projetor isolado, com metadata/open/alocação da bancada, não da CLI. Variam entre rodadas; não extrapolar um percentual de ganho aos1.112 arquivos/faces. O envelope instrumentado de seeks no caminho real, cerca de4,47 ms, delimita a ordem de grandeza da parcela visada, com overhead incluído; não é promessa nem teto matemático do ganho end-to-end. O candidato não reduz descoberta fontdb, cobertura, conteúdo lido ou os3.803 probes do watch.

29 testes independentes passaram: buffers esperados sintéticos e reais, TTF/OTF/variável/TTC, faces distintas com offsets absolutos, truncamentos/bounds, limites, prefixos post e recusa de mudança pós-projeção com hooks para escrita in-place/substituição com mtime restaurado, symlink e remoção. Sete mutantes do candidato foram rejeitados: offset errado, cursor não atualizado, face TTC errada, omissão da comparação de caminho, omissão de ctime e enfraquecimento de caps de buffer/tabelas. Falhas parciais de pread/EINTR não foram injetadas pela API atual; continuam gate futuro.

Conclusão: candidato conservador aprovado apenas nesses controles de bancada, não otimização validada no produto. Próximo pacote, se autorizado: auditar/atualizar L0 selective antes de qualquer implementação, manter leitor genérico/fallback e seus testes de falha, validar FontInfo/coverage/render no caller real e medir ganho líquido CLI/startup/RSS. Não escolher esse candidato como solução para o watch quente.

## Avaliação das sugestões sobre frescor

**Vanilla:** fonte/pin conferidos confirmam LazyLock do catálogo e reset de arquivos/relógio sem reconstruir fontes. O watch faz scan antes da primeira compilação paginada. Contudo, dizer que toda alteração de fonte só aparece após reiniciar é mais forte que o fonte sustenta: um slot ainda não demandado pode ler bytes atuais ao primeiro get. O catálogo não é automaticamente rescaneado; slots carregados têm retenção própria. A medição de startup acima inclui essa varredura, em vez do tempo exibido que a exclui.

**Contrato:** a política de atualização pode ser uma decisão de produto distinta da do vanilla. ADR-0107 permite mecânicas diferentes, mas não revoga o L0 atual nem torna qualquer redução de frescor transparente. Validar só a fonte usada pode deixar de reconhecer uma fonte nova que venceria a seleção ou uma alteração de alias/configuração. Recarregar apenas sob comando/reinício também muda a garantia. Nenhuma dessas opções foi escolhida ou implementada nesta rodada; exigiriam proposta concreta e gate ADR-0127.

**Modelo de ameaça e custo de conteúdo:** geração usa dev/ino/size/mode/mtime/ctime com nanossegundos. A função de identidade produtiva foi extraída literalmente para os testes de bancada:13 testes passaram, incluindo substituição e escrita in-place com tamanho/mtime restaurados, com detecção por inode/ctime e sem leitura de payload na função. Onze mutantes foram rejeitados. Não se presume que metadata falhe ao retirar permissão de leitura; mode/ctime permitem reconhecer a mudança.

Esses controles incluem publicação normal com timestamps preservados e substituição atômica, além de robustez; não exigem ler toda a fonte para descobrir a mudança. Os46 MB do experimento3 eram leituras explícitas daquela bancada conservadora, não custo obrigatório da prova quente. Os51 MB seletivos medidos nesta CLI são pedidos para extrair metadados/cobertura na aquisição fresca, outra operação. Não misturar essas origens.

Limite da garantia: alterações após a última observação podem escapar; não há snapshot atômico de todo filesystem, nem defesa demonstrada contra forja de todos os campos de identidade ou filesystems com semânticas diferentes. Um teste real de alvo sem extensão modificado fora do diretório do symlink também passou, mantendo timestamps do diretório do alias: diretório sozinho não bastava. Validação por ctime não deve ser vendida como universal em qualquer plataforma; caminhos não elegíveis conservam fallback atual.

## Decisão desta rodada

Manter o contrato atual durante o trabalho técnico. Para CLI, levar somente leitura posicional ao próximo gate de especificação/validação, com expectativa restrita à aquisição seletiva. Para watch, registrar que o custo está majoritariamente nas observações/configuração exigidas e que as micro-opções medidas não prometem grande redução. Uma meta de redução substancial pode exigir uma nova prova completa equivalente ou uma decisão explícita sobre o contrato de atualização; nenhuma foi obtida por esta medição.

A política de frescor continua uma escolha do proprietário, não uma conclusão derivada de uma comparação de tempo. Esta rodada não registra uma aprovação nova de aceitar ou enfraquecer o custo, apenas descreve o estado ratificado e as consequências das alternativas.

## Proveniência e preservação

HEAD 3c6b0ce7b7632cc74b004cea6681e4746e63d28d, working tree não commitado. Início UTC 2026-11-04T19:28:43.539342+00:00; conferência final UTC 2026-11-04T19:58:54.651886+00:00. Os 2420 arquivos registrados permaneceram byte a byte iguais antes desta adição. Nenhuma fonte/L0/relatório anterior mudou.

SHA baseline `aa7689fb5e6f5628ef4d5870686dd1ff3a42fb3c1904449568904e1662113c81` (igual ao da primeira rodada); oracle `22897f93c7c2c8589cf5913b0ce735f1c01ba711c70f55c7a350f7c4b96c0b34`; sonda/ablação `601e868f41a07e48fe3cc8b8cab73e43806041e14efbcc503a3e2a688cfc13ac`. Baseline Rust1.92 offline/locked/release na RAM, jobs1, MemoryHigh8G/Max12G/swap0:8m30s; cópia instrumentada3m24s. Benchmarks sem build concorrente. Corpus, argumentos, flags experimentais e ambiente foram registrados nos resultados temporários. Não se alegou identidade de fonte por --version.

Skill tekt-materializacao-segregada: A implementou instrumentação e candidato isolado; B escreveu testes independentes sem consultar A; C integrou e executou.16 testes sintéticos do validador passaram; os dados completos passaram nos controles de cobertura/proveniência. O validador não certifica a semântica das sondas. Fontes experimentais, testes, logs e executáveis serão removidos da RAM no encerramento, conforme o skill; o resultado persistente é este diagnóstico.

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
