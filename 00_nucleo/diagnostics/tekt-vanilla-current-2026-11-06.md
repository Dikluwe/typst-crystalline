# Tekt atual × vanilla — CLI, startup e watch aquecido

O lote atual mede o produto depois do reúso adiado da stdlib, da correção de float e do Box autorizado. São os mesmos bytes de entrada para os dois compiladores, com um marcador de publicação acrescentado em todos os modos. O código recebido foi preservado; nenhuma otimização ou mudança de contrato foi integrada nesta comparação.

## Medições no mesmo lote

Medianas em **ms, Tekt / vanilla**, de 12 rodadas pareadas. Primeiro, wall até publicação completa do PDF:

| Fixture | CLI | Startup watch | Quente: mesmos bytes | Quente: edição inédita |
|---|---:|---:|---:|---:|
| Texto curto | 95,38 / 130,72 | 142,20 / 130,02 | 87,69 / 101,48 | 87,72 / 101,96 |
| Import: 2.000 bindings | 97,63 / 133,87 | 145,28 / 130,55 | 92,23 / 101,62 | 92,06 / 102,01 |
| Import: loop 10.000 | 97,45 / 132,96 | 142,43 / 131,98 | 94,34 / 101,51 | 93,97 / 102,02 |
| 16 imports locais | 101,59 / 129,93 | 147,21 / 129,26 | 95,91 / 101,79 | 95,49 / 102,24 |
| 100 parágrafos | 116,57 / 134,86 | 162,67 / 133,89 | 107,28 / 102,33 | 107,57 / 103,89 |
| Outline † | 221,43 / 139,16 | 267,38 / 137,91 | 206,22 / 103,20 | 205,10 / 104,33 |

CPU dos mesmos processos/intervalos, separada da espera do watch:

| Fixture | CLI | Startup watch | Quente: mesmos bytes | Quente: edição inédita |
|---|---:|---:|---:|---:|
| Texto curto | 93,74 / 128,70 | 140,01 / 128,04 | 18,10 / 0,86 | 18,02 / 1,37 |
| Import: 2.000 bindings | 96,07 / 131,82 | 143,37 / 127,99 | 22,58 / 0,90 | 22,81 / 1,50 |
| Import: loop 10.000 | 95,76 / 130,90 | 140,58 / 129,75 | 24,70 / 0,88 | 24,15 / 1,41 |
| 16 imports locais | 100,01 / 127,95 | 144,75 / 127,18 | 26,52 / 1,22 | 25,94 / 1,81 |
| 100 parágrafos | 114,99 / 132,63 | 160,45 / 131,20 | 37,70 / 1,88 | 37,88 / 3,36 |
| Outline † | 219,87 / 137,18 | 265,64 / 135,22 | 137,63 / 2,61 | 136,44 / 3,68 |

† Outline falha nos controles bilaterais de texto/geometria/pintura. Seus tempos são descritivos para a mesma entrada, não velocidade para trabalho equivalente. As outras cinco fixtures passam nos controles declarados.

As quatro publicações quentes de cada tipo são promediadas dentro do processo/rodada antes da estatística. Não são quatro amostras independentes. O texto curto publica mais cedo no watch neste protocolo, mas consome aproximadamente 18,02 ms de CPU contra 1,37 ms após edição inédita. A vantagem de wall não significa menor trabalho de compilação.

[Dados brutos normais](../../lab/results/construction/diagnostics/tekt-vanilla-current-2026-11-06/amostras-normal.csv), [medianas, razões pareadas e IC95%](../../lab/results/construction/diagnostics/tekt-vanilla-current-2026-11-06/resumo-normal.csv). Os intervalos exploratórios usam 10.000 reamostragens das rodadas, seed 0 e percentis empíricos; percentuais são medianas de razões por rodada, não razões das medianas desta tabela. Não houve correção de múltiplas comparações ou descarte posterior de rodadas.

## Fontes: custo inicial e prova de frescor

Uma cópia instrumentada separada do mesmo produto, sem candidato de otimização, observou no texto curto:

| Escopo L3 | CLI ms | Startup ms | Edição quente ms |
|---|---:|---:|---:|
| Catálogo inteiro, inclusivo | 84,927 | 131,221 | 13,992 |
| Descoberta fontdb | 36,255 | 36,260 | não chamado |
| Aquisição seletiva/metadados | 47,948 | 47,581 | não chamado |

No startup, as chamadas sequenciais da sessão observadas foram probe inicial 24,243 ms, preparação 42,443 ms, verificação preparada 2,864 ms, construção 48,356 ms, verificação final da geração 3,192 ms e verificação final rápida 9,695 ms. A preparação contém descoberta fontdb; a construção contém aquisição. Não somar esses pais com seus filhos nem exigir que a soma de medianas reproduza a mediana do total.

No aquecido do texto curto, o catálogo é reutilizado após validação: verificação rápida inicial 11,106 ms e verificação da geração 2,891 ms. Não houve prepare/build/descoberta nova nesses intervalos. Assim, o custo quente predominante deste documento é a comprovação de frescor do catálogo, não carregar novamente as fontes. As subdivisões rápidas incluem configuração e rechecagem das raízes; não são um custo de parse do documento.

O vanilla escaneia fontes antes da primeira compilação do watch (`lab/typst-original/crates/typst-cli/src/watch.rs:56-57`) e seu reset não refaz essa varredura. A diferença de garantia de frescor é mecânica, não uma exigência de paridade da linguagem. Esta comparação não autoriza relaxar a garantia do Tekt.

## Controle com Fontconfig apontando para diretório vazio

Não existe `--ignore-system-fonts` em compile/watch do Tekt atual; existe no subcomando fonts (`02_shell/src/cli.rs:640`). Uma tentativa foi rejeitada ainda no primeiro warmup, antes da criação de qualquer amostra estatística desse controle. Foi substituída por `FONTCONFIG_FILE` comum aos dois processos contendo `<fontconfig><dir>DIR_VAZIO_RAM</dir></fontconfig>`. Um XML sem dir provocaria fallback do fontdb para diretórios normais e não serve para essa ablação.

As listagens desse ambiente, normalizadas apenas em apresentação, corresponderam às mesmas 17 faces embutidas de `fonts --ignore-system-fonts` em ambos. Com as fontes normais, os registros de descoberta referem-se aos mesmos 1.086 arquivos/1.112 faces de sistema e 17 faces embutidas. Índices TTC do Tekt e o rótulo Variable do vanilla foram retirados somente para conferir caminhos físicos existentes; não se afirmou igualdade da apresentação ou de todos os metadados do catálogo.

CPU em ms, Tekt / vanilla, em lote separado de oito rodadas:

| Fixture | CLI | Startup watch | Quente: mesmos bytes | Quente: edição inédita |
|---|---:|---:|---:|---:|
| Texto curto | 8,85 / 10,89 | 7,81 / 9,26 | 3,30 / 0,96 | 3,34 / 1,54 |
| Import: 2.000 bindings | 16,59 / 13,41 | 14,53 / 12,22 | 8,43 / 1,19 | 8,64 / 1,77 |
| Import: loop 10.000 | 16,33 / 13,04 | 15,18 / 12,16 | 11,30 / 0,88 | 11,46 / 1,42 |
| 16 imports locais | 17,35 / 10,58 | 16,60 / 10,22 | 13,18 / 1,40 | 13,48 / 2,07 |
| 100 parágrafos | 29,56 / 15,89 | 28,09 / 14,01 | 24,60 / 1,98 | 23,92 / 3,52 |
| Outline † | 134,02 / 18,76 | 130,93 / 16,90 | 115,40 / 3,02 | 113,53 / 4,00 |

As 960 comparações normal versus configuração vazia, dentro de cada engine, preservaram as superfícies de saída verificadas. O controle altera o corpus e a rota de descoberta: FONTCONFIG_FILE personalizado faz o fast-key normal do Tekt declinar (`03_infra/src/fontdb/generation.rs:196`). Portanto ele não isola apenas a prova de frescor, e não há um IC pareado normal/vazio entre lotes feitos em horários diferentes. Ele confirma um piso e complementa as sondas; não é um ganho disponível ao produto mantendo acesso às fontes de sistema.

[Dados desse controle](../../lab/results/construction/diagnostics/tekt-vanilla-current-2026-11-06/amostras-fontconfig-vazio.csv), [resumo](../../lab/results/construction/diagnostics/tekt-vanilla-current-2026-11-06/resumo-fontconfig-vazio.csv).

## Eval, layout e exportação depois de separar o catálogo

Edição inédita aquecida, cópia instrumentada, ms. As três primeiras colunas retiram somente catalog.initialize atribuído à respectiva fase; não retiram todo shaping/trabalho relacionado a fontes.

| Fixture | Eval sem catálogo | Contexto sem catálogo | Layout ordinário sem catálogo | Exportação PDF |
|---|---:|---:|---:|---:|
| Texto curto | 1,581 | 0,037 | 0,537 | 0,467 |
| Import: 2.000 bindings | 7,658 | 0,035 | 0,477 | 0,445 |
| Import: loop 10.000 | 10,524 | 0,045 | 0,566 | 0,448 |
| 16 imports locais | 11,935 | 0,023 | 0,586 | 0,457 |
| 100 parágrafos | 3,773 | 0,185 | 8,356 | 4,830 |
| Outline † | 25,420 | 64,497 | 0,045 | 22,133 |

Outline executou três tentativas contextuais e seis chamadas físicas por intervalo quente observado: três discovery e três replay, todas Complete. Seus tempos físicos inclusivos foram 39,037 ms discovery e 27,878 ms replay. Eles estão dentro de contexto e podem incluir catálogo: não somar novamente à tabela. Os aproximadamente 0,045 ms de layout ordinário não representam o layout inteiro desse documento.

Fonte/parse: Source.construct inclui construção/reparse real nos pontos L3, não parse puro universal. No texto curto, edição construiu um Source em 0,021 ms; mesmos bytes não construíram. Em 16 imports foram 17 construções por ciclo, incluindo mesmos bytes, aproximadamente 0,19–0,21 ms no total. Isso é compatível com a sessão sintática limitada a oito entradas (`03_infra/src/world/source_session.rs:15,45-67`) e o conjunto sequencial de 17 arquivos. Não houve experimento de aumento desse limite; a parcela medida é pequena perante os cerca de 12 ms de eval, portanto não é sustentada como prioridade principal.

Calibração: produto real, sonda desligada e sonda ligada foram intercalados em quatro rodadas. Ligada/desligada usa exatamente o mesmo ELF. Houve perturbação mensurável em imports: CPU de edição +7,19% [3,70%;19,69%] em import_compute e +5,06% [1,52%;8,73%] em import_syntax. Outline: +1,23% [−1,46%;5,31%]; texto curto: −6,80% [−9,95%;1,02%], sem evidência de overhead zero ou ganho. Os tempos instrumentados orientam a ordem de grandeza e chamadas; não são uma partição exata do tempo do ELF real. Não subtrair fases instrumentadas dos números absolutos de outro lote.

[Fases por rodada](../../lab/results/construction/diagnostics/tekt-vanilla-current-2026-11-06/fases.csv), [amostras das sondas](../../lab/results/construction/diagnostics/tekt-vanilla-current-2026-11-06/amostras-sondas.csv), [calibração contra produto](../../lab/results/construction/diagnostics/tekt-vanilla-current-2026-11-06/calibracao-produto.csv), [calibração mesmo ELF](../../lab/results/construction/diagnostics/tekt-vanilla-current-2026-11-06/calibracao-mesmo-elf.csv).

## Espera do watch é uma frente própria

O Tekt espera por polling de 100 ms (`04_wiring/src/watch.rs:144-147`, `03_infra/src/watch.rs:140-146`); vanilla usa notify e batching de 100 ms (`lab/typst-original/crates/typst-kit/src/watcher.rs:38,129-149`). A cadência altera a fase do polling.

Texto curto, edição inédita, seis rodadas por cadência, ms Tekt / vanilla:

| Espera após publicação antes da próxima edição | Wall | CPU |
|---|---:|---:|
| 5 ms | 113,14 / 101,83 | 18,08 / 1,38 |
| 55 ms | 61,63 / 101,77 | 17,18 / 1,28 |
| 95 ms | 22,50 / 101,94 | 18,01 / 1,37 |

CPU quase constante e wall fortemente variável no Tekt confirmam a sensibilidade ao polling. Os cerca de 88 ms da tabela principal são válidos para a cadência de 30 ms adotada, não uma garantia de resposta com qualquer editor. Esses controles são lotes separados, não um teste pareado entre atrasos. Wall menos CPU inclui espera, I/O, agendamento e trabalho do lançador; não é uma medição isolada de debounce.

## Equivalência e limites

Todos os 3.120 PDFs medidos passaram na verificação do marcador esperado e de publicação nova/EOF. São 696 processos medidos. As comparações no mesmo lote foram: normal 600/720 comparáveis (os 120 outline falham); configuração vazia 400/480 (os 80 outline falham); calibração 288/288 entre cópias Tekt; cadências 144/144. Adicionalmente, normal/vazio 960/960 dentro do próprio engine passaram. Não houve instabilidade nas comparações internas por engine/fixture/modo/ciclo.

A tolerância geométrica foi 0,0001 pt; a maior diferença nas fixtures comparáveis foi 0,00004 pt. Fontes nominais preservam família/estilo/case, retirando apenas prefixo subset de seis letras maiúsculas e sufixo interno -Identity-H. Tipos/encoding PDF não definem paridade. Hash dos bytes foi usado somente para evitar decodificar duas vezes o mesmo arquivo, nunca para rejeitar PDFs diferentes com a mesma linguagem observável.

Outline mantém 13 páginas nos dois, mas Tekt tem 1.052 palavras extraídas e vanilla 1.136. A rasterização confirma uma diferença visível de alinhamento/quebra nas entradas Heading 10/11 e deslocamento abaixo do sumário. [Tekt](../../lab/results/construction/diagnostics/tekt-vanilla-current-2026-11-06/outline-tekt.png) / [vanilla](../../lab/results/construction/diagnostics/tekt-vanilla-current-2026-11-06/outline-vanilla.png). Não é apenas codificação da fonte. Em 44 pares de páginas CLI a 72 dpi, primeiro/último round, somente essas duas primeiras páginas de outline diferiram acima de 16 por canal: 8.451 pixels em cada par. Esse é um controle representativo, não raster de todas as publicações ou certificação global de paridade.

[Comparações completas](../../lab/results/construction/diagnostics/tekt-vanilla-current-2026-11-06/comparacoes.csv), [raster representativo](../../lab/results/construction/diagnostics/tekt-vanilla-current-2026-11-06/raster.csv), [entradas base](../../lab/results/construction/diagnostics/tekt-vanilla-current-2026-11-06/inputs/).

## Delimitação sustentada pelas medições

1. Fontes continuam como principal frente da CLI/startup e do watch de documentos pequenos: descoberta/aquisição inicial e verificações de frescor quentes têm custos separados e medidos. Reduzir só parse do documento não ataca esse orçamento. Preservar a prova de frescor continua requisito; o vanilla não é substituto equivalente dessa mecânica.
2. Nos imports há residual de eval depois do reúso adiado da stdlib. Aumentar somente o cache sintático não é sustentado como principal ganho por esta rodada. Uma próxima investigação deve medir o trabalho de eval que se repete entre compilações, separando-o das fontes; esta medição não qualifica um novo cache sem seus testes de identidade/inputs/Features/efeitos.
3. No outline, contexto/layout físico repetido domina, seguido de eval e PDF. A dívida de saída deve acompanhar qualquer experimento: não apresentar ganho bilateral equivalente enquanto ela falhar nos controles.
4. Responsividade do watch pede bancada de detecção/agendamento separada de CPU do compilador. Alterar a espera é outra decisão; não foi implementada aqui.

Não há um único gargalo universal nas seis entradas. O residual é localizado pelas fases acima, sem concluir automaticamente por arenas, índices u32, pools ou cache em disco.

## Método, receita e proveniência

Estado congelado: HEAD `3c6b0ce7b7632cc74b004cea6681e4746e63d28d`, **uncommitted working tree**, `2026-11-06T14:34:13.106211-03:00` (America/Sao_Paulo). O diffstat rastreado exato está abaixo. Todos os arquivos recebidos foram conferidos por conteúdo ao fim e permaneceram iguais; fontes produtivas/L0s/lock não foram alterados por esta comparação.

ELF Tekt SHA-256 `b7bb21acd220e02c9311f283bb6f9b01574f493a15c56fddb9256503075538db`. Oracle vanilla commit `64c5e8c02fe0394d913ead36e42ef8c1334d2107`, árvore `483bd6f1d8ac8810db47b65336d402647955fd17`, SHA-256 `22897f93c7c2c8589cf5913b0ce735f1c01ba711c70f55c7a350f7c4b96c0b34`, conferido antes da cópia para RAM. Sua [ratificação](../../lab/results/construction/diagnostics/vanilla-resync-2026-10-26/relatorio.md) permanece a referência; não se usou --version como identidade. ELF instrumentado `55738b65f2eec27b453b46485ec36377ea075e9c33baf5c46207688a595fbb42`, separado do produto.

Tekt compilado em RAM com Rust 1.92.0, Cargo offline/locked release, opt-level=s/ThinLTO/uma unidade, jobs=1, incremental=0, TYPST_COMMIT_SHA=HEAD, .cargo/config.toml recebido integral (incluindo alinhamento 3) e exceção zune-jpeg preservados. MemoryMax=12 GiB, swap=0; cache antigo somente copiado. O oracle ratificado não foi recompilado. São receitas normais de cada produto, não configurações internas/algoritmos idênticos.

CPU0, Rayon=1, stack 64 MiB, SOURCE_DATE_EPOCH=0, TZ UTC, NO_COLOR=1, fontes/cache de SO aquecidos, root e conteúdo equivalentes. Flags comuns: `compile|watch INPUT OUTPUT --root INPUTS --pdf-tagged=false`. Sem inputs extras/pacotes remotos. Relógio CLI usa Popen→fim e wait4; watch usa Popen→primeiro PDF e, nos ciclos, escrita→nova publicação estável com EOF, poll de 1 ms. CPU quente é delta do relógio do processo e não inclui todo trabalho após publicação; RSS é o pico final do processo, repetido nas linhas, não pico por ciclo.

O corpus base é o mesmo das entradas anteriores, acrescido de imports_16 e seus módulos e de um marcador visível em todas as compilações. Por isso layout agora ocupa cinco páginas nos dois compiladores; não comparar diretamente seus absolutos com a antiga rodada de quatro páginas sem marcador. O driver deriva da bancada existente outline-warm-residual, com TASK_RAM rebaseado e apenas a regra de edição tornada monotônica: `next_marker = current_marker + 1`, texto `data = initial_source + f'\n#"TEKT_EPOCH_{marker}"\n'.encode()`, marcador esperado no checker `cycle // 2`. startup/same/edit ficam 0/0/1/1/2/2/3/3/4. Não é alternância entre duas versões já aquecidas.

Uma repetição por rodada; ordem CLI/watch alternada, fixtures rotacionadas e engines alternados. Principal 12 rodadas, quatro pares same/edit por watch; Fontconfig vazio oito rodadas, quatro pares; sondas quatro rodadas, três variantes e dois pares; cada cadência seis rodadas, três pares. Warmups completos por engine/fixture/modo excluídos das estatísticas. Nenhum build/teste/trace/extrator da bancada concorreu com os relógios. Um inventário de 1.262 caminhos (fontes e configuração) manteve dev/ino/size/mode/mtime_ns/ctime_ns entre o início e fim; não é um snapshot atômico contra escritores adversários.

- `main`: 2026-11-06T14:51:37.062587-03:00 → 2026-11-06T14:55:21.241005-03:00.
- `empty-fontconfig`: 2026-11-06T15:02:18.502567-03:00 → 2026-11-06T15:04:22.453165-03:00.
- `phases`: 2026-11-06T15:04:22.453457-03:00 → 2026-11-06T15:05:44.320042-03:00.
- `cadence-5`: 2026-11-06T15:05:44.320346-03:00 → 2026-11-06T15:05:57.642648-03:00.
- `cadence-55`: 2026-11-06T15:05:57.642912-03:00 → 2026-11-06T15:06:13.683668-03:00.
- `cadence-95`: 2026-11-06T15:06:13.683944-03:00 → 2026-11-06T15:06:32.011678-03:00.

Corpus SHA-256 `0782c58e1c1943a9a83b87c102f18df91cca3a68a20e3dde3a52b4adcacec4bf`: nomes relativos e bytes de todos os arquivos de inputs, ordenados, ambos prefixados por tamanho u64 big-endian. O marcador é acrescentado conforme a regra acima.

Skill Tekt: A adaptou o driver; B escreveu checker/testes independentes sem ver A; C integrou e executou. Passaram 28 testes. Baseline anterior falhou na nova sequência esperada; 4/4 mutações efetivas foram rejeitadas (módulo 2, marcador constante, ignorar bbox, ignorar face). Um controle sintético confirmou média interna, mediana de razões pareadas e bootstrap. Não foi usada uma falha de compilação como mutação morta. Build e crystalline-lint encerraram com exit 0. Esta é validação da bancada/saídas, não uma nova execução da suíte inteira de produto já validada na tarefa anterior.

Workspaces, ELFs, sondas, caches, logs, mutantes e PDFs ficaram somente em /dev/shm/tekt-vanilla-current-20261106 e foram removidos ao concluir. Permanecem o diagnóstico e dados de domínio; não se versionam recibos/manifests/logs ou certificados. Sem commit, staging, instalação ou otimização produtiva.

### Diffstat exato do estado recebido e medido

```text
 00_nucleo/prompts/core/compiler/eval/bindings/field_access.md                 |  19 ++++-
 00_nucleo/prompts/core/compiler/eval/bindings/field_access/content_methods.md |  51 +++++++++++++-
 00_nucleo/prompts/core/compiler/eval/bindings/field_access/fields.md          |  30 +++++++-
 00_nucleo/prompts/core/compiler/eval/mod.md                                   |  33 ++++++++-
 00_nucleo/prompts/core/compiler/eval/modules.md                               |  36 +++++++++-
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
 00_nucleo/prompts/core/compiler/layout/block.md                               |  55 ++++++++++++++-
 00_nucleo/prompts/core/compiler/layout/checkpoint.md                          |  61 +++++++++++++++-
 00_nucleo/prompts/core/compiler/layout/columns.md                             |  60 ++++++++++++++--
 00_nucleo/prompts/core/compiler/layout/columns/flow.md                        |  18 ++++-
 00_nucleo/prompts/core/compiler/layout/columns/segmented.md                   |  34 ++++++++-
 00_nucleo/prompts/core/compiler/layout/columns/transitions.md                 |  16 ++++-
 00_nucleo/prompts/core/compiler/layout/cursor.md                              |  59 +++++++++++++++-
 00_nucleo/prompts/core/compiler/layout/equation.md                            |  34 ++++++++-
 00_nucleo/prompts/core/compiler/layout/figure.md                              |  32 ++++++++-
 00_nucleo/prompts/core/compiler/layout/flush.md                               |  19 ++++-
 00_nucleo/prompts/core/compiler/layout/footnote.md                            |  39 ++++++++++-
 00_nucleo/prompts/core/compiler/layout/footnote_flush.md                      |  24 ++++++-
 00_nucleo/prompts/core/compiler/layout/footnote_fragments.md                  |  31 +++++++-
 00_nucleo/prompts/core/compiler/layout/grid/cell_emission.md                  |  95 ++++++++++++++++++++++++-
 00_nucleo/prompts/core/compiler/layout/grid/row_groups.md                     |  10 ++-
 00_nucleo/prompts/core/compiler/layout/grid/tracks.md                         |  81 ++++++++++++++++++++-
 00_nucleo/prompts/core/compiler/layout/helpers.md                             |  10 ++-
 00_nucleo/prompts/core/compiler/layout/image.md                               |  58 ++++++++++++++-
 00_nucleo/prompts/core/compiler/layout/line_commit.md                         |  21 +++++-
 00_nucleo/prompts/core/compiler/layout/measurement.md                         |  19 ++++-
 00_nucleo/prompts/core/compiler/layout/mod.md                                 |  34 ++++++++-
 00_nucleo/prompts/core/compiler/layout/page_run.md                            |  34 ++++++++-
 00_nucleo/prompts/core/compiler/layout/page_running_requests.md               |  53 +++++++++++++-
 00_nucleo/prompts/core/compiler/layout/paragraph.md                           |  34 ++++++++-
 00_nucleo/prompts/core/compiler/layout/paragraph_indent.md                    |  34 ++++++++-
 00_nucleo/prompts/core/compiler/layout/place.md                               |  32 ++++++++-
 00_nucleo/prompts/core/compiler/layout/placement.md                           |  34 ++++++++-
 00_nucleo/prompts/core/compiler/layout/references.md                          |  48 ++++++++++++-
 00_nucleo/prompts/core/compiler/layout/region_progress.md                     |  61 +++++++++++++++-
 00_nucleo/prompts/core/compiler/layout/repeat.md                              |  34 ++++++++-
 00_nucleo/prompts/core/compiler/layout/set_page.md                            |  34 ++++++++-
 00_nucleo/prompts/core/compiler/layout/shape.md                               |  48 ++++++++++++-
 00_nucleo/prompts/core/compiler/layout/slicing.md                             |  10 ++-
 00_nucleo/prompts/core/compiler/layout/stack.md                               |  69 +++++++++++++++++-
 00_nucleo/prompts/core/compiler/layout/sub_frame.md                           |  17 ++++-
 00_nucleo/prompts/core/compiler/layout/text/style.md                          |  34 ++++++++-
 00_nucleo/prompts/core/compiler/layout/tiling.md                              |  48 ++++++++++++-
 00_nucleo/prompts/core/compiler/math/layout/delimited.md                      |  80 ++++++++++++++++++++-
 00_nucleo/prompts/core/compiler/math/layout/external_bridge.md                |  34 ++++++++-
 00_nucleo/prompts/core/compiler/math/layout/mod.md                            |  81 ++++++++++++++++++++-
 00_nucleo/prompts/core/compiler/math/layout/op.md                             |  52 +++++++++++++-
 00_nucleo/prompts/core/compiler/realize/figure.md                             |  32 ++++++++-
 00_nucleo/prompts/core/compiler/realize/figure_kind.md                        |  19 ++++-
 00_nucleo/prompts/core/compiler/stdlib/figure_image.md                        |  87 ++++++++++++++++++++++-
 00_nucleo/prompts/core/compiler/stdlib/tests/figure_image.md                  |  65 ++++++++++++++++-
 00_nucleo/prompts/core/compiler/stdlib/text/constructor.md                    |  42 ++++++++++-
 00_nucleo/prompts/core/compiler/stdlib/visualize.md                           |  48 ++++++++++++-
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
 00_nucleo/prompts/core/entities/plugin_func.md                                |  72 ++++++++++++++-----
 00_nucleo/prompts/core/entities/tiling.md                                     |  67 +++++++++++++++++-
 00_nucleo/prompts/infra/export/builder.md                                     |  15 ++++
 00_nucleo/prompts/infra/export/builder/resources.md                           |  15 +++-
 00_nucleo/prompts/infra/export/images.md                                      |  31 +++++++-
 00_nucleo/prompts/infra/export/images/png.md                                  |  20 +++++-
 00_nucleo/prompts/infra/export/images/scan.md                                 |  82 ++++++++++++++++++++--
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
 00_nucleo/prompts/wiring/tests/math_decoration_and_float_flush.md             |  21 +++++-
 01_core/src/compiler/eval/bindings/field_access.rs                            |   4 +-
 01_core/src/compiler/eval/bindings/field_access/content_methods.rs            |  21 ++++--
 01_core/src/compiler/eval/bindings/field_access/fields.rs                     |   6 +-
 01_core/src/compiler/eval/mod.rs                                              |   7 +-
 01_core/src/compiler/eval/modules.rs                                          |   9 ++-
 01_core/src/compiler/eval/observation/elements.rs                             |  34 ++++++---
 01_core/src/compiler/eval/repr/content.rs                                     |  12 +++-
 01_core/src/compiler/eval/rules/set.rs                                        |  54 +++++++++++++-
 01_core/src/compiler/eval/rules/set/text.rs                                   |  42 ++++++++++-
 01_core/src/compiler/eval/tests.rs                                            |   4 +-
 01_core/src/compiler/introspect.rs                                            |   4 +-
 01_core/src/compiler/introspect/convergence.rs                                |   4 +-
 01_core/src/compiler/introspect/figure_demands.rs                             |   4 +-
 01_core/src/compiler/introspect/materialize_time.rs                           |  33 ++++++++-
 01_core/src/compiler/introspect/page_running.rs                               |  68 +++++++++++++++---
 01_core/src/compiler/introspect/walk.rs                                       |  69 +++++++++++++++++-
 01_core/src/compiler/layout/bidi_origin.rs                                    |   7 +-
 01_core/src/compiler/layout/block.rs                                          | 254 ++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++--
 01_core/src/compiler/layout/checkpoint.rs                                     |  17 ++++-
 01_core/src/compiler/layout/columns.rs                                        |  32 +++++----
 01_core/src/compiler/layout/columns/flow.rs                                   |  58 ++++++++++++++-
 01_core/src/compiler/layout/columns/segmented.rs                              |  30 +++-----
 01_core/src/compiler/layout/columns/transitions.rs                            |  72 ++++++++++++++++++-
 01_core/src/compiler/layout/cursor.rs                                         |  76 +++++++++++++++++++-
 01_core/src/compiler/layout/equation.rs                                       |  10 ++-
 01_core/src/compiler/layout/figure.rs                                         | 240 ++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++--
 01_core/src/compiler/layout/flush.rs                                          |   3 +-
 01_core/src/compiler/layout/footnote.rs                                       | 171 +++++++++++++++++++++++++++++++++++++++++++--
 01_core/src/compiler/layout/footnote_flush.rs                                 |  71 ++++++++++++++++++-
 01_core/src/compiler/layout/footnote_fragments.rs                             |  99 ++++++++++++++++++++++++--
 01_core/src/compiler/layout/grid/cell_emission.rs                             | 167 +++++++++++++++++++++++++++++++++++++++++---
 01_core/src/compiler/layout/grid/row_groups.rs                                | 292 ++++++++++++++++++++++++++++++++++++++--------------------------------------
 01_core/src/compiler/layout/grid/tracks.rs                                    | 115 ++++++++++++++++++++++++++++--
 01_core/src/compiler/layout/helpers.rs                                        |   5 +-
 01_core/src/compiler/layout/image.rs                                          |  80 ++++++++++++++++++---
 01_core/src/compiler/layout/line_commit.rs                                    |   4 +-
 01_core/src/compiler/layout/measurement.rs                                    |   4 +-
 01_core/src/compiler/layout/mod.rs                                            |   6 +-
 01_core/src/compiler/layout/page_run.rs                                       |  16 +----
 01_core/src/compiler/layout/page_running_requests.rs                          |  79 ++++++++++++++++++++-
 01_core/src/compiler/layout/paragraph.rs                                      |  87 ++++++++++++++++++++---
 01_core/src/compiler/layout/paragraph_indent.rs                               |   9 +--
 01_core/src/compiler/layout/place.rs                                          |  67 ++++++++++++++++--
 01_core/src/compiler/layout/placement.rs                                      |  10 ++-
 01_core/src/compiler/layout/references.rs                                     | 141 ++++++++++++++++++++++++++++++++++++-
 01_core/src/compiler/layout/region_progress.rs                                |  88 +++++++++++++++++++++--
 01_core/src/compiler/layout/repeat.rs                                         |   8 +--
 01_core/src/compiler/layout/set_page.rs                                       |  16 +----
 01_core/src/compiler/layout/shape.rs                                          |  11 +--
 01_core/src/compiler/layout/slicing.rs                                        |   4 +-
 01_core/src/compiler/layout/stack.rs                                          |  17 +++--
 01_core/src/compiler/layout/sub_frame.rs                                      |  57 ++++++++++++++-
 01_core/src/compiler/layout/text/style.rs                                     |  11 ++-
 01_core/src/compiler/layout/tiling.rs                                         |  19 +++--
 01_core/src/compiler/math/layout/delimited.rs                                 | 211 ++++++++++++++++++++++++++++++++++++++++++++++++++++++-
 01_core/src/compiler/math/layout/external_bridge.rs                           |  10 ++-
 01_core/src/compiler/math/layout/mod.rs                                       |  10 ++-
 01_core/src/compiler/math/layout/op.rs                                        |  91 +++++++++++++++++++++++-
 01_core/src/compiler/realize/figure.rs                                        |  76 ++++++++++++++++++--
 01_core/src/compiler/realize/figure_kind.rs                                   |   6 +-
 01_core/src/compiler/stdlib/figure_image.rs                                   | 114 ++++++++++++++++++++++++------
 01_core/src/compiler/stdlib/tests/figure_image.rs                             | 282 +++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++--
 01_core/src/compiler/stdlib/text/constructor.rs                               |  20 +++++-
 01_core/src/compiler/stdlib/visualize.rs                                      |  11 +--
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
 01_core/src/entities/plugin_func.rs                                           | 379 ++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++------------
 01_core/src/entities/tiling.rs                                                | 166 +++++++++++++++++++++++++++++++++++++++++--
 02_shell/src/cli.rs                                                           |   4 +-
 03_infra/Cargo.toml                                                           |   7 ++
 03_infra/src/export/builder.rs                                                |   2 +-
 03_infra/src/export/builder/resources.rs                                      |   7 +-
 03_infra/src/export/images.rs                                                 |   7 +-
 03_infra/src/export/images/png.rs                                             |  93 +++++++++++++++++++++++-
 03_infra/src/export/images/scan.rs                                            | 288 +++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++--------
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
 04_wiring/tests/math_decoration_and_float_flush.rs                            | 158 ++++++++++++++++++++++++++++++++++++++++-
 Cargo.lock                                                                    | 275 ++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++---
 lab/benches/pdf_builder_bench.rs                                              |   1 +
 195 files changed, 9367 insertions(+), 706 deletions(-)
```

Fontes/owners/núcleos produtivos ainda não rastreados no estado recebido (também preservados):

```text
00_nucleo/prompts/_nuclei/export/raster-decode.toml
00_nucleo/prompts/_nuclei/figure-placement.toml
00_nucleo/prompts/_nuclei/image-source-provenance.toml
00_nucleo/prompts/_nuclei/layout/column-note-regions.toml
00_nucleo/prompts/_nuclei/layout/finite-block-text-region.toml
00_nucleo/prompts/_nuclei/layout/footnote-numbering.toml
00_nucleo/prompts/_nuclei/svg-preparation.toml
00_nucleo/prompts/core/compiler/eval/stdlib_snapshot.md
00_nucleo/prompts/core/compiler/eval/stdlib_snapshot/tests.md
00_nucleo/prompts/core/entities/svg_image.md
00_nucleo/prompts/infra/export/images/svg.md
00_nucleo/prompts/infra/svg_image.md
01_core/src/compiler/eval/stdlib_snapshot.rs
01_core/src/compiler/eval/stdlib_snapshot/tests.rs
01_core/src/entities/svg_image.rs
03_infra/src/export/images/svg.rs
03_infra/src/svg_image.rs
```
