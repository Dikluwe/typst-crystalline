# Ablação do seletor de linhas forward — 2026-10-02

## Proveniência

- Estado da medição: working tree não commitado sobre `787e02591e75eb8bfb078c32772167483fb00f4b`.
- Captura durante a compilação da rodada, `2026-10-02 18:37:03 UTC`: `git diff HEAD --stat` indicava somente `00_nucleo/prompts/compiler/layout/paragraph_select_forward.md` (10 linhas alteradas) e `01_core/src/compiler/layout/paragraph_select_forward.rs` (165 linhas alteradas), 137 inserções e 38 remoções no total. A árvore não mudou até o fim da medição. O relatório e o resselo foram feitos depois.
- Comando: `cargo test --release -p typst-core microbench_forward_vs_exact --lib -- --ignored --nocapture`. Criterion: 30 amostras, aquecimento de 1 s, medição de 2 s por variante. Log bruto local: `/tmp/tekt-paragraph-forward-prefix.log`.
- As quatro variantes receberam o mesmo `Prepared`, largura, custos e modo de justificação; o teste exigiu igualdade dos ranges com o seletor exato antes de cada grupo. Os casos são sintéticos e não representam toda a composição de parágrafos.

## Resultado

Mediana em µs; menor é melhor. Intervalos de confiança completos estão no log bruto.

| Caso | Exato | Forward completo | Só Simple | Simple prefixado | Exato / prefixado |
|---|---:|---:|---:|---:|---:|
| 40 palavras, 76pt | 2,110 | 3,698 | 2,503 | 2,146 | 0,98× |
| 160 palavras, 240pt | 19,772 | 13,127 | 10,062 | 7,571 | 2,61× |
| 1.000 palavras, 240pt | 119,28 | 170,97 | 143,53 | 132,62 | 0,90× |
| 1.000 palavras, 1.200pt, justificado | 509,96 | 148,37 | 129,90 | 52,866 | 9,65× |

As sementes packed/balanced nunca deram o limite escolhido nesses quatro casos (`bound_source=simple`). Removê-las melhorou os quatro cenários. Reutilizar as medidas prefixadas para montar a partição Simple melhorou novamente os quatro, sobretudo o caso largo: a função produtiva `Simple` recompõe cada intervalo a cada tentativa de acrescentar palavra, enquanto o protótipo já possui prefixos para consultar o mesmo intervalo em O(1).

## Decisão

Manter as variantes sob `cfg(test)` para pesquisa. O ganho depende da largura e da quantidade de candidatos: a variante prefixada ainda é cerca de 11% mais lenta no caso de 1.000 palavras a 240pt e empata aproximadamente no curto. Antes de propor uma seleção automática em produção, medir parágrafos reais, casos com hifenização e uma regra de escolha que não acrescente custo maior que o ganho. Os resultados comprovam potencial para entradas largas, não ganho geral nem paridade completa com o vanilla.
