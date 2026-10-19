# Cache da reconciliação contextual — 2026-10-05

## Resultado

O cache usado no layout contextual agora acompanha o candidato aceito até a reconciliação final. Na tabela de 320 textos distintos, a reconciliação manteve 321 consultas e passou de **321 falhas de cache para zero**. A CLI inteira caiu de **209,10 para 200,08 ms**, **4,31%**, nesta rodada. As duas chamadas de layout foram preservadas.

Esta etapa amplia o limite documentado da anterior: a sessão entregava apenas conteúdo, introspector e documento, descartando suas métricas. O caminho comum já as conservava até a reconciliação; o selecionado permanecia fora daquele escopo. Não atribuir por inferência esse limite a uma decisão por inércia nem tratar toda a frente de memoização como encerrada.

## Medição antes da decisão

Baseline SHA-256 `17201a15e36b06bb3a9ac8adba0a98c61b3dc925c4ce1f118a39d26bce26cf05`, fontes/estado em before-state.json.gz. A sonda original count-initial.json.gz foi executada antes de editar L0/código. Ela isolou a entrada em fix_line_positions_with_metrics e encontrou 321 cálculos após miss na fixture selected_distinct, contra zero no controle comum. Os contadores posteriores à entrada são relativos à compilação inteira; o fluxo não faz outra consulta de métricas após a reconciliação nas fixtures medidas.

`context_stabilization.rs:14–17,660–663` não transportava métricas em Prepared. `paged.rs:337–345,563–572` mantinha None para o caminho selecionado e acionava a fachada de reconciliação com métricas novas. O L0 do owner de reconciliation já permitia empréstimo de métricas aquecidas do mesmo World e compilação. Nenhuma equação de posição foi alterada.

## Implementação e invalidação

L0 primeiro, fluxo contínuo ADR-0127. Prepared continua privado ao módulo pipeline; seu lifetime e a fachada interna prepare explicitam o empréstimo existente de World/Source. O resultado settled passa a transportar também um clone do handle de métricas. Somente candidatos completos já aceitos pela Session o entregam, preservando sua política de budget/não convergência/erros. Resultados sem settled, HTML e bundles não retêm esse handle.

O coordenador usa as métricas selecionadas somente em fix_line_positions_with_metrics. O bidi selecionado permanece com métricas frescas, isolando a contribuição medida; o caminho comum mantém compartilhamento nos dois consumidores. O Núcleo compilation-metric-cache passou a permitir transporte sessão→coordenador dentro da mesma compilação. A retenção acaba no coordenador e não atravessa edições/watch ou compilações.

Nenhuma API pública, campo L1, flag ou fase mudou. Nenhum unsafe, cache global ou memoização de Content/frames/contexto foi acrescentado. A contribuição aceita continua isolada: métricas não certificam identidade ou validade de snapshots.

## Contagens no release

GDB por símbolos do próprio ELF, fora do benchmark, sem instrumentação produtiva. A contagem cobre queries, closure de cálculo e chamadas do wrapper. Fixtures sem tracking (o bypass exigiria classificação distinta). As consultas totais também permaneceram iguais.

| Fixture | Consultas na reconciliação | Cálculos na reconciliação | Cálculos totais | Layouts |
| --- | ---: | ---: | ---: | ---: |
| selected_distinct | 321 → 321 | 321 → 0 | 643 → 322 | 2 → 2 |
| selected_repeated | 321 → 321 | 2 → 0 | 5 → 3 | 2 → 2 |
| ordinary_control | 320 → 320 | 0 → 0 | 321 → 321 | 1 → 1 |
| page_control | 323 → 323 | 0 → 0 | 324 → 324 | 3 → 3 |
| selected_changing_style | 320 → 320 | 320 → 0 | 641 → 321 | 2 → 2 |
| selected_numbered | 321 → 321 | 321 → 0 | 646 → 325 | 2 → 2 |

Nos três casos selecionados de texto distinto/numeração/controle condicional de fonte, FallbackFontMetrics::new passou de 10 chamadas para 9, eliminando a instância da reconciliação. Controles comum e Page conservaram suas contagens. O texto repetido tinha apenas duas falhas nessa etapa e não demonstrou benefício relevante de CLI. Não interpretar a redução total de misses como redução de travessias ou construção de frames.

## A/B de tempo e memória

Uma rodada de aquecimento e nove amostras por fixture/motor, intercaladas na mesma CPU, RAYON_NUM_THREADS=1, SOURCE_DATE_EPOCH=0, PDF sem tags e pretty=false. Sem builds, testes ou GDB concorrentes. Inclui inicialização/fontes, avaliação, layout, shaping, exportação e escrita. Medianas sem teste estatístico formal.

| Fixture | Antes, ms | Depois, ms | Variação | Pico RSS, MiB |
| --- | ---: | ---: | ---: | ---: |
| selected_distinct | 209.10 | 200.08 | -4.31% | 42.97 → 42.97 |
| selected_repeated | 184.35 | 185.77 | 0.77% | 42.79 → 42.74 |
| ordinary_control | 189.27 | 188.27 | -0.53% | 42.68 → 42.75 |
| page_control | 211.02 | 212.33 | 0.62% | 43.12 → 43.20 |
| selected_changing_style | 206.86 | 198.60 | -3.99% | 42.96 → 42.93 |
| selected_numbered | 207.21 | 198.37 | -4.27% | 43.19 → 43.31 |

Para selected_distinct, faixas de parede 204.93–210.71 ms antes e 196.62–202.28 depois. Shape_ms, que inclui bidi/shaping/reconciliação, passou de 18,41 para 9,69 ms nessa fixture. Não somar medianas das fases nem atribuir todos os custos desse campo à reconciliação. O baseline já contém as melhorias anteriores; este delta mede apenas a nova retenção.

Os controles repetido, comum e Page tiveram variações pequenas. Não afirmar ganho universal ou redução de memória total; o RSS é uma medição do processo inteiro. Os recursos de watch continuam fora desta mudança.

## Verificação

RED: a sentinela encontrou uma instância fresca no braço de reconciliação quando exigia zero. GREEN com transporte interno e manutenção do texto/números lógicos. O controle sem produtores conserva settled=None e não retém cache de layout. Sondas somente cfg(test).

**1156 testes L3 completos passaram** com SOURCE_DATE_EPOCH=0. O núcleo não foi alterado nem sua suíte contada novamente. Build release passou. Oito controles CLI conservaram texto, caixas de palavras/dimensões e rasters a 36 dpi entre baseline e candidato, incluindo numeração, matemática/callback e RTL. Seus textos coincidiram com vanilla pinado; não é prova universal de geometria com upstream. Dois controles de erro conservaram returncodes/diagnósticos no A/B. Ver fontes, comandos e resultados completos em verification.json.gz.

Linter sem erros, 534 warnings e 1574 notes herdados, sem findings novos frente ao SARIF anterior. V5/V6/V15/V26 dos owners tocados passam; não afirmar zero violations global nem ressellar dívida externa. Coverage::from_ranges permanece pendente.

## Proveniência e continuação

HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, working tree não commitado, com todas as etapas anteriores. before-state/red-state/benchmark/execution preservam a lista exata de alterações, diff stat, fontes e UTC. Scope: sete arquivos existentes (três consumers, seus L0 e o Núcleo compartilhado), nenhuma mudança produtiva fora desses owners.

- Candidato SHA-256 `c35dc34f0f24466b0b7339fcb856d5fc59c6f8701308f54963a4a1704f7746b8`.
- Vanilla pin `9f2b6e8715237cb086899a42873660fe744622e8`, SHA-256 `7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea`.
- Benchmark UTC `2026-10-05T21:46:08.557854+00:00` a `2026-10-05T21:46:34.666319+00:00`.

Recibos/receitas em [contexto-cache-reconciliacao-2026-10-05](contexto-cache-reconciliacao-2026-10-05/). Próximo ponto de investigação: o bidi selecionado permanece fresco e os caminhos Page/contexto legado conservam ciclos próprios. Medir cada contribuição antes de ampliar retenção ou encurtar ciclos. Watch, memoização de frames e aquisição/metadados duráveis continuam frentes distintas.
