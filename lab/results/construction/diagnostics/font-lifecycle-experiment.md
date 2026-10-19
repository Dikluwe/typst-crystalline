# Experiência de ciclo de vida e cobertura das fontes

A adaptação da construção de cobertura para intervalos reduziu as medianas end-to-end em **35,76–36,95%** nas cinco fixtures qualificadas. Adiar a descoberta do catálogo, sozinho, não produziu ganho total consistente: o custo passou do setup para a primeira demanda em eval/layout. O ganho controlado aparece ao comparar o binário lazy com o mesmo ciclo de vida acrescido do novo construtor de cobertura. O Tekt ainda fica atrás do vanilla, especialmente em imagens e shapes que acionam métricas de fonte durante layout.

Este relatório é uma nova experiência autorizada pelo pedido de tentar o mecanismo do vanilla. A [baseline original](performance-baseline.md) e seu diretório foram preservados. O plano de [microbenchmarks](font-lifecycle-experiment/microbench-plan.md) separa as próximas perguntas; ele não acrescenta uma nova suíte nem novas APIs produtivas.

## Intervenções e contratos

Foram preservados três executáveis Tekt: `before`, o binário exato da baseline; `lazy`, com aquisição por demanda; e `interval`, com o mesmo ciclo de vida lazy mais a construção de cobertura por intervalos. A comparação lazy → interval controla o restante dos consumers. Fontes, inputs e opções permanecem iguais entre os quatro executáveis, incluindo o vanilla.

`SystemWorld` agora mantém slots e book em um `OnceLock` privado, com loader privado configurado pelos builders de fontes do sistema/combinadas. A primeira consulta a book, fonte ou candidatos inicializa o par uma vez; ele permanece pela vida do World, inclusive sob acesso concorrente. Os builders explícitos de slots/embutidas mantêm a aquisição imediata. Substituir a configuração invalida o cache de cobertura anterior. A composição CLI deixou de chamar o preload baseado no source bruto; a API explícita existente continua disponível. A resolução de recursos não consulta o book quando o documento exportado não contém combinações de fontes.

A ordem permanece texto embutido → sistema → math/code embutido → projeto. Fallback, cobertura exata, estilo, shaping e seleção conservam seus contratos. Não foram adicionados campos públicos, métodos de trait, assinaturas públicas, flags, fases de negócio, cache global ou catálogo persistente em disco. I/O continua em L3; L4 apenas configura e compõe os componentes. A integração watch continua criando um novo World: esta experiência não entrega a retenção do World que o vanilla usa entre recompilações watch.

O vanilla pinado constrói cobertura compacta em cada subtable Unicode ordenada e une intervalos (`lab/typst-original/crates/typst-library/src/text/font/info.rs:119`). Seu construtor genérico `Coverage::from_vec` ainda ordena/deduplica. O Tekt mantém `Coverage::from_codepoints(impl Into<Vec<u32>>)` e a codificação canônica gap/count: compacta sequências contíguas, ordena intervalos e une sobreposição/adjacência. **L3 ainda coleta o Vec de codepoints**; não é uma cópia literal do streaming vanilla nem eliminação daquela alocação. A [decisão da experiência](font-lifecycle-experiment/coverage-design.md) registra a fonte e os controles independentes.

O histórico explica a restrição conservada. [P517](../materialization/typst-passo-517.md) introduziu descoberta do sistema por utilidade do produto; [P674](../materialization/typst-passo-674.md) aproveitou bytes temporários do fontdb para metadados. [P938](typst-passo-938-relatorio.md) adiou cobertura, mas [P1410](../materialization/typst-passo-1410.md) identificou o custo de reter mapeamentos de muitas fontes e moveu a cobertura exata do sistema para os bytes temporários da descoberta. Essa política de memória permanece. [P931](typst-passo-931-relatorio.md) registra a experiência histórica de persistência, que não constitui um catálogo persistente de produção. O Fontconfig usado pelo fontdb fornece configuração/diretórios; não se demonstrou reaproveitamento de metadados de `fc-cache` como explicação para o vanilla.

## Proveniência e reprodução

HEAD em todas as medições: `6ef8b6986e74de7d5cc8f7b7e009868fc0daa5f5`, **working tree não commitado**. A árvore já continha alterações extensas. [initial.json](font-lifecycle-experiment/initial.json), [lista inicial exata](font-lifecycle-experiment/state/initial.status.txt), [diff stat inicial](font-lifecycle-experiment/state/initial.diff-stat.txt) e [patch inicial](font-lifecycle-experiment/state/initial.patch) registram o estado recebido. Backups `.snapshot` preservam os bytes anteriores dos owners; [task-files.json](font-lifecycle-experiment/task-files.json) e [patch exclusivo da experiência](font-lifecycle-experiment/state/task-only.patch) distinguem a contribuição desta tarefa. Cada bloco possui sua própria lista exata, patch e fingerprint antes/depois em `state/` e `receipts/`.

A confirmação do oráculo usa o pin **`9f2b6e8715237cb086899a42873660fe744622e8`**, árvore `6ab8bb9434b1bd4d44e1a10fbf1e407b3b86a1a4`, receita ratificada em [P1489](typst-p1489-resync-vanilla.md) e SHA-256 do executável `7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea`. O [registro de proveniência](font-lifecycle-experiment/vanilla-provenance.json), copiado da baseline e conferido nas amostras, também explicita a diferença local de `Cargo.lock`; não se reivindica identidade da árvore inteira com o upstream. A versão `0.15.1` é evidência auxiliar.

| Executável | SHA-256 |
| --- | --- |
| Tekt before | `9a94d37b711428a699e49bcba239de6a065a8c1602ae3362f9b55a6ac5e608e7` |
| Tekt lazy | `9392d62ca15e585803f6f3bb936020556e9459a31aea5255074c5d01843e3dd7` |
| Tekt interval | `13a8b31cf4912aa62d813b5f4aca088b5862193e41ad353b21e789de9b576216` |

Perfis Cargo release: opt-level 3, debuginfo 0, debug assertions e overflow checks desabilitados, Rust flags ambientais limpas. [variants.json](font-lifecycle-experiment/variants.json) e [probes.json](font-lifecycle-experiment/probes.json) registram comandos, perfis, fontes e identidades. Os archives lazy/interval conservam os sources/inputs da construção. O controle de consumers do executável before contra os bytes recebidos não encontrou diferenças, inclusive adições/remoções, conforme [before-consumer-control.json](font-lifecycle-experiment/before-consumer-control.json).

Máquina: Dell G15, AMD Ryzen 7 5800H, Linux x86_64 kernel 6.17.0-42-generic, rustc 1.92.0 (`ded5c06cf`, LLVM 21.1.3), Cargo 1.92.0. CPU 4 fixada por processo, Rayon 1 e vanilla `--jobs 1`. Governor powersave, boost habilitado, frequência não travada, sibling 5 e desktop/SO ativos. [host.json](font-lifecycle-experiment/host.json) registra detalhes e limitações. Nenhuma configuração global de frequência/cache/power foi alterada. Não há série de temperatura da CPU capaz de explicar causalmente a variação.

Preparação iniciou em `2026-09-27T11:06:21.965349+00:00`; amostras admitidas entre **2026-09-27T11:46:38.681559+00:00 e 2026-09-27T12:00:49.251772+00:00**. Todas as horas brutas estão em UTC. Ambiente inclui `SOURCE_DATE_EPOCH=0`, `TZ=UTC`, `LC_ALL=C.UTF-8`. Usaram-se PDF sem tagging e Pretty desabilitado, mesmo root, mesma pasta de fontes de projeto, mesmos bytes de fontes embutidas e mesmo universo do sistema, com fingerprints em [build-state.json](font-lifecycle-experiment/build-state.json). Seu SHA agregado congelado é `bb8e4453fded789d267c1ecf5c5dcf0f9472cfc44b24280ee12622bda82729f8`.

O [protocolo](font-lifecycle-experiment/protocol.md) fixa três rodadas, duas warmups por variante/fixture, 12 quadruplets por rodada para plain e seis para instrumentado, alternando a ordem. São 36 amostras plain e 18 instrumentadas por variante/fixture; catálogo possui 36 por variante. Processos frescos com filesystem aquecido; não se mediu cold-storage nem catálogo persistente. Gates de 30 segundos sem builds/testes/benchmarks precedem as rodadas, seguidos de monitoramento a cada 100 ms. Todos os 33 blocos tiveram fingerprint/HEAD/executáveis constantes e nenhuma contaminação detectada. Isso admite os blocos por integridade, não prova estabilidade estatística. [completion-audit.json](font-lifecycle-experiment/completion-audit.json) consolida a conferência final.

O tempo plain é spawn até wait4: inclui setup da CLI/World, compilação, fontes, exportação, I/O de output e destruição do processo. A escrita de stdout/stderr do harness é configurada fora da chamada; a própria atividade do processo permanece medida. O catálogo mede a chamada pública `fontdb::load_system_fonts`: I/O, parser, metadados/cobertura e construção de slots/book, incluindo destruição interna do database. Hash de resultado e destruição dos slots/book retornados ficam fora desse intervalo; a última destruição tem timer separado. Esses são componentes compostos, não operações isoladas da cmap.

Receitas e comandos completos estão no [README do dataset](font-lifecycle-experiment/README.md). `analyze.py` recalcula as distribuições; `validate.py` reproduz controles de outputs. Os comandos de medição reutilizam blocos previamente admitidos; uma nova coleta deve usar um novo diretório/protocolo, sem sobrescrever esta evidência ou a baseline original. System fonts são identificadas por hash, mas devem estar disponíveis na máquina de reprodução. Os scripts auxiliares residem somente no diagnóstico atribuído.

## Comparação end-to-end

Medianas **em milissegundos**, população plain sem tracing:

| Fixture | Antes | Lazy | Lazy + intervalos | Vanilla | Redução antes → intervalos |
| --- | ---: | ---: | ---: | ---: | ---: |
| context | 292.029 | 289.029 | 184.126 | 136.423 | 36.95% |
| image | 287.997 | 290.440 | 182.639 | 11.189 | 36.58% |
| pipeline-single-font | 290.818 | 286.696 | 184.603 | 135.635 | 36.52% |
| shapes | 292.066 | 295.690 | 186.558 | 11.908 | 36.12% |
| text-dense | 303.981 | 301.158 | 195.264 | 140.831 | 35.76% |

Dispersão e diferenças pareadas, também em milissegundos:

| Fixture | Antes p05–p95 | Intervalos p05–p95 | Ganho pareado médio antes → intervalos | Ganho pareado médio lazy → intervalos | Gap pareado médio intervalos → vanilla |
| --- | ---: | ---: | ---: | ---: | ---: |
| context | 274.569–307.961 | 173.082–197.698 | 107.408 | 103.371 | 45.528 |
| image | 283.679–304.350 | 179.576–199.613 | 103.720 | 106.732 | 174.244 |
| pipeline-single-font | 275.196–330.073 | 174.737–216.789 | 105.573 | 101.308 | 51.325 |
| shapes | 278.376–308.601 | 176.097–199.757 | 105.266 | 108.289 | 174.681 |
| text-dense | 279.787–346.647 | 180.744–233.930 | 109.028 | 105.952 | 52.554 |

Distribuições completas, incluindo n, média, mediana, SD, min/max, percentis e médias por rodada estão em [e2e-summary.json](font-lifecycle-experiment/e2e-summary.json); resultados individuais, comandos, CPU user/system, RSS e saídas estão em `font-lifecycle-experiment/samples/`. RSS via wait4 pode herdar um piso do processo pai, por isso não é uma estimativa isolada de memória do compilador.

O lazy sozinho apresentou ganhos pareados médios de +4,04 ms em context, −3,01 ms em image, +4,27 ms em Hello, −3,02 ms em shapes e +3,08 ms em text-dense. Os sinais e magnitudes não sustentam um ganho material geral. Lazy → interval reduziu a média pareada em aproximadamente 101–108 ms, com o mesmo restante dos consumers; before → interval, em aproximadamente 104–109 ms. Essa intervenção sustenta a contribuição do mecanismo de construção de cobertura ao ganho observado. Ainda não separa o custo de coleta, parser, ordenação, allocations e queries em operações isoladas.

O gap das medianas atuais para o vanilla é aproximadamente 48–54 ms nas fixtures com texto e 171–175 ms em image/shapes. Não é causalmente atribuído ao Vec remanescente. No vanilla image/shapes não acionam descoberta de fontes nos traces; no Tekt há demanda legítima de baseline/cap-height durante layout, mesmo sem recursos de fonte no PDF. Remover uma consulta de exportação sem fontes não elimina a demanda de layout. O stack registrado aponta `font_metrics::resolution`/`text_edges` e layout de shape, e não autoriza alterar geometria para obter um número menor.

## Custos compatíveis de fases e limites

As fronteiras amplas são disjuntas: eval inicial; estabilização completa incluindo introspecção/context/layout/shaping; exportação PDF completa; residual até o término do processo. No Tekt exportação soma subset e render; no vanilla usa a fronteira PDF completa do trace. Residual inclui setup anterior à instrumentação, I/O e teardown não capturados. Uma descoberta de fontes dentro de eval/layout é **aninhada** e não deve ser somada novamente. Tracing constitui outra população: não subtrair suas médias das medianas plain.

Context manteve a diferença média/mediana das médias de rodada abaixo de 0,45% nos quatro executáveis. A tabela seguinte usa **médias instrumentadas da mesma fixture**, em ms, com as quatro partes somando cada total:

| Variante | Eval inicial | Estabilização/layout | Exportação PDF completa | Residual | Total instrumentado |
| --- | ---: | ---: | ---: | ---: | ---: |
| before | 1.943 | 0.805 | 0.607 | 288.862 | 292.217 |
| lazy | 283.496 | 0.834 | 0.641 | 9.678 | 294.649 |
| interval | 178.979 | 0.945 | 0.665 | 10.177 | 190.766 |
| vanilla | 126.364 | 0.752 | 0.893 | 10.320 | 138.328 |

Antes → lazy deslocou cerca de 279 ms do residual para eval, sem acelerar o total. Lazy → interval reduziu eval em **104,518 ms**, com mudanças pequenas nas outras partes e redução do total instrumentado de **103,883 ms**. O vanilla incluiu média de 125,008 ms de descoberta de fontes dentro de eval; essa duração já está contida nos 126,364 ms. A diferença de eval interval → vanilla nessa população é 52,615 ms, enquanto a diferença dos totais instrumentados é 52,438 ms. Essa associação localiza trabalho a investigar, sem decompor causalmente o restante da aquisição.

As outras quatro fixtures instrumentadas tiveram uma terceira rodada muito mais lenta; falharam no critério descritivo de diferença média/mediana <3% emprestado da ADR-0115, com diferenças de aproximadamente 15–35%. Seus [phase-samples.json](font-lifecycle-experiment/phase-samples.json) e [phase-summary.json](font-lifecycle-experiment/phase-summary.json) ficam disponíveis **como dados descritivos, sem fechamento de custos absolutos reproduzíveis**. Em Hello, as médias totais before por rodada foram aproximadamente 293, 292 e 428 ms; vanilla, 135, 135 e 226 ms. CPU user/system também cresceram. Não foram encontrados fingerprints alterados ou workloads proibidos; throttling, clock, sistema e caches são hipóteses, não causas demonstradas.

Todas as 20 combinações plain fixture/variante passaram aquele critério média/mediana; isso não basta para estabilidade absoluta: o range das médias por rodada chega a cerca de 22%. [repeatability.json](font-lifecycle-experiment/repeatability.json) publica os critérios e resultados, sem descartar a rodada lenta. O plano de microbenchmarks adiciona um limite prévio para variação entre rodadas. A evidência atual apoia o ganho pareado da intervenção, mas não uma previsão universal do tempo em outra máquina.

O probe separado do catálogo do sistema encontrou **1112 faces**, com controle hash de metadados ordenados/cobertura `9156f28234a10295` igual nas duas variantes em todas as amostras. Discovery lazy teve mediana 488,426 ms (p05–p95 465,430–593,053), interval 306,577 ms (287,517–410,986), redução relativa de 37,23%. Diferença pareada média 172,661 ms, SD 42,683, mediana 181,200. Drop retornado: medianas 0,507 e 0,546 ms. [catalog-summary.json](font-lifecycle-experiment/catalog-summary.json) e `catalog/` preservam tudo.

Esse probe foi substancialmente mais lento que a fase contendo aquisição na CLI, embora chame o loader produtivo público. Foram conferidos flags, ambiente, loader usado pela CLI e ausência de um allocator global alternativo no código pesquisado. Ele cobre só sistema, enquanto a CLI inclui embutidas/projeto e executa outros setups; cache/allocator/clock/processo também diferem. **Não somar nem subtrair os 172,661 ms de seus resultados end-to-end.** O benefício relativo pareado é um controle adicional da intervenção; o valor absoluto não resolve a parcela da CLI. O plano prevê comparar contextos de setup dentro do mesmo executável e universo antes de qualquer atribuição.

Exportação PDF completa permaneceu pequena no context estável (0,607–0,893 ms). Nos demais traces, seus valores são apenas descritivos. Os benchmarks `stream_bench` e `pdf_builder_bench` medem exportação completa com builder, recursos e serialização; não se pode transferir esse total para o stream ou somá-lo a microbenchmarks de outras fixtures.

## Validação e arquivos

Testes serializados concluídos antes da medição: 29 testes core de font_book; 271 testes infra selecionados de world/pipeline/fonts/fontdb/font_metrics/shaper; dois testes wiring de compile. Os quatro testes de ciclo de vida verificaram discovery diferido, inicialização concorrente única, invalidação na troca do catálogo e ausência de consulta a fontes durante resolução de recursos de um shape já diagramado. Os logs RED/GREEN foram preservados. O primeiro teste de shape estava indevidamente bloqueando consultas durante todo o layout; foi corrigido para a fronteira de recursos, com backtrace e explicação retidos, sem usar a falha de premissa como prova do problema.

Dois controles independentes BTreeSet cobrem inputs vazios, duplicados, sobreposição, ordem reversa, ordem pseudorrandômica, gaps e máximo Unicode. Não são testes que repetem o algoritmo. Nos PDFs representativos de cada rodada/variante, páginas, dimensões, texto normalizado e nomes de fontes selecionadas passaram. Nos documentos image/shapes sem texto, todas as variantes Tekt mantêm recursos PDF padrão Helvetica/Helvetica-Bold/Helvetica-Oblique; o vanilla não os inclui. Portanto a lista completa de recursos de fonte desses PDFs difere entre engines, sem mudança entre variantes Tekt. A qualificação exige o controle de fontes para documentos com texto, não identidade de recursos não usados. Controles multiscript e fallback calculado preservaram texto/fontes entre variantes Tekt. **Hashes de PDFs completos diferem** e são auxiliares; esses controles não demonstram conformidade visual completa nem paridade global de linguagem. Veja [output-validation.json](font-lifecycle-experiment/output-validation.json) e [fallback-validation.json](font-lifecycle-experiment/fallback-validation.json).

As quatro fixtures históricas P1410, nos quatro executáveis, compilaram sob **RLIMIT_AS de 512 MiB**; os tempos dessas validações não entram nos resultados de desempenho. [memory-results.json](font-lifecycle-experiment/memory-results.json) registra comandos e saídas. Assim não se reintroduziu, nessas sondas, o problema de reter todos os mapeamentos para consultar cobertura.

`cargo bench --manifest-path benches/Cargo.toml --no-run --offline --locked --message-format=json` compilou todos os oito targets existentes, sem editá-los; opt-level 3 e debug assertions desabilitados conferidos em [benchmark-build-profiles.json](font-lifecycle-experiment/benchmark-build-profiles.json). Os runners observation/stdlib continuam protegendo o perfil efetivamente executado. Não se coletaram novas amostras Criterion nesta intervenção; as medições são CLI e probe de catálogo. Os microbenchmarks anteriores permanecem na baseline original, com seus limites.

O linter read-only terminou sem errors; owners modificados não têm V3/V4/V5/V6/V13/V14/V15/V26. Há warnings globais anteriores, inclusive V5 fora desses owners; não se reivindica zero violations global. [lint-audit.json](font-lifecycle-experiment/lint-audit.json) registra essa distinção. Apenas os arquivos Rust editados foram formatados, e somente seus pares L0 receberam resselo local. Não houve formatação ou resselo global, edição de ADR/núcleo compartilhado ou de infra/export.

Os dez pares de arquivos alterados por esta experiência são:

| Consumer | Owner L0 |
| --- | --- |
| `01_core/src/entities/font_book.rs` | `00_nucleo/prompts/entities/font_book.md` |
| `03_infra/src/world.rs` | `00_nucleo/prompts/infra/system-world.md` |
| `03_infra/src/world/fonts.rs` | `00_nucleo/prompts/infra/system-world/fonts.md` |
| `03_infra/src/world/world_impl.rs` | `00_nucleo/prompts/infra/system-world/world_impl.md` |
| `03_infra/src/world/tests.rs` | `00_nucleo/prompts/infra/system-world/tests.md` |
| `03_infra/src/pipeline/resources.rs` | `00_nucleo/prompts/infra/pipeline/resources.md` |
| `03_infra/src/pipeline/pdf.rs` | `00_nucleo/prompts/infra/pipeline/pdf.md` |
| `03_infra/src/pipeline/bundle.rs` | `00_nucleo/prompts/infra/pipeline/bundle.md` |
| `03_infra/src/pipeline/tests.rs` | `00_nucleo/prompts/infra/pipeline/tests.md` |
| `04_wiring/src/compile.rs` | `00_nucleo/prompts/wiring/compile.md` |

Os L0s foram editados antes de seus consumers. font_book agora possui exclusivamente o contrato de book/coverage, referencia o owner comprovado `00_nucleo/prompts/world-types.md` para Font e não duplica o contrato de mmap. Corrigiram-se afirmações documentais falsas sobre clone O(1) e contains logarítmico, sem mudar esses métodos. O histórico anterior está preservado em snapshots e no registro de desenho. `03_infra/src/fonts.rs` e seu L0 foram conservados byte a byte, assim como as alterações recebidas nas demais frentes; [completion-audit.json](font-lifecycle-experiment/completion-audit.json) verifica esse limite.

## Questões ainda abertas

O [plano de microbenchmarks](font-lifecycle-experiment/microbench-plan.md) especifica controles capazes de confirmar ou enfraquecer cada explicação: construtor com a mesma sequência real de cmap e clone fora do timer; decode/collect separado e comparação diagnóstica streaming; metadata/primeiro FontSlot versus slot retido; queries no mesmo book; shaping público com fontes frias/aquecidas; World reaproveitado; e aquisição com setup equivalente para explicar o probe mais lento.

A contribuição do Vec restante, parser/metadata, I/O e queries ao gap do vanilla não foi demonstrada isoladamente. A demanda de métricas em documentos sem texto é observada, mas suprimir métricas não é uma correção autorizada de paridade. Persistência em disco e integração de retenção do World em watch permanecem fora desta experiência. Uma nova coleta instrumentada estável é necessária para fechar custos absolutos das quatro fixtures ruidosas. Nenhum microbenchmark de fixture diferente foi somado ao total, e nenhuma correlação foi declarada causa comprovada.
