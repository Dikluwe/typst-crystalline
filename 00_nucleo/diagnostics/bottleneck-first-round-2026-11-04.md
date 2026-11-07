# Primeira rodada de medição de gargalos

O custo dominante depende do documento. Nos controles pequenos, a aquisição e a validação do catálogo de fontes dominam o trabalho observado, inclusive quando contabilizadas dentro de `layout_ms`. No documento com outline, o trabalho de layout/repetição de candidatos passa a predominar; foram contadas seis finalizações de candidatos por compilação. A rodada não sustenta escolher memoização geral de imports como primeira correção para todos os documentos.

Estado: branch Tekt, HEAD `3c6b0ce7b7632cc74b004cea6681e4746e63d28d`, working tree não commitado preservado. [before.json](../../lab/results/construction/diagnostics/bottleneck-first-round-2026-11-04/before.json) contém UTC, status, diffstat exato e hashes de fontes/L0/configuração. [verification.json](../../lab/results/construction/diagnostics/bottleneck-first-round-2026-11-04/verification.json) confirma que nenhuma fonte recebida mudou. Nenhuma otimização ou mudança de L0 foi implementada.

## Receita e método

Foi compilado um release novo, offline/locked, Rust 1.92.0, usando `/dev/shm/tekt-bottleneck-20261104`, um job, MemoryMax=12 GiB, MemoryHigh=8 GiB, MemorySwapMax=0. Build concluído em 519,320 s, sem swap; receita e identidade em [build.json](../../lab/results/construction/diagnostics/bottleneck-first-round-2026-11-04/build.json). Binário SHA-256 `aa7689fb5e6f5628ef4d5870686dd1ff3a42fb3c1904449568904e1662113c81`. Não foi usado o release antigo como representante da árvore atual.

Cinco fixtures: texto pequeno; import com 2.000 declarações; import com cálculo de 10.000 iterações; 100 parágrafos; outline com 12 seções. Na CLI houve uma execução inicial descartada e três amostras medidas por fixture, sempre processos novos. No watch houve dois processos por fixture, cada qual com seis ciclos alternando escrita dos mesmos bytes, edição pequena e adição estrutural. Startup foi medido separadamente. CPU fixa, RAYON_NUM_THREADS=1, SOURCE_DATE_EPOCH=0; ambientes/argv/fixtures completos em [measurements.json](../../lab/results/construction/diagnostics/bottleneck-first-round-2026-11-04/measurements.json).

As saídas CLI foram extraídas e guardadas; os ciclos watch verificaram o texto da revisão esperada. Isso controla publicação/staleness destas fixtures, não certifica paridade universal ou equivalência geométrica com vanilla. Não houve comparação de velocidade com vanilla nesta rodada de atribuição local. Os tempos foram executados antes das sondas, sem build/debugger/sampler concorrentes desta tarefa.

## Tempos observados

Medianas descritivas, em ms; poucas amostras, sem intervalo de confiança. CLI wall inclui processo inteiro; CPU watch soma threads entre edição e publicação concluída. As condições individuais, startup, latência, fases e memória estão em [summary.json](../../lab/results/construction/diagnostics/bottleneck-first-round-2026-11-04/summary.json).

| Fixture | CLI wall | Eval nativo CLI | Layout nativo CLI | CPU ciclo watch quente | Eval nativo watch | Layout nativo watch |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| Texto pequeno | 95,15 | 1,52 | 85,92 | 19,09 | 1,74 | 15,83 |
| Import sintático | 98,20 | 8,62 | 81,66 | 23,49 | 7,60 | 14,32 |
| Import cálculo | 97,64 | 9,14 | 80,80 | 25,26 | 11,21 | 12,84 |
| 100 parágrafos | 117,64 | 4,19 | 95,68 | 38,18 | 4,26 | 22,88 |
| Outline | 217,60 | 26,54 | 0,04 | 130,54 | 26,58 | 0,04 |

A coluna de layout nativo **não** significa tempo exclusivo do algoritmo de layout. Para outline, `expand_context_ms` foi 152,71 ms na CLI e 72,97 ms no watch: esse envelope também executa layout. Na fonte, `paged.rs:772–781` envolve eval, `:830` encerra introspecção, `:853` encerra preparação contextual, `:1130` encerra o envelope final de layout, e `:1163` encerra shaping/bidi/posições. Os rótulos nativos não separam as chamadas internas por domínio.

Exportação PDF foi medida por `render_ms` e `subset_ms`, sem escrita/publicação do arquivo. Medianas render/subset CLI: texto 0,52/0,14 ms; imports aproximadamente 0,25–0,26/0,11–0,13 ms; parágrafos 4,83/0,21 ms; outline 20,46/0,38 ms. No watch: texto/imports aproximadamente 0,27–0,29/0,11–0,12 ms, parágrafos 4,53/0,20 ms, outline 20,10/0,36 ms. `pdf.rs:472,555–566` inicia o relógio depois da resolução de recursos; `total_ms` é soma parcial de fases e não o total da CLI.

## Fontes e atribuição por amostragem

Perf foi tentado e negado pelo kernel (`perf_event_paranoid=4`); nenhuma configuração do sistema foi alterada. Foi usado o sampler diagnóstico ITIMER_PROF já existente no projeto, adaptado apenas no diretório de evidências. Ele registra PC interrompido e backtrace a cada 1 ms de CPU; a simbolização considera ELF EXEC/PIE e base do módulo. Nenhum PC interrompido das dez sondas ficou desconhecido.

| Fixture | Amostras CLI com ancestral de fontes / total | Amostras watch com ancestral de fontes / total |
| --- | ---: | ---: |
| Texto pequeno | 89 / 93 | 338 / 391 |
| Import sintático | 81 / 92 | 308 / 439 |
| Import cálculo | 82 / 93 | 299 / 470 |
| Parágrafos | 89 / 113 | 328 / 676 |
| Outline | 82 / 207 | 297 / 1931 |

São contagens **inclusivas**, não percentuais de tempo exclusivo: uma amostra pode conter fontes e layout. Watch agrega startup e ciclos; não permite atribuição exata de cada edição. Backtrace/sinais perturbam execução; as durações instrumentadas não entram nos benchmarks.

O detalhe identifica uma causa concreta: no watch pequeno, 191 amostras tiveram `statx` como PC interrompido; 190 pertenciam ao caminho `with_font_catalog_session`. Dessas pilhas, 134 incluíam `ProbeKey::unchanged`, 144 `scan_stamp`, 41 `GenerationKey::unchanged`, e apenas três `prepare_from`. Essas categorias também se sobrepõem. A evidência aponta para o custo da **validação de metadados da geração**, mesmo havendo retenção e prova rápida; não autoriza dizer que o catálogo inteiro é redescoberto a cada ciclo. Na CLI pequena, as pilhas apontam para `fresh_catalog`, `load_system_fonts` e aquisição de informações de fontes.

No outline watch, 1005/1931 amostras contêm layout, 393 eval e 285 export; o perfil já não é dominado por fontes. A repetição de layout merece medição específica nessa classe de documento. Arquivos `profile*.json` conservam amostras, símbolos, fonte do sampler e hashes. A primeira tentativa de encerramento watch por SIGINT produziu recibos vazios; os logs foram mantidos. A repetição usou SIGUSR2 para descarregar o sampler e encerrar o processo. Esse handler e backtrace não são async-signal-safe: trata-se de sonda diagnóstica limitada, não código de produto nem instrumento de precisão absoluta.

## Parse e tentativas

O `parse_ms` nativo permaneceu zero porque essa fachada não mede construção de Source. Zero não significa parsing gratuito. Contagens GDB separadas dos tempos usaram o símbolo exato `Source::new_with_parser`; nos controles com imports houve duas construções iniciais. Escrever os mesmos bytes não acrescentou construção; editar o main acrescentou uma, mantendo o CST do import. O eval raiz foi executado novamente em todos os ciclos. Funções de import e catálogo selecionadas não apareceram como símbolos independentes no release; a ausência de breakpoint não é contagem zero de execução.

A sonda contou `finish_with_running` exato, e não `layout_content` recursivo. Essa é a finalização comum dos coordenadores `convergence.rs:139,316`; `finish` delega a ela. Texto, imports e parágrafos tiveram uma finalização por compilação. Outline teve seis, tanto CLI quanto a cada ciclo watch: acumulados 6/12/18/24 em startup/mesmos bytes/duas edições. São candidatos entre coordenadores, não seis iterações de um único loop cujo limite é cinco. Recibos e saídas GDB estão nos arquivos `counts-*.json`.

**Limite desta rodada:** parse e duração individual de cada tentativa não receberam relógios exclusivos confiáveis. Parse aparece em poucas amostras (três no import sintático CLI; outras sondas muitas vezes zero), insuficientes para estimar milissegundos. Há contagens por ciclo e evidência CPU inclusiva, mas não uma decomposição temporal completa e aditiva. Não se deve dividir o tempo agregado pelas seis finalizações para inventar o custo de cada tentativa.

## Próximo passo fundamentado

Priorizar uma segunda rodada focal em `ProbeKey::unchanged`/`GenerationKey::unchanged` nos documentos pequenos, preservando integralmente o contrato de frescor (adição/substituição/remoção/fontconfig/symlinks). Para outline, investigar os seis candidatos e quais dependências realmente mudam. Imports possuem custo repetido mensurável, mas não explicam a maior parte do controle pequeno; a retenção sintática já funciona.

Antes de qualquer otimização, completar a decomposição por tentativa/parse com instrumentação diagnóstica especificada e controle de overhead, sem alterar contratos ou atribuir tempos por subtração de medianas de fixtures diferentes. Qualquer mudança produtiva continua dependente dos L0 e gates aplicáveis. Agentes Luna pesquisaram comemo e os limites dos contadores: apenas plugins têm memoize; os contadores existentes de eviction/Introspector não são hits de cache.
