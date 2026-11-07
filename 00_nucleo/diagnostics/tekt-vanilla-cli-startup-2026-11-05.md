# Tekt atual × vanilla: mesmo lote CLI e startup do watch

**Resultado:** o Tekt teve menor tempo total de CLI nas quatro fixtures com saídas comparáveis, mas maior tempo para publicar o primeiro PDF no watch. No texto curto: 94,18 versus 127,26ms na CLI; 138,42 versus 128,14ms no startup. Outline produziu texto extraído/geometria diferentes e seus tempos são somente descritivos. Nenhuma otimização, correção de layout ou alteração produtiva/L0 foi implementada.

## Tempo total no mesmo lote

Medianas de 12 médias de três processos novos por compilador/cenário; ms wall. Todos os valores desta tabela pertencem ao mesmo lote:

|Fixture|CLI Tekt|CLI vanilla|Startup watch Tekt|Startup watch vanilla|
|---|---:|---:|---:|---:|
|plain|94.18|127.26|138.42|128.14|
|import_syntax|96.48|131.37|141.18|131.69|
|import_compute|95.52|130.41|141.84|130.87|
|layout|112.48|133.41|161.21|132.75|
|outline †|216.64|136.23|261.54|136.25|

† Mesma entrada, porém saídas não comparáveis nos controles de texto/geometria. Não classificar outline como ganho/perda para trabalho equivalente. O lote não é uma demonstração de superioridade universal de nenhum compilador.

Variação percentual pareada do wall Tekt em relação ao vanilla, somente nas saídas comparáveis. Negativo significa menos tempo no Tekt. IC95% por bootstrap de 12 rounds pareados, 10.000 reamostragens de rounds, sem tratar os 36 processos por engine como amostras independentes:

|Fixture|Cenário|Delta pareado|IC95%|
|---|---|---:|---:|
|plain|cli|-26.49%|[-26.98%; -25.78%]|
|plain|startup|+8.33%|[+7.83%; +9.47%]|
|import_syntax|cli|-26.70%|[-27.71%; -25.94%]|
|import_syntax|startup|+7.01%|[+6.11%; +9.85%]|
|import_compute|cli|-27.14%|[-27.24%; -26.78%]|
|import_compute|startup|+8.09%|[+7.01%; +9.31%]|
|layout|cli|-14.78%|[-16.11%; -14.03%]|
|layout|startup|+21.57%|[+20.49%; +22.99%]|

Assim, na entrada curta o Tekt consumiu cerca de 26,5% menos wall na CLI e 8,3% mais no startup do watch. As razões são calculadas em cada round antes da mediana e não necessariamente coincidem com a divisão das medianas da tabela. A variação do host permanece nas amostras; não foram descartados outliers, substituídas execuções ou escolhidos mínimos.

CPU e pico RSS dos mesmos processos (CPU em ms, RSS em MiB, 1MiB=2²⁰ bytes):

|Fixture|Cenário|CPU Tekt / vanilla|Pico RSS Tekt / vanilla|
|---|---|---:|---:|
|plain|cli|92.26 / 124.96|24.45 / 29.63|
|plain|startup|138.34 / 128.11|25.42 / 29.85|
|import_syntax|cli|94.71 / 129.13|26.18 / 30.84|
|import_syntax|startup|141.35 / 131.60|27.55 / 30.98|
|import_compute|cli|93.67 / 128.15|24.87 / 29.89|
|import_compute|startup|141.51 / 130.54|26.16 / 30.01|
|layout|cli|110.63 / 131.08|26.26 / 31.28|
|layout|startup|161.17 / 132.66|27.15 / 31.31|
|outline †|cli|214.86 / 134.00|50.75 / 33.12|
|outline †|startup|263.59 / 136.20|51.62 / 32.97|

CPU é utime+stime via wait4, incluindo threads já encerrados; RSS é ru_maxrss Linux em KiB convertido em bytes. Na CLI ambos abrangem o processo completo. No watch, wall vai do instante anterior ao Popen até o primeiro PDF completo; espera-se 30ms depois da publicação, envia-se SIGTERM e coleta-se wait4. CPU/RSS do watch incluem essa janela e o encerramento; wall e CPU não têm exatamente a mesma fronteira. Os banners de tempo interno do vanilla não foram usados, pois não abrangem todo o startup/descoberta inicial.

## Conferência das saídas

Todos os 720 PDFs foram extraídos depois do lote. Houve 360 pares bilaterais e 700 comparações com a primeira saída do próprio engine/fixture/modo. Não houve variação interna que falhasse nos controles. Cada fixture apresenta os mesmos resultados no modo CLI e no startup:

|Fixture|Páginas Tekt / vanilla|Palavras extraídas Tekt / vanilla|Pares bilaterais que passam|
|---|---:|---:|---:|
|plain|1 / 1|2 / 2|72/72|
|import_syntax|1 / 1|2 / 2|72/72|
|import_compute|1 / 1|2 / 2|72/72|
|layout|4 / 4|1001 / 1001|72/72|
|outline|13 / 13|1051 / 1135|0/72|

Nos 288 pares das quatro fixtures comparáveis, texto e sequência de palavras foram iguais, assim como dimensões/páginas/fontes normalizadas. A maior diferença de bbox foi 0,00003pt, dentro da tolerância 0,0001pt declarada antes da execução. Fontes normalizam apenas prefixo de subset de seis letras maiúsculas e o sufixo terminal interno -Identity-H, preservando família, estilo e caixa. Não se usou igualdade de bytes PDF nem igualdade mecânica como definição de linguagem.

Raster 72dpi, primeiro e último round/repeat 0: 20 pares de documentos, 80 pares de páginas. Os 16 pares das fixtures comparáveis foram idênticos pixel a pixel. Nos quatro pares de outline, somente a primeira página diferiu: 10.926 pixels por par, máximo 255 em um canal; 76/80 pares de páginas foram idênticos. Isso é controle representativo, não rasterização de todas as 720 saídas nem certificação global de paridade.

Outline: ambos produziram 13 páginas, mas 1051 versus 1135 palavras extraídas. A inspeção das imagens mostra diferenças de alinhamento/espaçamento nas entradas Heading 10/Heading 11 do sumário, deslocando o conteúdo abaixo; não foi apenas diferença de codificação do nome da fonte. [Primeira página Tekt](../../lab/results/construction/diagnostics/tekt-vanilla-cli-startup-2026-11-05/outline-tekt.png) e [vanilla](../../lab/results/construction/diagnostics/tekt-vanilla-cli-startup-2026-11-05/outline-vanilla.png). O achado foi registrado sem implementar reparo.

## Método e limites da conclusão

São cinco fixtures herdadas, sem alteração: texto curto; import de 2.000 bindings; import com loop 10.000; 100 parágrafos; outline com 12 headings/lorem/pagebreak. As [sete entradas incluindo imports](../../lab/results/construction/diagnostics/font-discovery-acquisition-2026-11-05/inputs/) foram copiadas para um mesmo diretório RAM usado pelos dois binários. Flags comuns: `compile|watch entrada saida --root fixtures --pdf-tagged=false`. Fontes de sistema ficaram habilitadas nos dois; não houve ablação.

Dois warmups por engine/fixture/modo: 40 fora das estatísticas; depois 12 rounds×5 fixtures×2 modos×3 repetições×2 engines=720 processos. Ordem dos engines alternada AB/BA por round/fixture/modo/repetição; ordem das fixtures rotacionada e ordem CLI/startup alternada. Cada comparação usa processos novos, inclusive watch. Não mede recompilação quente.

Mesma afinidade CPU0, RAYON_NUM_THREADS=1, stack 64MiB, SOURCE_DATE_EPOCH=0, TZ UTC, NO_COLOR=1 e TMPDIR RAM; variáveis TYPST_/CRYSTALLINE_/sondas herdadas removidas igualmente. Root/fontconfig/host iguais. Sem mudança de governador, limpeza de page cache, compilação, strace, extração ou rasterização de PDF concorrente durante o lote. Caches do host aquecidos; não é boot frio. PDF completo detectado por trailer %%EOF, poll 1ms no watch; CLI usa pidfd/select bloqueante e wait4, sem wait(timeout) quantizado.

Para cada round, calcula-se a média de três processos por engine; a tabela usa a mediana das 12 médias. Deltas/percentuais e IC usam os mesmos 12 rounds pareados; bootstrap 10.000 com seed 0, estatística mediana e percentis interpolados 2,5%/97,5%. Esses são intervalos empíricos de uma amostra pequena nesse host/corpus, não garantia de estabilidade ou generalização. Timings, CPU, RSS e intervalos completos estão nos CSVs.

A comparação mede os produtos em suas receitas release normais, não algoritmos equivalentes com configurações internas idênticas: Tekt opt-level=s/lto=thin/codegen-units 1; oracle ratificado release/lto=thin/codegen-units 1 e CLI strip. O custo total não atribui a diferença a uma função, ao read_at ou à prova de frescor. Os ~110ms instrumentados de fontes de uma rodada anterior não devem ser subtraídos dos 94ms desta CLI: horários/métodos diferentes não formam pares. Este lote resolve a comparação bilateral total, sem nova decomposição de fontes.

## Validação e proveniência

Skill Tekt: A escreveu a bancada; B escreveu 25 testes independentes sem consultar A; C integrou/executou. Passaram 25 testes e 15/15 mutações dirigidas (CPU versus wall, unidade RSS, média/mediana, pareamento/razões/bootstrap, hashes, sinal do IC, texto/fontes/geometria). Os 720 processos concluíram com status válido, sem timeout, SIGKILL forçado ou amostra descartada. Quatro smokes prévios CLI/startup também passaram; não entram nas estatísticas. Build do Tekt e crystalline-lint . terminaram com exit 0; não foi executada a suíte completa core/infra nem otimização/reparo.

Inventário de descoberta original fontdb antes/depois: mesmos 1.086 arquivos/1.112 faces, ordem path/índice e dev/ino/size/mode/mtime_ns/ctime_ns. Isso qualifica o corpus observado, sem snapshot atômico ou teste adversarial de escritores concorrentes.

Tekt HEAD `3c6b0ce7b7632cc74b004cea6681e4746e63d28d`, uncommitted working tree, congelamento `2026-11-05T11:41:10.751844+00:00`. O diffstat rastreado é **exatamente idêntico** à [lista de 182 arquivos da rodada anterior](font-discovery-acquisition-2026-11-05.md#proveniência-e-reprodução), conferida byte a byte; todos os arquivos recebidos foram preservados. Nenhuma fonte produtiva, prompt, lock ou selo mudou.

Início do lote incluindo warmups `2026-11-05T11:51:11.865190+00:00`; primeira amostra medida `2026-11-05T11:51:18.179545+00:00`; fechamento `2026-11-05T11:53:11.578458+00:00`. Binários e sete entradas permaneceram fixos em todo o intervalo.

Tekt SHA-256 `902cfde0215fe0a4c5e4e9713820a1eae1b339e7aaa0d8005439849c2fdaa84c`, recompilado em RAM e idêntico ao produto congelado das rodadas anteriores. Oracle vanilla commit `64c5e8c02fe0394d913ead36e42ef8c1334d2107`, árvore `483bd6f1d8ac8810db47b65336d402647955fd17`, SHA-256 `22897f93c7c2c8589cf5913b0ce735f1c01ba711c70f55c7a350f7c4b96c0b34`: bytes ratificados verificados antes de copiar/executar em RAM; não houve recompilação ou troca de pin do vanilla. A [receita e verificação de origem](../../lab/results/construction/diagnostics/vanilla-resync-2026-10-26/relatorio.md) continuam a referência. Nenhum --version foi usado como prova de identidade.

Build atual: Rust 1.92.0, Linux 6.17.0-42-generic x86_64, Cargo offline/locked release --bin typst, jobs 1, incremental 0, TYPST_COMMIT_SHA igual ao HEAD, teto MemoryHigh 8G/MemoryMax 12G/MemorySwapMax 0. Workspace/lock recebidos copiados sem mudanças de dependências. Executáveis, caches, fontes de bancada, testes, mutantes, logs e PDFs descartáveis ficaram exclusivamente em /dev/shm/tekt-vanilla-paired-20261105, removido ao concluir; nenhuma limpeza de outra bancada.

Corpus SHA-256 `ae34f4eb653d9cde880be9c1ec43004cdb4a547c26bacf8c21ac72e0225cd517`: os sete nomes ordenados e seus bytes, cada nome/comprimento de conteúdo prefixado por tamanho u64 big-endian. Hashes individuais das entradas principais e identidades dos binários acompanham cada amostra.

[720 amostras](../../lab/results/construction/diagnostics/tekt-vanilla-cli-startup-2026-11-05/amostras.csv), [240 médias de blocos](../../lab/results/construction/diagnostics/tekt-vanilla-cli-startup-2026-11-05/blocos.csv), [resumo CPU/wall/RSS e IC](../../lab/results/construction/diagnostics/tekt-vanilla-cli-startup-2026-11-05/resumo.csv), [360 comparações bilaterais](../../lab/results/construction/diagnostics/tekt-vanilla-cli-startup-2026-11-05/comparacoes.csv) e [80 pares de páginas raster](../../lab/results/construction/diagnostics/tekt-vanilla-cli-startup-2026-11-05/raster.csv). CSVs e duas imagens são dados locais de domínio sob o ignore de lab; não foram forçados ao Git. Não se preservaram manifestos/logs/mutantes descartáveis.
