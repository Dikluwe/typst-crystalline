# Startup do watch: prova de fontes versus publicação

**Resultado:** o trabalho adicional dominante do startup é estabelecer e revalidar a geração de fontes retida para os ciclos seguintes. Na entrada curta, as cinco etapas disjuntas dessa prova somaram cerca de 51,76ms wall; setup/prelúdio/arm/publicação/notificação somaram 0,104ms. Não houve uma segunda descoberta nem uma segunda aquisição do catálogo. Nenhuma otimização, remoção de verificação ou alteração produtiva/L0 foi implementada.

## Lote e calibração

Cópia original do Tekt, mesmo binário instrumentado Off e On; oito rounds×cinco fixtures×CLI/startup×três repetições×três variantes=720 processos medidos. Um warmup por fixture/modo/variante, 30 fora das estatísticas. Fixture/root/fonts/flags/env/afinidade iguais; cada watch é um processo novo. Sem compiladores, strace ou extratores/rasterizadores concorrentes nos relógios.

Medianas de oito médias de três processos na entrada plain; wall até saída CLI ou primeiro PDF completo no watch:

|Modo|Original ms|Sonda Off ms|Sonda On ms|On−Off CPU / wall pareado|
|---|---:|---:|---:|---:|
|cli|97.72|95.47|95.77|+0.31% / +0.30%|
|startup|148.92|155.29|154.56|+0.12% / -0.07%|

Todas as dez combinações passaram no critério de módulo <3% para os deltas pareados On−Off e On−original em CPU/wall. O maior módulo On−Off wall foi 1,87% (outline CLI). Não há candidato otimizado: números negativos são variação/efeito da instrumentação/código gerado, não ganho. Deltas são calculados em cada round antes da mediana; não equivalem à divisão das medianas da tabela.

CPU=wait4 utime+stime e pico RSS=ru_maxrss Linux KiB×1024. Watch: poll 1ms até trailer %%EOF completo, espera 30ms, SIGTERM e wait4; CPU/RSS incluem estabilização/encerramento e threads encerrados, wall não. As fases internas são **wall por Instant**, não CPU. O final do ciclo instrumentado ocorre após publicação/notificação e antes de evict/wait; não coincide exatamente com a observação externa do PDF.

## Decomposição da entrada curta

On, mediana das oito médias de três processos; valores por fase inclusivos. Os campos abaixo que formam a prova são disjuntos dentro da aquisição inicial da sessão:

|Etapa da prova adicional|Wall ms|
|---|---:|
|Capturar configuração/raízes antes da descoberta|26.957|
|Montar identidades e autovalidar a preparação|7.355|
|Revalidar a preparação antes da construção|3.375|
|Validar a geração depois de construir o catálogo|3.469|
|Revalidar configuração/raízes depois da construção|10.607|
|Soma por amostra das cinco etapas|51.757|

A soma é feita em cada observação antes de calcular médias/medianas; não somar as medianas da tabela. Esses 51,76ms não são inteiramente elimináveis demonstrados: as verificações ocupam barreiras temporais diferentes e asseguram a retenção correta. Chamadas ao mesmo path não são automaticamente redundantes.

A preparação da prova inclui um percurso inicial de raízes, autovalidação e rechecagem final. Observação auxiliar, aninhada nos pais acima: fast.traverse 23,522ms (uma chamada), fast.config 6,063ms (duas), probe.verify 15,194ms (duas), generation.verify 10,499ms (três). Não somar esses valores aos pais. Generation.verify ocorre durante prepare_from, depois de prepare e depois da construção; não são três tentativas equivalentes de layout.

Infraestrutura do watch, fora da compilação:

|Escopo|Wall ms|
|---|---:|
|watch.setup|0.0279|
|watch.prelude|0.0543|
|watch.arm|0.0135|
|watch.publish|0.0098|
|watch.post_publish|0.0001|
|watch.shell_total|0.1039|

Setup inclui nomes de staging e construção das sessões; seus construtores continuam lazy, sem aquisição. Arm captura dependências antes de publicar; publish mede rename/resultado; post_publish é o bloco de notificação, vazio no PDF. Esses números qualificam a bancada com entradas/saídas em tmpfs, não todo storage/servidor/HTML. Escopos muito pequenos podem ter proporção relevante de relógio e não são custos puros exatos. Ainda assim, sua escala não explica dezenas de milissegundos adicionais.

Contexto inclusivo das fases comuns e da compilação, no mesmo lote On:

|Escopo|CLI ms|Startup watch ms|
|---|---:|---:|
|cycle.total|90.025|149.349|
|compile.observed|90.024|149.251|
|compile.pipeline|89.526|148.971|
|discovery.system|36.833|39.748|
|catalog.acquire|48.256|52.207|
|catalog.compose|85.739|52.827|
|font.session|0.000|144.759|
|pipeline.other|3.871|4.362|

Catalog.compose tem fronteiras diferentes conforme a rota: fresh_catalog na CLI inclui descoberta+aquisição; prepared_catalog no watch recebe uma descoberta já preparada e inclui aquisição/composição. Font.session engloba a rota retida inteira, inclusive prova/preparação/construção. Pipeline.other subtrai por observação font.session no watch ou catalog.compose na CLI do match de compilação; é o restante desse match, não uma separação pura de parse/eval/layout/export. Não somar pais e filhos.

Estabelecimento da prova nas outras fixtures, com a mesma matriz de chamadas:

|Fixture|Prova adicional ms|Infraestrutura watch ms|
|---|---:|---:|
|plain|51.757|0.1039|
|import_syntax|49.629|0.1238|
|import_compute|49.148|0.1163|
|layout|58.079|0.1117|
|outline|55.867|0.1106|

Esses dados explicam a ordem de grandeza do acréscimo do watch sobre a CLI do próprio Tekt. Este lote não reexecutou vanilla; 51,76ms não é a diferença Tekt−vanilla. Não misturar os tempos absolutos com os de outras rodadas/horários para atribuir uma regressão ou ganho.

## Consultas e ausência de aquisição duplicada

Trace posterior, fora dos relógios; somente requests cujos paths pertencem aos 1.086 arquivos do catálogo. Original e On tiveram contagens idênticas por operação/path tanto na CLI como no startup:

|Operação sobre paths do catálogo|CLI|Startup watch|
|---|---:|---:|
|openat|2199|2199|
|close|2199|2199|
|mmap|1087|1087|
|pread64|11180|11180|
|statx|4423|15243|

São 10.820 statx adicionais no startup sobre esse conjunto de paths, sem aberturas/leituras posicionais adicionais de fontes. Não é a contagem completa do filesystem: configuração, diretórios e paths fora do catálogo ficam fora desse filtro. O trace não mede performance nem prova sozinho equivalência de argumentos/ordens. As rechecagens seguem a ordem atual dos oito campos Unix e as provas de raízes/configuração do L0.

Nas 240 observações instrumentadas houve exatamente uma descoberta e uma aquisição/composição; sessão foi chamada uma vez no watch e zero na CLI. Todo primeiro ciclo watch teve probe/prepare/build e verificações esperadas; zero cached-hit/reuse/fresh-fallback. Não se confirmou a hipótese de repetir fontdb ou reconstruir duas vezes o mesmo catálogo no startup.

Controle posterior Alpha→Beta: o segundo ciclo publicou Beta, teve cached_fast/cached_generation/fast.verify/probe.verify/generation.verify uma vez cada e zero setup/probe_before/prepare/build/discovery/acquire/compose. Isso confirma a separação entre estabelecimento inicial e reúso validado. É um controle único de lifecycle, não benchmark ou certificação de cache/frescor universal; o arquivo RAM foi restaurado após o teste.

## Decisão após a medição

A frente dominante é a coleta e rechecagem da prova inicial, não os construtores de sessão, publicação ou uma segunda aquisição. Não retirar GenerationKey::unchanged, mover checks, memoizar resultados entre barreiras ou reduzir frescor com base nas contagens. Os owners atuais [generation](../prompts/infra/fontdb/generation.md), [font_session](../prompts/infra/world/font_session.md) e [watch](../prompts/wiring/watch.md) exigem essas barreiras e capture-before-publish.

Candidato para a próxima bancada, **ainda sem prova de segurança/ganho**: investigar coleta de evidências durante a descoberta, confrontando catálogo/prova com o preparador fresco atual e preservando rechecagens/ordem/associações/erros. A medição torna essa região prioritária, mas não autoriza integrar um novo scanner nem reusar uma prova entre fases. É diferente de copiar fontes eager por compilação, que o [experimento 5](bottleneck-experiment5-2026-11-04.md) já não sustentou.

Primeiro delimitar quais observações poderiam ser compartilhadas na mesma barreira; depois modelo adversarial de alterações durante varredura/construção/configuração/links/fontes inválidas e recuperação, e medição líquida do pipeline. Se não houver prova equivalente e benefício repetível, rejeitar. Ganho de 51,76ms não foi demonstrado nem é promessa. Qualquer alteração produtiva exige L0 atual antes do código e classificação ADR0127; mudar deliberadamente a garantia é decisão de contrato separada. Nenhuma dessas mudanças ocorreu aqui.

## Validação e proveniência

Skill Tekt: A instrumentação, B testes independentes, C integração/execução. Passaram 12 testes Rust, 14 Python e 16/16 mutações dirigidas (7 aritmética/labels/encode, 7 parser, 2 lifecycle runtime), sem sobreviventes. Omissão de CycleTotal e falta de reset do TLS foram rejeitadas pelo controle real de dois ciclos. Off/modo inválido não criaram dump. Builds original/instrumentado e crystalline-lint . terminaram com exit 0; não foi executada a suíte completa core/infra.

Setecentos e vinte processos medidos sem falha; 240 TSVs íntegros, 29 labels ordenados, errors 0, matriz de chamadas e resíduos disjuntos não negativos. Após o lote, 720 PDFs extraídos: 480 comparações pareadas Off/On com original e 690 de estabilidade por variante passaram em texto, bbox, páginas e fontes. Vinte pares raster 72dpi, todas as 80 páginas comparadas idênticas ao original. Isso preserva também o comportamento atual de outline; não é nova certificação de paridade com vanilla.

Inventário fontdb antes/depois preservou ordem path/índice e dev/ino/size/mode/mtime_ns/ctime_ns dos 1.086 arquivos/1.112 faces. Corpus observado estável não significa snapshot atômico ou prova de concorrência. Nenhuma consulta de filesystem extra foi introduzida pelo medidor na aquisição/prova; ele usa array TLS fixo e Instant, sem eventos por query/face. Um TSV de 29 registros é escrito por ciclo depois da publicação no watch; não entra no wall até publicação, mas pode entrar na CPU coletada até encerramento. Calibração cobre esse efeito operacional.

HEAD `3c6b0ce7b7632cc74b004cea6681e4746e63d28d`, uncommitted working tree. Congelamento `2026-11-05T12:17:03.203167+00:00`. Diffstat rastreado **exatamente idêntico** à [lista de 182 arquivos da rodada anterior](font-discovery-acquisition-2026-11-05.md#proveniência-e-reprodução), conferido byte a byte. Todos os arquivos recebidos foram preservados; somente este diagnóstico e CSVs de domínio foram acrescentados. Instrumentação existiu apenas em cópias RAM de seis consumers L3/L4 e crate std-only, sem código L1, mudanças de versões de dependências, L0 ou selos.

Lote incluindo warmups `2026-11-05T12:31:05.051789+00:00` até `2026-11-05T12:33:30.943818+00:00`; binários/entradas fixos nesse intervalo. Controle descartável Alpha→Beta posterior `2026-11-05T12:43:12.158096+00:00` até `2026-11-05T12:43:12.439177+00:00`, com bytes restaurados após o teste. Fonte produtiva não mudou entre essas medições.

Baseline SHA-256 `902cfde0215fe0a4c5e4e9713820a1eae1b339e7aaa0d8005439849c2fdaa84c` (idêntico às rodadas anteriores); instrumentado `02da7738579553c76a1719ce0be7e38b08ec4043d0220f6a1385bf77519bc510`. Rust 1.92.0, Linux 6.17.0-42-generic x86_64, fontdb 0.23.0. Cargo offline/locked release --bin typst, jobs 1/incremental 0, TYPST_COMMIT_SHA igual ao HEAD; build MemoryHigh 8G/MemoryMax 12G/MemorySwapMax 0.

Mesmo CPU0, RAYON_NUM_THREADS=1, stack 64MiB, SOURCE_DATE_EPOCH=0, TZ UTC e NO_COLOR=1. Flags `compile|watch entrada saida --root fixtures --pdf-tagged=false`; sem remoção de fontes de sistema, mudanças de governador ou limpeza de page cache. [Sete fixtures herdadas](../../lab/results/construction/diagnostics/font-discovery-acquisition-2026-11-05/inputs/) copiadas sem alterações. CPU/peakRSS externos por wait4; CLI bloqueia em pidfd/select, watch observa EOF. Timers internos inclusivos não são CPU e não foram somados quando aninhados. Todo build, fonte descartável, cache, teste, PDF, trace/log e mutante ficou em /dev/shm/tekt-watch-startup-20261105, removido após verificação; nenhuma limpeza de outra bancada.

[Amostras](../../lab/results/construction/diagnostics/watch-startup-decomposition-2026-11-05/amostras.csv), [240 médias de blocos](../../lab/results/construction/diagnostics/watch-startup-decomposition-2026-11-05/blocos.csv), [resumo/calibração](../../lab/results/construction/diagnostics/watch-startup-decomposition-2026-11-05/resumo.csv), [6.960 registros de fase](../../lab/results/construction/diagnostics/watch-startup-decomposition-2026-11-05/fases.csv), [480 comparações](../../lab/results/construction/diagnostics/watch-startup-decomposition-2026-11-05/comparacoes.csv), [contagem de consultas](../../lab/results/construction/diagnostics/watch-startup-decomposition-2026-11-05/consultas-fontes.csv) e [58 contagens do controle hot](../../lab/results/construction/diagnostics/watch-startup-decomposition-2026-11-05/controle-hot.csv). São dados locais sob o ignore existente de lab, sem force-add. Não se preservaram manifestos/logs/mutantes descartáveis.
