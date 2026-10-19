# Compartilhamento de métricas na sessão contextual — 2026-10-05

## Resultado e classificação

A sessão contextual selecionada passou a fornecer seu handle de métricas às tentativas de layout. Na tabela com 320 textos distintos, as consultas permaneceram iguais, mas os cálculos após miss caíram de **965 para 643**, mantendo duas chamadas de layout. A CLI caiu de **215,59 para 205,43 ms**, **4,71%**, na rodada final. Não se memoizou execução contextual, Content, posições ou frames.

Isto amplia um limite documentado do estudo [grid-cache-share-ab-2026-10-03.md](grid-cache-share-ab-2026-10-03.md): ele compartilhou métricas no caminho comum e explicitamente deixou os wrappers contextuais/bundle frescos. Não atribuir esse limite por conveniência à construção por inércia. A origem mais antiga do wrapper fresco não foi auditada nesta etapa. O trabalho pertence às frentes C/F: recuperar reutilização e delimitar sua invalidação, com prova mensurável.

## Medição antes da decisão e implementação

`context_stabilization.rs` usava o wrapper sem métricas; o ramo `shared_metrics=None` em `paged.rs` criava FallbackFontMetrics a cada passagem. A sessão já possuía seu próprio handle. A inspeção dos owners confirmou que Clone compartilha caches Arc no mesmo World e que os algoritmos/chaves permanecem no domínio de fontes. `before-state.json.gz` preserva fontes e binário baseline. A primeira sonda precedeu a decisão L0; `count-baseline.json.gz` é a repetição final contra esse mesmo binário preservado, que reproduziu os números e acrescentou dois controles. Não confundir sua data posterior com uma captura anterior da árvore candidata.

Os L0 dos dois coordenadores e testes foram editados antes do código. Núcleo `compilation-metric-cache.toml` declara lifetime por compilação/World e isolamento dos candidatos; os owners o consomem com digest canônico pinado. Fluxo contínuo ADR-0127: nenhuma API pública, default ou fase mudou. Apenas o helper interno com métricas ganhou visibilidade pub(super). O wrapper privado sem consumidor e seu import foram removidos, com L0 próprio do pipeline atualizado primeiro.

O handle dura somente a sessão. Contextos, regiões, snapshots, documentos, stores de callbacks, sinks e orçamento continuam próprios de cada tentativa. Reconciliação final selecionada, bundles, HTML e edições watch conservam seus recursos atuais. Nenhum unsafe ou estado global produtivo foi acrescentado.

## Contagem no release

Sonda GDB executada fora do benchmark, usando símbolos do próprio ELF porque este build não contém linhas de debug. Conta consultas a cached_shaped_width, execução da closure de cálculo, entradas no wrapper e FallbackFontMetrics::new. As fixtures medidas não usam tracking, cujo bypass exigiria interpretação distinta. A medição cobre a compilação inteira, incluindo etapas posteriores ao layout.

| Fixture | Consultas | Cálculos após miss | Wrappers de layout | Instâncias de métricas |
| --- | ---: | ---: | ---: | ---: |
| selected_distinct | 2243 → 2243 | 965 → 643 | 2 → 2 | 12 → 10 |
| selected_repeated | 2243 → 2243 | 8 → 5 | 2 → 2 | 12 → 10 |
| ordinary_control | 1280 → 1280 | 321 → 321 | 1 → 1 | 7 → 7 |
| page_control | 3212 → 3212 | 324 → 324 | 3 → 3 | 15 → 15 |
| selected_changing_style | 2240 → 2240 | 962 → 641 | 2 → 2 | 12 → 10 |
| selected_numbered | 2247 → 2247 | 968 → 646 | 2 → 2 | 12 → 10 |

As duas instâncias de métricas evitadas correspondem às duas chamadas selecionadas agora alimentadas pelo handle existente. O restante do pipeline ainda apresenta trabalho separado; não interpretar 643 como somente o custo do primeiro layout, nem afirmar que todas as falhas de cache foram eliminadas.

## A/B de CLI inteira

Uma rodada de aquecimento e nove amostras intercaladas por fixture/motor, mesma CPU, RAYON_NUM_THREADS=1, SOURCE_DATE_EPOCH=0, PDF sem tags/pretty. Inclui fontes, avaliação, layout, shaping, exportação e escrita. Não houve builds, testes ou GDB concorrentes. Medianas, sem teste estatístico formal.

| Fixture | Antes, ms | Depois, ms | Variação | Pico RSS, MiB |
| --- | ---: | ---: | ---: | ---: |
| selected_distinct | 215.59 | 205.43 | -4.71% | 42.79 → 42.82 |
| selected_repeated | 184.80 | 184.15 | -0.35% | 42.66 → 42.78 |
| ordinary_control | 186.78 | 186.34 | -0.24% | 42.79 → 42.67 |
| page_control | 209.42 | 208.78 | -0.31% | 43.11 → 43.30 |
| selected_changing_style | 214.95 | 206.36 | -3.99% | 42.88 → 42.99 |
| selected_numbered | 215.05 | 206.53 | -3.96% | 43.29 → 43.35 |

Para selected_distinct, as faixas foram 213,26–217,28 ms antes e 203,68–207,61 depois, separadas nessa rodada. O controle condicional de tamanho de fonte também teve faixas separadas: 212,95–216,86 e 205,00–208,74. Com numbering houve pequena sobreposição. Uma rodada anterior à remoção do wrapper privado já apresentava redução de 8–9 ms nos três casos distintos; seu recibo foi preservado em bench-pre-cleanup.json.gz. Não usar essa rodada como o binário final.

Textos repetidos e controles ordinário/Page não demonstraram benefício relevante. O RSS permaneceu próximo nas amostras, sem evidência de redução geral de memória. Não extrapolar os ganhos para todo contexto ou para watch.

A instrumentação existente contabiliza o layout selecionado dentro de expand_context_ms: selected_distinct passou de 39,56 para 30,62 ms nessa fase. Seu layout_ms isolado ficou perto de zero porque prepared.settled já traz o documento diagramado. Não tratar esse campo como uma nova medição do layout real nem somar medianas para explicar a CLI.

## Verificação e limites

RED: a nova sentinela encontrou duas alocações frescas de métricas no wrapper, quando exigia zero. GREEN: o mesmo programa conserva conteúdo e vinculação de páginas e deixa de criar caches independentes. Os testes usam apenas uma sonda cfg(test), sem instrumentação produtiva.

**1155 testes completos L3 passaram** com SOURCE_DATE_EPOCH=0. O núcleo não mudou nesta etapa; sua suíte anterior não é contabilizada como execução nova. A execução inicial apresentou uma falha num teste antigo de bytes PDF: os arrays diferiam nos timestamps CreationDate/ModDate de 21:13:27 para 21:13:28, e nos campos derivados. O log foi preservado; timestamp fixo eliminou a variação sem alterar exportação ou enfraquecer o teste.

Sete controles CLI compilaram nos dois binários Tekt e no vanilla pinado, incluindo callback math.cancel. Baseline/candidato conservaram texto, caixas de palavras, dimensões e hashes de rasters a 36 dpi; os textos também coincidiram com vanilla. Dois controles de erro conservaram returncode e diagnóstico entre baseline/candidato, incluindo erro independente e panic no callback de numeração. Isso valida as fixtures medidas, não paridade universal de geometria com vanilla.

Build release passou. Linter sem erros: 534 warnings e 1574 notes herdados, nenhum finding novo em relação ao SARIF da etapa anterior; V5/V6/V15/V26 dos quatro owners desta etapa passam. Não afirmar zero violations global nem ressellar outros owners. Coverage::from_ranges permanece pendente.

## Proveniência

HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, working tree não commitado com as etapas anteriores. before-state, red-state, benchmark e execution registram lista exata de alterações, diff stat, fontes/hashes e UTC. scope-review mostra oito arquivos existentes alterados e um Núcleo novo, sem mudança produtiva fora dos quatro pares L0/consumer.

- Baseline SHA-256 `5bc45f4ec0972c6c46450c0cab3479e2ae2ef41c286c424201c1b7fc07823eb1`.
- Candidato SHA-256 `17201a15e36b06bb3a9ac8adba0a98c61b3dc925c4ce1f118a39d26bce26cf05`.
- Vanilla pin `9f2b6e8715237cb086899a42873660fe744622e8`, SHA-256 `7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea`.
- Benchmark UTC `2026-10-05T21:19:45.340482+00:00` a `2026-10-05T21:20:11.819937+00:00`.

Receitas e recibos em [contexto-cache-fontes-2026-10-05](contexto-cache-fontes-2026-10-05/): bench.py, count.py, verify.py, receipts.py, fontes/estado inicial/RED, contagens, A/B, geometria e execução. Próximo candidato de investigação: a reconciliação final selecionada ainda cria outro cache; é necessário medir sua contribuição e lifetime antes de estender a retenção. Memoização de frames e recursos entre edições continuam frentes separadas.
