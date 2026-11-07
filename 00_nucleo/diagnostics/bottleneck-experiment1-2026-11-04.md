# Experimento 1 — fontes, parse e candidatos de layout

Medição de 2026-11-04. O gargalo pequeno permanece aquisição CLI e prova de fontes no watch. Foram identificadas somente 26 repetições da mesma operação/caminho na mesma invocação da prova: hipótese estreita para o experimento 2, sem benefício demonstrado. Nenhuma otimização ou alteração de fonte/L0 produtivo.

## Método e limites

Skill tekt-materializacao-segregada: A instrumentou uma cópia RAM; B escreveu 11 testes independentes sem consultar A; C integrou e validou dados reais, com quatro mutantes direcionados mortos em quatro tentados (tempo exclusivo, chave de fase, contenção de intervalos e saída stale). Os 300 registros temporais passaram no validador de intervalos/cobertura e controles de texto PDF. Isso não certifica paridade geométrica universal.

Cinco fixtures da primeira rodada, três rodadas intercaladas, CLI em processos novos; watch persistente com startup, mesmos bytes, edição pequena e estrutura. CPU fixa, Rayon=1, SOURCE_DATE_EPOCH=0. Duas sondas off/on: detalhada para contagens, reduzida sem eventos stat/lstat/scan_stamp para tempos. Cada sonda gerou 150 registros; strace separado gerou 25 janelas em dez processos originais. Strace não entrou nos benchmarks. Parse guarda apenas parser; Source/hash/numberize são envelopes distintos. Candidate inicia no loop antes de construir Layouter e termina após finish, antes do teste de convergência. Export inclui resolução de recursos/render; escrita e publicação ficam no tempo externo. Tempos exclusivos subtraem filhos instrumentados, não representam exclusividade universal de funções não instrumentadas.

A sonda detalhada perturbou o texto pequeno watch de 19,04 para 588,75 ms CPU: usada apenas para contagens. A sonda reduzida ainda tem overhead e ruído; nenhum tempo é corrigido por subtração de medianas. Durações são wall monotônico por span, CPU externa por processo/threads, RSS CLI pico e watch residente. Medianas de três amostras, sem intervalo de confiança.

## CPU e fases na sonda reduzida

Milissegundos. Off/on são CPU externa; parse/eval/fontes/export são durações exclusivas instrumentadas. Fontes soma os spans de domínio exclusivos, não seus pais. Parse “sem chamada” indica reúso observado, não custo de parsing desconhecido igualado a zero. Startup watch é separado.

|Fixture|Modo/ciclo|CPU off|CPU on|Parse|Eval|Fontes|Export|RSS off/on MiB|
|---|---|---:|---:|---:|---:|---:|---:|---:|
|import_compute|cli/startup|94.136|95.373|0.075|9.278|78.078|0.739|24.7/24.7|
|import_compute|watch/startup|135.183|138.642|0.067|9.135|123.027|0.790|25.9/26.0|
|import_compute|watch/samebytes|25.786|27.235|sem chamada|11.899|12.617|0.351|27.6/27.7|
|import_compute|watch/small|25.225|27.031|0.016|11.177|12.980|0.349|27.7/27.7|
|import_compute|watch/structure|25.790|27.203|0.027|10.925|12.770|0.452|27.7/27.8|
|import_syntax|cli/startup|94.264|96.007|2.258|6.000|79.229|0.748|25.5/26.2|
|import_syntax|watch/startup|134.332|134.306|2.260|6.036|119.733|0.709|27.5/27.3|
|import_syntax|watch/samebytes|23.862|25.271|sem chamada|8.187|14.175|0.483|28.4/27.8|
|import_syntax|watch/small|23.617|25.051|0.032|7.700|14.386|0.573|28.4/28.2|
|import_syntax|watch/structure|23.821|25.242|0.023|7.390|14.449|0.672|28.4/28.2|
|layout|cli/startup|115.870|114.580|0.077|4.050|84.851|5.540|26.4/26.6|
|layout|watch/startup|151.097|158.490|0.078|4.125|128.915|5.459|27.0/27.0|
|layout|watch/samebytes|37.274|40.499|sem chamada|4.381|15.396|4.655|27.2/27.1|
|layout|watch/small|37.864|38.718|0.062|4.095|14.846|4.639|26.7/27.1|
|layout|watch/structure|38.742|39.361|0.037|4.123|14.830|4.922|27.1/27.2|
|outline|cli/startup|213.870|214.944|0.095|26.955|79.409|20.616|50.8/50.7|
|outline|watch/startup|246.103|256.531|0.096|26.803|126.056|20.540|47.2/47.6|
|outline|watch/samebytes|122.714|128.066|sem chamada|27.003|13.012|19.432|47.4/47.5|
|outline|watch/small|126.715|124.919|0.075|25.970|12.904|20.744|47.3/47.6|
|outline|watch/structure|120.460|122.272|0.052|23.917|12.876|19.499|47.4/47.6|
|plain|cli/startup|94.431|95.729|0.072|1.508|85.794|1.100|24.5/24.5|
|plain|watch/startup|138.275|137.645|0.056|1.387|129.310|0.950|25.6/26.0|
|plain|watch/samebytes|18.917|20.659|sem chamada|1.681|16.241|0.561|25.4/25.8|
|plain|watch/small|19.640|20.386|0.024|1.674|15.753|0.676|25.4/25.8|
|plain|watch/structure|19.730|20.791|0.023|1.573|15.905|0.797|25.4/25.9|

## Latência externa até conclusão/publicação

Medianas em ms; CLI inclui término do processo; watch inclui debounce e polling de publicação. A CPU watch é amostrada após estabilização de 80 ms, que não entra na latência.

|Fixture|Modo/ciclo|Off|On|
|---|---|---:|---:|
|import_compute|cli/startup|115.395|115.446|
|import_compute|watch/startup|138.172|140.456|
|import_compute|watch/samebytes|29.689|31.311|
|import_compute|watch/small|30.175|30.152|
|import_compute|watch/structure|29.558|35.491|
|import_syntax|cli/startup|115.544|115.616|
|import_syntax|watch/startup|136.208|136.173|
|import_syntax|watch/samebytes|27.714|29.330|
|import_syntax|watch/small|30.196|29.525|
|import_syntax|watch/structure|27.076|30.050|
|layout|cli/startup|165.518|165.503|
|layout|watch/startup|152.466|160.937|
|layout|watch/samebytes|133.487|137.832|
|layout|watch/small|133.343|134.695|
|layout|watch/structure|134.039|135.362|
|outline|cli/startup|215.648|265.809|
|outline|watch/startup|248.373|259.545|
|outline|watch/samebytes|172.660|182.365|
|outline|watch/small|180.027|179.099|
|outline|watch/structure|175.909|176.723|
|plain|cli/startup|115.536|115.577|
|plain|watch/startup|140.219|140.047|
|plain|watch/samebytes|23.063|25.843|
|plain|watch/small|23.586|25.145|
|plain|watch/structure|25.165|24.996|

Controle final adicional com executável original: uma rodada, 25 registros, todos com texto esperado. CPU original CLI/watch samebytes em ms: plain 95.59/18.43; import_syntax 95.81/23.16; import_compute 96.53/22.24; layout 116.91/36.52; outline 216.57/126.57. Uma rodada não demonstra ausência de perturbação de compilação; a comparação off/on é sobre a mesma cópia instrumentada, e valores próximos ao controle não provam equivalência estatística.

## Candidatos de outline

Medianas por posição, ms inclusivos (incluem fontes quando demandadas). Seis candidatos entre coordenadores, não seis iterações de um único loop. Não se presume igualdade de dependências.

|Modo/ciclo|1|2|3|4|5|6|
|---|---:|---:|---:|---:|---:|---:|
|cli/startup|83.999|2.373|18.701|12.655|10.089|11.961|
|watch/startup|130.663|2.446|18.354|12.454|9.777|11.563|
|watch/samebytes|16.621|2.089|10.690|11.218|8.945|10.854|
|watch/small|16.597|2.132|10.890|11.979|9.210|10.765|
|watch/structure|16.647|2.148|10.333|10.867|8.544|10.945|

## Observações de metadados

Em todos os 45 ciclos watch quentes instrumentados: 3.803 consultas wrappers por ciclo; 2 de configuração em fast_key_verify, 2.689 em probe_verify e 1.112 em generation_verify. Há 2.715 pares caminho/operação distintos, mas a diferença de 1.088 não é redundância removível: atravessa provas e observações com papéis distintos.

Somente 26 repetições dentro da mesma invocação: NotoSansCJK-Bold.ttc e Regular.ttc com dez stats cada; NotoSerifCJK-Bold.ttc e Regular.ttc com cinco cada. Quatro arquivos, trinta observações, quatro chaves: 26 extras. Equivale a 0,68% das consultas wrappers deste ambiente; não é percentual de CPU economizável. A varredura probe não teve repetição da mesma operação/caminho dentro da sua invocação. Lstat e stat não foram unidos.

Wrappers não cobrem chamadas internas de descoberta fontdb. Strace do executável original complementa cobertura externa; inclui loader, fontes, fontes de texto, watcher e publicação. As janelas watch contêm 100 ms de estabilização após publicação, portanto não são contagem exclusiva do compilador. Uma rodada externa; números são específicos deste ambiente.

|Fixture|Modo/ciclo|statx|openat|readlink|getdents64|
|---|---|---:|---:|---:|---:|
|plain|cli/startup|4678|2334|2687|144|
|plain|watch/startup|19344|2685|3566|290|
|plain|watch/samebytes|3926|116|450|2|
|plain|watch/small|3926|116|450|2|
|plain|watch/structure|3926|116|450|2|
|import_syntax|cli/startup|4679|2335|2694|144|
|import_syntax|watch/startup|19349|2688|3580|290|
|import_syntax|watch/samebytes|3930|118|464|2|
|import_syntax|watch/small|3933|120|464|2|
|import_syntax|watch/structure|3931|119|464|2|
|import_compute|cli/startup|4679|2335|2694|144|
|import_compute|watch/startup|19349|2688|3580|290|
|import_compute|watch/samebytes|3931|119|464|2|
|import_compute|watch/small|3933|120|464|2|
|import_compute|watch/structure|3932|119|464|2|
|layout|cli/startup|4678|2334|2687|144|
|layout|watch/startup|19343|2684|3566|290|
|layout|watch/samebytes|3926|116|450|2|
|layout|watch/small|3926|116|450|2|
|layout|watch/structure|3926|116|450|2|
|outline|cli/startup|4679|2335|2687|144|
|outline|watch/startup|19345|2686|3566|290|
|outline|watch/samebytes|3926|116|450|2|
|outline|watch/small|3926|116|450|2|
|outline|watch/structure|3926|116|450|2|

## Experimento 2 delimitado

Primeiro modelo adversarial, sem conectar ao produto: compartilhar identidade de um mesmo arquivo TTC dentro de uma única GenerationKey::unchanged, mantendo associação do índice de cada face. Comparar com a prova atual e aquisição fresca, intercalando substituição, erro/permissão, symlink retarget e restauração de mtime entre observações. Qualquer retenção indevida refuta a variante. Não memoizar verify; não compartilhar entre configuração, ProbeKey, GenerationKey, precheck/postcheck ou ciclos.

O máximo observado dessa hipótese é eliminar 26 consultas por ciclo neste catálogo. Isso enfraquece a expectativa de ganho grande apenas com deduplicação de TTC. Exigir benefício repetível com chave/memória incluídos antes de integrar. Estudar eventual substituição stat após lstat de arquivo regular como outra hipótese, pois muda as oportunidades de observar concorrência; não é deduplicação demonstrada. A maior parcela restante da prova precisa investigação própria.

CLI fresca demanda outra frente: descoberta/leitura de metadados de fontes; o cache TTC intra-prova não a resolve. Para outline, os custos distintos dos candidatos exigem rastrear contexto/regiões/introspecção antes de memoizar. Source/CST já evita parse em samebytes; eval raiz continua executando.

L0 vigente permanece vinculante: 00_nucleo/prompts/infra/world/font_session.md e infra/fontdb/generation.md. Este diagnóstico não autoriza alteração das barreiras ou da garantia de reconhecimento na próxima recompilação.

## Proveniência

Início registrado: 2026-11-04T17:36:13.752733+00:00. Encerramento de análise: 2026-11-04T18:02:36.633706+00:00. HEAD 3c6b0ce7b7632cc74b004cea6681e4746e63d28d, working tree não commitado. Conferência final: os 2413 arquivos fonte/L0/configuração registrados permanecem byte a byte iguais.

Binário baseline SHA-256 `aa7689fb5e6f5628ef4d5870686dd1ff3a42fb3c1904449568904e1662113c81`; sonda reduzida `ea07e4520bed2c516073a201834902e2839daa28940abe15d52e2dff062f37f4`. Rust 1.92.0, offline/locked/release, CARGO_BUILD_JOBS=1, CARGO_INCREMENTAL=0; builds systemd MemoryMax=12G/MemoryHigh=8G/MemorySwapMax=0, todos caches/targets e cópias em /dev/shm/tekt-e1-20261104. Baseline 8m38s, sonda detalhada 4m32s, reduzida 4m30s; zero swap. Uma tentativa intermediária reutilizou core não instrumentado e foi interrompida; recompilação forçada validou eventos reais antes das medições.

As sondas são diagnósticas em cópia descartável: relógio/global auxiliar não foi introduzido em L1 produtivo. Conforme skill, logs, mutantes e builds são temporários em RAM; o resultado persistente é esta medição. A metodologia e limites acima definem a interpretação; não há prova de benefício de otimização.

<details><summary>git diff HEAD --stat exato do estado medido</summary>

```text
 .../core/compiler/eval/bindings/field_access.md    |  19 +-
 .../eval/bindings/field_access/content_methods.md  |  51 +-
 .../compiler/eval/bindings/field_access/fields.md  |  30 +-
 .../core/compiler/eval/observation/elements.md     |  63 ++-
 .../prompts/core/compiler/eval/repr/content.md     |  30 +-
 00_nucleo/prompts/core/compiler/eval/rules/set.md  |  41 +-
 .../prompts/core/compiler/eval/rules/set/text.md   |  42 +-
 00_nucleo/prompts/core/compiler/eval/tests.md      |  29 +-
 00_nucleo/prompts/core/compiler/introspect.md      |  19 +-
 .../core/compiler/introspect/convergence.md        |  19 +-
 .../core/compiler/introspect/figure_demands.md     |  19 +-
 .../core/compiler/introspect/materialize_time.md   |  32 +-
 .../core/compiler/introspect/page_running.md       |  23 +-
 00_nucleo/prompts/core/compiler/introspect/walk.md |  16 +-
 .../prompts/core/compiler/layout/bidi_origin.md    |  34 +-
 00_nucleo/prompts/core/compiler/layout/block.md    |  34 +-
 .../prompts/core/compiler/layout/checkpoint.md     |  61 ++-
 00_nucleo/prompts/core/compiler/layout/columns.md  |  60 ++-
 .../prompts/core/compiler/layout/columns/flow.md   |  18 +-
 .../core/compiler/layout/columns/segmented.md      |  34 +-
 .../core/compiler/layout/columns/transitions.md    |  16 +-
 00_nucleo/prompts/core/compiler/layout/cursor.md   |  40 +-
 00_nucleo/prompts/core/compiler/layout/equation.md |  34 +-
 00_nucleo/prompts/core/compiler/layout/figure.md   |  32 +-
 00_nucleo/prompts/core/compiler/layout/flush.md    |  19 +-
 00_nucleo/prompts/core/compiler/layout/footnote.md |  39 +-
 .../prompts/core/compiler/layout/footnote_flush.md |  24 +-
 .../core/compiler/layout/footnote_fragments.md     |  31 +-
 .../core/compiler/layout/grid/cell_emission.md     |  95 +++-
 .../prompts/core/compiler/layout/grid/tracks.md    |  81 ++-
 00_nucleo/prompts/core/compiler/layout/helpers.md  |  10 +-
 00_nucleo/prompts/core/compiler/layout/image.md    |  58 +-
 .../prompts/core/compiler/layout/measurement.md    |  19 +-
 00_nucleo/prompts/core/compiler/layout/mod.md      |  34 +-
 00_nucleo/prompts/core/compiler/layout/page_run.md |  34 +-
 .../core/compiler/layout/page_running_requests.md  |  53 +-
 .../prompts/core/compiler/layout/paragraph.md      |  34 +-
 .../core/compiler/layout/paragraph_indent.md       |  34 +-
 00_nucleo/prompts/core/compiler/layout/place.md    |  32 +-
 .../prompts/core/compiler/layout/placement.md      |  34 +-
 .../prompts/core/compiler/layout/references.md     |  48 +-
 .../core/compiler/layout/region_progress.md        |  42 +-
 00_nucleo/prompts/core/compiler/layout/repeat.md   |  34 +-
 00_nucleo/prompts/core/compiler/layout/set_page.md |  34 +-
 00_nucleo/prompts/core/compiler/layout/shape.md    |  37 +-
 00_nucleo/prompts/core/compiler/layout/slicing.md  |  10 +-
 00_nucleo/prompts/core/compiler/layout/stack.md    |  69 ++-
 .../prompts/core/compiler/layout/sub_frame.md      |  17 +-
 .../prompts/core/compiler/layout/text/style.md     |  34 +-
 00_nucleo/prompts/core/compiler/layout/tiling.md   |  37 +-
 .../prompts/core/compiler/math/layout/delimited.md |  80 ++-
 .../core/compiler/math/layout/external_bridge.md   |  34 +-
 00_nucleo/prompts/core/compiler/math/layout/mod.md |  81 ++-
 00_nucleo/prompts/core/compiler/math/layout/op.md  |  52 +-
 00_nucleo/prompts/core/compiler/realize/figure.md  |  32 +-
 .../prompts/core/compiler/realize/figure_kind.md   |  19 +-
 .../prompts/core/compiler/stdlib/figure_image.md   |  87 ++-
 .../core/compiler/stdlib/tests/figure_image.md     |  65 ++-
 .../core/compiler/stdlib/text/constructor.md       |  42 +-
 .../prompts/core/compiler/stdlib/visualize.md      |  37 +-
 .../prompts/core/entities/content/construction.md  |  70 ++-
 00_nucleo/prompts/core/entities/content/fields.md  |  30 +-
 .../core/entities/content/tests/traversal.md       |  19 +-
 .../prompts/core/entities/content/traversal.md     |  20 +-
 00_nucleo/prompts/core/entities/elements/figure.md |  20 +-
 00_nucleo/prompts/core/entities/elements/image.md  |  64 ++-
 00_nucleo/prompts/core/entities/image_format.md    |  26 +-
 00_nucleo/prompts/core/entities/image_sizer.md     |  26 +-
 00_nucleo/prompts/core/entities/layout_types.md    |  31 +-
 00_nucleo/prompts/core/entities/mod.md             |  24 +-
 00_nucleo/prompts/core/entities/tiling.md          |  37 +-
 00_nucleo/prompts/infra/export/builder.md          |  15 +
 .../prompts/infra/export/builder/resources.md      |  15 +-
 00_nucleo/prompts/infra/export/images.md           |  31 +-
 00_nucleo/prompts/infra/export/images/png.md       |  20 +-
 00_nucleo/prompts/infra/export/images/scan.md      |  31 +-
 00_nucleo/prompts/infra/export/render.md           |  36 +-
 00_nucleo/prompts/infra/export/stream/draw.md      |  10 +-
 00_nucleo/prompts/infra/export/svg/images.md       |  19 +-
 00_nucleo/prompts/infra/export/svg/tests.md        |  10 +-
 00_nucleo/prompts/infra/export/tests/mod.md        |  27 +-
 00_nucleo/prompts/infra/lib.md                     |  17 +-
 .../infra/pipeline/context_stabilization.md        |  19 +-
 00_nucleo/prompts/infra/pipeline/paged.md          |  17 +-
 00_nucleo/prompts/infra/pipeline/tests.md          | 125 ++++-
 00_nucleo/prompts/infra/shaper/bidi.md             |  37 +-
 00_nucleo/prompts/infra/shaper/shaping.md          |  35 +-
 .../prompts/infra/tests/svg_composite_graph.md     |  10 +-
 00_nucleo/prompts/shell/cli.md                     |  19 +-
 01_core/src/compiler/eval/bindings/field_access.rs |   4 +-
 .../eval/bindings/field_access/content_methods.rs  |  21 +-
 .../compiler/eval/bindings/field_access/fields.rs  |   6 +-
 01_core/src/compiler/eval/observation/elements.rs  |  34 +-
 01_core/src/compiler/eval/repr/content.rs          |  12 +-
 01_core/src/compiler/eval/rules/set.rs             |  54 +-
 01_core/src/compiler/eval/rules/set/text.rs        |  42 +-
 01_core/src/compiler/eval/tests.rs                 |   4 +-
 01_core/src/compiler/introspect.rs                 |   4 +-
 01_core/src/compiler/introspect/convergence.rs     |   4 +-
 01_core/src/compiler/introspect/figure_demands.rs  |   4 +-
 .../src/compiler/introspect/materialize_time.rs    |  34 +-
 01_core/src/compiler/introspect/page_running.rs    |  68 ++-
 01_core/src/compiler/introspect/walk.rs            |  69 ++-
 01_core/src/compiler/layout/bidi_origin.rs         |   7 +-
 01_core/src/compiler/layout/block.rs               |   9 +-
 01_core/src/compiler/layout/checkpoint.rs          |  17 +-
 01_core/src/compiler/layout/columns.rs             |  32 +-
 01_core/src/compiler/layout/columns/flow.rs        |  58 +-
 01_core/src/compiler/layout/columns/segmented.rs   |  30 +-
 01_core/src/compiler/layout/columns/transitions.rs |  72 ++-
 01_core/src/compiler/layout/cursor.rs              |  75 ++-
 01_core/src/compiler/layout/equation.rs            |   9 +-
 01_core/src/compiler/layout/figure.rs              | 240 ++++++++-
 01_core/src/compiler/layout/flush.rs               |   3 +-
 01_core/src/compiler/layout/footnote.rs            | 171 +++++-
 01_core/src/compiler/layout/footnote_flush.rs      |  71 ++-
 01_core/src/compiler/layout/footnote_fragments.rs  |  99 +++-
 01_core/src/compiler/layout/grid/cell_emission.rs  | 167 +++++-
 01_core/src/compiler/layout/grid/tracks.rs         | 114 +++-
 01_core/src/compiler/layout/helpers.rs             |   5 +-
 01_core/src/compiler/layout/image.rs               |  80 ++-
 01_core/src/compiler/layout/measurement.rs         |   4 +-
 01_core/src/compiler/layout/mod.rs                 |   6 +-
 01_core/src/compiler/layout/page_run.rs            |  16 +-
 .../src/compiler/layout/page_running_requests.rs   |  79 ++-
 01_core/src/compiler/layout/paragraph.rs           |  78 ++-
 01_core/src/compiler/layout/paragraph_indent.rs    |   9 +-
 01_core/src/compiler/layout/place.rs               |  67 ++-
 01_core/src/compiler/layout/placement.rs           |   9 +-
 01_core/src/compiler/layout/references.rs          | 141 ++++-
 01_core/src/compiler/layout/region_progress.rs     |  86 ++-
 01_core/src/compiler/layout/repeat.rs              |   7 +-
 01_core/src/compiler/layout/set_page.rs            |  16 +-
 01_core/src/compiler/layout/shape.rs               |   7 +-
 01_core/src/compiler/layout/slicing.rs             |   4 +-
 01_core/src/compiler/layout/stack.rs               |  17 +-
 01_core/src/compiler/layout/sub_frame.rs           |  57 +-
 01_core/src/compiler/layout/text/style.rs          |  11 +-
 01_core/src/compiler/layout/tiling.rs              |  12 +-
 01_core/src/compiler/math/layout/delimited.rs      | 211 +++++++-
 .../src/compiler/math/layout/external_bridge.rs    |  10 +-
 01_core/src/compiler/math/layout/mod.rs            |  10 +-
 01_core/src/compiler/math/layout/op.rs             |  91 +++-
 01_core/src/compiler/realize/figure.rs             |  76 ++-
 01_core/src/compiler/realize/figure_kind.rs        |   6 +-
 01_core/src/compiler/stdlib/figure_image.rs        | 114 +++-
 01_core/src/compiler/stdlib/tests/figure_image.rs  | 282 +++++++++-
 01_core/src/compiler/stdlib/text/constructor.rs    |  20 +-
 01_core/src/compiler/stdlib/visualize.rs           |   7 +-
 01_core/src/entities/content/construction.rs       |   9 +-
 01_core/src/entities/content/fields.rs             |   4 +-
 01_core/src/entities/content/tests/traversal.rs    |   4 +-
 01_core/src/entities/content/traversal.rs          |   4 +-
 01_core/src/entities/elements/figure.rs            |  79 ++-
 01_core/src/entities/elements/image.rs             |  21 +-
 01_core/src/entities/image_format.rs               |   9 +-
 01_core/src/entities/image_sizer.rs                |  13 +-
 01_core/src/entities/layout_types.rs               |   4 +-
 01_core/src/entities/mod.rs                        |   4 +-
 01_core/src/entities/tiling.rs                     |   7 +-
 02_shell/src/cli.rs                                |   4 +-
 03_infra/Cargo.toml                                |   7 +
 03_infra/src/export/builder.rs                     |   2 +-
 03_infra/src/export/builder/resources.rs           |   7 +-
 03_infra/src/export/images.rs                      |   7 +-
 03_infra/src/export/images/png.rs                  |  93 +++-
 03_infra/src/export/images/scan.rs                 |  27 +-
 03_infra/src/export/render.rs                      |  58 +-
 03_infra/src/export/stream/draw.rs                 |   3 +-
 03_infra/src/export/svg/images.rs                  |   8 +-
 03_infra/src/export/svg/tests.rs                   |   4 +-
 03_infra/src/export/tests/mod.rs                   |  15 +-
 03_infra/src/lib.rs                                |   4 +-
 03_infra/src/pipeline/context_stabilization.rs     |   4 +-
 03_infra/src/pipeline/paged.rs                     |   4 +-
 03_infra/src/pipeline/tests.rs                     | 593 ++++++++++++++++++++-
 03_infra/src/shaper/bidi.rs                        |  78 ++-
 03_infra/src/shaper/shaping.rs                     |  14 +-
 03_infra/tests/svg_composite_graph.rs              |   3 +-
 Cargo.lock                                         | 275 +++++++++-
 180 files changed, 7181 insertions(+), 437 deletions(-)
```

</details>
