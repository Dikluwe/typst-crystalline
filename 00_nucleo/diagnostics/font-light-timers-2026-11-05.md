# Fontes: descoberta e aquisição com timers leves

**Resultado:** no texto curto, descoberta e aquisição inicial somaram aproximadamente 110ms wall nesta bancada: cerca de 49ms na descoberta e 62ms na aquisição. Dentro da aquisição, projeção/transporte ficou em torno de 34ms; derivação de FontInfo em 2,3ms; segundo Face::parse em 0,4ms. A confirmação do medidor ficou dentro do critério de 3% de diferença pareada. Isso sustenta investigar transporte/projeção e descoberta antes de memoizar os metadados. Nenhuma otimização ou alteração de L0/produto foi implementada.

## Medição e controle do observador

Continuação de [duplicação descoberta/aquisição](font-discovery-acquisition-2026-11-05.md). A sonda anterior registrava cada operação em um journal, serializado por ciclo, e acrescentava cerca de 100ms ao processo; suas contagens continuam úteis, mas não isolavam o custo líquido original. A nova sonda usa somente std, array fixo em TLS e Instant. Sem eventos por read, paths/strings/HashMap/Mutex por face. Um TSV de 14 linhas é gravado ao terminar cada compilação.

Coarse cronometra somente fontdb::Database::load_system_fonts e catalog_from_database. Detail acrescenta scopes agregados por arquivo/face e sete blocos disjuntos de acquire: open, metadata inicial, project, FontInfo, Face da cobertura, cobertura, guards finais. Coarse mantém os detalhes zerados; Off não chama Instant nem grava arquivo. Os hooks ficam exclusivamente no fork/cópias RAM. Não houve mudanças a parsers, read_at, budgets, ordem, fallback, frescor ou bytes transportados.

Primeiro lote: sete rodadas, quatro variantes, cinco fixtures herdadas, CLI/startup em processos novos: 280 execuções. A variação entre execuções foi grande: Coarse versus baseline teve delta wall pareado de −4,23% a +4,98% nas combinações. O critério de 3% não foi considerado demonstrado por esse lote, e as amostras foram mantidas.

Confirmação: somente plain, oito rodadas com ordem das quatro variantes rotacionada, dez processos consecutivos por variante/modo: 640 execuções. Nenhum extrator/rasterizador de PDF, compilador ou strace concorrente durante esses blocos. Comparações de PDF ocorreram depois do lote. Medianas de oito médias de dez processos:

|Modo|Original CPU / wall ms|Off CPU / wall ms|Coarse CPU / wall ms|Detail CPU / wall ms|
|---|---:|---:|---:|---:|
|cli|124.43 / 126.96|121.63 / 124.86|121.52 / 123.99|122.55 / 125.96|
|startup|185.76 / 186.68|185.35 / 186.24|185.44 / 186.13|183.50 / 185.02|

Diferença pareada do observador ligado versus Off, mesmo binário e input:

|Modo|Coarse CPU / wall|Detail CPU / wall|
|---|---:|---:|
|cli|+0.72% / +0.53%|+0.84% / +1.11%|
|startup|+0.52% / +0.16%|+0.89% / +0.92%|

Os deltas versus baseline também ficaram em módulo abaixo de 3% na confirmação (Coarse CLI −1,20% CPU/−1,38% wall; startup +0,61%/+0,65%). Valores negativos não são ganho de uma otimização: Off também variou, e não há algoritmo candidato. Deltas são calculados por bloco pareado antes da mediana, portanto não equivalem à divisão das medianas da primeira tabela. O controle qualifica a ordem de grandeza nesta bancada; não certifica overhead universal ou todos os componentes individualmente.

CLI usa wait4 para CPU usuário+kernel e pico RSS do filho completo. Watch mede wall até o primeiro PDF completo (poll 1ms); termina 30ms depois e recolhe CPU/RSS por wait4, incluindo threads encerrados e o encerramento do processo. CPU e wall do watch têm janelas diferentes. Isso corrige a limitação da rodada anterior, que somava apenas schedstat de threads ainda vivos. Não comparar os dois métodos como regressão do produto. Tempos por fase abaixo são **wall, não CPU**.

## Onde o tempo se concentra

Confirmação plain; mediana das oito médias de dez processos. Coarse:

|Escopo|CLI ms|Startup watch ms|
|---|---:|---:|
|discovery.total|48.94|47.72|
|acquisition.total|62.24|61.65|
|fonts.subset|109.83|109.53|

fonts.subset soma os dois intervalos disjuntos em cada amostra antes de agregá-los. Não somar medianas independentes. Esses dois scopes representaram aproximadamente 89% do wall da CLI curta e 59% do startup medido em Coarse. Não incluem toda preparação/validação de geração nem os demais custos do watch. Discovery.total cobre a descoberta fontdb; acquisition.total cobre a construção do catálogo, incluindo pairing/inserção.

A dispersão continua visível: os oito blocos Coarse tiveram discovery 46,63–54,04ms CLI e 46,95–57,62ms startup; acquisition 60,45–67,80ms e 59,98–71,56ms. MAD/mediana da descoberta CLI foi 3,53%, portanto não se reivindica precisão fina nessa parcela. A soma dos dois scopes teve MAD/mediana 1,06% CLI e 1,60% startup. A priorização abaixo usa diferenças amplas, não centésimos de milissegundo.

Detail, mesmos blocos e corpo produtivo:

|Escopo observado|CLI ms|Startup watch ms|
|---|---:|---:|
|discovery.file_residual|28.03|28.06|
|discovery.outer|14.08|13.56|
|discovery.faces|6.03|6.05|
|discovery.names|5.49|5.48|
|acquisition.project|33.51|33.74|
|acquisition.coverage|10.04|9.91|
|acquisition.open|5.36|5.50|
|acquisition.initial_metadata|1.51|1.55|
|acquisition.guards|5.14|5.24|
|acquisition.metadata|2.27|2.31|
|acquisition.coverage_face|0.41|0.43|
|acquisition.residual|3.98|4.04|

Resíduos calculados por amostra, nunca por subtração de medianas: discovery.file_residual = files−faces; discovery.outer = total−files; acquisition.residual = total−soma dos sete blocos disjuntos. Files−faces inclui open/mmap/unmap, inserções e outras operações fora do parser da face; não isola uma syscall nem somente I/O. Total−files inclui configuração/percurso/outros trabalhos da descoberta. O resíduo da aquisição inclui pairing/book, closes, liberações e overhead dos scopes. Faces contém nomes/OS2/post; esses spans aninhados não devem ser somados ao pai.

Cada observação Detail teve 1.086 scopes de arquivo e 1.112 de cada etapa por face. Não foram misturadas as 17 embutidas nos rótulos seletivos desta sonda; fonts.rs e o callback de fallback permaneceram intocados. Os controles confirmaram o mesmo número de requests por path entre original, Coarse e Detail: 2.199 openat, 1.087 mmap, 2.199 close, 11.180 pread64 e 4.423 statx sobre paths do catálogo. Não se afirma equivalência de todos os argumentos/ordens somente a partir dessas contagens. A abertura/mapping adicional fora das 2.198 aberturas descoberta+aquisição é de NotoColorEmoji. Isso refina a interpretação anterior: o PDF curto embute somente LibertinusSerif; a contagem adicional não prova que a fonte mapeada tenha sido usada no PDF.

Scopes pequenos podem ter proporção maior de custo do relógio; 0,4ms do segundo parse não é custo puro exato. Ainda assim, a diferença em relação aos 34ms de projeção é ampla. A leitura atual de post representa 32,93 dos 51,12MB da aquisição conforme a captura anterior, mas **esta bancada não isolou quanto dos 34ms pertence a post**. Projeção também inclui outras tabelas, headers, diretórios, alocação/cópia e chamadas de read_at.

## Próximo experimento delimitado

Priorizar uma bancada observacional dentro do projector atual: separar post das demais faixas e distinguir read_at, alocação/cópia e montagem do buffer. Manter corpo do parser, validação completa de bounds/prefixos, budgets, ordem, fallback e guards após a derivação. Calibrar esse detalhamento antes de atribuir a post todo o custo de project.

A hipótese é reduzir transporte exigido só pela representação intermediária; a fração de bytes não é promessa de tempo poupado. O L0 selective vigente exige os índices post v2 e proíbe fabricar versão/pitch/índices. A medição não autoriza remover leituras, normalizar a tabela ou compartilhar mmap mutável como snapshot. Um eventual candidato produtivo depende de L0 atualizado/classificação ADR0127 e bancada adversarial própria.

Memoizar FaceInfo→FontInfo permanece sem base de correção (482/1.112 divergências nos campos comuns da rodada anterior) e a derivação seletiva custa apenas cerca de 2,3ms nesta bancada. Reutilizar o segundo Face::parse também não aparece como frente principal. A descoberta tem trabalho substancial fora do parse de faces; eliminar somente parses não elimina seus cerca de 49ms. Prova de frescor no watch aquecido permanece separada desta medição inicial.

## Validação, proveniência e limites

Skill Tekt: A código, B testes independentes, C integração/execução. Passaram 12 testes Rust, 16 Python e 14/14 mutações dirigidas (7 coletor, 7 analisador), sem sobreviventes. Controles de runtime validaram Off sem arquivo, modo inválido desligado, headers repetidos e reset de dois ciclos. Nos lotes: 920 execuções, 460 TSVs íntegros; 210 pares funcionais no lote inicial e 12 na confirmação preservaram texto, geometria/páginas e nomes de fontes; 30 pares raster 72dpi de todas as páginas passaram na primeira rodada. As outras execuções da confirmação validaram PDF completo e TSV, sem equivaler a uma comparação funcional individual. Não foi executada a suíte completa core/infra. Builds release passaram; crystalline-lint . terminou com exit 0 e notas informativas existentes.

Inventário original fontdb antes/depois de ambos os lotes preservou ordem path/índice e dev/ino/size/mode/mtime_ns/ctime_ns dos 1.086 arquivos/1.112 faces. É uma qualificação do corpus observado estável, não snapshot atômico ou prova adversarial sob escritores concorrentes. Mesma afinidade CPU0, RAYON_NUM_THREADS=1, stack 64MiB, SOURCE_DATE_EPOCH=0; caches do host aquecidos, sem limpeza de page cache ou mudança do governador de CPU. Sem nova medição contra vanilla.

HEAD `3c6b0ce7b7632cc74b004cea6681e4746e63d28d`, uncommitted working tree, congelado em `2026-11-05T04:01:55.968274+00:00`. O diffstat rastreado recebido é **exatamente idêntico** à [lista de 182 arquivos da rodada anterior](font-discovery-acquisition-2026-11-05.md#proveniência-e-reprodução), conferido antes de usar essa referência. Nenhum arquivo recebido de código/L0/configuração/diagnóstico foi alterado. O novo resultado é este diagnóstico e seus CSVs locais; os deltas de instrumentação existiram somente na cópia RAM.

Lote inicial `2026-11-05T04:13:54.104489+00:00` até `2026-11-05T04:15:24.152257+00:00`; confirmação `2026-11-05T04:17:14.971148+00:00` até `2026-11-05T04:19:06.887434+00:00`. Mesmos binários/entradas durante os dois lotes; apenas ordem, repetições e retirada dos extratores entre amostras mudaram na confirmação.

Baseline SHA-256 `902cfde0215fe0a4c5e4e9713820a1eae1b339e7aaa0d8005439849c2fdaa84c` (idêntico à rodada anterior); instrumentado `d309018f0f66f4ed84403f0f1271cccd55b4a0259deed8b7db8e61458c94cbd6`. Rust 1.92.0, Linux 6.17.0-42-generic x86_64, fontdb 0.23.0, ttf-parser 0.25.1. Cargo offline/locked release/jobs 1/incremental 0; único pacote adicional std-only font_light, sem atualização de versões existentes. Builds MemoryHigh 8G/MemoryMax 12G/MemorySwapMax 0, TYPST_COMMIT_SHA igual ao HEAD.

Receita CLI/watch: `compile|watch entrada saida --root fixtures --pdf-tagged=false`; as [sete fixtures herdadas](../../lab/results/construction/diagnostics/font-discovery-acquisition-2026-11-05/inputs/) foram copiadas sem mudanças. Baseline não tem hooks; Off/Coarse/Detail usam o mesmo binário, com E_LIGHT_MODE/E_LIGHT_OUT exclusivos da bancada. 14 labels fixos, ns por Instant e um TSV por ciclo; sem medição CPU por fase. Compiladores, caches, instrumentação, logs, PDFs, testes e mutantes ficaram exclusivamente em /dev/shm/tekt-font-light-20261105, removido após verificação; nenhuma limpeza de outra bancada e nenhuma integração ao produto.

[Amostras de 920 processos](../../lab/results/construction/diagnostics/font-light-timers-2026-11-05/amostras.csv), [64 blocos de confirmação](../../lab/results/construction/diagnostics/font-light-timers-2026-11-05/blocos-confirmacao.csv) e [6.440 registros de fase](../../lab/results/construction/diagnostics/font-light-timers-2026-11-05/fases.csv). CSVs de domínio seguem o ignore local de lab, sem force-add; não se preservaram logs/manifestos descartáveis.
