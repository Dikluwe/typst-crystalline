# Microbenchmark dos seletores de parágrafo com 1.000 palavras

## Proveniência

- Medição concluída em 2026-10-01 08:40:57.914500640 -0300, horário do log `/tmp/tekt-paragraph-microbench-1000.log`.
- Estado durante a medição: HEAD `a30ba7fc0932d6e3453864c95759d60af088eed4`, working tree não commitado. `git diff HEAD --stat`: `00_nucleo/prompts/compiler/layout/paragraph_select_bounded.md | 6 +++++-`; `01_core/src/compiler/layout/paragraph_select_bounded.rs | 5 ++++-`; total `2 files changed, 9 insertions(+), 2 deletions(-)`. Não havia arquivos não rastreados; a árvore não mudou durante a execução.
- Comando: `cargo test --release -p typst-core microbench_exact_vs_bounded --lib -- --ignored --nocapture`.
- Ambiente: Linux x86_64, AMD Ryzen 7 5800H, rustc 1.92.0, Criterion 0.5. Para cada variante: 30 amostras, 1 s de aquecimento e 2 s de medição.
- Entradas: 1.000 palavras com largura determinística `((i * 17) % 19 + 9)` pt, gaps 3pt, tracking zero, fonte 12pt, custo default. Mesmo `Prepared` reutilizado por variante. Equivalência dos ranges verificada antes de medir. Medição inclui construção do limite guloso e seleção inteira; exclui construção do `Prepared`.

## Resultados locais

| Caso | Largura | Justificado | Exato (µs, intervalo Criterion) | Com poda (µs, intervalo Criterion) | Exato/com poda | Avaliados/podados pelo protótipo |
|---|---:|:---:|---:|---:|---:|---:|
| large_narrow | 76pt | não | 56.524 [56.159, 57.084] | 84.918 [84.058, 86.027] | 0.67× | 4253 / 3 |
| large_medium | 240pt | não | 133.14 [132.56, 133.90] | 179.86 [178.87, 180.96] | 0.74× | 10300 / 1790 |
| large_wide_justified | 1200pt | sim | 565.24 [562.89, 567.68] | 380.57 [379.34, 381.89] | 1.49× | 3139 / 55685 |

## Interpretação e limite

A poda não escala monotonamente com o número de palavras. Em 76pt quase nada é podado; em 240pt a economia é insuficiente para amortizar a passada gulosa e a medição/checagem extra por candidato. Em 1200pt, muitos custos de linha são evitados e o protótipo vence nesta entrada. O mecanismo exato de custo relativo é inferência a partir das contagens, não perfil de CPU. Não extrapolar para parágrafos reais com outras distribuições, hifenização ou compilação completa; nenhuma troca de default decorre deste ensaio. Os casos de 40/160 palavras estão no relatório de 2026-09-30.
