# Protótipo: início fixo e fim variável na seleção de linhas

## Proveniência

- Medição concluída em 2026-10-02 15:18:55.146589449 -0300, mtime do log `/tmp/tekt-paragraph-forward-bench.log`.
- Estado medido: HEAD `e0abf217f827e2ec9b098e0ce8dd7c5c9812be57`, working tree não commitado. `git diff HEAD --stat`: `00_nucleo/prompts/compiler/layout.md | 6 +++++-`, `01_core/src/compiler/layout/mod.rs | 4 +++-`, total `2 files changed, 8 insertions(+), 2 deletions(-)`. Não rastreados na medição: `00_nucleo/prompts/compiler/layout/paragraph_select_forward.md`, `01_core/src/compiler/layout/paragraph_select_forward.rs`. Nenhum outro arquivo modificado; a árvore ficou estável durante a execução.
- Comando: `cargo test --release -p typst-core microbench_forward_vs_exact --lib -- --ignored --nocapture`.
- Ambiente: Linux x86_64, AMD Ryzen 7 5800H, 16 CPUs lógicas, rustc/cargo 1.92.0, Criterion 0.5. Cada variante: 30 amostras, 1 s de aquecimento e 2 s de medição.
- Entradas: palavras sintéticas de largura `((i * 17) % 19 + 9)` pt, gap 3pt, tracking zero, fonte 12pt, custo default. Mesmo `Prepared` por par. Ranges idênticos entre seletores confirmados antes do tempo. A medição inclui prefixos, dois candidatos iniciais, busca, alocação e retrace; exclui construção do `Prepared`.

## Resultado local

| Caso | Palavras | Largura | Justificado | Exato (µs, intervalo Criterion) | Início fixo (µs, intervalo Criterion) | Exato/início fixo | Candidatos avaliados / inícios podados | Melhor limite inicial |
|---|---:|---:|:---:|---:|---:|---:|---:|---|
| short_narrow | 40 | 76pt | não | 2.1050 [2.0870, 2.1279] | 3.5386 [3.5200, 3.5613] | 0.59× | 94 / 16 | Simple |
| medium | 160 | 240pt | não | 19.329 [19.231, 19.432] | 13.002 [12.949, 13.067] | 1.49× | 343 / 131 | Simple |
| large_medium | 1000 | 240pt | não | 118.36 [117.99, 118.77] | 164.90 [164.25, 165.79] | 0.72× | 10300 / 149 | Simple |
| large_wide_justified | 1000 | 1200pt | sim | 515.49 [511.91, 519.21] | 148.68 [147.52, 150.41] | 3.47× | 3139 / 947 | Simple |

## Interpretação e limites

Fixar o início e percorrer finais permitiu pular todo o vetor de finais de um início quando seu custo parcial já excedia o limite superior. O ganho foi grande no caso largo com 1.000 palavras, moderado no caso médio com 160 e inexistente nos outros dois. As composições novas “mais cheia” e “equilibrada” não melhoraram o limite guloso Simple em nenhum dos quatro casos; consumiram tempo, portanto não há evidência de vantagem desses dois candidatos neste corpus. A atribuição exata do ganho entre ordem do laço, poda por início e efeitos de cache requer ablação/perfil separado. Não alterar produção com base nesses dados. O protótipo não inclui hifenização, UAX14 completo, largura variável, compilação end-to-end nem corpus de linguagem real. A suíte L1 após a medição passou com 6.492 testes e 4 ignorados.
