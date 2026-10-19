# Primeira bateria de microbenchmarks de fontes

A bateria foi implementada e executada: **56 casos, quatro fontes, três rodadas e 30 amostras Criterion por caso/rodada**. Os três blocos passaram nos controles de integridade, sem mudanças de inputs/consumers/HEAD/executável ou builds/testes/benchmarks concorrentes detectados. Contudo, **55 casos falharam nos critérios prévios de estabilidade absoluta**. Portanto esta coleta é exploratória, não uma baseline estável de custos absolutos.

O construtor atual por intervalos foi mais rápido que o algoritmo histórico nas 12 comparações fonte/rodada, com fatores observados de **1,52× a 3,32×**. Essa direção reforça a explicação mecânica da [experiência anterior](font-lifecycle-experiment.md), mas a variação impede declarar um fator fixo. O histórico aqui é uma reprodução do algoritmo extraído, não medição do crate histórico inteiro nem do vanilla. Os dados não fecham a causa do gap restante para vanilla nem foram somados aos tempos CLI.

## Implementação, fontes e intervalos

Código e scripts ficam exclusivamente em [font-microbenchmarks/](font-microbenchmarks/). Não houve alteração de produção, L0, ADR, núcleo compartilhado ou benchmark existente. Os dados da baseline e da experiência anterior foram preservados. Foram formatados somente main.rs/build.rs novos, sem formatação global; o source histórico extraído permanece byte a byte conforme seu controle.

O crate diagnóstico independente chama as APIs públicas existentes `Coverage::from_codepoints`, `Coverage::contains`, `font_info_from_bytes` e `FontSlot::get`. A cmap é percorrida por um helper diagnóstico com a mesma ordem e preservação de duplicatas do consumer `03_infra/src/fonts.rs:403`; não se apresenta esse helper como medição da função produtiva privada `extract_coverage`. Nenhuma visibilidade produtiva foi ampliada.

`src/historical.rs` contém o trecho exato de Coverage (derive/struct/impl/default/debug) do snapshot anterior, sem headers de consumer produtivo. [historical-provenance.json](font-microbenchmarks/historical-provenance.json) identifica origem, hashes e limite da comparação. As duas versões são compiladas no novo harness/toolchain; diferenças da compilação/linkagem histórica completa não são representadas. O construtor público atual continua sendo o código produtivo.

[inputs.json](font-microbenchmarks/inputs.json) fixa bytes por SHA-256, caminho e índice zero de face de Libertinus Serif, New Computer Modern Math, Noto Sans CJK e Noto Sans Math. O CJK vem de TTC; os demais são fontes simples. A seleção utiliza os mesmos hashes congelados da experiência anterior. [fixture-controls.json](font-microbenchmarks/fixture-controls.json) registra formatos Unicode, sequência little-endian/hash, tamanho coletado/distinto, hash da codificação canônica e pontos de consulta. Os pares antigos/atuais passaram contra BTreeSet independente em membership e iteração, inclusive pontos fora do conjunto. Metadados têm cobertura vazia, conforme seu contrato; todos os slots são válidos.

O [protocolo](font-microbenchmarks/protocol.md) foi escrito antes da medição. Construtor exclui leitura de fontes, decoding da cmap e clone do Vec de setup; inclui sua construção interna e consumo/desalocação do Vec recebido. clone_and_build inclui clone. Coleta exclui parse da face e I/O. Metadata pública inclui parse e extração de metadata, sem coverage. Primeiro slot inclui tudo que get realiza, mas criação do slot e destruição do slot/Font retornado ficam fora pelo batching por referência. Slot retido inclui consulta/clonagem do handle, com destruição retornada fora. Contains tem loop/black_box e bool trivial; controles bool e batched unit indicam overhead, sem subtração cega.

A pré-validação, warmup e fingerprints aquecem bytes e filesystem. Um slot fresco **não** representa disco frio. As queries são pontos fixos em início/meio/fim, gaps e fora do conjunto; não representam por si uma distribuição real de fallback. A criação de fixtures, clones e destruição externos ao timer ainda afetam caches/allocator/frequência do processo. Os parâmetros são iguais entre comparadores, mas a ordem por rodada não equivale a pares intercalados por chamada.

## Proveniência e validação

HEAD: `6ef8b6986e74de7d5cc8f7b7e009868fc0daa5f5`, working tree não commitado. Fontes/inputs congelados: `d56bf20513fa733a4f8b1606103f3c1d9f9550687f164908c7b6bd61a0c6d4cf`. O [frozen-state.json](font-microbenchmarks/frozen-state.json), listas dirty/untracked, patches e diff stats em `state/` registram a árvore exata, incluindo alterações existentes. [build-provenance.json](font-microbenchmarks/build-provenance.json) registra máquina, toolchain, flags e identidade do executável; receitas exatas estão em `state/build-receipt.json` e `state/existing-benches-build-receipt.json`.

Executável: SHA-256 `fa572544608e9e71a369cb29720489dd7aa07cea2ab9c5e042bacf6a5723ea4c`. Cargo release opt-level 3, debuginfo 0, debug assertions/overflow checks false; flags Rust e overrides de perfil limpos pelo preparador. Toolchain rustc/Cargo 1.92.0, Linux 6.17.0-42-generic, Dell G15 AMD Ryzen 7 5800H. Executável na CPU 4, driver na CPU 6, Rayon um. Powersave e boost ativos; sem travar clock ou alterar política global. Critérios e limitações não foram mudados após observar os resultados.

A coleta ocorreu de **2026-09-27T12:42:30.037415+00:00 a 2026-09-27T12:46:16.290283+00:00**, UTC. Três processos frescos, ordem forward/reverse/forward, warmup 200 ms, medição alvo 600 ms, 30 amostras e 5000 bootstrap resamples por caso. Cada rodada teve gate de 30 segundos e monitor de processos a cada 100 ms. A detecção é observacional/heurística; não isola desktop/SO ou garante ausência de processos breves entre polls. A integridade dos três receipts não prova repetibilidade estatística.

O load médio de um minuto observado durante as rodadas variou aproximadamente 10–13 numa máquina de 16 CPUs lógicas. Leituras pontuais de frequência da CPU 4 variaram de aproximadamente 2,84 a 3,74 GHz antes/depois das rodadas. Não existe série por caso que atribua sua variação a clock, thermal, outro processo ou estado de caches. Não se declara throttling ou background load como causa demonstrada do ruído.

Uma compilação diagnóstica com debug assertions explicitamente ativadas foi executada e recusou antes da leitura de fixtures/Criterion, conforme [debug-guard.json](font-microbenchmarks/debug-guard.json). Essa sonda também demonstra por que JSON do perfil Cargo sozinho não descreve um override explícito de rustc; os builds suportados limpam flags e usam o comando fixo. O release final passou na pré-validação.

`cargo bench --manifest-path benches/Cargo.toml --no-run --offline --locked --message-format=json` compilou os oito targets existentes; perfis otimizados sem debug assertions e hashes estão em [benchmark-build-profiles.json](font-microbenchmarks/benchmark-build-profiles.json). Observation/stdlib e a classificação de stream como exportação completa permaneceram intactos. Não foram executados testes/builds junto das rodadas, nem coletados novos números desses benchmarks existentes.

O Cargo.lock diagnóstico foi inicialmente resolvido offline sem seed, mas requisitou zerocopy-derive não disponível. A preparação foi corrigida usando o lockfile recebido da raiz como seed para conservar versões cached; nenhum download ou alteração do lockfile raiz foi necessário. Essa falha de preparação não produziu amostras. Todos os comandos e falhas de integridade futuras devem ser separados das populações aceitas.

## Resultados descritivos

Médias dos 30 valores `tempo / iterações` por rodada, **em µs**. Cada amostra tem seu próprio número de iterações. A razão é entre médias sequenciais da mesma fonte/rodada, não uma estimativa pareada intercalada:

| Fonte | Codepoints coletados / distintos | Atual µs por rodada | Histórico extraído µs por rodada | Histórico / atual por rodada |
| --- | ---: | --- | --- | --- |
| LibertinusSerif-Regular | 9522 / 2382 | 40.997 / 32.162 / 49.570 | 62.220 / 84.271 / 90.904 | 1.52× / 2.62× / 1.83× |
| NewCMMath-Regular | 16768 / 4913 | 49.268 / 60.369 / 50.191 | 112.020 / 148.654 / 124.627 | 2.27× / 2.46× / 2.48× |
| NotoSansCJK-Regular | 174060 / 44810 | 417.801 / 694.719 / 423.723 | 1329.238 / 1600.096 / 1406.994 | 3.18× / 2.30× / 3.32× |
| NotoSansMath-Regular | 7610 / 2472 | 25.639 / 22.381 / 25.555 | 59.016 / 48.538 / 51.840 | 2.30× / 2.17× / 2.03× |

Nenhum dos dois construtores de nenhuma fonte passou nos dois critérios de estabilidade absoluta. Noto CJK atual variou de aproximadamente 418 a 695 µs entre rodadas; seu algoritmo histórico, de 1329 a 1600 µs. Tanto a magnitude do benefício quanto o custo absoluto dependem de condições não controladas suficientemente nesta coleta.

Para orientação de novas experiências, a tabela seguinte publica a **mediana das três médias de rodada**, também em µs. Todos esses componentes falharam na estabilidade e ficam descritivos:

| Fonte | Coleta diagnóstica cmap | Metadata pública | Primeiro slot em filesystem aquecido | Slot retido |
| --- | ---: | ---: | ---: | ---: |
| LibertinusSerif-Regular | 17.309 | 2.711 | 34.112 | 0.033 |
| NewCMMath-Regular | 24.166 | 1.737 | 46.723 | 0.043 |
| NotoSansCJK-Regular | 456.523 | 2.283 | 29129.552 | 0.039 |
| NotoSansMath-Regular | 18.349 | 1.704 | 24.821 | 0.041 |

A única combinação que passou foi Noto CJK/contains/hit_late: médias de aproximadamente 2,926 / 2,828 / 2,854 µs, range 3,41%. Não se extrapola esse ponto fixo para todas as consultas ou para custo do catálogo inteiro. O controle de batched unit de Libertinus variou de aproximadamente 28 a 36 ns; isso limita inferências sobre chamadas retidas muito curtas, mas não autoriza descontar o controle de outra operação.

A classificação aplica diferença entre média/mediana das médias de rodada <3% (critério descritivo ADR-0115) **e** range das médias dividido pela sua mediana <=5%, conforme protocolo. As falhas e todas as distribuições estão em [summary.json](font-microbenchmarks/summary.json). `rounds/r*/criterion/**/new/sample.json` preserva iterações e ns; estimates.json preserva estimates e intervalos Criterion. Stdout/stderr, ordem, source fingerprints e telemetria também foram retidos. Nenhuma rodada foi apagada ou omitida para passar um critério.

## O que o primeiro carregamento CJK compõe

`03_infra/src/fonts.rs:153` mostra que `FontSlot::get` chama `extract_collection_face` para coleções. `:201` e o owner [infra/fonts.md](../prompts/infra/fonts.md) determinam reconstrução do header/diretório sfnt, cópia das tabelas, alinhamento, offsets e recomputação de checksums. Portanto os cerca de 27–31 ms observados no primeiro slot TTC não são um custo isolado de mmap, cmap, parser, coverage ou discovery. FontSlot retido não repete essa composição.

A presença da reconstrução nessa chamada está demonstrada pelo caminho produtivo/fixture. A parcela de tempo pertencente a ela, ao checksum ou à cópia **não foi isolada**. Uma experiência controlada deve comparar a mesma face na TTC e numa sfnt previamente reconstruída via resultado público de get, validar coverage/metadata/forma de glifos e depois comparar primeiro get completo com armazenamento aquecido. Essa intervenção troca o container e também tamanho/acesso aos bytes; sua diferença ainda não seria uma medição direta da função privada. Nenhuma API deve ser ampliada para medir aquele helper.

## Continuação controlada

O próximo protocolo deve reduzir confundimento de ordem e condições do host antes de fechar números absolutos: escolher uma janela com CPU física e sibling observados ociosos, manter a mesma configuração entre variantes, captar CPU-time/context switches/frequência por caso fora do timer e intercalar old/current com ordem balanceada no mesmo input. Repetir um conjunto completo de rodadas em outro diretório; manter esta coleta, sem reescrever seus critérios ou escolher somente a rodada rápida.

Também permanecem as baterias de contexto de aquisição equivalente (para a diferença entre probe e CLI), consultas de book com carga real, shaping público com caches declarados e compilação repetida com World retido. A suíte entregue começa por coverage/acquisition e não afirma que todo o [plano](font-lifecycle-experiment/microbench-plan.md) já foi medido. Construção de cobertura, metadata e primeiro slot têm fixtures/intervalos distintos: **não somar seus resultados como decomposição end-to-end**. Nenhuma otimização produtiva foi feita nesta tarefa.
