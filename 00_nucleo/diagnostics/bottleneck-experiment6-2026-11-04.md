# Experimento 6 — eventos de filesystem e cache nativo Fontconfig

Resultado: **eventos servem como sinais de invalidação, sem prova universal pela fila vazia; cache Fontconfig nativo apresentou metadados obsoletos em controles que a descoberta fresca reconheceu**. São duas bancadas independentes. Nenhuma otimização ou alteração produtiva/L0 foi integrada.

## Método comum e alcance

Skill tekt-materializacao-segregada: A escreveu política/probes/runners; B escreveu testes independentes sem consultar A; C revisou, compilou e executou. Todos fontes temporários, caches, builds, executáveis, fixtures, logs e mutantes em /dev/shm/tekt-e6-20261104. Rust 1.92.0 std-only para modelo; cc -O2 para inotify e libfontconfig; Cargo offline/release/jobs1/target RAM para fontdb 0.23.0 usando lockfile do repositório. Dependências do produto não foram atualizadas.

Ambiente observado: Linux 6.17.0-42-generic, fixture em tmpfs RAM; Fontconfig/libfontconfig instalado 2.15.0; inotify max_queued_events=16.384 e max_user_watches=501.556, somente lidos. Não se alteraram limites do kernel. Não se instalou componente novo.

30 testes Rust passaram, incluindo 2.401 sequências de quatro operações no modelo de eventos; dez testes sintéticos do validador Python passaram. Doze mutantes compilados foram rejeitados, 12/12: dispensar validação em estados limpos/sujos; force_fresh derivado do flag errado; evento curando unknown; overflow/perda sem unknown; recusa limpando flags; comparação que ignora ordem, multiplicidade, path, face ou família.

Validador independente aplicado aos dados reais: 11 cenários de eventos e 31 comparações de catálogo passaram nos critérios de integridade e escopo. Divergência nativa é resultado admissível e preservado, não uma falha corrigida ajustando o oracle. O validador não certifica a implementação do observador nem paridade completa do Tekt.

## Bancada A — eventos reais e validação conservadora

Probe C usa inotify_init1 não bloqueante, instala watch antes das alterações normais e drena a fila por comando. Máscara só de mutações: MODIFY, ATTRIB, CLOSE_WRITE, CREATE, DELETE, MOVED_FROM/TO, DELETE_SELF, MOVE_SELF e UNMOUNT. Não usa ACCESS/OPEN/CLOSE_NOWRITE, para que a leitura do próprio oracle não polua os sinais. Overflow é observado pela flag IN_Q_OVERFLOW gerada pelo kernel.

O oracle da fixture enumera recursivamente paths e compara lstat/stat, identidades e hashes de bytes regulares. Depois executa nova varredura conservadora e compara resultados reais, registrando hashes de before/oracle/conservative temporariamente. Ambas varreduras usam o mesmo observador; não são implementações independentes da prova L0. Não certificam atomicidade do filesystem.

O modelo exige validação em toda demanda, mesmo com fila vazia; estados unknown por perda/overflow/falha exigem caminho fresco. Um evento normal marca dirty sem curar unknown; somente validação estável limpa flags. A estratégia conservadora da bancada simplesmente refaz o inventário completo; não implementa watcher incremental do produto nem demonstra ganho de CPU.

|Cenário|Eventos reais drenados|Eventos visíveis à estratégia|Oracle detectou mudança|Validação conservadora coincidiu|
|---|---:|---:|---|---|
|add|3|3|True|True|
|remove|1|1|True|True|
|replace|5|5|True|True|
|chmod|1|1|True|True|
|retarget|5|5|True|True|
|nested-rename|2|2|True|True|
|nested-miss|0|0|True|True|
|late-watch|0|0|True|True|
|lost-simulated|2|0|True|True|
|newdir-watch-gap|0|0|True|True|
|real-overflow|16385|16385|True|True|

Overflow real: 17408 arquivos criados sem drenar a fila, limite 16384; drenagem de 16385 eventos incluiu 1 IN_Q_OVERFLOW. O caso é real, distinto da perda simulada, e não exigiu mudar sysctl.

Contraexemplos de fila vazia: escrita dentro de subdiretório não observado; alteração antes da instalação do watch; criação de arquivo antes de registrar o watch do diretório novo. Na perda simulada, dois eventos reais foram deliberadamente ocultados da aplicação. Em todos, um caminho que mantivesse o cache apenas porque a fila estava vazia conservaria o before, diferente do oracle atual. A estratégia conservadora reconheceu a mudança porque manteve a varredura completa.

O caso newdir-watch-gap registrou o evento do pai e depois verificou que registrar o watch do filho não reproduz retrospectivamente a criação do arquivo. Recuperação correta requer enumerar o conteúdo pertinente; a bancada mostra a lacuna, não implementa um algoritmo portátil completo de registro/barreira.

Falha de backend/rootchange e resets foram testados no modelo. Não se induziram quota de watches, mount sobreposto, mmap, filesystem remoto, entrega tardia fora dos agendamentos da fixture, macOS ou Windows. Portanto não se declara cobertura dessas condições. A comparação de before com oracle é um controle explícito da estratégia ingênua; não representa código atual do Tekt.

## Bancada B — Fontconfig nativo versus descoberta fresca

Fontes e cache nativo isolados na RAM. FcConfigCreate + FcConfigParseAndLoad usa XML próprio com dir e cachedir da fixture e rescan=0; FONTCONFIG_FILE, FONTCONFIG_PATH, XDG_CACHE_HOME e TMPDIR dos subprocessos apontam para a bancada. Não se executou fc-cache sobre diretórios do sistema. Cada probe nativo é um processo novo; FcDirCacheRead(force=false) mede cache em disco, force=true força rescaneamento. Um caso usa FcConfigBuildFonts/FcConfigGetFonts para mudança de diretório configurado.

Oracle desta bancada: fontdb::Database::load_fonts_dir fresco, como ponto de descoberta subjacente ao produto. Compara path, índice e primeira família, preservando ordem/multiplicidade e também conferindo multiset para localizar diferenças. **Não é o FontInfo final do Tekt**: acquire_info/selective, overrides de nomes, cobertura, seleção/scoring, fontes embutidas/projeto e renderização não foram executados. Igualdade desses três campos não certifica paridade de linguagem; diferença de ordem/caminho por si só não prova divergência semântica.

|Caso|Cache normal: ordem / multiset iguais ao fresco|Após rescan forçado: ordem / multiset|
|---|---|---|
|same-size-restored-mtime|False / False|True / True|
|invalid-repaired|False / False|True / True|
|add|False / True|False / True|
|remove|True / True|True / True|
|retarget|False / False|False / False|
|config-dir-change|True / True|não aplicado|
|inventory-order|False / True|False / True|

Substituição com mesmo tamanho/mtime: inicial Noto Sans com padding zero até 759.720 bytes. A precondição confirmou aceitação e mesma família nos dois loaders antes da alteração. Escrita in-place de DejaVu Sans com exatamente 759.720 bytes, restaurando mtime do arquivo e do diretório. Inode, tamanho e mtime foram preservados; ctime mudou, registrado antes/depois. Cache normal continuou retornando Noto Sans; fontdb fresco retornou DejaVu Sans. Force=true corrigiu a família.

Fonte inválida reparada: arquivo inválido de 556.216 bytes virou Noto Sans válido de mesmo tamanho, com mtimes de arquivo/diretório restaurados. Cache normal permaneceu vazio; descoberta fresca encontrou a fonte; force=true corrigiu. ctime mudou. Isso demonstra que a validade nativa observada não substitui automaticamente o regime de identidade/configuração exigido pelo Tekt. A prova L0 do Tekt inclui ctime e candidatos inválidos; não se executou essa prova completa na bancada para alegar certificação adicional.

Após retarget de symlink, o catálogo nativo forçado conservou três caminhos, incluindo alias.ttf; fontdb fresco retornou dois caminhos canônicos sem aquele alias. Na adição e no inventário de cinco fontes, conjuntos de linhas coincidiram, mas a ordem diferiu. Não se normalizaram ou deduplicaram os resultados para esconder isso. A mudança de diretório no XML foi reconhecida no controle nativo de catálogo configurado; aliases Fontconfig não foram auditados.

## Tempos do cache nativo

Três rodadas independentes com duas fontes, cache vazio→quente, sempre processos novos. Clock nativo inclui criação/configuração/leitura do cache, mas é encerrado antes de imprimir as linhas; fontdb inclui Database::new/load_fonts_dir e encerra antes de imprimir. São operações com conteúdo e parsers distintos, não etapas equivalentes de compilação. Cold significa cache nativo removido, não page cache do kernel frio. Ordem cold→warm→fontdb fixa, sem intervalo de confiança.

|Rodada|Nativo cold ms|Nativo warm ms|Fontdb fresco ms|Cache em disco bytes|
|---:|---:|---:|---:|---:|
|0|0.659794|0.137324|0.092862|6704|
|1|0.791128|0.128048|0.097291|6704|
|2|0.632874|0.099595|0.086641|6704|

Medianas: nativo cold 0.659794 ms; warm 0.128048 ms; fontdb 0.092862 ms. O cache nativo acelera seu próprio controle cold→warm nesta fixture; **não foi demonstrada vantagem para a aquisição do Tekt** nem há razão válida para calcular speedup CLI/watch destes tempos. A validação completa continua necessária na hipótese conservadora.

RSS pico dos processos runners, com probes como subprocessos: eventos 65340 KiB; Fontconfig 23744 KiB. Não são RSS isolados das bibliotecas, nem comparação de memória do produto. O burst e os inventários influem no runner de eventos. Cache final da fixture de duas fontes: 6.704 bytes; não extrapolar ao catálogo do sistema.

## Decisão e lacunas

Eventos podem antecipar invalidação, agrupar mudanças ou indicar estado desconhecido. Esta rodada **não autoriza remover stats/varreduras**, e não mediu ganho da variante que preserva a prova completa. Watchers devem observar cobertura real e recuperar perda/overflow/registro de diretórios; ausência de eventos não prova frescor.

Fontconfig pode ser estudado como pista de metadados sob validação, mas a troca direta pelo catálogo atual não está sustentada: houve obsolescência com mesmas identidades fracas e diferenças de representação mesmo após rescan. Force=true resolver dois casos não prova equivalência universal, nem a semântica final de seleção. Validar e reconstruir todos os dados exigidos pode consumir a economia do cache.

Permanecem fora da cobertura: TTC e seus índices/variações nativas, aliases/includes complexos e configurações de projeto, Fontconfig de outras versões, FS de rede/plataformas, custo de catálogo grande e caminho completo CLI/watch. O experimento compara recursos/observações mecânicas; a aceitação de uma mudança produtiva continua por semântica/sintaxe/morfologia, seleção, diagnósticos e render relevantes.

L0 generation.md exclui watcher novo, fc-list/disk cache e provas só por mtime no caminho vigente; font_session exige reconhecimento na próxima recompilação e preservação das barreiras. Nenhum L0 foi alterado para contornar essas condições. Não integrar qualquer das duas variantes com base neste resultado.

## Proveniência e preservação

HEAD 3c6b0ce7b7632cc74b004cea6681e4746e63d28d, working tree não commitado. Registro inicial UTC 2026-11-04T18:48:19.079668+00:00; conferência final UTC 2026-11-04T18:57:17.577417+00:00. Os 2414 arquivos registrados de fontes/L0/diagnósticos permanecem byte a byte iguais. Única adição persistente: este diagnóstico. Todo material de bancada é temporário na RAM e removido no encerramento conforme o skill.

A revisão C retirou IN_ALL_EVENTS antes das execuções para evitar eventos das leituras do oracle e exigiu comparação efetiva entre novas varreduras, substituindo um flag preliminar constante no runner. Na adaptação do formato para B, o caso newdir-watch-gap não tinha flag explícito conservative_validate; sua execução foi conferida no fonte e nos hashes das duas varreduras antes de normalizar. Testes de B não foram alterados. Não se apagaram divergências observadas para passar no validador.

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
