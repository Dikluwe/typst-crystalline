# Fontes: duplicação entre descoberta e aquisição inicial

**Resultado:** CLI e startup do watch repetem aberturas e extração parcial, mas o DTO do fontdb não substitui o FontInfo do Tekt. Em todas as 30 capturas, houve 1.086 arquivos/1.112 faces, 1.086 aberturas na descoberta e 1.112 na aquisição. Os cinco campos comuns divergiram em 482 faces. Não foi implementada nem integrada otimização.

## O que o pipeline realmente repete

Instrumentação descartável do fontdb 0.23.0 e dos consumidores atuais, executada dentro dos binários CLI/watch, sem segunda descoberta introduzida pelo medidor. Eventos preservam path lexical, índice TTC original e ocorrência; embutidas ficam em classe separada. Não há deduplicação de faces, leituras substituídas, novo cache ou mudança de guards/fallbacks.

|Operação por compilação inicial|Descoberta fontdb|Aquisição Tekt|
|---|---:|---:|
|Arquivos distintos|1.086|1.086|
|Aberturas de arquivo|1.086|1.112|
|Faces|1.112|1.112|
|RawFace::parse|1.112|0|
|Face::parse para metadados|0|1.112|
|Face::parse para cobertura|0|1.112|
|Chamadas de conversão de nome|2.641|3.301 to_string + 2.302 MacRoman|
|Leituras posicionais retornadas|0 registradas nesta etapa mmap|11.180|
|Aquisição seletiva aceita / fallback|—|1.112 / 0|

Os 26 índices adicionais de coleção permanecem. RawFace e Face fazem trabalhos diferentes; três parses por face não significam três parses completos equivalentes. Existem ainda 17 parses de metadados das fontes embutidas, fora da junção por arquivo. O fontdb não computa a cobertura exata que o Tekt requer.

Nomes: descoberta tentou ID16 995 vezes, ID1 534 e ID6 1.112; aquisição tentou ID1 1.827, ID4 1.885 e ID6 1.891 vezes. São tentativas de conversão, incluindo insucesso de to_string seguido por MacRoman; não são nomes distintos nem bytes decodificados. O Tekt fez 3.301 buscas find_name nos arquivos; as embutidas acrescentaram 44 buscas/conversões to_string e 26 conversões MacRoman. Os spans globais incluem essas chamadas das embutidas. Os IDs procurados e as políticas de escolha diferem.

Fonte: fontdb original lib.rs:267 (open/mmap),1034 (RawFace),1057 (nomes),1145 (OS/2),1167 (post); [fontdb.rs](../../03_infra/src/fontdb.rs:22), [fonts.rs](../../03_infra/src/fonts.rs:372) e [selective.rs](../../03_infra/src/fontdb/selective.rs:200). O fork só observou operações já existentes.

Controle independente com strace na CLI: baseline e sonda desligada contaram, cada um, 2.199 openat, 1.087 mmap, 2.199 close e 11.180 pread64 sobre paths do catálogo. A abertura/mapping adicional vem do carregamento final da fonte usada; não pertence às 2.198 aberturas descoberta+aquisição. Trace não foi usado como relógio, nem prova por si só equivalência de todos os pedidos.

## Bytes: sobreposição de tabelas não é leitura física duplicada

Unidades decimais abaixo. Mapeamentos somaram 4.296.275.016 bytes de extensão virtual. Isso não mede pages tocadas, RSS de fontes ou tráfego de disco. O medidor não contou page faults nem bytes físicos lidos pelo mmap.

A aquisição retornou 51.122.918 bytes por read_at, iguais ao orçamento solicitado observado; não houve erro de read. União por path das faixas retornadas:51.081.340 bytes. O valor menor elimina sobreposições de prefixo/header/coleção, sem autorizar eliminar essas observações.

|Tabela retida na projeção|Soma por ocorrência de face, bytes|
|---|---:|
|post|32.933.496|
|cmap|14.526.173|
|name|3.090.753|
|OS/2|106.412|
|head|60.048|
|hhea|40.032|
|maxp|28.694|

O restante das leituras inclui cabeçalhos, diretórios e prefixos relidos. União por path dos intervalos retidos:50.781.572 bytes. União das views de name/OS/2/post oferecidas ao fontdb:307.254.277 bytes; interseção com intervalos retidos/lidos pela aquisição:36.127.717 bytes. **Essa interseção descreve intervalos declarados de tabelas, não 36MB de processamento repetido comprovado.**

A distinção muda a prioridade: o fontdb recebe uma slice post inteira, porém seu parser consulta somente offsets 4..8 e 12..16. São 8 bytes por face (8.896 somados; união por path 8.688), não os 304.060.056 bytes das views post. Portanto os 32,93 MB de post transportados pela aquisição não são leitura equivalente já feita pela descoberta. Name/OS2 também oferecem slices cujo tamanho não equivale a bytes efetivamente acessados.

Em 823 faces o post retido ultrapassa 32 bytes;32.897.912 bytes estão além desses 32. A fonte desse volume é o prefixo v2 contendo índices de nomes de glifos, exigido pela projeção atual para passar no parser tipado. O catálogo usa metadados/cobertura, não nomes de glifos. Isto delimita uma hipótese de redução de transporte; não demonstra que esses bytes possam ser removidos com correção, nem quantifica ganho de CPU.

## Metadados: por que não reutilizar FaceInfo diretamente

Comparação ordinal por(path,índice) e ocorrência, com stretch normalizado para unidades 1000:630 pares iguais nos cinco campos comuns e 482 diferentes. Divergências por campo (não exclusivas): família 479, estilo 16, peso 6, stretch 2, monospace 0. Todos os 1.112 pares foram comparados; nenhuma igualdade prova identidade do recurso ou geração.

Exemplo do host: IosevkaTermSlab Nerd Font no fontdb e IosevkaTermSlab NF no Tekt. O fontdb prioriza ID16/aliases/EnglishUS; o Tekt usa ID1, aparo de sufixos, inferência por nome completo e exceções PostScript. A ausência de diferenças em monospace nesta máquina não prova universalidade. Serif/PANOSE e cobertura não estão disponíveis no DTO comparado do fontdb. Mesmo os 630 pares coincidentes não autorizam substituir a derivação.

O [L0 de fonts](../prompts/infra/fonts.md) exige essas regras; [fontdb](../prompts/infra/fontdb.md) exige ordem, pairing, cobertura e fallback, e [selective](../prompts/infra/fontdb/selective.md) exige guards frescos após derivação e proíbe fabricar versão/pitch/índices post. Nenhum foi alterado. A descoberta inicial já é preparada uma vez para o watch; não se encontrou uma segunda varredura inicial Tekt a eliminar.

## Tempos e efeito do medidor

Três rodadas com ordem baseline/off/on rotacionada; cinco entradas: texto curto, import com 2.000 bindings, import com loop 10.000, layout 100 parágrafos e outline 12 seções. Cada combinação CLI/startup roda num processo novo, mesma afinidade CPU0, RAYON_NUM_THREADS=1, stack 64MiB, SOURCE_DATE_EPOCH=0 e PDF sem tags. Caches do host aquecidos; sem limpeza de page cache e sem build/strace concorrentes no lote cronometrado. Não é medição de boot frio.

Wall em ms até fim do processo CLI ou primeiro PDF completo no watch (poll 2ms). Medianas de três amostras; não são precisão universal nem ganho de candidato.

|Fixture|CLI baseline / off / on|Watch startup baseline / off / on|
|---|---:|---:|
|plain|99.98 / 101.17 / 203.98|143.41 / 144.08 / 249.73|
|import_syntax|103.15 / 103.76 / 207.05|142.54 / 145.42 / 250.91|
|import_compute|103.01 / 101.76 / 206.99|147.87 / 146.24 / 247.41|
|layout|123.27 / 121.66 / 228.79|166.78 / 166.66 / 272.36|
|outline|222.82 / 226.92 / 326.22|262.04 / 265.81 / 376.91|

Off é o binário instrumentado com a sonda desligada; on coleta eventos, spans e serializa o dump antes da publicação. Na entrada curta, CPU CLI foi 95,15 /96,10 /199,23ms. O overhead on é cerca de 104ms CLI e 106ms wall no startup dessa entrada; o coletor altera muito o tempo/RSS. Não subtrair esse total proporcionalmente dos componentes. Off variou em relação ao baseline e não é controle perfeito de código gerado.

CLI CPU usa rusage do filho e RSS máximo via time. Watch CPU é soma de schedstat dos threads ainda presentes, amostrada 80ms após o primeiro PDF; não inclui threads já encerrados e não delimita precisamente a publicação. RSS do watch é amostra nesse ponto, distinto do pico CLI. Na entrada curta, pico CLI baseline/on 24,54/72,83MiB; watch RSS 25,39/52,20MiB. Esse aumento é da instrumentação/journal, não custo de um cache proposto.

Spans inclusivos abaixo são sinais do binário **on**, medianas das 15 capturas por modo; não representam o custo original sem sonda. Componentes aninhados e medianas independentes não devem ser somados.

|Span|CLI ms|Startup watch ms|
|---|---:|---:|
|discovery.total|43.661|42.366|
|discovery.face|9.492|9.121|
|discovery.raw_face|0.337|0.472|
|discovery.parse_names|6.498|6.165|
|discovery.parse_os2|0.404|0.432|
|discovery.parse_post|0.453|0.445|
|acquisition.total|64.540|63.367|
|acquisition.open|4.851|4.737|
|acquisition.project|34.382|33.571|
|acquisition.font_info_from_bytes|6.505|6.431|
|acquisition.coverage_face|0.634|0.622|
|acquisition.coverage|7.617|7.843|
|acquisition.guards|4.265|4.226|

Discovery.total cobre fontdb::load_system_fonts, não toda a preparação do mundo. Acquisition.total cobre as 1.112 aquisições de arquivo. Font_info_from_bytes inclui 17 embutidas adicionais nesse rótulo; cobertura é medida separadamente. O custo líquido eliminável por compartilhar descoberta/aquisição permanece **desconhecido**: não houve contrafactual otimizado nem sonda de baixo overhead para atribuí-lo.

## Próxima bancada delimitada

1. Antes de escolher uma otimização entre descoberta/aquisição, medir os agregados com timers de baixo overhead, sem journal de cada leitura. As contagens atuais mostram trabalho repetido, mas seus tempos detalhados não sustentam promessa de ganho.
2. Priorizar investigar o transporte do prefixo post: ele responde por 64,4% dos bytes retornados na aquisição, enquanto o fontdb só consulta 8 bytes por face. Candidato de pesquisa: projeção específica de metadados que evite transportar índices de glifos sem mudar validade, campos, cobertura, fallback, budgets ou guards. O L0 atual proíbe simplesmente substituir índices/versão; qualquer código produtivo depende primeiro de atualização do owner, classificação ADR0127 e prova adversarial. Testar versões post válidas/inválidas, contagem/arrays truncados, duplicatas, TTC, limites e mutações concorrentes, além do corpus real e ganho líquido CLI/startup. Não é candidato seguro já aprovado.
3. Compartilhar bytes entre descoberta/aquisição fica como hipótese posterior: requer provar identidade e janela de observação, sem transformar mmap mutável em snapshot, alterar fallback/retained SharedFile ou conservar catálogo obsoleto. Igualdade path/índice ou DTO não basta. O volume de views sobrepostas não prova seu benefício.

Não há evidência para memoizar FaceInfo→FontInfo como substituição geral. A segunda Face::parse dentro da aquisição é repetição mensurada, mas seu span pequeno e instrumentado não justifica tratá-la como gargalo principal. A prova de frescor e o watch aquecido continuam assuntos separados.

## Validação e limites

Skill Tekt com A implementando a instrumentação, B testes independentes e C integração/execução. Passaram 17 testes do coletor e 16 do analisador; 12/12 mutações dirigidas foram rejeitadas (4 coletor,8 analisador). A omissão inicial de OS/2 no filtro do analisador foi corrigida com teste próprio, e as 30 capturas foram reanalisadas. A instrumentação não alterou o parser da fonte.

Noventa execuções: 60 pares de texto, geometria de palavras/páginas e nomes de fontes preservados; 20 pares raster 72dpi de todas as páginas da primeira rodada preservados. Trinta journals completos, cada um com 65.171 eventos, sem erros, descarte ou spans abertos, mesmos intervalos por path e mesma distribuição 630/482. Um controle adicional editou Alpha→Beta no watch: segunda compilação publicou Beta e registrou zero descoberta/aquisição de arquivos; não se infere aceleração desse ciclo.

Inventário original fontdb antes/depois: mesma ordem path/índice e mesmos dev/ino/size/mode/mtime_ns/ctime_ns dos 1.086 arquivos. Isso qualifica o corpus estável observado, não cria snapshot atômico ou teste de concorrência. Não foram executadas as suítes completas de core/infra nem certificação de fontes/plataformas ausentes. Builds release original/instrumentado passaram; crystalline-lint . terminou com exit 0, sem violações bloqueantes (notas informativas existentes). Todos os arquivos recebidos foram verificados byte a byte e preservados; somente este diagnóstico e amostras locais de domínio foram acrescentados.

## Proveniência e reprodução

HEAD `3c6b0ce7b7632cc74b004cea6681e4746e63d28d`, uncommitted working tree. Congelamento `2026-11-05T02:55:17.466931+00:00`. Lote principal `2026-11-05T03:24:10.642879+00:00` até `2026-11-05T03:24:54.482964+00:00`. O código original permaneceu fixo; a cópia RAM recebeu apenas hooks de observação e fork fontdb. As mudanças de testes/analisador posteriores não mudaram binários nem entradas do lote. Lista rastreada exata do estado recebido ao final.

Baseline SHA-256 `902cfde0215fe0a4c5e4e9713820a1eae1b339e7aaa0d8005439849c2fdaa84c`; instrumentado `c177342be465ca2662ecb9a1f7d9a4f89f86c1ea18a2bd680c88b2a1d4bf6602`. Não houve nova medição contra vanilla; nenhum resultado daqui certifica outro pin.

Rust 1.92.0, Linux 6.17.0-42-generic x86_64, fontdb0.23.0, ttf-parser0.25.1. Cópia do workspace/lock recebido; único pacote adicional foi a sonda local, sem atualização de versões existentes. Cargo offline/locked release, jobs 1, incremental 0, mesmo TYPST_COMMIT_SHA do HEAD. Builds em unidade com MemoryHigh 8G/MemoryMax 12G/MemorySwapMax 0; toda compilação, fork, testes, PDFs, logs e mutantes descartáveis ficaram em /dev/shm/tekt-font-dup-20261104 e foram removidos após a verificação. Nenhuma limpeza de outras bancadas.

Receita de entradas: prefixo `#set page(width:400pt,height:600pt)`; plain=`Alpha text.`; import_syntax importa value=v1999 de2.000 bindings; import_compute importa soma 0..9999; layout itera 100 parágrafos; outline executa outline e 12 headings com lorem(30)/pagebreak. Flags comuns: `compile|watch entrada saida --root fixtures --pdf-tagged=false`. CLI cronometra o processo completo; watch encerra após publicar o primeiro PDF, sem edição nas amostras de startup. As hashes das entradas principais estão no CSV; os [sete arquivos de entrada](../../lab/results/construction/diagnostics/font-discovery-acquisition-2026-11-05/inputs/) preservam também os imports. As capturas foram observacionais, sem candidato alterando aquisição.

[Amostras](../../lab/results/construction/diagnostics/font-discovery-acquisition-2026-11-05/amostras.csv), [spans inclusivos/exclusivos em ns](../../lab/results/construction/diagnostics/font-discovery-acquisition-2026-11-05/fases.csv) e [comparação por face](../../lab/results/construction/diagnostics/font-discovery-acquisition-2026-11-05/faces.csv). São dados locais de domínio sob o ignore de lab, não foram forçados ao Git. Não se preservaram journals/logs/manifestos descartáveis.

<details>
<summary>git diff HEAD --stat do estado recebido</summary>

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
